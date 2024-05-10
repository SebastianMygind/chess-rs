mod chess_logic;

use crate::chess_logic::chessboard::is_fen_valid;
fn main() {
    let test_true = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let test_false = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 2 1";

    println!("this should be true: {}", is_fen_valid(test_true));
    println!("this should be false: {}", is_fen_valid(test_false));

}
