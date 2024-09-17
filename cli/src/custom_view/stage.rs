use cursive::{Printer, Rect, Vec2, View, XY};
use cursive::backends::crossterm::crossterm::cursor::position;
use cursive::style::ColorStyle;
use cursive::views::TextView;
use engine::board::checkers::Checkers;
use engine::constant::player::Side;
use engine::stage::Stage;
use engine::types::dice_pair::DicePair;
use engine::types::pip::Pip;

pub struct StageView {
    white_checkers: Checkers,
    black_checkers: Checkers,
    active_side: Option<Side>,
    dice_pair: Option<DicePair>,
    taken_checker_pip: Option<Pip>,
}

impl<T: Stage> From<T> for StageView {
    fn from(stage: T) -> Self {
        Self {
            white_checkers: stage.white_checkers(),
            black_checkers: stage.black_checkers(),
            active_side: stage.active_side(),
            dice_pair: stage.dice_pair(),
            taken_checker_pip: stage.taken_checker_pip(),
        }
    }
}

impl View for StageView {
    fn draw(&self, printer: &Printer) {
        /* TODO */
        self.render_borders(printer);

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

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        Vec2::new(20, 15)
    }
}

impl StageView {
    fn render_borders(&self, printer: &Printer) {
        let width = 20;
        let height = 15;

        printer.print(
            (0, 0),
            std::iter::repeat('█').take(width - 1).collect::<String>().as_str()
        );

        for row in 0..height {
            printer.print(
                (0, row),
                "█"
            );
            printer.print(
                (width - 1, row),
                "█"
            );
        }

        printer.print(
            (0, height - 1),
            std::iter::repeat('█').take(width - 1).collect::<String>().as_str()
        );

        // printer.print(
        //     (0, 0),
        //     "█"
        // );
        // printer.print(
        //     (1, 0),
        //     "█"
        // );
        // printer.print(
        //     (2, 0),
        //     "█"
        // );
        // printer.print(
        //     (3, 0),
        //     "█"
        // );
        //
        //
        // printer.print(
        //     (0, 3),
        //     "█"
        // );
        // printer.print(
        //     (1, 3),
        //     "█"
        // );
        // printer.print(
        //     (2, 3),
        //     "█"
        // );
        // printer.print(
        //     (3, 3),
        //     "█"
        // );
        //
        //
        // printer.print(
        //     (0, 1),
        //     "█"
        // );
        // printer.print(
        //     (0, 2),
        //     "█"
        // );
        // printer.print(
        //     (3, 1),
        //     "█"
        // );
        // printer.print(
        //     (3, 2),
        //     "█"
        // );
    }
}
