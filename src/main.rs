use engine::types::from_pip::FromPip;
use engine::types::to_pip::ToPip;
use engine::Backgammon;

fn main() {
    let mut stage = Backgammon::new();

    let mut stage = stage.throw_dices();

    let mut stage = stage.take_checker(FromPip::new(1));

    let mut stage = stage.unwrap().move_checker(ToPip::new(2));

    println!("Hello world");
}
