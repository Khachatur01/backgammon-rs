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
            }
        }
    }
}

impl StageView {
    fn get_top_left_row(&self) -> Row {
        let half_width: usize = *self.theme.half_width;

        Row {
            range: Range {
                start: 0,
                end: half_width
            },
            y: 1
        }
    }

    fn get_top_right_row(&self) -> Row {
        let half_width: usize = *self.theme.half_width;

        Row {
            range: Range {
                start: half_width + 1,
                end: half_width + 2 + half_width - 1
            },
            y: 1
        }
    }

    fn get_bottom_left_row(&self) -> Row {
        let half_width: usize = *self.theme.half_width;
        let height: usize = *self.theme.height;

        Row {
            range: Range {
                start: 0,
                end: half_width
            },
            y: height
        }
    }

    fn get_bottom_right_row(&self) -> Row {
        let half_width: usize = *self.theme.half_width;
        let height: usize = *self.theme.height;

        Row {
            range: Range {
                start: half_width + 1,
                end: half_width + 2 + half_width - 1
            },
            y: height
        }
    }
}
