mod row;

use std::ops::Range;
use crate::stage_theme::StageTheme;
use cursive::{Printer, Vec2, View};
use engine::board::checkers::Checkers;
use engine::constant::player::Side;
use engine::stage::Stage;
use engine::types::dice_pair::DicePair;
use engine::types::pip::Pip;
use crate::custom_view::stage_view::row::Row;

pub struct StageView {
    white_checkers: Checkers,
    black_checkers: Checkers,
    active_side: Option<Side>,
    dice_pair: Option<DicePair>,
    taken_checker_pip: Option<Pip>,
    theme: StageTheme,
}

impl View for StageView {
    fn draw(&self, printer: &Printer) {
        /* TODO */

        self.render_borders(printer);
        self.render_separators(printer);
        self.render_checkers(printer);

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

impl StageView {
    pub fn from<T: Stage>(stage: T, theme: StageTheme) -> Self {
        Self {
            white_checkers: stage.white_checkers(),
            black_checkers: stage.black_checkers(),
            active_side: stage.active_side(),
            dice_pair: stage.dice_pair(),
            taken_checker_pip: stage.taken_checker_pip(),
            theme
        }
    }

    fn render_borders(&self, printer: &Printer) {
        let half_width: usize = *self.theme.half_width;
        let height: usize = *self.theme.height;
        let board_border: char = self.theme.board_border;

        /* Calculations */
        let (horizontal_border_length, vertical_border_length) = self.theme.get_max_size();

        let board_border: String = board_border.to_string();
        let board_border: &str = board_border.as_str();

        let horizontal_border: String = std::iter::repeat(board_border)
            .take(horizontal_border_length - 3)
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

            /* 2 separators in the middle */
            printer.print(
                (half_width, row),
                board_border
            );
            printer.print(
                (half_width + 1, row),
                board_border
            );

            /* Right border */
            printer.print(
                (half_width * 2 + 1, row),
                board_border
            );
        }

        /* Top bottom */
        printer.print((0, height + 1), horizontal_border);
    }

    fn render_separators(&self, printer: &Printer) {
        let half_width: usize = *self.theme.half_width;
        let pips_separator: char = self.theme.pips_separator;

        let pips_separator: String = pips_separator.to_string();

        let pip_size: usize = half_width / 6;

        let rows: [Row; 4] = [
            self.get_top_left_row(),
            self.get_top_right_row(),
            self.get_bottom_left_row(),
            self.get_bottom_right_row(),
        ];

        for row in rows {
            for separator_x in row.range.step_by(pip_size).skip(1) {
                printer.print(
                    (separator_x, row.y),
                    &pips_separator
                );

                printer.print(
                    (separator_x, row.y),
                    &pips_separator
                );
            }
        }
    }

    fn render_checkers(&self, printer: &Printer) {
        let half_width: usize = *self.theme.half_width;
        let white_checker: char = self.theme.white_checker;
        let black_checker: char = self.theme.black_checker;

        let white_checker: String = white_checker.to_string();
        let black_checker: String = black_checker.to_string();

        let pip_size: usize = half_width / 6;

        let top_left_row: Row = self.get_top_left_row();
        let top_right_row: Row = self.get_top_right_row();
        let bottom_left_row: Row = self.get_bottom_left_row();
        let bottom_right_row: Row = self.get_bottom_right_row();

        let (active_side_checkers,
            active_checker,
            opponent_side_checkers,
            opponent_checker
        ) = match self.active_side {
            Some(Side::White) | None => (
                &self.white_checkers,
                white_checker,
                &self.black_checkers,
                black_checker,
            ),
            Some(Side::Black) => (
                &self.black_checkers,
                black_checker,
                &self.white_checkers,
                white_checker,
            ),
        };

        ////////////////////////////////////////////////////////////////////////////////////////////
        let peaces = top_left_row
            .range
            .step_by(pip_size)
            .map(|x| x + pip_size / 2)
            .enumerate();

        for (index, separator_x) in peaces {
            let active_side_checkers: usize = active_side_checkers.on_board[index + 12] as usize;
            let opponent_side_checkers: usize = opponent_side_checkers.on_board[index] as usize;

            for checker in 0..active_side_checkers {
                printer.print(
                    (separator_x, top_left_row.y + checker),
                    &active_checker
                );
            }

            for checker in 0..opponent_side_checkers {
                printer.print(
                    (separator_x, top_left_row.y + checker),
                    &opponent_checker
                );
            }
        }

        ////////////////////////////////////////////////////////////////////////////////////////////
        let peaces = top_right_row
            .range
            .step_by(pip_size)
            .map(|x| x + pip_size / 2)
            .enumerate();

        for (index, separator_x) in peaces {
            let active_side_checkers: usize = active_side_checkers.on_board[6 + index + 12] as usize;
            let opponent_side_checkers: usize = opponent_side_checkers.on_board[6 + index] as usize;

            for checker in 0..active_side_checkers {
                printer.print(
                    (separator_x, top_right_row.y + checker),
                    &active_checker
                );
            }

            for checker in 0..opponent_side_checkers {
                printer.print(
                    (separator_x, top_right_row.y + checker),
                    &opponent_checker
                );
            }
        }

        ////////////////////////////////////////////////////////////////////////////////////////////
        let peaces = bottom_left_row
            .range
            .step_by(pip_size)
            .map(|x| x + pip_size / 2)
            .enumerate();

        for (index, separator_x) in peaces {
            let active_side_checkers: usize = active_side_checkers.on_board[11 - index] as usize;
            let opponent_side_checkers: usize = opponent_side_checkers.on_board[23 - index] as usize;

            for checker in 0..active_side_checkers {
                printer.print(
                    (separator_x, bottom_left_row.y - checker),
                    &active_checker
                );
            }

            for checker in 0..opponent_side_checkers {
                printer.print(
                    (separator_x, bottom_left_row.y - checker),
                    &opponent_checker
                );
            }
        }

        ////////////////////////////////////////////////////////////////////////////////////////////
        let peaces = bottom_right_row
            .range
            .step_by(pip_size)
            .map(|x| x + pip_size / 2)
            .enumerate();

        for (index, separator_x) in peaces {
            let active_side_checkers: usize = active_side_checkers.on_board[11 - index - 6] as usize;
            let opponent_side_checkers: usize = opponent_side_checkers.on_board[23 - index - 6] as usize;

            for checker in 0..active_side_checkers {
                printer.print(
                    (separator_x, bottom_right_row.y - checker),
                    &active_checker
                );
            }

            for checker in 0..opponent_side_checkers {
                printer.print(
                    (separator_x, bottom_right_row.y - checker),
                    &opponent_checker
                );
            }
        }
    }
}

impl StageView {
    fn get_top_left_row(&self) -> Row {
        Row {
            range: self.get_left_range(),
            y: 1
        }
    }
    fn get_top_right_row(&self) -> Row {
        Row {
            range: self.get_right_range(),
            y: 1
        }
    }

    fn get_bottom_left_row(&self) -> Row {
        let height: usize = *self.theme.height;

        Row {
            range: self.get_left_range(),
            y: height
        }
    }
    fn get_bottom_right_row(&self) -> Row {
        let height: usize = *self.theme.height;

        Row {
            range: self.get_right_range(),
            y: height
        }
    }

    fn get_left_range(&self) -> Range<usize> {
        let half_width: usize = *self.theme.half_width;

        Range {
            start: 0,
            end: half_width
        }
    }
    fn get_right_range(&self) -> Range<usize> {
        let half_width: usize = *self.theme.half_width;

        Range {
            start: half_width + 1,
            end: half_width + 2 + half_width - 1
        }
    }
}

impl StageView {
    fn get_top_checkers(&self) -> Vec<u8> {
        let (active_side_checkers, opponent_side_checkers) = match self.active_side {
            Some(Side::White) | None => (&self.white_checkers, &self.black_checkers),
            Some(Side::Black) => (&self.black_checkers, &self.white_checkers),
        };



        vec![]
    }
}
