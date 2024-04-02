#[derive(Clone)]
pub struct Game {
    pub canvas: Canvas,
}

#[derive(Clone)]
pub struct Canvas {
    pub width: u16,
    pub height: u16,
}
