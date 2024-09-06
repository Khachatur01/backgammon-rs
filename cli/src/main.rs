use cursive::align::{Align, HAlign, VAlign};
use cursive::views::TextView;

fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    let mut text_view = TextView::new("Hello TUI!\nPress <q> to quit.");

    text_view = text_view.align(Align::new(HAlign::Left, VAlign::Top));

    siv.add_layer(text_view);
    siv.run();
}
