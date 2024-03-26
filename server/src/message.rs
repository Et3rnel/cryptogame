use crate::command::SendCommand;
use crate::player::Player;
use std::collections::HashMap;

pub fn create_player_position_message(x: f64, y: f64) -> Vec<u8> {
    let mut bytes = Vec::new();

    let command_id = SendCommand::PlayerPosition as u8;

    bytes.push(command_id);
    bytes.extend_from_slice(&x.to_be_bytes());
    bytes.extend_from_slice(&y.to_be_bytes());

    bytes
}

pub fn create_global_state_message(user_states: &mut HashMap<String, Player>) -> Vec<u8> {
    let mut buffer = Vec::new();

    for (uuid, player) in user_states.iter_mut() {
        // convert into bytes sequence
        let id = uuid::Uuid::parse_str(uuid).expect("Failed to parse UUID");
        let id_bytes = id.as_bytes(); // slice of 16 bytes
        buffer.extend_from_slice(id_bytes);

        // add player position
        buffer.extend_from_slice(&player.x.to_be_bytes());
        buffer.extend_from_slice(&player.y.to_be_bytes());
    }

    buffer
}
