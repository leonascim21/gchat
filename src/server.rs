mod state;
mod routes;
mod utils;

use axum::extract::ws::WebSocket;
use axum::http::header;
use axum::http::{HeaderValue, Method};
use axum::extract::{Form, Path};
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use axum::Extension;
use axum::{
    extract::{ws::Message, Query, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::Router,
    Json,
};

use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use gauth::models::{Auth, Claims, User};
use gauth::validate_token;
use routes::group;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use state::ServerState;
use uuid::Uuid;

use std::sync::Arc;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::SystemTime;

use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc, Mutex};

use tower_http::cors::CorsLayer;
use utils::types::{LoginForm, RegisterForm};
use utils::queries::is_user_in_group;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let auth = Auth::new(db_url.clone())
        .await
        .expect("Failed to create auth instance");
    let auth = std::sync::Arc::new(auth);

    // DB connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool");

    // Shared DB state
    let state = ServerState {
        db: pool,
        channels: Arc::new(Mutex::new(HashMap::new()))
    };

    let state = std::sync::Arc::new(state);

    // Configure CORS with proper origin matching
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "https://www.gchat.cloud".parse::<HeaderValue>().unwrap(),
            "https://gchat.cloud".parse::<HeaderValue>().unwrap(),
            "https://api.gchat.cloud".parse::<HeaderValue>().unwrap(),
            "http://www.gchat.cloud".parse::<HeaderValue>().unwrap(),
            "http://gchat.cloud".parse::<HeaderValue>().unwrap(),
            "http://api.gchat.cloud".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])        
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .allow_credentials(true);

    let app = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .route("/ws/group/:group_id", get(ws_handler))
        .route(
            "/login",
            get(|| async { Html(include_str!("../templates/login.html")) }),
        )
        .route("/login", post(handle_login))
        .route("/get-group-messages", get(get_group_messages))
        .route("/register", post(handle_registration))
        .route("/check-token", get(check_token))
        .route("/get-user-info", get(get_user_info))
        //.nofollow.route("/api/validate-token", get(validate_token_handler))
        //.route("/api/me", get(get_user_info))
        .route("/api/logout", post(handle_logout))
        .nest("/", routes::app_routes().with_state(state.clone()))
        .layer(cors)
        .layer(Extension(auth))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_group_messages(
    State(state): State<Arc<ServerState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token = params.get("token");
    if token.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Missing token" })),
        )
            .into_response();
    }

    let group_id = params.get("group_id");
    if group_id.is_none() {

    }
    let group_id = match group_id {
        Some(group_id) => group_id.parse::<i32>().unwrap(),
        None => {
            return (
                StatusCode::UNAUTHORIZED, Json(json!({ "error": "Missing group id" })),
            ).into_response();
        }
    };
    


    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    let user_id = match validate_token(token.unwrap(), jwt_key).await {
        Ok(claims) => claims.sub.parse::<i32>().unwrap(),
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Invalid token" })),
            )
            .into_response()
        }
    };

    match is_user_in_group(user_id, group_id, &state.db).await {
        Ok(_) => {},
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Invalid token" })),
            )
            .into_response()
        }
    }

    let result = sqlx::query!(
        r#"
        SELECT m.id, m.content, m.user_id, m.timestamp, u.username, u.profile_picture 
        FROM messages m
        JOIN users u ON m.user_id = u.id
        WHERE m.group_id = $1
        ORDER BY m.timestamp
        "#,
        group_id
    )
    .fetch_all(&state.db)
    .await;
    
    match result {
        Ok(messages) => {
            let message_data = messages.iter().map(|m| {
                json!({
                    "id": m.id,
                    "content": m.content,
                    "user_id": m.user_id,
                    "username": m.username,
                    "timestamp": m.timestamp.to_rfc3339(),
                    "profile_picture": m.profile_picture
                })
            }).collect::<Vec<_>>();
            
            (StatusCode::OK, Json(message_data)).into_response()
        },
        Err(err) => {
            eprintln!("Database error: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch messages" })),
            ).into_response()
        }
    }
}


async fn check_token(
    Extension(auth): Extension<Arc<Auth>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token = params.get("token");
    if token.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Missing token" })),
        )
           .into_response();
    }
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    match validate_token(token.unwrap(), jwt_key).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "valid": true }))).into_response(),
        Err(_) => (StatusCode::OK, Json(json!({ "valid": false }))).into_response(),
    }
}


async fn handle_registration(
    Extension(auth): Extension<Arc<Auth>>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    // Validate passwords match
    if form.password != form.confirm_password {
        return Err((
            StatusCode::BAD_REQUEST,
            "Passwords do not match".to_string(),
        ));
    }

    // Create a new user
    let user = User {
        id: None,
        username: form.username,
        email: Some(form.email),
        profile_picture: form.profile_picture,
        password: form.password, // Note: You should hash this password
        created_at: None,
    };

    // Register the user
    match auth.register_user(user).await {
        Ok(new_user) =>  {

            let user_id = new_user
                .id
                .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, "No user ID".into()))?;

            let claims = Claims {
                sub: user_id.to_string(),
                exp: (SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    + (60 * 60 * 24 * 7)),
            };

            let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &jsonwebtoken::EncodingKey::from_secret(jwt_key.as_bytes()),
            )
            .unwrap();

            // Return JSON with token
            Ok(Json(json!({ "token": token })))
        }
        Err(_) => Err((
            StatusCode::BAD_REQUEST,
            "Username or email already exists".to_string(),
        )),
    }
}

