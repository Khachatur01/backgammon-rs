mod row;
pub mod render_for;

use crate::custom_view::stage_view::render_for::RenderFor;
use crate::stage_theme::StageTheme;
use cursive::event::Event;
use cursive::reexports::ahash::HashMapExt;
use cursive::{Printer, Vec2, View};
use engine::board::checkers::Checkers;
use engine::constant::player::Side;
use engine::constant::{PIPS_SIZE};
use engine::stage::Stage;
use engine::types::dice_pair::DicePair;
use engine::types::pip::Pip;
use std::usize;

type EventHandler = Box<dyn Fn(Event) -> () + Send + Sync + 'static>;

pub struct StageView {
    white_checkers: Checkers,
    black_checkers: Checkers,
    active_side: Option<Side>,
    dice_pair: Option<DicePair>,
    taken_checker_pip: Option<Pip>,
    focused_pip: Option<Pip>,

    render_for: Side,
    theme: StageTheme,
}

impl StageView {
    pub fn from<T: Stage>(stage: &T, theme: StageTheme, render_for: RenderFor) -> Self {
        let render_for = match render_for {
            RenderFor::WhiteSide => { Side::White },
            RenderFor::BlackSide => { Side::Black },
            RenderFor::ActiveSide => { stage.active_side().unwrap_or(Side::White) }
        };

        Self {
            white_checkers: stage.white_checkers(),
            black_checkers: stage.black_checkers(),
            active_side: stage.active_side(),
            dice_pair: stage.dice_pair(),
            taken_checker_pip: stage.taken_checker_pip(),
            focused_pip: stage.focused_pip(),

            render_for,
            theme
        }
    }

    fn render_borders(&self, printer: &Printer) {
        let half_width: usize = self.theme.get_half_width();
        let height: usize = *self.theme.height;
        let board_border: char = self.theme.board_border;
        let bore_off_column_width: usize = self.theme.bore_off_column_width;

        /* Calculations */
        let (horizontal_border_length, vertical_border_length) = self.theme.get_max_size();

        let board_border: String = board_border.to_string();
        let board_border: &str = board_border.as_str();

        let horizontal_border: String = std::iter::repeat(board_border)
            .take(horizontal_border_length)
            .collect::<String>();

        let horizontal_border: &str = horizontal_border.as_str();

        /* Rendering */

        /* Top border */
        printer.print((0, 0), horizontal_border);

        for row in 0..vertical_border_length {
            /* Left border */
            printer.print(
                (0, row),
                board_border
            );
            printer.print(
                (1 + bore_off_column_width, row),
                board_border
            );

            /* 2 separators in the middle */
            printer.print(
                (1 + bore_off_column_width + 1 + half_width, row),
                board_border
            );
            printer.print(
                (1 + bore_off_column_width + 1 + half_width + 1, row),
                board_border
            );

            /* Right border */
            printer.print(
                (1 + bore_off_column_width + 1 + half_width + 2 + half_width, row),
                board_border
            );
            printer.print(
                (1 + bore_off_column_width + 1 + half_width + 2 + half_width + bore_off_column_width + 1, row),
                board_border
            );
        }

        /* Bottom border */
        printer.print((0, height + 1), horizontal_border);
    }

    fn render_separators(&self, printer: &Printer) {
        let pips_separator: char = self.theme.pips_separator;
        let pips_separator: String = pips_separator.to_string();

        let first_board_left: u8 = PIPS_SIZE / 4;
        let second_board_left: u8 = PIPS_SIZE / 2;
        let third_board_left: u8 = first_board_left + second_board_left;
        let pip_size: usize = self.theme.pip_size as usize;

        /* FIXME */
        let pips_range = (0..PIPS_SIZE)
            .filter(|pip_index|
                *pip_index != 0 &&
                *pip_index != 6 &&
                *pip_index != 17 &&
                *pip_index != 23
            );

        for pip in pips_range {
            if pip < second_board_left {
                let (physical_left, board_left) =
                    if pip < first_board_left {
                        /* 5   4   3   2   1   0 */ /* bottom right */
                        (self.get_right_board_physical_left(), first_board_left)
                    } else {
                        /* 11  10   9   8   7   6 */ /* bottom left */
                         (self.get_left_board_physical_left(), second_board_left)
                    };

                let n = (board_left - pip) as usize;
                let x: usize = physical_left + pip_size * n + (n - 1);

                printer.print(
                    (x, *self.theme.height),
                    &pips_separator
                );
            } else {
                let (physical_left, board_left) =
                    if pip < third_board_left {
                        /* 12  13  14  15  16  17 */ /* top left */
                        (self.get_left_board_physical_left(), second_board_left)
                    } else {
                        /* 18  19  20  21  22  23 */ /* top right */
                        (self.get_right_board_physical_left(), third_board_left)
                    };

                let n: usize = (pip - board_left + 1) as usize;
                let x: usize = physical_left + pip_size * n + (n - 1);

                printer.print(
                    (x, 1),
                    &pips_separator
                );
            }
        }
    }

