mod row;
pub mod render_for;

use crate::custom_view::stage_view::render_for::RenderFor;
use crate::stage_theme::StageTheme;
use cursive::reexports::ahash::HashMapExt;
use cursive::{Printer, Vec2, View};
use engine::board::checkers::Checkers;
use engine::constant::player::Side;
use engine::constant::{BOTTOM_LEFT_BOARD_LEFT_PIP, BOTTOM_LEFT_BOARD_RIGHT_PIP, BOTTOM_RIGHT_BOARD_LEFT_PIP, BOTTOM_RIGHT_BOARD_RIGHT_PIP, MAX_PIPS, TOP_LEFT_BOARD_LEFT_PIP, TOP_LEFT_BOARD_RIGHT_PIP, TOP_RIGHT_BOARD_LEFT_PIP, TOP_RIGHT_BOARD_RIGHT_PIP};
use engine::stage::Stage;
use engine::types::checker_move::CheckerMove;
use engine::types::dice_pair::DicePair;
use engine::types::pip::Pip;
use std::fmt::Pointer;
use std::usize;

type PipStack = Vec<char>;
type PipsStack = [PipStack; MAX_PIPS as usize];

pub struct StageView {
    white_checkers: Checkers,
    black_checkers: Checkers,
    active_side: Option<Side>,
    dice_pair: Option<DicePair>,
    taken_checker_pip: Option<Pip>,
    focused_pip: Option<Pip>,
    possible_moves: Option<Vec<CheckerMove>>,

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
            possible_moves: stage.possible_moves(),

            render_for,
            theme,
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

        let pip_size: usize = self.theme.pip_size as usize;

        let pips_range = (0..MAX_PIPS)
            /* filter all right border pips to avoid rendering separator for them */
            .filter(|pip_index|
                ![
                    TOP_RIGHT_BOARD_RIGHT_PIP,
                    TOP_LEFT_BOARD_RIGHT_PIP,
                    BOTTOM_LEFT_BOARD_RIGHT_PIP,
                    BOTTOM_RIGHT_BOARD_RIGHT_PIP
                ].contains(pip_index)
            );

        for pip in pips_range {
            let (physical_left, pip_step, y_position) = if pip < TOP_LEFT_BOARD_LEFT_PIP {
                let (physical_left, board_left_pip) =
                    if pip < BOTTOM_LEFT_BOARD_RIGHT_PIP {
                        /* 5   4   3   2   1   0 */ /* bottom right */
                        (self.get_right_board_physical_left(), BOTTOM_RIGHT_BOARD_LEFT_PIP)
                    } else {
                        /* 11  10   9   8   7   6 */ /* bottom left */
                         (self.get_left_board_physical_left(), BOTTOM_LEFT_BOARD_LEFT_PIP)
                    };

                let pip_step: usize = (board_left_pip - pip) as usize;
                (physical_left, pip_step, *self.theme.height)
            } else {
                let (physical_left, board_left_pip) =
                    if pip < TOP_RIGHT_BOARD_LEFT_PIP {
                        /* 12  13  14  15  16  17 */ /* top left */
                        (self.get_left_board_physical_left(), TOP_LEFT_BOARD_LEFT_PIP)
                    } else {
                        /* 18  19  20  21  22  23 */ /* top right */
                        (self.get_right_board_physical_left(), TOP_RIGHT_BOARD_LEFT_PIP)
                    };

                let pip_step: usize = (pip - board_left_pip) as usize;
                (physical_left, pip_step, 1)
            };

            let x_position: usize = physical_left + pip_size * (pip_step + 1) + pip_step;

            printer.print(
                (x_position, y_position),
                &pips_separator
            );
        }
    }

    fn render_pips_columns(&self, printer: &Printer, pips_columns: [PipStack; MAX_PIPS as usize]) {
        pips_columns
            .iter()
            .enumerate()
            .for_each(|(pip_index, pip_column)| {
                pip_column
                    .iter()
                    .enumerate()
                    .for_each(|(column_index, char)| {
                        let position = self.get_position_for_active_side(Pip::new(pip_index as u8), column_index);

                            let char: String = char.to_string();

                        printer.print(
                            position,
                            &char
                        )
                    });
            });
    }

    fn render_bore_off_checkers(&self, printer: &Printer) {
        /* TODO */
    }

    fn render_dices(&self, printer: &Printer) {
        if let Some(dice_pair) = self.dice_pair {
            let first_dice_view: char = self.theme.dices[(dice_pair.first() - 1) as usize];
            let first_dice_number: char = self.theme.numbers[(dice_pair.first() - 1) as usize];

            let second_dice_view: char = self.theme.dices[(dice_pair.second() - 1) as usize];
            let second_dice_number: char = self.theme.numbers[(dice_pair.second() - 1) as usize];

            let y_position: usize = (*self.theme.height) / 2 + 1;

            let left_board_middle_position: usize = self.get_left_board_physical_left() + self.theme.get_half_width() / 2;
            let right_board_middle_position: usize = self.get_right_board_physical_left() + self.theme.get_half_width() / 2;

            printer.print(
                (left_board_middle_position, y_position),
                &format!("{} {}", &first_dice_view, &first_dice_number)
            );
            printer.print(
                (right_board_middle_position, y_position),
                &format!("{} {}", &second_dice_view, &second_dice_number)
            );
        }
    }
}

