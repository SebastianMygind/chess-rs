mod chess_logic;

use crate::chess_logic::chessboard::{
    is_valid_fen,
};

fn main() {
    let test_true = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR ";
    let test_false = "8//8";

    println!("this should be false: {}", is_valid_fen(test_false));
    println!("this should be true: {}", is_valid_fen(test_true));


}