    fn render_board_checkers(&self, printer: &Printer) {
        fn render_checkers(this: &StageView,
                           checkers: Checkers,
                           checker_view: String,
                           printer: &Printer,
                           max_checkers_to_show: usize,
                           get_position: impl Fn(&StageView, Pip, usize) -> (usize, usize)) {

            for (pip, active_side_checkers_in_pip) in checkers.on_board.iter().enumerate() {
                let active_side_checkers_in_pip: usize = *active_side_checkers_in_pip as usize;
                let cut_off_count: usize = usize::min(active_side_checkers_in_pip, max_checkers_to_show);

                for column in 0..cut_off_count {
                    let pip: Pip = Pip::new(pip as u8);
                    let position: (usize, usize) = get_position(this, pip, column);
                    printer.print(
                        position,
                        &checker_view
                    );
                }

                if active_side_checkers_in_pip > cut_off_count {
                    let pip: Pip = Pip::new(pip as u8);
                    let position: (usize, usize) = get_position(this, pip, cut_off_count);

                    printer.print(
                        position,
                        &this.theme.numbers[active_side_checkers_in_pip - 1].to_string()
                    );
                }
            }
        }

        let white_checker: char = self.theme.white_checker;
        let white_checker: String = white_checker.to_string();

        let black_checker: char = self.theme.black_checker;
        let black_checker: String = black_checker.to_string();

        let max_checkers_to_show: usize = self.theme.get_max_checkers_to_show();

        let (active_side_checkers,
            active_side_checker_view,
            opponent_side_checkers,
            opponent_checker_view
        ) = match self.render_for {
            Side::White => (
                self.white_checkers,
                white_checker,
                self.black_checkers,
                black_checker,
            ),
            Side::Black => (
                self.black_checkers,
                black_checker,
                self.white_checkers,
                white_checker,
            ),
        };

        render_checkers(self,
                        active_side_checkers,
                        active_side_checker_view,
                        printer,
                        max_checkers_to_show,
                        |_, pip, column| {
                            self.get_position_for_active_side(pip, column)
                        }
        );

        render_checkers(self,
                        opponent_side_checkers,
                        opponent_checker_view,
                        printer,
                        max_checkers_to_show,
                        |_, pip, column| {
                            self.get_position_for_opponent(pip, column)
                        }
        );
    }

    fn render_bore_off_checkers(&self, printer: &Printer) {
        /* TODO */
    }

    fn render_dices(&self, printer: &Printer) {
        /* TODO */
        if let Some(dice_pair) = self.dice_pair {
            printer.print(
                (5, 5),
                &format!("{} {}", &dice_pair.first(), &dice_pair.second())
            );
        }
    }

    fn render_hints(&self, printer: &Printer) {
        /* TODO */
    }

    fn render_focused_pip(&self, printer: &Printer) {
        if let Some(focused_pip) = self.focused_pip {
            let (active_side_checkers, opponent_side_checkers) = match self.render_for {
                Side::White => (
                    self.white_checkers,
                    self.black_checkers,
                ),
                Side::Black => (
                    self.black_checkers,
                    self.white_checkers,
                ),
            };

            let active_side_checkers_count: usize = active_side_checkers.on_board[*focused_pip as usize] as usize;

            let focused_pip_for_opponent: Pip = self.get_opponent_pip(focused_pip);
            let opponent_side_checkers_count: usize = opponent_side_checkers.on_board[*focused_pip_for_opponent as usize] as usize;

            let max_checkers_to_show: usize = self.theme.get_max_checkers_to_show();

            let position: (usize, usize) =
                if active_side_checkers_count > opponent_side_checkers_count {
                    let cut_off_count: usize = usize::min(active_side_checkers_count, max_checkers_to_show);

                    self.get_position_for_active_side(
                        focused_pip,
                        cut_off_count + 1
                    )
                } else {
                    let cut_off_count: usize = usize::min(opponent_side_checkers_count, max_checkers_to_show);

                    self.get_position_for_opponent(
                        focused_pip_for_opponent,
                        cut_off_count + 1
                    )
                };

            let focused_pip: char = self.theme.focused_pip;
            let focused_pip: String = focused_pip.to_string();

            printer.print(
                position,
                focused_pip.as_str()
            );
        }
    }
}

