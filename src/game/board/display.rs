use crossterm::style::Color;
use crate::game::constant::player::Player;
use crate::game::constant::SPACE;

#[derive(Copy, Clone)]
pub struct Pixel(char, Color);

pub struct Image {
    buffer: Vec<Vec<Pixel>>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            buffer: vec![
                vec![
                    Pixel(SPACE, Color::Black);
                    width
                ];
                height
            ]
        }
    }

    pub fn set_pixel(&mut self, row: usize, col: usize, char: char, color: Color) {
        self.buffer[row][col] = Pixel(char, color);
    }
}

pub trait Render {
    fn render(&self, for_player: Player) -> Image;
}