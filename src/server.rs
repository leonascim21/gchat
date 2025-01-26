use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{accept_async, connect_async, tungstenite::Message};
use tokio::net::TcpStream;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::WebSocketStream;

#[tokio::main]
async fn main(){
    let server = TcpListener::bind("127.0.0.1:3012").await.unwrap();
    while let Ok((stream, addr)) = server.accept().await {
        
        
        tokio::spawn(async move {
            let mut websocket = accept_async(stream).await.unwrap();
            
                loop {

                let message = websocket.next().await.unwrap();


                match message {
                    Ok(msg) => {
                        println!("[Broadcasting]: {}", msg);
                        let message_content = msg.to_string();

                            if let Err(e) = websocket.send(Message::Text(message_content.clone())).await {
                                println!("Error sending message to a client: {}", e);
                            }
                    },

                    Err(e) => {
                        println!("Error reading message");
                    }
                
                } 
            
                }
        });
    
    }
}
