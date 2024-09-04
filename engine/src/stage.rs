use crate::board::checkers::Checkers;
use crate::constant::player::Side;
use crate::types::dice_pair::DicePair;
use crate::types::pip::Pip;

pub mod start;
pub mod dices_thrown;
pub mod checker_taken;
pub mod out_of_moves;
pub mod checker_moved;
pub mod win;
pub mod side_switched;
pub mod moves_commited;
pub mod no_possible_moves;
pub mod after_throwing_dices;

pub trait Stage {
    fn white_checkers(&self) -> &Checkers;
    fn black_checkers(&self) -> &Checkers;
    fn active_side(&self) -> Option<Side>;
    fn dice_pair(&self) -> Option<DicePair>;
    fn taken_checker_pip(&self) -> Option<Pip>;
}
