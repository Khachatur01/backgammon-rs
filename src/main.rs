use engine::stage::checker_moved::CheckerMoved;
use engine::types::pip::Pip;
use engine::start_game;

fn main() {
    let mut stage = start_game();

    let mut stage = stage.throw_dices();

    let mut stage = stage.take_checker(Pip::new(23));

    let mut stage = stage.unwrap().play_checker(Pip::new(3));

    // match stage.unwrap() {
    //     CheckerMoved::DicesThrown(dices_thrown) => {
    //         let mut stage = dices_thrown.take_checker(Pip::new(23));
    //     }
    //     CheckerMoved::OutOfMoves(out_of_moves) => {
    //         let mut stage = out_of_moves.commit_moves();
    //     }
    // };

    println!("Hello world");
}
