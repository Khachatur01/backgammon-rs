use crate::backgammon::board::Board;
use crate::backgammon::constant::player::Side;
use crate::backgammon::types::dices::DicePair;
use crate::backgammon::types::r#move::Move;
use crate::constant::error::{BearOffError, CommitError, MoveError, SwitchSideError, TakeError, ThrowDicesError};
use crate::types::from_pip::FromPip;
use crate::types::to_pip::ToPip;
use rand::Rng;
use crate::constant::error::SwitchSideError::ActiveSideDoesntFinishPlaying;
use crate::constant::error::ThrowDicesError::{DicesAlreadyThrown, GameDoesntNotStartedYet};
use crate::types::action::Action;

pub mod constant;
pub mod board;
pub mod types;

pub type Result<E> = std::result::Result<(), E>;

/**
    If we have active side, then game is already started.
    If we have winner side, then game is already over.
*/
pub struct Backgammon {
    active_side: Option<Side>,
    winner_side: Option<Side>,

    board: Board,

    last_action: Option<Action>,

    taken_checker: Option<FromPip>,
    dices: Option<DicePair>,
    done_moves: Vec<Move>
}

impl Backgammon {
    pub fn new() -> Self {
        Self {
            active_side: None,
            winner_side: None,

            board: Board::new(),

            last_action: None,

            taken_checker: None,
            dices: None,
            done_moves: vec![]
        }
    }

    pub fn throw_first_dices(&mut self) {
        if self.last_action.is_some() {
            panic!("Can't throw first dices. Game already started");
        }

        /* generate random dices until dices are equal */
        let dice_pair: DicePair = loop {
            let first_dice: u8 = rand::thread_rng().gen_range(1..=6);
            let second_dice: u8 = rand::thread_rng().gen_range(1..=6);

            if first_dice != second_dice {
                break DicePair::new(first_dice, second_dice);
            }
        };

        self.active_side =
            if dice_pair.0 > dice_pair.1 {
                Some(Side::White)
            } else {
                Some(Side::Black)
            };

        let action: Action = Action::ThrowFirstDices(dice_pair);

        self.last_action = Some(action);
    }

    pub fn take_checker(&mut self, from: FromPip) -> Result<TakeError> {
        Ok(())
    }

    pub fn move_checker(&mut self, to: ToPip) -> Result<MoveError> {
        match self.last_action.as_ref() {
            Some(Action::TakeChecker(_)) => {}
            _ => {
                return Err(MoveError::NoCheckerTaken)
            }
        }

        Ok(())
    }

    pub fn bear_off_checker(&mut self) -> Result<BearOffError> {
        match self.last_action.as_ref() {
            Some(Action::TakeChecker(_)) => {}
            _ => {
                return Err(BearOffError::NoCheckerTaken)
            }
        }

        Ok(())
    }

    pub fn commit_moves(&mut self) -> Result<CommitError> {
        self.last_action = Some(Action::CommitMoves);

        Ok(())
    }

    pub fn cancel_moves(&mut self) {
        self.last_action = Some(Action::CommitMoves);
    }

    pub fn switch_side(&mut self) -> Result<SwitchSideError> {
        if self.dices.is_some() {
            return Err(ActiveSideDoesntFinishPlaying);
        }

        /* toggle active side */
        self.active_side =
            if let Some(Side::White) = self.active_side {
                Some(Side::Black)
            } else {
                Some(Side::White)
            };

        Ok(())
    }

    pub fn throw_dices(&mut self) -> Result<ThrowDicesError> {
        if self.dices.is_some() {
            return Err(DicesAlreadyThrown);
        };

        if self.active_side.is_none() {
            return Err(GameDoesntNotStartedYet);
        }

        let first_dice: u8 = rand::thread_rng().gen_range(1..=6);
        let second_dice: u8 = rand::thread_rng().gen_range(1..=6);

        self.dices = Some(DicePair::new(first_dice, second_dice));

        Ok(())
    }
}
