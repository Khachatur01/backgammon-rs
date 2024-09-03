use crate::backgammon::board::checkers::Checkers;
use crate::backgammon::constant::player::Side;
use crate::backgammon::constant::PIPS_SIZE;
use crate::backgammon::types::checker_move::CheckerMove;
use crate::backgammon::types::checker_move::CheckerMove::{BearOff, Step};
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
#[derive(Default)]
pub struct Board {
    white_checkers: Checkers,
    black_checkers: Checkers,
}

impl Board {
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

    pub fn get_possible_moves(&self, for_side: Side, available_steps: &[u8]) -> Vec<CheckerMove> {
        let mut moves: Vec<CheckerMove> = Vec::new();

        let active_side_checkers: &Checkers = match for_side {
            Side::White => &self.white_checkers,
            Side::Black => &self.black_checkers
        };

        for (index, checkers_in_pip) in active_side_checkers.on_board.iter().enumerate() {
            let index = index as u8;

            if *checkers_in_pip == 0 {
                continue;
            }

            let mut moves_from_index: Vec<CheckerMove> = self.get_possible_moves_from(
                for_side,
                available_steps,
                Pip::new(index)
            );

            moves.append(&mut moves_from_index);
        }

        moves
    }

    pub fn get_possible_moves_from(&self,
                                   for_side: Side,
                                   available_steps: &[u8],
                                   from: Pip) -> Vec<CheckerMove> {

        /* playing from the head */
        if *from == PIPS_SIZE - 1 {

        }

        /* TODO */
        vec![]
    }

    pub fn move_checker(&mut self, for_side: Side, checker_move: CheckerMove) {
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

        let opponent_pip: Pip = self.to_opponent_pip(pip);

        if opponent_side_checkers.on_board[*opponent_pip as usize] != 0 {
            return CheckerAvailability::ReferringToOpponentPip;
        };

        if active_side_checkers.on_board[*pip as usize] == 0 {
            return CheckerAvailability::NoCheckerFound;
        };

        CheckerAvailability::Available
    }

    fn to_opponent_pip(&self, pip: Pip) -> Pip {
        let half_count: u8 = PIPS_SIZE / 2;

        if *pip >= half_count {
            Pip::new(*pip - half_count)
        } else {
            Pip::new(*pip + half_count)
        }
    }
}
