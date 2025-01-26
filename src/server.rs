use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::select;

#[tokio::main]
async fn main(){
    let server = TcpListener::bind("127.0.0.1:3012").await.unwrap();
    let(tx, mut _rx) = broadcast::channel::<String>(100);

    while let Ok((stream, _addr)) = server.accept().await {
        
        let tx_clone = tx.clone();
        
        tokio::spawn(async move {
            
            let mut websocket = accept_async(stream).await.unwrap();
            let mut rx = tx_clone.subscribe();
                
            loop {
                    select! {
                        Ok(rec_msg) = rx.recv() => {
                            websocket.send(Message::Text(rec_msg)).await.unwrap();
                        }

                        Some(rec_msg) = websocket.next() => {
                            if let Ok(rec_msg) = rec_msg {
                                println!("[Broadcasting]: {}", rec_msg);
                                tx_clone.send(rec_msg.to_string());
                            }
                        }

                        else => break
                    }
                }
        });
    }
}

