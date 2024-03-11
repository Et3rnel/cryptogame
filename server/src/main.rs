use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async};

use tokio::sync::Mutex;
use std::sync::Arc;
use tokio_tungstenite::tungstenite::protocol::Message;

struct Player {
    id: String,
    position: (f64, f64),
}

struct GameState {
    players: HashMap<String, Player>,
}

enum Message {
    PlayerJoin(String),
    PlayerMove { id: String, position: (f64, f64) },
}

async fn handle_connection(stream: TcpStream) {
    if let Err(e) = accept_async(stream).await {
        println!("Error during the websocket handshake: {}", e);
        return;
    }
    println!("WebSocket connection established");
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