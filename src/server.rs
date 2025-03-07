use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, tungstenite::Message};

use gauth::models::{Auth, User};

mod routes;
use crate::routes::auth;
use axum::extract::Form;
use axum::http::StatusCode;
use axum::response::{Html, Redirect};
use axum::routing::{get, post, Router};
use axum::Extension;

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

    let server = TcpListener::bind("0.0.0.0:3012").await.unwrap();
    let (tx, mut _rx) = broadcast::channel::<String>(100);

    let app = Router::new()
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
        .layer(Extension(auth));
    let http_server = TcpListener::bind("0.0.0.0:3013").await.unwrap();

    // Shared DB state
    let state = ServerState {
        db: pool,
        tx: tx.clone(),
    };
    let state = std::sync::Arc::new(state);

    tokio::join!(
        handle_http_server(http_server, app),
        handle_websocket_connections(server, tx, state)
    );
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

async fn handle_websocket_connections(
    server: TcpListener,
    tx: broadcast::Sender<String>,
    state: std::sync::Arc<ServerState>,
) {
    while let Ok((stream, _addr)) = server.accept().await {
        let tx_clone = tx.clone();
        let state = state.clone(); // Lightweight since state is wrapped in Arc

        tokio::spawn(async move {
            let mut websocket: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream> =
                accept_async(stream).await.unwrap();
            let mut rx: broadcast::Receiver<String> = tx_clone.subscribe();

            loop {
                select! {
                    Ok(rec_msg) = rx.recv() => {
                        websocket.send(Message::Text(rec_msg)).await.unwrap();
                    }

                    Some(rec_msg) = websocket.next() => {
                        if let Ok(rec_msg) = rec_msg {
                            println!("[Broadcasting]: {}", rec_msg);

                            if let Err(e) = sqlx::query!(
                                "INSERT INTO messages (content) VALUES ($1)",
                                rec_msg.to_string()
                            )
                            .execute(&state.db)
                            .await {
                                eprintln!("Failed to store message: {}", e);
                            }

                            tx_clone.send(rec_msg.to_string());
                        }
                    }

                    else => break
                }
            }
        });
    }
}

async fn handle_http_server(listener: TcpListener, app: Router) {
    axum::serve(listener, app).await.unwrap();
}
