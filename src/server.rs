use axum::extract::ws::WebSocket;
use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::broadcast;
use tokio_tungstenite::accept_async;

use gauth::models::{Auth, User};
use gauth::{jwt, validate_token};

mod routes;
use crate::routes::auth;
use axum::extract::Form;
use axum::http::StatusCode;
use axum::response::{Html, Redirect};
use axum::routing::{get, post};
use axum::Extension;
use axum::{
    body::Bytes,
    extract::{ws::Message, Query, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::{any, Router},
};

use axum_extra::TypedHeader;
use std::collections::HashMap;
use std::{net::SocketAddr, path::PathBuf};

struct ServerState {
    db: sqlx::PgPool,
    tx: broadcast::Sender<String>,
}

#[derive(Deserialize)]
struct RegisterForm {
    username: String,
    email: String,
    password: String,
    #[serde(rename = "confirm-password")]
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
        .route("/ws", get(ws_handler))
        .route(
            "/login",
            get(|| async { Html(include_str!("../templates/login.html")) }),
        )
        .route("/login", post(handle_login))
        .route(
            "/register",
            get(|| async { Html(include_str!("../templates/register.html")) }),
        )
        .route("/register", post(handle_registration))
        .route(
            "/success",
            get(|| async { Html(include_str!("../templates/success.html")) }),
        )
        .layer(Extension(auth))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
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

// Pending adding login functionality to gauth

async fn handle_login(
    Extension(auth): Extension<Arc<Auth>>,
    Form(form): Form<LoginForm>,
) -> Result<Redirect, (StatusCode, String)> {
    let user = User {
        id: None,
        username: form.username,
        email: None,
        password: form.password,
        created_at: None,
    };

    match auth.user_login(user).await {
        Ok(Some(user)) => Ok(Redirect::to("/success")),
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
    dotenv().ok();
    let token = params.get("token").cloned();
    let key = std::env::var("JWT_KEY");

    // Checking if there's a token
    if token.is_none() {
        return (StatusCode::UNAUTHORIZED, "Missing authentication token").into_response();
    }

    let token = token.unwrap();

    match validate_token(&token, String::from("key")).await {
        Ok(Claims) => ws.on_upgrade(move |socket| handle_socket(socket, Claims.sub, state)),
        Err(e) => return (StatusCode::UNAUTHORIZED, "Invalid token").into_response(),
    }
}

async fn handle_socket(socket: WebSocket, user_id: String, state: std::sync::Arc<ServerState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut msg_rx = state.tx.subscribe();

    // Task to broadcast messages to this client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = msg_rx.recv().await {
            sender.send(Message::Text(msg));
        }
    });

    // Task to handle messages from this client
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            /*
            // (Save to database, broadcast to other clients, etc.)
            if let Err(e) = sqlx::query!(
                "INSERT INTO messages (content) VALUES ($1)",
                rec_msg.to_string()
            )
            .execute(&state.db)
            .await
            {
                eprintln!("Failed to store message: {}", e);
            }
            */

            state.tx.send(msg.into_text().unwrap());
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

async fn handle_http_server(listener: TcpListener, app: Router<()>) {
    axum::serve(listener, app).await.unwrap();
}