impl StageView {
    fn get_position_for_active_side(&self, pip: Pip, column: usize) -> (usize, usize) {
        let pip_size: usize = self.theme.pip_size as usize;

        let pip: u8 = *pip;

        if pip < BOTTOM_LEFT_BOARD_RIGHT_PIP {
            /* 5   4   3   2   1   0 */ /* bottom right */
            let physical_left: usize = self.get_right_board_physical_left();
            let offset: usize = (BOTTOM_LEFT_BOARD_RIGHT_PIP - 1 - pip) as usize * (pip_size + 1) + pip_size / 2;

            (physical_left + offset, *self.theme.height - column)
        } else if pip >= BOTTOM_LEFT_BOARD_RIGHT_PIP && pip < TOP_LEFT_BOARD_LEFT_PIP {
            /* 11  10   9   8   7   6 */ /* bottom left */
            let physical_left: usize = self.get_left_board_physical_left();
            let offset: usize = (TOP_LEFT_BOARD_LEFT_PIP - 1 - pip) as usize * (pip_size + 1) + pip_size / 2;

            (physical_left + offset, *self.theme.height - column)
        } else if pip >= TOP_LEFT_BOARD_LEFT_PIP && pip < TOP_RIGHT_BOARD_LEFT_PIP {
            /* 12  13  14  15  16  17 */ /* top left */
            let physical_left: usize = self.get_left_board_physical_left();
            let offset: usize = (pip - TOP_LEFT_BOARD_LEFT_PIP) as usize * (pip_size + 1) + pip_size / 2;

            (physical_left + offset, 1 + column)
        } else if pip >= TOP_RIGHT_BOARD_LEFT_PIP {
            /* 18  19  20  21  22  23 */ /* top right */
            let physical_left: usize = self.get_right_board_physical_left();
            let offset: usize = (pip - TOP_RIGHT_BOARD_LEFT_PIP) as usize * (pip_size + 1) + pip_size / 2;

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
        if *pip < MAX_PIPS / 2 {
            Pip::new(*pip + MAX_PIPS / 2)
        } else {
            Pip::new(*pip - MAX_PIPS / 2)
        }
    }

    fn get_left_board_physical_left(&self) -> usize {
        1 + self.theme.bore_off_column_width + 1
    }

    fn get_right_board_physical_left(&self) -> usize {
        1 + self.theme.bore_off_column_width + 1 + self.theme.get_half_width() + 2
    }
}

impl StageView {
    fn add_board_checkers(&self, pips_stack: &mut PipsStack) {
        fn add_checkers(this: &StageView,
                        checkers: Checkers,
                        checker_view: char,
                        pips_columns: &mut PipsStack,
                        show_focused_pip: bool,
                        get_pip_index: impl Fn(usize) -> usize) {

            let max_checkers_to_show: usize = this.theme.get_max_checkers_to_show();

            for (pip_index, checkers_in_pip) in checkers.on_board.iter().enumerate() {
                let mut checkers_in_pip: usize = *checkers_in_pip as usize;

                if show_focused_pip && this.taken_checker_pip.is_some() && this.focused_pip.is_some() {
                    if *this.focused_pip.unwrap() == pip_index as u8 {
                        checkers_in_pip += 1;
                    }
                    if *this.taken_checker_pip.unwrap() == pip_index as u8 {
                        checkers_in_pip -= 1;
                    }
                }

                let checkers_cut_off_count: usize = usize::min(checkers_in_pip, max_checkers_to_show);

                let pip_index: usize = get_pip_index(pip_index);

                (0..checkers_cut_off_count).for_each(|_| {
                    pips_columns[pip_index].push(checker_view);
                });

                if checkers_in_pip > checkers_cut_off_count {
                    let checkers_count_hint: char = this.theme.numbers[checkers_in_pip - 1];

                    pips_columns[pip_index].push(checkers_count_hint);
                }
            }
        }

        let (active_side_checkers,
            active_side_checker_view,
            opponent_side_checkers,
            opponent_checker_view
        ) = match self.render_for {
            Side::White => (
                self.white_checkers,
                self.theme.white_checker,
                self.black_checkers,
                self.theme.black_checker,
            ),
            Side::Black => (
                self.black_checkers,
                self.theme.black_checker,
                self.white_checkers,
                self.theme.white_checker,
            ),
        };

        add_checkers(self,
                     active_side_checkers,
                     active_side_checker_view,
                     pips_stack,
                     true,
                     |pip| pip
        );

        add_checkers(self,
                     opponent_side_checkers,
                     opponent_checker_view,
                     pips_stack,
                     false,
                     |pip| {
                            *self.get_opponent_pip(Pip::new(pip as u8)) as usize
                        }
        );
    }