impl StageView {
    fn get_position_for_active_side(&self, pip: Pip, column: usize) -> (usize, usize) {
        let pip_size: usize = self.theme.pip_size as usize;

        let first_board_left: u8 = PIPS_SIZE / 4;
        let second_board_left: u8 = PIPS_SIZE / 2;
        let third_board_left: u8 = first_board_left + second_board_left;

        let pip: u8 = *pip;

        if pip < first_board_left { /* 5   4   3   2   1   0 */ /* bottom right */
            let physical_left: usize = self.get_right_board_physical_left();
            let offset: usize = (first_board_left - 1 - pip) as usize * (pip_size + 1) + pip_size / 2;

            (physical_left + offset, *self.theme.height - column)
        } else if pip >= first_board_left && pip < second_board_left { /* 11  10   9   8   7   6 */ /* bottom left */
            let physical_left: usize = self.get_left_board_physical_left();
            let offset: usize = (second_board_left - 1 - pip) as usize * (pip_size + 1) + pip_size / 2;

            (physical_left + offset, *self.theme.height - column)
        } else if pip >= second_board_left && pip < third_board_left { /* 12  13  14  15  16  17 */ /* top left */
            let physical_left: usize = self.get_left_board_physical_left();
            let offset: usize = (pip - second_board_left) as usize * (pip_size + 1) + pip_size / 2;

            (physical_left + offset, 1 + column)
        } else if pip >= third_board_left { /* 18  19  20  21  22  23 */ /* top right */
            let physical_left: usize = self.get_right_board_physical_left();
            let offset: usize = (pip - third_board_left) as usize * (pip_size + 1) + pip_size / 2;

            (physical_left + offset, 1 + column)
        } else {
            unreachable!()
        }
    }

    fn get_position_for_opponent(&self, pip: Pip, column: usize) -> (usize, usize) {
        let opponent_pip: Pip = self.get_opponent_pip(pip);

        self.get_position_for_active_side(opponent_pip, column)
    }

    fn get_opponent_pip(&self, pip: Pip) -> Pip {
        if *pip < PIPS_SIZE / 2 {
            Pip::new(*pip + PIPS_SIZE / 2)
        } else {
            Pip::new(*pip - PIPS_SIZE / 2)
        }
    }

    fn get_left_board_physical_left(&self) -> usize {
        1 + self.theme.bore_off_column_width + 1
    }

    fn get_right_board_physical_left(&self) -> usize {
        1 + self.theme.bore_off_column_width + 1 + self.theme.get_half_width() + 2
    }

    // fn get_top_left_row(&self) -> Row {
    //     Row {
    //         range: self.get_left_range(),
    //         y: 1
    //     }
    // }
    // fn get_top_right_row(&self) -> Row {
    //     Row {
    //         range: self.get_right_range(),
    //         y: 1
    //     }
    // }
    //
    // fn get_bottom_left_row(&self) -> Row {
    //     let height: usize = *self.theme.height;
    //
    //     Row {
    //         range: self.get_left_range(),
    //         y: height
    //     }
    // }
    // fn get_bottom_right_row(&self) -> Row {
    //     let height: usize = *self.theme.height;
    //
    //     Row {
    //         range: self.get_right_range(),
    //         y: height
    //     }
    // }
    //
    // fn get_left_range(&self) -> Range<usize> {
    //     let half_width: usize = *self.theme.half_width;
    //     let bore_off_width: usize = self.theme.bore_off_column_width;
    //
    //     Range {
    //         start: 1 + bore_off_width,
    //         end: half_width
    //     }
    // }
    // fn get_right_range(&self) -> Range<usize> {
    //     let half_width: usize = *self.theme.half_width;
    //     let bore_off_width: usize = self.theme.bore_off_column_width;
    //
    //     Range {
    //         start: 1 + bore_off_width + half_width + 1,
    //         end: 1 + bore_off_width + half_width + 2 + half_width - 1
    //     }
    // }
}

impl View for StageView {
    fn draw(&self, printer: &Printer) {
        self.render_borders(printer);
        self.render_separators(printer);
        self.render_board_checkers(printer);
        self.render_bore_off_checkers(printer);
        self.render_dices(printer);
        self.render_hints(printer);
        self.render_focused_pip(printer);

        // printer.with_color(ColorStyle::title_primary(), |printer| {
        //     printer.print(
        //         (0, 0),
        //         &"123456789abcdef"
        //     );
        //     printer.print(
        //         (0, 1),
        //         &"123456789abcdef"
        //     )
        // });
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        let (width, height) = self.theme.get_max_size();

        Vec2::new(width, height)
    }
}
