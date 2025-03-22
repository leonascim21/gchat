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
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
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

    let app = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .route("/ws", get(ws_handler))
        .route(
            "/login",
            get(|| async { Html(include_str!("../templates/login.html")) }),
        )
        .route("/login", post(handle_login))
        .route("/register", post(handle_registration))
        .route("/check-token", get(check_token))
        //.nofollow.route("/api/validate-token", get(validate_token_handler))
        //.route("/api/me", get(get_user_info))
        .route("/api/logout", post(handle_logout))
        .layer(Extension(auth))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("Server running on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
) -> Result<Redirect, (StatusCode, String)> {
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
        password: form.password, // Note: You should hash this password
        created_at: None,
    };

    // Register the user
    match auth.register_user(user).await {
        Ok(_) => Ok(Redirect::to("/login")),
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

                // Store in database
                if let Err(e) =
                    sqlx::query!("INSERT INTO messages (content) VALUES ($1)", text_content)
                        .execute(&state.db)
                        .await
                {
                    eprintln!("Failed to store message: {}", e);
                }

                // Broadcast to all clients
                state.tx.send(text_content);
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
            match auth.get_user_by_username(claims.sub).await {
                Ok(Some(user)) => {
                    // Don't include password in response
                    (
                        StatusCode::OK,
                        Json(json!({
                            "id": user.id,
                            "username": user.username,
                            "email": user.email
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

