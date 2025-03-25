use axum::extract::ws::WebSocket;
use axum::http::header;
use axum::http::Request;
use axum::http::{HeaderValue, Method};
use axum::middleware::{self, Next};
use axum::response::Response;
use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};

use gauth::models::{Auth, Claims, User};
use gauth::{jwt, validate_token};

mod routes;
use axum::extract::Form;
use axum::http::StatusCode;
use axum::response::{Html, Redirect};
use axum::routing::{get, post};
use axum::Extension;
use axum::{
    extract::{ws::Message, Query, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::{any, Router},
    Json,
};

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;
use std::time::SystemTime;

struct ServerState {
    db: sqlx::PgPool,
    tx: broadcast::Sender<String>,
}

#[derive(Deserialize)]
struct RegisterForm {
    username: String,
    email: String,
    password: String,
    #[serde(rename = "confirmPassword")]
    confirm_password: String,
    #[serde(rename = "profilePicture")]
    profile_picture: Option<String>,
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct FriendRequestForm {
    #[serde(rename = "receiverUsername")]
    receiver_username: String,
    token: String,
}

#[derive(Deserialize)]
struct AcceptFriendRequestForm {
    #[serde(rename = "userId")]
    user_id: i32,
    token: String,
}


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

    let (tx, mut _rx) = broadcast::channel::<String>(100);

    // Shared DB state
    let state = ServerState {
        db: pool,
        tx: tx.clone(),
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
        .route("/ws", get(ws_handler))
        .route(
            "/login",
            get(|| async { Html(include_str!("../templates/login.html")) }),
        )
        .route("/login", post(handle_login))
        .route("/get-all-messages", get(get_all_messages))
        .route("/register", post(handle_registration))
        .route("/check-token", get(check_token))
        .route("/get-user-info", get(get_user_info))
        .route("/send-friend-request", post(send_friend_request))
        .route("/get-friend-requests", get(get_friend_requests))
        .route("/accept-friend-request", post(accept_friend_request))
        //.nofollow.route("/api/validate-token", get(validate_token_handler))
        //.route("/api/me", get(get_user_info))
        .route("/api/logout", post(handle_logout))
        .layer(cors)
        .layer(Extension(auth))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("Server running on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_all_messages(
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
    
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    match validate_token(token.unwrap(), jwt_key).await {
        Ok(_) => {
            let result = sqlx::query!(
                r#"
                SELECT m.id, m.content, m.user_id, m.timestamp, u.username, u.profile_picture 
                FROM messages m
                JOIN users u ON m.user_id = u.id
                ORDER BY m.timestamp
                "#
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
                            "timestamp": m.timestamp.to_string(),
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
        },
        Err(_) => {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Invalid token" })),
            )
            .into_response()
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


async fn get_friend_requests(
    State(state): State<Arc<ServerState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token = match params.get("token") {
        None => {
            return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Missing token" }))).into_response()
        }
        Some(token) => token,
    };
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    let user_id = match validate_token(token, jwt_key).await {
        Ok(claims) => claims.sub.parse::<i32>().unwrap(),
        Err(_) => {
            return (StatusCode::UNAUTHORIZED,Json(json!({ "error": "Invalid token" }))).into_response()
        }
    };

    let outgoing_result = sqlx::query!(
        r#"
        SELECT fr.sender_id, fr.receiver_id, u.username
        FROM friend_requests fr
        JOIN users u ON fr.receiver_id = u.id
        WHERE fr.sender_id = $1
        "#,
        user_id
    )
    .fetch_all(&state.db)
    .await;

    let outgoing_requests = match outgoing_result {
        Ok(requests) => requests
            .iter()
            .map(|r| {
                json!({
                    "sender_id": r.sender_id,
                    "receiver_id": r.receiver_id,
                    "username": r.username,
                })
            })
            .collect::<Vec<_>>(),
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({ "error": "Failed to fetch friend requests" }))).into_response()
        }
    };

    let incoming_result = sqlx::query!(
        r#"
        SELECT fr.sender_id, fr.receiver_id, u.username
        FROM friend_requests fr
        JOIN users u ON fr.sender_id = u.id
        WHERE fr.receiver_id = $1
        "#,
        user_id
    )
    .fetch_all(&state.db)
    .await;

    let incoming_requests = match incoming_result {
        Ok(requests) => requests
            .iter()
            .map(|r| {
                json!({
                    "sender_id": r.sender_id,
                    "receiver_id": r.receiver_id,
                    "username": r.username,
                })
            })
            .collect::<Vec<_>>(),
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({ "error": "Failed to fetch friend requests" }))).into_response()
        }
    };

    (StatusCode::OK,Json(json!({"outgoing": outgoing_requests,"incoming": incoming_requests }))).into_response()
}

