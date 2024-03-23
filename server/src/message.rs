use crate::command::SendCommand;

pub fn create_player_position_message(x: i32, y: i32) -> Vec<u8> {
    let mut bytes = Vec::new();

    let command_id = SendCommand::PlayerPosition as u8;

    bytes.push(command_id);
    bytes.extend_from_slice(&x.to_be_bytes());
    bytes.extend_from_slice(&y.to_be_bytes());

    bytes
}
