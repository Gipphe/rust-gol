mod model;
use model::BoardBuilder;

fn main() {
    let board = BoardBuilder::new()
        .make();
    println!("{:?}", board);
}
