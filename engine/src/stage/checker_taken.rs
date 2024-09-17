use crate::stage::dices_thrown::DicesThrown;
use crate::stage::out_of_moves::OutOfMoves;
use crate::board::checkers::Checkers;
use crate::board::Board;
use crate::constant::error::{BearOffError, MoveError};
use crate::constant::player::Side;
use crate::constant::result::CheckerAvailability;
use crate::stage::checker_moved::CheckerMoved;
use crate::stage::Stage;
use crate::types::checker_move::CheckerMove;
use crate::types::dice_pair::DicePair;
use crate::types::pip::Pip;

pub struct CheckerTaken {
    board: Board,
    done_moves: Vec<CheckerMove>,
    from_pip: Pip,
    active_side: Side,
    dice_pair: DicePair,
}

impl Stage for CheckerTaken {
    fn white_checkers(&self) -> Checkers { self.board.white_checkers }
    fn black_checkers(&self) -> Checkers { self.board.black_checkers }
    fn active_side(&self) -> Option<Side> { Some(self.active_side) }
    fn dice_pair(&self) -> Option<DicePair> { Some(self.dice_pair) }
    fn taken_checker_pip(&self) -> Option<Pip> { Some(self.from_pip) }
}

impl CheckerTaken {
    pub fn new(board: Board,
               done_moves: Vec<CheckerMove>,
               from_pip: Pip,
               active_side: Side,
               dice_pair: DicePair) -> Self {

        Self {
            board,
            done_moves,
            from_pip,
            active_side,
            dice_pair
        }
    }

    pub fn play_checker(mut self, to_pip: Pip) -> Result<CheckerMoved, MoveError> {
        let from_pip: Pip = self.from_pip;
        let play: CheckerMove = CheckerMove::Play(from_pip, to_pip);

        self.check_target_pip_availability(to_pip)?;
        self.check_if_blocking_opponent(play)?;
        if self.check_move_possibility(play).is_err() {
            return Err(MoveError::InconsistentWithDices)
        };

        let next_stage: CheckerMoved = self.move_checker(play);

        Ok(next_stage)
    }

    pub fn bear_off_checker(mut self) -> Result<CheckerMoved, BearOffError> {
        let active_side: Side = self.active_side;
        let from_pip: Pip = self.from_pip;
        let bear_off: CheckerMove = CheckerMove::BearOff(from_pip);

        if !self.board.are_all_checkers_in_home(active_side) {
            return Err(BearOffError::NotAllCheckersAreInHome);
        }

        if self.check_move_possibility(bear_off).is_err() {
            return Err(BearOffError::InconsistentWithDices)
        };

        let next_stage: CheckerMoved = self.move_checker(CheckerMove::BearOff(from_pip));

        Ok(next_stage)
    }

    pub fn cancel(self) -> DicesThrown {
        DicesThrown::new(self.board, self.done_moves, self.active_side, self.dice_pair)
    }

    fn check_target_pip_availability(&self, to_pip: Pip) -> Result<(), MoveError> {
        match self.board.get_checker_availability(self.active_side, Pip::from(to_pip)) {
            CheckerAvailability::ReferringToOpponentPip => Err(MoveError::PipIsOccupiedByOpponent),
            _ => Ok(())
        }
    }

    fn check_move_possibility(&self, checker_move: CheckerMove) -> Result<(), ()> {
        let from_pip = match checker_move {
            CheckerMove::Play(from_pip, _) => from_pip,
            CheckerMove::BearOff(from_pip) => from_pip,
        };

        let mut possible_moves: Vec<CheckerMove> = self.board.get_possible_moves_from(
            self.active_side,
            self.dice_pair,
            self.done_moves.as_slice(),
            from_pip,
        );

        let move_is_possible: bool = possible_moves
            .iter()
            .any(|current_checker_move: &CheckerMove|
                *current_checker_move == checker_move
            );

        if !move_is_possible {
            return Err(());
        }

        Ok(())
    }

    fn check_if_blocking_opponent(&self, checker_move: CheckerMove) -> Result<(), MoveError> {
        let is_blocking: bool = match checker_move {
            CheckerMove::Play(from_pip, to_pip) =>
                self.board.is_blocking_opponent(self.active_side, from_pip, to_pip),
            CheckerMove::BearOff(_) => false,
        };

        if is_blocking {
            return Err(MoveError::BlockingOpponent);
        }

        Ok(())
    }

    fn move_checker(mut self, checker_move: CheckerMove) -> CheckerMoved {
        self.board.move_checker(self.active_side, checker_move);
        self.done_moves.push(checker_move);

        let out_of_moves: bool = self.board.get_possible_moves(
            self.active_side,
            self.dice_pair,
            self.done_moves.as_slice()
        ).len() == 0;

        if out_of_moves {
            CheckerMoved::OutOfMoves(
                OutOfMoves::new(self.board, self.done_moves, self.active_side, self.dice_pair)
            )
        } else {
            CheckerMoved::DicesThrown(
                DicesThrown::new(self.board, self.done_moves, self.active_side, self.dice_pair)
            )
        }
    }
}
