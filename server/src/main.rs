use std::{env, io::Error};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

use futures_util::{future, StreamExt, TryStreamExt};
use tokio::net::{TcpListener, TcpStream};
use once_cell::sync::Lazy;

struct Position {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Position { x: i32, y: i32 },
}

// Globally accessible state
static USER_STATES: Lazy<Arc<Mutex<HashMap<String, Position>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});


#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let client_id = Uuid::new_v4().to_string();

    println!("New WebSocket connection: {}", client_id);

    let (write, mut read) = ws_stream.split();

    while let Some(message_result) = read.next().await {
        match message_result {
            Ok(message) => {
                match message {
                    Message::Binary(data) => {
                        let command_type = data[0]; // First byte to tell which command we use
                        match command_type {
                            0x01 => { // Position command
                                let direction = data[1];
                                match direction {
                                    0x01 => println!("Move up"),
                                    0x02 => println!("Move down"),
                                    0x03 => println!("Move left"),
                                    0x04 => println!("Move right"),
                                    _ => println!("Unknown direction"),
                                }
                            },
                            _ => println!("Unknown command"),
                        }
                    },
                    _ => (),
                }
            },
            Err(e) => {
                println!("Error receiving message: {:?}", e);
                // Todo: we could want to close the connection, to try again..
                break; // Handling example: stop the loop in case of error
            },
        }
    }
}