async fn send_friend_request(
    Extension(auth): Extension<Arc<Auth>>,
    State(state): State<Arc<ServerState>>,
    Form(form): Form<FriendRequestForm>,
) -> impl IntoResponse {

    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    // Validate the token.
    let claims = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized"})),
            );
        }
    };
    let receiver =  match auth.get_user_by_username(form.receiver_username).await {
        Ok(user) => if user.is_some() { 
            user.unwrap() 
        } else { 
            return (StatusCode::BAD_REQUEST, Json(json!({"message": "User not found" }))); 
        },
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error, please try again"})),
            );
        }
    };
    let result = sqlx::query!(
        r#"
        INSERT INTO friend_requests (sender_id, receiver_id)
        VALUES ($1, $2)
        "#,
        claims.sub.parse::<i32>().unwrap(),
        receiver.id,
    ).execute(&state.db).await;
    match result {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Friend Request sent successfully" }))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Failed to send friend request" }))),
    }
}

async fn accept_friend_request(
    State(state): State<Arc<ServerState>>,
    Form(form): Form<AcceptFriendRequestForm>,
) -> impl IntoResponse {
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    // Validate the token.
    let claims = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized"})),
            );
        }
    };

    let existing_request = sqlx::query!(
        r#"
        SELECT * 
        FROM friend_requests 
        WHERE sender_id = $1 AND receiver_id = $2
        "#,
        form.user_id,
        claims.sub.parse::<i32>().unwrap()
    ).fetch_one(&state.db).await;

    let existing_request = match existing_request {
        Ok(existing_request) => existing_request,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error, please try again"})),
            );
        }
    };
    println!("flag2");
    let added_friend = sqlx::query!(
        r#"
        INSERT INTO friendships (user_id, friend_id)
        VALUES ($1, $2);
        "#,
        form.user_id,
        claims.sub.parse::<i32>().unwrap()
    ).execute(&state.db).await;

    let reverse_friend = sqlx::query!(
        r#"
        INSERT INTO friendships (user_id, friend_id)
        VALUES ($1, $2);
        "#,
        claims.sub.parse::<i32>().unwrap(),
        form.user_id,
    ).execute(&state.db).await;

    let delete_request = sqlx::query!(
        r#"
        DELETE
        FROM friend_requests 
        WHERE sender_id = $1 AND receiver_id = $2
        "#,
        form.user_id,
        claims.sub.parse::<i32>().unwrap()
    ).execute(&state.db).await;

    match (added_friend, reverse_friend, delete_request) {
        (Ok(_), Ok(_), Ok(_)) => {
            (StatusCode::OK, Json(json!({"message": "Friend request accepted"})))
        },
        _ => {
            (StatusCode::INTERNAL_SERVER_ERROR, 
             Json(json!({"message": "Failed to accept friend request"})))
        }
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
    State(state): State<Arc<ServerState>>,
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token = params.get("token").cloned();

    if token.is_none() {
        return (StatusCode::UNAUTHORIZED, "Missing authentication token").into_response();
    }
    dotenv().ok();

    let key = std::env::var("JWT_KEY").expect("Must set JWT_KEY environment variable");

    let token = token.unwrap();

    match validate_token(&token, key).await {
        Ok(claims) => ws.on_upgrade(move |socket| handle_socket(socket, claims.sub, state)),
        Err(_) => return (StatusCode::UNAUTHORIZED, "Invalid token").into_response(),
    }
}

async fn handle_socket(socket: WebSocket, user_id: String, state: std::sync::Arc<ServerState>) {

    //Parse user_id from jwt sub
    let user_id: i32 = match user_id.parse() {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to parse user id: {}", e);
            return;
        }
    };


    let (mut sender, mut receiver) = socket.split();
    let mut msg_rx = state.tx.subscribe();

    // Task to broadcast messages to this client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = msg_rx.recv().await {
            sender.send(Message::Text(msg)).await;
        }
    });

    // Task to handle messages from this client
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // Check if this is a text message
            if let Message::Text(text_content) = msg {
                println!("[Broadcasting]: {}", text_content);

                // Store in database and return inserted record + username
                let result = sqlx::query!(
                    r#"
                    WITH inserted AS (
                      INSERT INTO messages (user_id, content)
                      VALUES ($1, $2)
                      RETURNING id, user_id, content, timestamp
                    )
                    SELECT i.id, i.user_id, i.content, i.timestamp, u.username, u.profile_picture
                    FROM inserted i
                    JOIN users u ON i.user_id = u.id;
                    "#,
                    user_id,
                    text_content,
                )
                .fetch_one(&state.db)
                .await;
                
                // convert result to json and send to all clients
                match result {
                    Ok(record) => {
                        let message = json!({
                            "id": record.id,
                            "user_id": record.user_id,
                            "content": record.content,
                            "timestamp": record.timestamp.to_string(),
                            "username": record.username,
                            "profile_picture": record.profile_picture
                        });
                        
                        if let Err(e) = state.tx.send(message.to_string()) {
                            eprintln!("Failed to broadcast message: {}", e);
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to store message: {}", e);
                    }
                }
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            send_task.abort();
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

