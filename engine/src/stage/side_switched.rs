use crate::board::checkers::Checkers;
use crate::board::Board;
use crate::constant::player::Side;
use crate::stage::after_throwing_dices::AfterThrowingDices;
use crate::stage::dices_thrown::DicesThrown;
use crate::stage::no_possible_moves::NoPossibleMoves;
use crate::stage::Stage;
use crate::types::checker_move::CheckerMove;
use crate::types::dice_pair::DicePair;
use crate::types::pip::Pip;
use rand::Rng;

pub struct SideSwitched {
    board: Board,
    done_moves: Vec<CheckerMove>,
    active_side: Side,
    dice_pair: DicePair,
}

impl Stage for SideSwitched {
    fn white_checkers(&self) -> Checkers { self.board.white_checkers }
    fn black_checkers(&self) -> Checkers { self.board.black_checkers }
    fn active_side(&self) -> Option<Side> { Some(self.active_side) }
    fn dice_pair(&self) -> Option<DicePair> { Some(self.dice_pair) }
    fn taken_checker_pip(&self) -> Option<Pip> { None }
    fn focused_pip(&self) -> Option<Pip> { None }
}

impl SideSwitched {
    pub fn new(board: Board,
               done_moves: Vec<CheckerMove>,
               active_side: Side,
               dice_pair: DicePair) -> Self {

        Self {
            board,
            done_moves,
            active_side,
            dice_pair
        }
    }

    pub fn throw_dices(self) -> AfterThrowingDices {
        let dice_pair: DicePair = {
            let first_dice: u8 = rand::thread_rng().gen_range(1..=6);
            let second_dice: u8 = rand::thread_rng().gen_range(1..=6);

            DicePair::new(first_dice, second_dice)
        };

        let possible_moves: Vec<CheckerMove> = self.board.get_possible_moves(
            self.active_side,
            dice_pair,
            self.done_moves.as_slice()
        );

        let next_stage: AfterThrowingDices = if possible_moves.len() == 0 {
            AfterThrowingDices::NoPossibleMoves(
                NoPossibleMoves::new(self.board, self.done_moves, self.active_side, self.dice_pair)
            )
        } else {
            AfterThrowingDices::DicesThrown(
                DicesThrown::new(self.board, self.done_moves, self.active_side, dice_pair)
            )
        };

        next_stage
    }
}
