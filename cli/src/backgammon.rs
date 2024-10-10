use crate::custom_view::stage_view::render_for::RenderFor;
use crate::custom_view::stage_view::StageView;
use crate::stage_theme::StageTheme;
use cursive::event::{Event, Key};
use engine::constant::PIPS_SIZE;
use engine::stage::{PossibleStage, Stage};
use engine::types::pip::Pip;
use engine::{stage, start_game};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use engine::constant::error::TakeError;
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
            PossibleStage::AfterThrowingDices(after_throwing_dices) => {
                PossibleStage::AfterThrowingDices(after_throwing_dices)
            }
            PossibleStage::CheckerTaken(checker_taken) => {
                PossibleStage::CheckerTaken(checker_taken)
            }
            PossibleStage::CheckerMoved(checker_moved) => {
                PossibleStage::CheckerMoved(checker_moved)
            }
            PossibleStage::NoPossibleMoves(no_possible_moves) => {
                PossibleStage::NoPossibleMoves(no_possible_moves)
            }
            PossibleStage::OutOfMoves(out_of_moves) => {
                PossibleStage::OutOfMoves(out_of_moves)
            }
            PossibleStage::MovesCommited(moves_commited) => {
                PossibleStage::MovesCommited(moves_commited)
            }
            PossibleStage::SideSwitched(side_switched) => {
                PossibleStage::SideSwitched(side_switched)
            }
            PossibleStage::Win(win) => {
                PossibleStage::Win(win)
            }
        }
    }


    fn on_focus_pip(&self, current_stage: PossibleStage, direction: Key) -> PossibleStage {
        fn get_pip_to_focus(focused_pip: Pip, direction: Key) -> Result<Pip, String> {
            let focused_pip: u8 = *focused_pip;

            let result_pip = match direction {
                Key::Left => {
                    if focused_pip == PIPS_SIZE / 2 || focused_pip == PIPS_SIZE / 2 - 1 {
                        Pip::new(focused_pip)
                    } else if focused_pip < PIPS_SIZE / 2 {
                        Pip::new(focused_pip + 1)
                    } else {
                        Pip::new(focused_pip - 1)
                    }
                }
                Key::Right => {
                    if focused_pip == 0 || focused_pip == PIPS_SIZE - 1 {
                        Pip::new(focused_pip)
                    } else if focused_pip < PIPS_SIZE / 2 {
                        Pip::new(focused_pip - 1)
                    } else {
                        Pip::new(focused_pip + 1)
                    }
                }
                Key::Down => {
                    if focused_pip < PIPS_SIZE / 2 {
                        Pip::new(focused_pip)
                    } else {
                        /* get the pip in front of focused pip */
                        Pip::new((PIPS_SIZE / 2) - 1 - (focused_pip - PIPS_SIZE / 2))
                    }
                }
                Key::Up => {
                    if focused_pip >= PIPS_SIZE / 2 {
                        Pip::new(focused_pip)
                    } else {
                        /* get the pip in front of focused pip */
                        Pip::new((PIPS_SIZE / 2) + (PIPS_SIZE / 2 - 1 - focused_pip))
                    }
                },
                _ => {
                    return Err(format!("Invalid focus direction {:?}", direction))
                }
            };

            Ok(result_pip)
        }

        if let PossibleStage::DicesThrown(mut dices_thrown_stage) = current_stage {
            if let Some(focused_pip) = dices_thrown_stage.focused_pip() {
                if let Ok(pip_to_focus) = get_pip_to_focus(focused_pip, direction) {
                    dices_thrown_stage.focus_pip(pip_to_focus);

                    self.send_view(&dices_thrown_stage);
                }
            }
            return PossibleStage::DicesThrown(dices_thrown_stage);
        }

        current_stage
    }
}
