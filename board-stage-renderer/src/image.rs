use pixel::Pixel;

pub mod pixel;

pub struct Image {
    buffer: Vec<Vec<Pixel>>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![vec![Pixel::default(); width]; height]
        }
    }

    pub fn set_pixel(&mut self, row: usize, col: usize, pixel: Pixel) {
        *self.buffer[row][col] = pixel;
    }
}
