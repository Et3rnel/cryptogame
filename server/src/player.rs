use rand::Rng;
use std::collections::HashMap;

use crate::game::Canvas;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub direction: f64,
}

impl Player {
    pub fn update_direction(&mut self, direction_code: u8) {
        let rotation_step = 2.0;
        match direction_code {
            0x01 => self.direction -= rotation_step, // left
            0x02 => self.direction += rotation_step, // right
            _ => (),
        }
        self.direction = (self.direction + 360.0) % 360.0; // normalize angle between 0 and 360

        println!("Current direction:  {}", self.direction);
    }

    pub fn update_position(&mut self) {
        let step_size = 1.5;
        let rad = self.direction.to_radians();

        self.x += rad.cos() * step_size;
        self.y += rad.sin() * step_size;
    }
}

pub fn calculate_player_position(
    existing_players: &HashMap<String, Player>,
    canvas: &Canvas,
) -> (f64, f64) {
    let width = f64::from(canvas.width);
    let height = f64::from(canvas.height);

    let mut rng = rand::thread_rng();
    let margin = 0.2; // 20% margin
    let min_distance = 0.1 * f64::min(width, height); // 10% of the smallest dimension

    let min_x = width * margin;
    let max_x = width * (1.0 - margin);
    let min_y = height * margin;
    let max_y = height * (1.0 - margin);

    'attempt: for _ in 0..100 {
        let new_x = rng.gen_range(min_x..max_x);
        let new_y = rng.gen_range(min_y..max_y);

        for player in existing_players.values() {
            let distance = ((player.x - new_x).powi(2) + (player.y - new_y).powi(2)).sqrt();
            if distance < min_distance {
                continue 'attempt;
            }
        }

        return (new_x, new_y);
    }

    error!("Cannot find a matching start position for player after 100 tries.");
    return (width/2, height/2); // default to the middle of the canvas
}
