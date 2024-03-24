mod command;
mod message;
mod player;
mod state;

use crate::message::create_player_position_message;
use crate::player::Player;
use crate::state::USER_STATES;
use futures_util::{SinkExt, StreamExt};
use std::{env, io::Error};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{interval, Duration};
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

    let mut tick_interval = interval(Duration::from_millis(16)); // 60 FPS

    let client_id = Uuid::new_v4().to_string();
    USER_STATES
        .lock()
        .await
        .entry(client_id.clone())
        .or_insert(Player {
            x: 350, // TODO: initial start on the middle of the canvas, to be changed later
            y: 350,
            direction: 0.0,
        });

    println!("New WebSocket connection: {}", client_id);

    let (mut write, mut read) = ws_stream.split();

    loop {
        tokio::select! {
            _ = tick_interval.tick() => {
                let mut user_states = USER_STATES.lock().await;

                if let Some(player) = user_states.get_mut(&client_id) {
                    player.update_position();
                    let position_message = create_player_position_message(player.x, player.y);
                    let _ = write.send(Message::Binary(position_message)).await;
                }
            },
            message_result = read.next() => {
                if let Some(Ok(message)) = message_result {
                    match message {
                        Message::Binary(data) => {
                            let command_type = data[0];
                            if command_type == 0x01 {
                                let direction = data[1];
                                let mut user_states = USER_STATES.lock().await;
                                if let Some(player) = user_states.get_mut(&client_id) {
                                    player.update_direction(direction);
                                } else {
                                    println!("Player not found");
                                }
                            }
                        },
                        _ => (),
                    }
                } else {
                    break; // exit loop if read.next() return None (connection closed) or an error
                }
            },
        }
    }
}
