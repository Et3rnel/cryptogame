pub struct Player {
    pub x: f64,
    pub y: f64,
    pub direction: f64,
}

impl Player {
    pub fn update_direction(&mut self, direction_code: u8) {
        let rotation_step = 10.0;
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
