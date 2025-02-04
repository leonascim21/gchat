use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::select;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

use std::fs::File;
use std::io::BufReader;
use rustls::{Certificate, PrivateKey, ServerConfig};
use tokio_rustls::TlsAcceptor;

fn load_certs(path: &str) -> Vec<Certificate> {
    let cert_file = File::open(path).expect("Failed to open certificate file");
    let mut reader = BufReader::new(cert_file);
    rustls_pemfile::certs(&mut reader)
        .unwrap()
        .iter()
        .map(|v| Certificate(v.clone()))
        .collect()
}

fn load_private_key(path: &str) -> PrivateKey {
    let key_file = File::open(path).expect("Failed to open private key file");
    let mut reader = BufReader::new(key_file);
    let keys = rustls_pemfile::pkcs8_private_keys(&mut reader).unwrap();
    PrivateKey(keys[0].clone())
}

struct ServerState {
    db: sqlx::PgPool,
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // TLS configuration
    let certs = load_certs("/etc/letsencrypt/live/ws.gchat.cloud/fullchain.pem");
    let key = load_private_key("/etc/letsencrypt/live/ws.gchat.cloud/privkey.pem");

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("Failed to create TLS config");

    // DB connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool");
    
    let acceptor = TlsAcceptor::from(std::sync::Arc::new(config));

    let server = TcpListener::bind("0.0.0.0:3012").await.unwrap();
    let(tx, mut _rx) = broadcast::channel::<String>(100);

    // Shared DB state
    let state = ServerState { db: pool, tx: tx.clone() };
    let state = std::sync::Arc::new(state);

    while let Ok((stream, _addr)) = server.accept().await {
        let acceptor = acceptor.clone();
        let tx_clone = tx.clone();
        let state = state.clone(); // Lightweight since state is wrapped in Arc
        
        tokio::spawn(async move {
            let tls_stream = acceptor.accept(stream).await.unwrap();
            let mut websocket = accept_async(tls_stream).await.unwrap();
            let mut rx = tx_clone.subscribe();
                
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

