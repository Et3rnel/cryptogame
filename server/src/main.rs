mod networking;
mod player;
mod state;

use crate::player::Player;
use crate::state::USER_STATES;
use futures_util::{SinkExt, StreamExt};
use std::{env, io::Error};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

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
    USER_STATES
        .lock()
        .unwrap()
        .entry(client_id.clone())
        .or_insert(Player { x: 0, y: 0 });

    println!("New WebSocket connection: {}", client_id);

    let (mut write, mut read) = ws_stream.split();

    while let Some(message_result) = read.next().await {
        match message_result {
            Ok(message) => {
                match message {
                    Message::Binary(data) => {
                        let command_type = data[0]; // First byte to tell which command we use
                        match command_type {
                            0x01 => {
                                // Move command
                                let direction = data[1];

                                let result = {
                                    let mut user_states = USER_STATES.lock().unwrap();

                                    if let Some(player) = user_states.get_mut(&client_id) {
                                        let result = player.move_in_direction(direction);
                                        result
                                    } else {
                                        println!("Player not found");
                                        (0, 0)
                                    }
                                };

                                let move_result_text =
                                    format!("Position: ({}, {})", result.0, result.1);
                                write.send(Message::Text(move_result_text)).await;
                            }
                            _ => println!("Unknown command"),
                        }
                    }
                    _ => (),
                }
            }
            Err(e) => {
                println!("Error receiving message: {:?}", e);
                // Todo: we could want to close the connection, to try again..
                break; // Handling example: stop the loop in case of error
            }
        }
    }
}
