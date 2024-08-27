mod display;
mod dices;
mod r#move;
mod checkers;

use crate::game::constant::player::Player;
use crossterm::style::Stylize;
use std::fmt::Display;
use crate::game::board::dices::Dices;
use crate::game::board::display::{Image, Render};
use checkers::Checkers;
use crate::game::constant::error::{BearOffError, MoveError, TakeError};
use crate::game::constant::PIPS_SIZE;

/**
Board pips schema (Looking from player 1 point of view):

            +--< 11  10   9   8   7   6       5   4   3   2   1   0  <- Player 1
            |    23  22  21  20  19  18      17  16  15  14  13  12 <--+
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
            +--> 12  13  14  15  16  17      18  19  20  21  22  23    |
    Player 2 ->   0   1   2   3   4   5       6   7   8   9  10  11 >--+


Expanded:

    Player 1:
        0   1   2   3   4   5       6   7   8   9  10  11 >--+ +--< 12  13  14  15  16  17      18  19  20  21  22  23
    Player 2:
        23  22  21  20  19  18     17  16  15  15  13  12 >--+ +--< 11  10  9   8   7   6       5   4   3   2   1   0
*/
pub struct Board {
    white_checkers: Checkers,
    black_checkers: Checkers,
    active_player: Option<Player>,
    dices: Option<Dices>,
    taken_checker: Option<usize>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            white_checkers: Default::default(),
            black_checkers: Default::default(),
            active_player: None,
            dices: None,
            taken_checker: None
        }
    }

    pub fn set_active_player(&mut self, active_player: Player) {
        self.active_player = Some(active_player);
    }

    pub fn set_dices(&mut self, dices: Dices) {
        let _ = self.active_player.expect(
            "Can't set dices.\
                Active playes is not set.\
                Call set_active_player(...) first."
        );

        self.dices = Some(dices);
    }

    pub fn take_checker(&mut self, from: usize) -> Result<(), TakeError> {
        let active_player: Player = self.active_player.expect(
            "Can't grab checker from {from}.\
                Active playes is not set.\
                Call set_active_player(...) first."
        );

        let (active_player_checkers, opponent_checkers) = match active_player {
            Player::White => (&mut self.white_checkers, &self.black_checkers),
            Player::Black => (&mut self.black_checkers, &self.white_checkers)
        };

        if opponent_checkers.on_board[from] != 0 {
            return Err(TakeError::TakingOpponentPip);
        }

        if active_player_checkers.on_board[from] == 0 {
            return Err(TakeError::NotEnoughCheckers);
        }

        self.taken_checker = Some(from);

        Ok(())
    }

    pub fn release_checker(&mut self) {
        self.taken_checker = None;
    }

    pub fn move_checker(&mut self, to: usize) -> Result<(), MoveError> {
        let active_player: Player = self.active_player.expect(
            "Can't grab checker from {from}.\
                Active playes is not set.\
                Call set_active_player(...) first."
        );

        let from: usize = self.taken_checker.expect(
            "Can't move checker to position {to}.\
                No checker grabbed yet.\
                Call 'grab_checker(...)' method first"
        );

        if from >= PIPS_SIZE || to >= PIPS_SIZE {
            panic!(
                "Invalid 'from' or 'to' parameter used.\
                Valid vales range is [0, 24).\
                Values used: from - {from}, to - {to}"
            );
        }

        let (active_player_checkers, opponent_checkers) = match active_player {
            Player::White => (&mut self.white_checkers, &self.black_checkers),
            Player::Black => (&mut self.black_checkers, &self.white_checkers)
        };

        if opponent_checkers.on_board[to] != 0 {
            return Err(MoveError::PlaceIsTakenByOpponent);
        }

        active_player_checkers.on_board[from] -= 1;
        active_player_checkers.on_board[to] += 1;

        Ok(())
    }

    pub fn bear_off_checker(&mut self) -> Result<(), BearOffError> {
        let active_player: Player = self.active_player.expect(
            "Can't grab checker from {from}.\
                Active playes is not set.\
                Call set_active_player(...) first."
        );

        let (active_player_checkers) = match active_player {
            Player::White => &mut self.white_checkers,
            Player::Black => &mut self.black_checkers
        };

        let not_all_checkers_are_in_home: bool = active_player_checkers.on_board[0..(PIPS_SIZE - 6)]
            .iter()
            .filter(|checkers| **checkers == 0)
            .count() != 0;

        if not_all_checkers_are_in_home {
            return Err(BearOffError::NotAllCheckersAreInHome);
        }

        /* If we reached to this point, then 'from' is a valid value in range [18, 24) */
        let from: usize = self.taken_checker.expect(
            "Can't move checker to position {to}.\
                No checker grabbed yet.\
                Call 'grab_checker(...)' method first"
        );

        active_player_checkers.on_board[from] -= 1;
        active_player_checkers.bore_off_count += 1;

        Ok(())
    }
}

impl Render for Board {
    fn render(&self, for_player: Player) -> Image {
        let (active_player_checkers, opponent_checkers) = match for_player {
            Player::White => (&self.white_checkers, &self.black_checkers),
            Player::Black => (&self.black_checkers, &self.white_checkers)
        };

        Image::new(100, 100)
    }
}
