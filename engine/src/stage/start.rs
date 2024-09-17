use crate::stage::dices_thrown::DicesThrown;
use crate::board::checkers::Checkers;
use crate::board::Board;
use crate::constant::player::Side;
use crate::stage::Stage;
use crate::types::checker_move::CheckerMove;
use crate::types::dice_pair::DicePair;
use crate::types::pip::Pip;
use rand::Rng;

pub struct Start {
    board: Board,
    done_moves: Vec<CheckerMove>,
}

impl Stage for Start {
    fn white_checkers(&self) -> Checkers { self.board.white_checkers }
    fn black_checkers(&self) -> Checkers { self.board.black_checkers }
    fn active_side(&self) -> Option<Side> { None }
    fn dice_pair(&self) -> Option<DicePair> { None }
    fn taken_checker_pip(&self) -> Option<Pip> { None }
}

impl Start {
    pub fn new(board: Board, done_moves: Vec<CheckerMove>) -> Self {
        Self {
            board, done_moves
        }
    }

    pub fn throw_dices(self) -> DicesThrown {
        /* generate random dices until dices are equal */
        let dice_pair: DicePair = loop {
            let first_dice: u8 = rand::thread_rng().gen_range(1..=6);
            let second_dice: u8 = rand::thread_rng().gen_range(1..=6);

            if first_dice != second_dice {
                break DicePair::new(first_dice, second_dice);
            }
        };

        let active_side: Side =
            if dice_pair.first() > dice_pair.second() {
                Side::White
            } else {
                Side::Black
            };

        DicesThrown::new(self.board, self.done_moves, active_side, dice_pair)
    }
}
