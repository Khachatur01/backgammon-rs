use engine::types::pip::Pip;
use engine::start_game;

fn main() {
    let mut stage = start_game();

    let mut stage = stage.throw_dices();

    let mut stage = stage.take_checker(Pip::new(23));

    let mut stage = stage.unwrap().move_checker(Pip::new(2));

    println!("Hello world");
}
