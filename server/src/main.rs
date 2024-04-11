mod command;
mod config;
mod game;
mod message;
mod player;
mod state;

use crate::message::create_global_state_message;
use crate::player::{calculate_player_position, Player};
use crate::state::USER_STATES;
use config::Config;
use futures_util::{SinkExt, StreamExt};
use game::{Canvas, Game};
use std::collections::HashMap;
use std::path::Path;
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
    env_logger::init();

    let config_path = Path::new("config.json");
    let config =
        Config::load(config_path).expect("Game cannot start without a valid configuration file");

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    let clients: ClientMap = Arc::new(Mutex::new(HashMap::new()));
    let clients_for_tick = clients.clone();

    let game = Arc::new(Game {
        canvas: Canvas {
            width: 600,
            height: 600,
        },
    });

    let game_clone = game.clone();

    tokio::spawn(async move {
        let mut tick_interval = interval(Duration::from_millis(16)); // 60 FPS
        loop {
            tick_interval.tick().await;
            let mut user_states = USER_STATES.lock().await;

            for (_, player) in user_states.iter_mut() {
                if player.alive {
                    player.update_position();

                    let canvas = &game_clone.canvas;
                    player.check_border_collision(canvas)
                }
            }

            let message = Message::Binary(create_global_state_message(&user_states));
            let clients_guard = clients_for_tick.lock().await;
            for tx in clients_guard.values() {
                let _ = tx.send(message.clone());
            }
        }
    });

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(
            stream,
            clients.clone(),
            game.clone(),
            Arc::new(config.clone()),
        ));
    }

    Ok(())
}

async fn accept_connection(
    stream: TcpStream,
    clients: ClientMap,
    game: Arc<Game>,
    config: Arc<Config>,
) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let client_id = Uuid::new_v4().to_string();
    println!("New WebSocket connection: {}", client_id);

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    clients.lock().await.insert(client_id.clone(), tx);

    place_new_player(client_id.clone(), game.clone(), config.clone()).await;

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

async fn place_new_player(client_id: String, game: Arc<Game>, config: Arc<Config>) {
    let (x, y) = {
        let user_states = USER_STATES.lock().await;
        calculate_player_position(&*user_states, &game.canvas)
    };

    let mut user_states = USER_STATES.lock().await;
    user_states.insert(
        client_id,
        Player {
            x,
            y,
            direction: 0.0,
            alive: true,
            radius: config.player_radius,
        },
    );
}
