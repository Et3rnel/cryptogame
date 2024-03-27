use crate::command::SendCommand;
use crate::player::Player;
use log::error;
use std::collections::HashMap;

pub fn create_global_state_message(user_states: &HashMap<String, Player>) -> Vec<u8> {
    let mut buffer = Vec::new();
    let command_id = SendCommand::PlayerPosition as u8;
    buffer.push(command_id);

    for (uuid, player) in user_states.iter() {
        match uuid::Uuid::parse_str(uuid) {
            Ok(id) => {
                let id_bytes = id.as_bytes();
                buffer.extend_from_slice(id_bytes);
                buffer.extend_from_slice(&player.x.to_be_bytes());
                buffer.extend_from_slice(&player.y.to_be_bytes());
            }
            Err(e) => {
                error!("Failed to parse UUID: {}. Error: {}", uuid, e);
                // Skip the problematic player UUID and continue
            }
        }
    }

    buffer
}
