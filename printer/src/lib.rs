use engine::stage::Stage;

fn print(stage: Box<dyn Stage>) {
    stage.white_checkers();
    stage.black_checkers();
}