    fn add_taken_checker_hint(&self, pips_stack: &mut PipsStack) {
        let taken_checker_pip: Pip = match self.taken_checker_pip {
            Some(pip) => pip,
            None => return,
        };
        let active_side: Side = match self.active_side {
            Some(side) => side,
            None => return,
        };

        /* render hints only for active side */
        if active_side != self.render_for {
            return;
        }

        let pointer_view: char = match *taken_checker_pip {
            0..BOTTOM_LEFT_BOARD_LEFT_PIP => {
                self.theme.up
            }
            _ => self.theme.down
        };

        pips_stack[*taken_checker_pip as usize].push(pointer_view);
    }

    fn add_possible_moves_hints(&self, pips_stack: &mut PipsStack) {
        let possible_moves: &Vec<CheckerMove> =
            if let Some(possible_moves) = &self.possible_moves {
                possible_moves
            } else {
                return;
            };

        possible_moves.iter().for_each(|checker_move: &CheckerMove| {
            match checker_move {
                CheckerMove::Play(_, to_pip) => {
                    pips_stack[**to_pip as usize].push(self.theme.possible_move);
                }
                CheckerMove::BearOff(_) => { /* @TODO */ }
            }
        });
    }

    fn add_focused_pip_hint(&self, pips_stack: &mut PipsStack) {
        let focused_pip_index: usize = if let Some(focused_pip) = self.focused_pip {
            *focused_pip as usize
        } else {
            return;
        };

        pips_stack[focused_pip_index].push(self.theme.focused_pip);
    }
}

impl View for StageView {
    fn draw(&self, printer: &Printer) {
        let mut pips_stack: PipsStack = std::array::from_fn(|_| Vec::new());

        self.add_board_checkers(&mut pips_stack);
        self.add_focused_pip_hint(&mut pips_stack);
        self.add_taken_checker_hint(&mut pips_stack);
        self.add_possible_moves_hints(&mut pips_stack);

        self.render_borders(printer);
        self.render_separators(printer);
        self.render_pips_columns(printer, pips_stack);
        self.render_bore_off_checkers(printer);
        self.render_dices(printer);
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        let (width, height) = self.theme.get_max_size();

        Vec2::new(width, height)
    }
}
