use color::Color;

mod color;

#[derive(Copy, Clone)]
pub struct Pixel {
    pub symbol: char,
    pub color: Color
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            symbol: ' ',
            color: Color::default()
        }
    }
}
