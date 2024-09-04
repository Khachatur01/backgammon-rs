mod image;

use engine::board::checkers::Checkers;
use engine::constant::player::Side;
use engine::stage::Stage;
use engine::types::dice_pair::DicePair;
use engine::types::pip::Pip;
use crate::image::Image;
use image::pixel::Pixel;

const WIDTH: usize = 42;
const HEIGHT: usize = 42;

pub struct StageRenderer {
    pub numbers: [Pixel; 15],
    pub dices: [Pixel; 6],
    pub board_border: Pixel,
    pub space: Pixel,
    pub pips_separator: Pixel,
    pub white_checker: Pixel,
    pub black_checker: Pixel,
    pub possible_move: Pixel,
    pub up: Pixel,
    pub down: Pixel,
    pub right: Pixel,
    pub left: Pixel,
    pub bear_off: Pixel,
}

impl StageRenderer {
    pub fn render(&self, stage: Box<dyn Stage>) -> Image {
        let white_checkers: &Checkers = stage.white_checkers();
        let black_checkers: &Checkers = stage.black_checkers();
        let active_side: Option<Side> = stage.active_side();
        let dice_pair: Option<DicePair> = stage.dice_pair();
        let taken_checker_pip: Option<Pip> = stage.taken_checker_pip();

        let mut result_image: Image = Image::new(WIDTH, HEIGHT);

        self.render_borders(&mut result_image);

        result_image
    }

    fn render_borders(&self, result_image: &mut Image) {
        for row in [0, HEIGHT - 1] {
            for col in 0..WIDTH {
                result_image.set_pixel(row, col, self.board_border);
            }
        }

        for col in [0, WIDTH - 1] {
            for row in 0..HEIGHT {
                result_image.set_pixel(row, col, self.board_border);
            }
        }
    }
}
