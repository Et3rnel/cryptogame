pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Player {
    pub fn move_in_direction(&mut self, direction_code: u8) -> (i32, i32) {
        let step_size = 1; // Define the step size for movement

        match direction_code {
            0x01 => self.y -= step_size, // Move up
            0x02 => self.y += step_size, // Move down
            0x03 => self.x -= step_size, // Move left
            0x04 => self.x += step_size, // Move right
            _ => println!("Unknown direction code"),
        }

        println!("Current position: x = {}, y = {}", self.x, self.y);

        (self.x, self.y)
    }
}
