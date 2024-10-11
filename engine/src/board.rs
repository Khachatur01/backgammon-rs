use crate::board::checkers::Checkers;
use crate::constant::player::Side;
use crate::constant::result::CheckerAvailability;
use crate::constant::{CHECKER_PER_PLAYER, MAX_PIPS, PIPS_PER_HALF_BOARD};
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

/* public interface */
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

            let mut moves_from_index: Vec<CheckerMove> = self.get_possible_moves_from_pip(
                for_side,
                dice_pair,
                done_moves,
                Pip::new(index)
            );

            moves.append(&mut moves_from_index);
        }

        moves
    }

    pub fn get_possible_moves_from_pip(&self,
                                       for_side: Side,
                                       dice_pair: DicePair,
                                       done_moves: &[CheckerMove],
                                       from_pip: Pip) -> Vec<CheckerMove> {

        let is_playing_from_head: bool = *from_pip == MAX_PIPS - 1;
        let can_play_from_head: bool = self.can_play_from_head(for_side, dice_pair);

        if is_playing_from_head && !can_play_from_head {
            return vec![];
        }

        let potential_steps: Vec<u8> = if dice_pair.first() == dice_pair.second() {
            (1..=4)
                .map(|dice_index| dice_index * dice_pair.first())
                .collect()
        } else {
            vec![
                dice_pair.first(),
                dice_pair.second(),
                dice_pair.first() + dice_pair.second()
            ]
        };

        let potential_target_pips: Vec<Pip> = potential_steps
            .into_iter()
            .map(|step| *from_pip - step)
            .filter(|pip_index| *pip_index < MAX_PIPS)
            .map(|pip_index| Pip::new(pip_index))
            .collect();

        /* TODO */
        potential_target_pips
            .iter()
            .map(|to_pip: &Pip| Play(from_pip, *to_pip))
            .collect()
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

        let mut non_home_pips_range = (MAX_PIPS / 4)..MAX_PIPS;

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
        let left_side: &[u8] = &active_side_checkers.on_board[target_pip_index..MAX_PIPS as usize];

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
}

/* private logic */
impl Board {
    fn opponent_has_checker_in_pip(&self, for_side: Side, pip: Pip) -> bool {
        let opponent_pip: Pip = self.to_opponent_pip(pip);

        let opponent_checkers: &Checkers = match for_side {
            Side::White => &self.black_checkers,
            Side::Black => &self.white_checkers
        };

        opponent_checkers.on_board[*opponent_pip as usize] != 0
    }

    fn to_opponent_pip(&self, pip: Pip) -> Pip {
        let half_count: u8 = MAX_PIPS / 2;

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
            .on_board[*pip as usize..MAX_PIPS as usize]
            .iter()
            .any(|checkers: &u8| *checkers > 0)
    }

    fn can_play_from_head(&self, for_side: Side, dice_pair: DicePair) -> bool {
        let active_side_checkers: &Checkers = match for_side {
            Side::White => &self.white_checkers,
            Side::Black => &self.black_checkers
        };

        /* if not played from head yet */
        if active_side_checkers.on_board[MAX_PIPS as usize - 1] == CHECKER_PER_PLAYER {
            return true;
        }

        /* if already played from head, but dices are not equal */
        if dice_pair.first() != dice_pair.second() {
            return false;
        }

        /* if opponent checker can be found on the road of dice moves */
        let dice_can_not_be_fully_played: bool = (1..=4)
            .filter(|i| i * dice_pair.first() < MAX_PIPS)
            .map(|i| Pip::new(i * dice_pair.first()))
            .find(|pip| self.opponent_has_checker_in_pip(for_side, *pip))
            .is_some();


        /* if already played from head and dices are equal and dices can't be fully played */
        dice_can_not_be_fully_played
    }
}
