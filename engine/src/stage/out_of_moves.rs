use crate::board::checkers::Checkers;
use crate::board::Board;
use crate::constant::error::CommitError;
use crate::constant::player::Side;
use crate::stage::dices_thrown::DicesThrown;
use crate::stage::moves_commited::MovesCommited;
use crate::stage::side_switched::SideSwitched;
use crate::stage::win::Win;
use crate::stage::Stage;
use crate::types::checker_move::CheckerMove;
use crate::types::dice_pair::DicePair;
use crate::types::pip::Pip;

pub struct OutOfMoves {
    board: Board,
    done_moves: Vec<CheckerMove>,
    active_side: Side,
    dice_pair: DicePair,
}

impl Stage for OutOfMoves {
    fn white_checkers(&self) -> Checkers { self.board.white_checkers }
    fn black_checkers(&self) -> Checkers { self.board.black_checkers }
    fn active_side(&self) -> Option<Side> { Some(self.active_side) }
    fn dice_pair(&self) -> Option<DicePair> { Some(self.dice_pair) }
    fn taken_checker_pip(&self) -> Option<Pip> { None }
    fn focused_pip(&self) -> Option<Pip> { None }
    fn possible_moves(&self) -> Option<Vec<CheckerMove>> { None }
}

impl OutOfMoves {
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

    pub fn commit_moves(self) -> Result<MovesCommited, CommitError<Self>> {
        if self.board.is_opponent_blocked(self.active_side) {
            return Err(CommitError::OpponentBlocked(self));
        }

        if !self.board.are_all_dices_played(self.active_side, self.dice_pair, &self.done_moves) {
            return Err(CommitError::NotAllDicesPlayed(self));
        }

        let next_stage: MovesCommited = if self.board.has_checkers(self.active_side) {
            /* Switch active side */
            let new_active_side: Side = match self.active_side {
                Side::White => Side::Black,
                Side::Black => Side::White,
            };

            MovesCommited::SideSwitched(
                SideSwitched::new(self.board, self.done_moves, new_active_side, self.dice_pair)
            )
        } else {
            MovesCommited::Win(
                Win::new(self.board, self.active_side, self.dice_pair)
            )
        };

        Ok(next_stage)
    }

    pub fn cancel_moves(mut self) -> DicesThrown {
        self.done_moves
            .iter()
            .for_each(|checker_move: &CheckerMove|
                self.board.undo_move(self.active_side, *checker_move)
            );

        DicesThrown::new(self.board, self.done_moves, self.active_side, self.dice_pair)
    }
}
