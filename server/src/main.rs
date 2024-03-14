use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async};
use tokio_tungstenite::tungstenite::protocol::Message;

use std::collections::HashMap;

async fn handle_connection(stream: TcpStream) {
    let ws_stream = match accept_async(stream).await{
        Ok(ws) => ws,
        Err(e) => {
            println!("Error during the websocket handshake: {}", e);
            return;
        },
    };
    println!("WebSocket connection established");

    // let (write, mut read) = ws_stream.split();
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}