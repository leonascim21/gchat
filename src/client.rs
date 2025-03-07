use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use tokio::io::AsyncBufReadExt;

#[tokio::main]
async fn main() {
    let url = url::Url::parse("wss://ws.gchat.cloud").unwrap();
    let (ws_stream, response) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();

    let (tx, mut rx) = mpsc::channel::<String>(32);

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    
    for (header, _value) in response.headers() {
        println!("* {header}");
    }

    // Async block for sending messages
    let tx_clone = tx.clone();
    tokio::spawn( async move {
        let stdin = tokio::io::stdin();
        let mut reader = tokio::io::BufReader::new(stdin);
        let mut buffer = String::new();

        loop {
            buffer.clear();
            reader.read_line(&mut buffer).await
                .expect("Failed to read message.");
            tx_clone.send(buffer.clone()).await.unwrap();
        }
    });
    
    loop{

    
        tokio::select! {
            Some(sent_msg) = rx.recv() => {
                write.send(Message::Text(sent_msg)).await.unwrap();
            }
        
            Some(rec_msg) = read.next() => {
                if let Ok(rec_msg) = rec_msg {
                    println!("Received: {}", rec_msg);
                }
            }

            else => break
        }
    }


    //socket.close(None);
}
