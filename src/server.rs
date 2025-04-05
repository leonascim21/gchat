mod state;
mod routes;
mod utils;

use axum::extract::ws::WebSocket;
use axum::http::header;
use axum::http::{HeaderValue, Method};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Extension;
use axum::{
    extract::{ws::Message, Query, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::Router,
    Json,
};

use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use gauth::models::Auth;
use gauth::validate_token;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use state::ServerState;
use uuid::Uuid;

use std::sync::Arc;
use std::collections::HashMap;
use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex};

use tower_http::cors::CorsLayer;
use utils::queries::{insert_message_in_db, is_user_in_group};

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
        .nest("/", routes::app_routes().with_state(state.clone()))
        .layer(cors)
        .layer(Extension(auth))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
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

    match is_user_in_group(user_id, group_id, &state.db).await {
        Ok(_) => {},
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Unauthorized" })),
            )
            .into_response()
        }
    }

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
    let recv_task = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(msg) => {
                    match msg {
                        Message::Text(text_content) => {    
                            // convert result to json and send to all clients
                            match insert_message_in_db(user_id, group_id, text_content, &state_clone.db).await {
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