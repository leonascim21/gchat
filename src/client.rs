use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use tokio::io::AsyncBufReadExt;
use url::Url;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let url = url::Url::parse("ws://localhost:3012/socket").unwrap();
    let (ws_stream, response) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();

    let (tx, mut rx) = mpsc::channel::<String>(32);

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    
    for (header, _value) in response.headers() {
        println!("* {header}");
    }

    // Thread for sending messages
    let tx_clone = tx.clone();
    tokio::spawn( async move {
        let mut stdin = tokio::io::stdin();
        let mut reader = tokio::io::BufReader::new(stdin);
        let mut buffer = String::new();

        loop {
            buffer.clear();
            reader.read_line(&mut buffer).await
                .expect("Failed to read message.");
            tx_clone.send(buffer.clone()).await.unwrap();
            //socket.send(Message::Text(message.into())).unwrap();
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
