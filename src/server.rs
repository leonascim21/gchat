use std::{net::TcpListener, thread::spawn};
use tungstenite::accept;

fn main() {
    let server = TcpListener::bind("127.0.0.1:3012").unwrap();
    for stream in server.incoming() {
        dbg!("New connection opened");
        spawn ( move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {

                let msg = websocket.read().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    println!("[Received]: {}", msg);
                    //websocket.send(msg).unwrap();
                }
            }
        });
    }
}
