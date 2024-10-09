use crate::board::checkers::Checkers;
use crate::constant::player::Side;
use crate::constant::result::CheckerAvailability;
use crate::constant::PIPS_SIZE;
use crate::types::checker_move::CheckerMove;
use crate::types::checker_move::CheckerMove::{BearOff, Play};
use crate::types::dice_pair::DicePair;
use crate::types::pip::Pip;

pub mod checkers;

/**
Board pips schema (Looking from point of view Active Player):

            +--> 12  13  14  15  16  17      18  19  20  21  22  23  <- Active Player
            |    0   1   2   3   4   5       6   7   8   9   10  11 >---+
            |                                                           |
            |                                                           |
            |                                                           |
            |                                                           |
            |                                                           |
            |                                                           |
            |                                                           |
            |                                                           |
            |                                                           |
            |                                                           |
            |                                                           |
            +--> 11  10   9   8   7   6       5   4   3   2   1   0     |
    Opponent ->  23  22  21  20  19  18      17  16  15  14  13  12 <---+


Expanded:

    Active Player:
        23  22  21  20  19  18     17  16  15  15  13  12 >--+ +--< 11  10  9   8   7   6       5   4   3   2   1   0
    Opponent:
        11  10  9   8   7   6      5   4   3   2   1   0  >--+ +--< 23  22  21  20  19  18      17  16  15  14  13  12
*/
#[derive(Default)]
pub struct Board {
    pub(crate) white_checkers: Checkers,
    pub(crate) black_checkers: Checkers,
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

    pub fn get_possible_moves(&self, for_side: Side, dice_pair: DicePair, done_moves: &[CheckerMove]) -> Vec<CheckerMove> {
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
                dice_pair,
                done_moves,
                Pip::new(index)
            );

            moves.append(&mut moves_from_index);
        }

        moves
    }

    pub fn get_possible_moves_from(&self,
                                   for_side: Side,
                                   dice_pair: DicePair,
                                   done_moves: &[CheckerMove],
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
            Play(from, to) => {
                active_side_checkers.on_board[*from as usize] -= 1;
                active_side_checkers.on_board[*to as usize] += 1;
            }
            BearOff(from) => {
                active_side_checkers.on_board[*from as usize] -= 1;
                active_side_checkers.bore_off_count += 1;
            }
        }
    }

    pub fn undo_move(&mut self, for_side: Side, checker_move: CheckerMove) {
        let active_side_checkers: &mut Checkers = match for_side {
            Side::White => &mut self.white_checkers,
            Side::Black => &mut self.black_checkers
        };

        match checker_move {
            Play(from, to) => {
                active_side_checkers.on_board[*from as usize] += 1;
                active_side_checkers.on_board[*to as usize] -= 1;
            }
            BearOff(from) => {
                active_side_checkers.on_board[*from as usize] += 1;
                active_side_checkers.bore_off_count -= 1;
            }
        };
    }

    pub fn are_all_checkers_in_home(&self, for_side: Side) -> bool {
        let active_side_checkers: &Checkers = match for_side {
            Side::White => &self.white_checkers,
            Side::Black => &self.black_checkers
        };

        let mut non_home_pips_range = (PIPS_SIZE / 4)..PIPS_SIZE;

        let has_checker_outside_home: bool = non_home_pips_range.any(|pip_index: u8| {
            active_side_checkers.on_board[pip_index as usize] != 0
        });

        !has_checker_outside_home
    }

    pub fn is_blocking_opponent(&self, for_side: Side, from_pip: Pip, to_pip: Pip) -> bool {
        let opponent_pip: Pip = self.to_opponent_pip(to_pip);

        if self.has_checker_after_pip(for_side.opponent(), opponent_pip) {
            return false;
        }

        let active_side_checkers: &Checkers = match for_side {
            Side::White => &self.white_checkers,
            Side::Black => &self.black_checkers
        };

        let target_pip_index: usize = *to_pip as usize;

        let right_side: &[u8] = &active_side_checkers.on_board[0..target_pip_index];
        let left_side: &[u8] = &active_side_checkers.on_board[target_pip_index..PIPS_SIZE as usize];

        let mut continuous_pieces_count: u8 = 0;

        for (index, pip) in left_side.iter().rev().enumerate() {
            if (*from_pip == index as u8 && *pip == 1) || *pip == 0 {
                break;
            }

            continuous_pieces_count += 1;
        }

        for (index, pip) in right_side.iter().enumerate() {
            if (*from_pip == index as u8 && *pip == 1) || *pip == 0 {
                break;
            }

            continuous_pieces_count += 1;
        }

        continuous_pieces_count > 5
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

    pub fn has_checkers(&self, for_side: Side) -> bool {
        let active_side_checkers: &Checkers = match for_side {
            Side::White => &self.white_checkers,
            Side::Black => &self.black_checkers
        };

        active_side_checkers
            .on_board
            .iter()
            .any(|checkers: &u8| *checkers != 0)
    }

    fn to_opponent_pip(&self, pip: Pip) -> Pip {
        let half_count: u8 = PIPS_SIZE / 2;

        if *pip >= half_count {
            Pip::new(*pip - half_count)
        } else {
            Pip::new(*pip + half_count)
        }
    }

    fn has_checker_after_pip(&self, for_side: Side, pip: Pip) -> bool {
        let mut active_side_checkers: &Checkers = match for_side {
            Side::White => &self.white_checkers,
            Side::Black => &self.black_checkers
        };

        active_side_checkers
            .on_board[*pip as usize..PIPS_SIZE as usize]
            .iter()
            .any(|checkers: &u8| *checkers > 0)
    }
}
