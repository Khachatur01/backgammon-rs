use crate::backgammon::board::checkers::Checkers;
use crate::backgammon::constant::PIPS_SIZE;
use crate::backgammon::constant::player::Side;
use crate::backgammon::types::from_pip::FromPip;
use crate::backgammon::types::r#move::Move;
use crate::backgammon::types::r#move::Move::{BearOff, Step};
use crate::constant::result::CheckerAvailability;
use crate::types::pip::Pip;

mod checkers;

/**
Board pips schema (Looking from player 1 point of view):

            +--> 12  13  14  15  16  17      18  19  20  21  22  23  <- Player 1
            |    0   1   2   3   4   5       6   7   8   9  10  11 >---+
            |                                                          |
            |                                                          |
            |                                                          |
            |                                                          |
            |                                                          |
            |                                                          |
            |                                                          |
            |                                                          |
            |                                                          |
            |                                                          |
            |                                                          |
            +--> 11  10   9   8   7   6       5   4   3   2   1   0    |
    Player 2 ->  23  22  21  20  19  18      17  16  15  14  13  12 <--+


Expanded:

    Player 1:
        23  22  21  20  19  18     17  16  15  15  13  12 >--+ +--< 11  10  9   8   7   6       5   4   3   2   1   0
    Player 2:
        11  10  9   8   7   6      5   4   3   2   1   0  >--+ +--< 23  22  21  20  19  18      17  16  15  14  13  12
*/
pub struct Board {
    white_checkers: Checkers,
    black_checkers: Checkers,
}

impl Board {
    pub fn new() -> Self {
        Self {
            white_checkers: Default::default(),
            black_checkers: Default::default(),
        }
    }

    pub fn calculate_pip_count_score(&self, for_side: Side) -> u16 {
        let active_side_checkers: &Checkers = match for_side {
            Side::White => &self.white_checkers,
            Side::Black => &self.black_checkers
        };

        active_side_checkers.on_board
            .iter()
            .enumerate()
            .fold(0, |score, (pip_index, checkers_in_pip)| {
                let score_for_pip: u16 = (pip_index as u16 + 1) * (*checkers_in_pip) as u16;

                score + score_for_pip
            })
    }

    pub fn get_possible_moves(&self, for_side: Side, available_steps: &[u8]) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        let active_side_checkers: &Checkers = match for_side {
            Side::White => &self.white_checkers,
            Side::Black => &self.black_checkers
        };

        for (index, checkers_in_pip) in active_side_checkers.on_board.iter().enumerate() {
            let index = index as u8;

            if *checkers_in_pip == 0 {
                continue;
            }

            let mut moves_from_index: Vec<Move> = self.get_possible_moves_from(
                for_side,
                available_steps,
                FromPip::new(index)
            );

            moves.append(&mut moves_from_index);
        }

        moves
    }

    pub fn get_possible_moves_from(&self,
                                   for_side: Side,
                                   available_steps: &[u8],
                                   from: FromPip) -> Vec<Move> {

        /* playing from the head */
        if *from == PIPS_SIZE - 1 {

        }

        /* TODO */
        vec![]
    }

    pub fn move_checker(&mut self, for_side: Side, checker_move: Move) {
        let active_side_checkers: &mut Checkers = match for_side {
            Side::White => &mut self.white_checkers,
            Side::Black => &mut self.black_checkers
        };

        match checker_move {
            Step(from, to) => {
                active_side_checkers.on_board[*from as usize] -= 1;
                active_side_checkers.on_board[*to as usize] += 1;
            }
            BearOff(from) => {
                active_side_checkers.on_board[*from as usize] -= 1;
                active_side_checkers.bore_off_count += 1;
            }
        }
    }

    pub fn get_checker_availability(&self, for_side: Side, pip: Pip) -> CheckerAvailability {
        let (active_side_checkers, opponent_side_checkers) = match for_side {
            Side::White => (&self.white_checkers, &self.black_checkers),
            Side::Black => (&self.black_checkers, &self.white_checkers)
        };

        if opponent_side_checkers.on_board[*pip as usize] != 0 {
            return CheckerAvailability::ReferringToOpponentPip;
        };

        if active_side_checkers.on_board[*pip as usize] == 0 {
            return CheckerAvailability::NoCheckerFound;
        };

        CheckerAvailability::Available
    }
}