async fn handle_login(
    Extension(auth): Extension<Arc<Auth>>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    let user = User {
        id: None,
        username: form.username,
        email: None,
        profile_picture: None,
        password: form.password,
        created_at: None,
    };

    match auth.user_login(user).await {
        Ok(Some(user)) => {
            let claims = Claims {
                sub: user.id.unwrap().to_string(),
                exp: (SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    + (60 * 60 * 24 * 7)),
            };

            let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &jsonwebtoken::EncodingKey::from_secret(jwt_key.as_bytes()),
            )
            .unwrap();

            // Return JSON with token
            Ok(Json(json!({ "token": token })))
        }
        Ok(None) => Err((
            StatusCode::UNAUTHORIZED,
            "No user with these credentials exists".to_string(),
        )),
        Err(_) => Err((
            StatusCode::BAD_REQUEST,
            "Username or password wrong".to_string(),
        )),
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<ServerState>>,
    Path(group_id): Path<i32>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token = params.get("token").cloned();

    if token.is_none() {
        return (StatusCode::UNAUTHORIZED, "Missing authentication token").into_response();
    }
    dotenv().ok();

    let key = std::env::var("JWT_KEY").expect("Must set JWT_KEY environment variable");

    let token = token.unwrap();

    let user_id = match validate_token(&token, key).await {
        Ok(claims) => claims.sub.parse::<i32>().unwrap(),
        Err(_) => return (StatusCode::UNAUTHORIZED, "Invalid token").into_response(),
    };

    //TODO: CHECK IF GROUP EXIST
    //TODO: CHECK IF USER IS PART OF GROUP
    ws.on_upgrade(move |socket: WebSocket| handle_socket(socket, user_id, state, group_id))

}

async fn handle_socket(socket: WebSocket, user_id: i32, state: std::sync::Arc<ServerState>, group_id: i32) {
    let connection_id = Uuid::new_v4(); 

    let (mut sender, mut receiver) = socket.split();
    let (mpsc_tx, mut mpsc_rx) = mpsc::unbounded_channel::<Message>();

    {        
        let mut channels = state.channels.lock().await;
        let channel = channels.entry(group_id).or_default();
        channel.insert(connection_id, mpsc_tx.clone());
    }

    // Task to broadcast messages to this client
    let send_task = tokio::spawn(async move {
        while let Some(msg) = mpsc_rx.recv().await {
            if sender.send(msg).await.is_err() {
                eprintln!("error trying to send message, connection_id: {}", connection_id);
                break;
            }
        }
    });

    let state_clone = state.clone();
    // Task to handle messages from this client
    let mut recv_task = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(msg) => {
                    match msg {
                        Message::Text(text_content) => {

                            // Store in database and return inserted record + username
                            let result = sqlx::query!(
                                r#"
                                WITH inserted AS (
                                  INSERT INTO messages (user_id, content, group_id)
                                  VALUES ($1, $2, $3)
                                  RETURNING id, user_id, content, timestamp, group_id
                                )
                                SELECT i.id, i.user_id, i.content, i.timestamp, i.group_id, u.username, u.profile_picture
                                FROM inserted i
                                JOIN users u ON i.user_id = u.id;
                                "#,
                                user_id,
                                text_content,
                                group_id
                            )
                            .fetch_one(&state_clone.db)
                            .await;
                            
                            // convert result to json and send to all clients
                            match result {
                                Ok(record) => {
                                    let message_json = json!({
                                        "id": record.id,
                                        "user_id": record.user_id,
                                        "content": record.content,
                                        "timestamp": record.timestamp.to_rfc3339(),
                                        "username": record.username,
                                        "profile_picture": record.profile_picture,
                                        "group_id": record.group_id
                                    });
                                    
                                    let broadcast_msg = Message::Text(message_json.to_string());

                                    broadcast_message(
                                        state_clone.clone(),
                                        group_id,
                                        broadcast_msg
                                    ).await;
                                }
                                Err(e) => {
                                    eprintln!("Failed to store message: {}", e);
                                }
                            }
                        }
                        Message::Close(_) => {}
                        _ =>{}
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving message, connection_id: {}, {:?}", connection_id, e)
                }
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    //clean up channles
    {
        let mut channels = state.channels.lock().await;
        if let Some(channel) = channels.get_mut(&group_id) {
            channel.remove(&connection_id);

            if channel.is_empty() {
                channels.remove(&group_id);
            }
        }
    }
}

//helper function to broadcast messages
async fn broadcast_message(
    state: Arc<ServerState>,
    group_id: i32,
    msg: Message,
) {
    let channels = state.channels.lock().await;

    if let Some(channel) = channels.get(&group_id) {
        for (peer_connection_id, peer_tx) in channel.iter() {
            if peer_tx.send(msg.clone()).is_err() {
                eprintln!(
                    "Failed to send message to {}",
                    peer_connection_id
                );
            }
        }
    }
}

async fn get_user_info(
    Extension(auth): Extension<Arc<Auth>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token = params.get("token");

    if token.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Missing token" })),
        )
            .into_response();
    }

    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");

    match validate_token(token.unwrap(), jwt_key).await {
        Ok(claims) => {
            // Get user details from DB using the user ID in the token
            match auth.get_user_by_id(claims.sub.parse::<i32>().unwrap()).await {
                Ok(Some(user)) => {
                    // Don't include password in response
                    (
                        StatusCode::OK,
                        Json(json!({
                            "id": user.id,
                            "username": user.username,
                            "email": user.email,
                            "profile_picture": user.profile_picture
                        })),
                    )
                        .into_response()
                }
                _ => (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "User not found" })),
                )
                    .into_response(),
            }
        }
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid token" })),
        )
            .into_response(),
    }
}

async fn handle_logout() {}

