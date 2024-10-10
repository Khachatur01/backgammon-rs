use crate::custom_view::stage_view::render_for::RenderFor;
use crate::custom_view::stage_view::StageView;
use crate::stage_theme::StageTheme;
use cursive::event::{Event, Key};
use engine::constant::MAX_PIPS;
use engine::stage::{PossibleStage, Stage};
use engine::types::pip::Pip;
use engine::{stage, start_game};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use engine::constant::error::{MoveError, TakeError};
use engine::stage::checker_moved::CheckerMoved;
use engine::stage::dices_thrown::DicesThrown;

pub struct Backgammon {
    stage_theme: StageTheme,
    render_for: RenderFor,
    view_sender: Sender<StageView>
}

impl Backgammon {
    pub fn run(stage_theme: StageTheme, render_for: RenderFor) -> (Sender<Event>, Receiver<StageView>) {
        let (view_sender, view_receiver) = mpsc::channel();
        let (event_sender, event_receiver) = mpsc::channel();

        let this: Self = Self {
            stage_theme, render_for, view_sender
        };

        thread::spawn(move || {
            this.start(event_receiver);
        });

        (event_sender, view_receiver)
    }

    fn start(self, mut event_receiver: Receiver<Event>) {
        let started_stage: stage::started::Started = start_game();
        let start_stage_view: StageView = StageView::from(&started_stage, self.stage_theme, self.render_for);

        self.view_sender.clone().send(start_stage_view).unwrap_or(());

        let mut current_stage = PossibleStage::Started(started_stage);

        while let Ok(event) = event_receiver.recv() {
            match event {
                Event::Key(Key::Enter) => {
                    current_stage = self.on_enter(current_stage);
                }
                Event::Key(Key::Esc) => {
                    current_stage = self.on_esc(current_stage);
                }
                Event::Key(key @ (Key::Left | Key::Right | Key::Down | Key::Up)) => {
                    current_stage = self.on_focus_pip(current_stage, key);
                }
                _ => {}
            }
        }
    }

    fn send_view(&self, stage: &impl Stage) {
        let dices_thrown_stage_view: StageView = StageView::from(stage, self.stage_theme, self.render_for);
        self.view_sender.clone().send(dices_thrown_stage_view).unwrap_or(());
    }
}


/* event handlers */
impl Backgammon {
    fn on_enter(&self, current_stage: PossibleStage) -> PossibleStage {
        match current_stage {
            PossibleStage::Started(started_stage) => {
                let dices_thrown_stage: DicesThrown = started_stage.throw_dices();
                self.send_view(&dices_thrown_stage);

                PossibleStage::DicesThrown(dices_thrown_stage)
            }
            PossibleStage::DicesThrown(dices_thrown_stage) => {
                match dices_thrown_stage.take_checker() {
                    Ok(checker_taken_stage) => {
                        self.send_view(&checker_taken_stage);

                        PossibleStage::CheckerTaken(checker_taken_stage)
                    }
                    Err(take_error) => {
                        match take_error {
                            TakeError::NotEnoughCheckers(dices_thrown_stage) |
                            TakeError::TakingOpponentPip(dices_thrown_stage) =>
                                PossibleStage::DicesThrown(dices_thrown_stage)
                        }
                    }
                }
            }
            PossibleStage::CheckerTaken(checker_taken_stage) => {
                match checker_taken_stage.play_checker() {
                    Ok(checker_moved_stage) => {
                        match checker_moved_stage {
                            CheckerMoved::DicesThrown(dices_thrown_stage) => {
                                self.send_view(&dices_thrown_stage);

                                PossibleStage::DicesThrown(dices_thrown_stage)
                            }
                            CheckerMoved::OutOfMoves(out_of_moves_stage) => {
                                self.send_view(&out_of_moves_stage);

                                PossibleStage::OutOfMoves(out_of_moves_stage)
                            }
                        }
                    }
                    Err(move_error) => {
                        match move_error {
                            MoveError::BlockingOpponent(checker_taken_stage) |
                            MoveError::PipIsOccupiedByOpponent(checker_taken_stage) |
                            MoveError::InconsistentWithDices(checker_taken_stage) =>
                                PossibleStage::CheckerTaken(checker_taken_stage)
                        }
                    }
                }
            }
            _ => current_stage,
        }
    }
    fn on_esc(&self, current_stage: PossibleStage) -> PossibleStage {
        match current_stage {
            PossibleStage::CheckerTaken(checker_taken) => {
                let dices_thrown_stage: DicesThrown = checker_taken.cancel();
                self.send_view(&dices_thrown_stage);

                PossibleStage::DicesThrown(dices_thrown_stage)
            }
            _ => current_stage,
        }
    }

    fn on_focus_pip(&self, current_stage: PossibleStage, direction: Key) -> PossibleStage {
        fn get_pip_to_focus(focused_pip: Pip, direction: Key) -> Result<Pip, String> {
            let focused_pip: u8 = *focused_pip;

            let result_pip = match direction {
                Key::Left => {
                    if focused_pip == MAX_PIPS / 2 || focused_pip == MAX_PIPS / 2 - 1 {
                        Pip::new(focused_pip)
                    } else if focused_pip < MAX_PIPS / 2 {
                        Pip::new(focused_pip + 1)
                    } else {
                        Pip::new(focused_pip - 1)
                    }
                }
                Key::Right => {
                    if focused_pip == 0 || focused_pip == MAX_PIPS - 1 {
                        Pip::new(focused_pip)
                    } else if focused_pip < MAX_PIPS / 2 {
                        Pip::new(focused_pip - 1)
                    } else {
                        Pip::new(focused_pip + 1)
                    }
                }
                Key::Down => {
                    if focused_pip < MAX_PIPS / 2 {
                        Pip::new(focused_pip)
                    } else {
                        /* get the pip in front of focused pip */
                        Pip::new((MAX_PIPS / 2) - 1 - (focused_pip - MAX_PIPS / 2))
                    }
                }
                Key::Up => {
                    if focused_pip >= MAX_PIPS / 2 {
                        Pip::new(focused_pip)
                    } else {
                        /* get the pip in front of focused pip */
                        Pip::new((MAX_PIPS / 2) + (MAX_PIPS / 2 - 1 - focused_pip))
                    }
                },
                _ => {
                    return Err(format!("Invalid focus direction {:?}", direction))
                }
            };

            Ok(result_pip)
        }

        match current_stage {
            PossibleStage::DicesThrown(mut dices_thrown_stage) => {
                if let Some(focused_pip) = dices_thrown_stage.focused_pip() {
                    if let Ok(pip_to_focus) = get_pip_to_focus(focused_pip, direction) {
                        dices_thrown_stage.focus_pip(pip_to_focus);

                        self.send_view(&dices_thrown_stage);
                    }
                }
                PossibleStage::DicesThrown(dices_thrown_stage)
            }
            PossibleStage::CheckerTaken(mut checker_taken_stage) => {
                if let Some(focused_pip) = checker_taken_stage.focused_pip() {
                    if let Ok(pip_to_focus) = get_pip_to_focus(focused_pip, direction) {
                        checker_taken_stage.focus_pip(pip_to_focus);

                        self.send_view(&checker_taken_stage);
                    }
                }
                PossibleStage::CheckerTaken(checker_taken_stage)
            }
            _ => current_stage
        }
    }
}
