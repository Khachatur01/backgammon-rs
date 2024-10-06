mod row;

use crate::custom_view::stage_view::row::Row;
use crate::stage_theme::StageTheme;
use cursive::event::Event;
use cursive::reexports::ahash::HashMapExt;
use cursive::{Printer, Vec2, View};
use engine::board::checkers::Checkers;
use engine::constant::player::Side;
use engine::constant::PIPS_SIZE;
use engine::stage::Stage;
use engine::types::dice_pair::DicePair;
use engine::types::pip::Pip;
use std::ops::Range;

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
    pub fn from<T: Stage>(stage: &T, theme: StageTheme, render_for: Side) -> Self {
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
        let half_width: usize = *self.theme.half_width;
        let height: usize = *self.theme.height;
        let board_border: char = self.theme.board_border;
        let bore_off_column_width: usize = self.theme.bore_off_column_width;

        /* Calculations */
        let (horizontal_border_length, vertical_border_length) = self.theme.get_max_size();

        let board_border: String = board_border.to_string();
        let board_border: &str = board_border.as_str();

        let horizontal_border: String = std::iter::repeat(board_border)
            .take(horizontal_border_length - 2)
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
                (1 + bore_off_column_width + half_width, row),
                board_border
            );
            printer.print(
                (1 + bore_off_column_width + half_width + 1, row),
                board_border
            );

            /* Right border */
            printer.print(
                (1 + bore_off_column_width + half_width * 2 + 1, row),
                board_border
            );
            printer.print(
                (1 + bore_off_column_width + half_width * 2 + 1 + bore_off_column_width + 1, row),
                board_border
            );
        }

        /* Bottom border */
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

    fn render_board_checkers(&self, printer: &Printer) {
        let half_width: usize = *self.theme.half_width;
        let white_checker: char = self.theme.white_checker;
        let black_checker: char = self.theme.black_checker;

        let white_checker: String = white_checker.to_string();
        let black_checker: String = black_checker.to_string();

        let pip_size: usize = half_width / 6;
        let board_height: isize = *self.theme.height as isize;
        let cut_off_height_percent: isize = *self.theme.peaces_cut_off_height_percent as isize;

        let (active_side_checkers,
            active_checker,
            opponent_side_checkers,
            opponent_checker
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

        let render_peaces = |row: Row,
                             get_active_side_index: Box<dyn Fn(usize) -> usize>,
                             get_opponent_side_index: Box<dyn Fn(usize) -> usize>,
                             direction: isize| {

            let peaces = row
                .range
                .step_by(pip_size)
                .map(|x| x + pip_size / 2)
                .enumerate();

            let row_y: isize = row.y as isize;

            for (index, separator_x) in peaces {
                /* define a closure to render peaces of single side */
                let render_side_peaces = |checkers_count: isize, checker_view: &String| {
                    let cut_off_count: isize = isize::min(checkers_count, board_height * cut_off_height_percent / 100);

                    for checker_index in 0..cut_off_count {
                        printer.print(
                            (separator_x, (row_y + checker_index * direction) as usize),
                            &checker_view
                        );
                    }

                    if checkers_count > cut_off_count {
                        printer.print(
                            (separator_x, (row_y + cut_off_count * direction) as usize),
                            &self.theme.numbers[(checkers_count - 1) as usize].to_string()
                        );
                    }
                };

                let active_side_index: usize = get_active_side_index(index);
                let opponent_side_index: usize = get_opponent_side_index(index);

                let active_side_checkers_count: isize = active_side_checkers.on_board[active_side_index] as isize;
                let opponent_side_checkers_count: isize = opponent_side_checkers.on_board[opponent_side_index] as isize;

                render_side_peaces(active_side_checkers_count, &active_checker);
                render_side_peaces(opponent_side_checkers_count, &opponent_checker);
            }
        };

        let top_left_row: Row = self.get_top_left_row();
        let top_right_row: Row = self.get_top_right_row();
        let bottom_left_row: Row = self.get_bottom_left_row();
        let bottom_right_row: Row = self.get_bottom_right_row();

        let pips_size: usize = PIPS_SIZE as usize;
        let pips_size_half: usize = pips_size / 2;
        let pips_size_quarter: usize = pips_size / 4;

        render_peaces(
            top_left_row,
            Box::new(|index| pips_size_half + index),
            Box::new(|index| index),
            1
        );
        render_peaces(
            top_right_row,
            Box::new(|index| pips_size_half + (index + pips_size_quarter)),
            Box::new(|index| index + pips_size_quarter),
            1
        );

        render_peaces(
            bottom_left_row,
            Box::new(|index| pips_size_half - 1 - index),
            Box::new(|index| pips_size - 1 - index),
            -1
        );
        render_peaces(
            bottom_right_row,
            Box::new(|index| pips_size_half - 1 - (index + pips_size_quarter)),
            Box::new(|index| pips_size - 1 - (index + pips_size_quarter)),
            -1
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
        /* TODO */
        if let Some(focused_pip) = self.focused_pip {
            printer.print(
                (4, 4),
                &format!("{}", &*focused_pip)
            );
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
        let bore_off_width: usize = self.theme.bore_off_column_width;

        Range {
            start: 1 + bore_off_width,
            end: half_width
        }
    }
    fn get_right_range(&self) -> Range<usize> {
        let half_width: usize = *self.theme.half_width;
        let bore_off_width: usize = self.theme.bore_off_column_width;

        Range {
            start: 1 + bore_off_width + half_width + 1,
            end: 1 + bore_off_width + half_width + 2 + half_width - 1
        }
    }
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
