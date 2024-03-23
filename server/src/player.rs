pub struct Player {
    pub x: i32,
    pub y: i32,
    pub direction: f64,
}

impl Player {
    pub fn move_in_direction(&mut self, direction_code: u8) -> (i32, i32) {
        let rotation_step = 5.0;

        match direction_code {
            0x01 => self.direction -= rotation_step, // left
            0x02 => self.direction += rotation_step, // right
            _ => println!("Unknown direction code"),
        }

        // normalize angle between 0 and 360
        self.direction = (self.direction + 360.0) % 360.0;

        // move player in the direction
        let rad = self.direction.to_radians();
        self.x += rad.cos().round() as i32;
        self.y += rad.sin().round() as i32;

        println!(
            "Current position: x = {}, y = {}, direction = {}",
            self.x, self.y, self.direction
        );

        (self.x, self.y)
    }
}
