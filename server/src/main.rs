mod command;
mod message;
mod player;
mod state;

use crate::message::create_player_position_message;
use crate::player::Player;
use crate::state::USER_STATES;
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use std::{env, io::Error};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

type ClientMap = Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    let clients: ClientMap = Arc::new(Mutex::new(HashMap::new()));
    let clients_for_tick = clients.clone();

    tokio::spawn(async move {
        let mut tick_interval = interval(Duration::from_millis(16)); // 60 FPS
        loop {
            tick_interval.tick().await;
            let mut user_states = USER_STATES.lock().await;

            for (id, player) in user_states.iter_mut() {
                player.update_position();
                let message = Message::Binary(create_player_position_message(player.x, player.y));
                let clients_guard = clients_for_tick.lock().await;
                for tx in clients_guard.values() {
                    let _ = tx.send(message.clone());
                }
            }
        }
    });

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, clients.clone()));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream, clients: ClientMap) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let client_id = Uuid::new_v4().to_string();
    clients.lock().await.insert(client_id.clone(), tx);

    USER_STATES
        .lock()
        .await
        .entry(client_id.clone())
        .or_insert(Player {
            x: 350.0, // TODO: initial start on the middle of the canvas, to be changed later
            y: 350.0,
            direction: 0.0,
        });

    println!("New WebSocket connection: {}", client_id);

    let (mut write, mut read) = ws_stream.split();

    loop {
        tokio::select! {
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
            Some(message) = rx.recv() => {
                if write.send(message).await.is_err() {
                    // TODO: handle the error
                    // TODO: check if we want to break if it fails
                    break;
                }
            },
        }
    }
}
