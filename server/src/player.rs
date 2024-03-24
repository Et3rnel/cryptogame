pub struct Player {
    pub x: i32,
    pub y: i32,
    pub direction: f64,
}

impl Player {
    pub fn update_direction(&mut self, direction_code: u8) {
        let rotation_step = 30.0;
        match direction_code {
            0x01 => self.direction -= rotation_step, // left
            0x02 => self.direction += rotation_step, // right
            _ => (),
        }
        self.direction = (self.direction + 360.0) % 360.0; // normalize angle between 0 and 360

        println!("Current direction:  {}", self.direction);
    }

    pub fn update_position(&mut self) {
        let step_size = 1.0;
        let rad = self.direction.to_radians();

        self.x += (rad.cos() * step_size).round() as i32;
        self.y += (rad.sin() * step_size).round() as i32;
    }
}
