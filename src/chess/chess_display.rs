use std::fmt;

use crate::chess::{BoardPiece, ChessBoard, Pieces, ARR_SIZE};

const T_LINE: &str = "┌—————┬—————┬—————┬—————┬—————┬—————┬—————┬—————┐\n";
const H_LINE: &str = "|—————|—————|—————|—————|—————|—————|—————|—————|\n";
const B_LINE: &str = "└—————┴—————┴—————┴—————┴—————┴—————┴—————┴—————┘\n";

/* This printing function will only print from the perspective of white. This means that the board
 * goes from top to bottom, respectively rank 8 to 1 */
impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_chessboard: Vec<String> = parse_chessboard_to_string(&self.board);

        if string_chessboard.len() != 8 {
            return write!(f, "Error parsing chessboard!");
        }
        // Prints the 8 ranks of a chessboard, zero indexed.
        write!(
            f,
            "{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}",
            T_LINE,
            string_chessboard[7],
            H_LINE,
            string_chessboard[6],
            H_LINE,
            string_chessboard[5],
            H_LINE,
            string_chessboard[4],
            H_LINE,
            string_chessboard[3],
            H_LINE,
            string_chessboard[2],
            H_LINE,
            string_chessboard[1],
            H_LINE,
            string_chessboard[0],
            B_LINE
        )
    }
}

fn parse_chessboard_to_string(board: &[BoardPiece; ARR_SIZE]) -> Vec<String> {
    let mut printable_board = Vec::new();
    
    for rank in 1..=8 {
        let start = (rank - 1) * 8;
        let mut pieces: Vec<char> = Vec::new();

        for i in 0..=7 {
            pieces.push(piece_type_to_char(board[start + i].piece_type));
        }

        let rank_string: String = format!(
            "|  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |",
            pieces[0], pieces[1], pieces[2], pieces[3], pieces[4], pieces[5], pieces[6], pieces[7],
        );

        printable_board.push(rank_string);
    }

    printable_board
}

fn piece_type_to_char(square_type: Pieces) -> char {
    match square_type {
        Pieces::Empty => ' ',

        Pieces::WPawn => 'P',
        Pieces::BPawn => 'p',

        Pieces::WBishop => 'B',
        Pieces::BBishop => 'b',

        Pieces::WKnight => 'N',
        Pieces::BKnight => 'n',

        Pieces::WRook => 'R',
        Pieces::BRook => 'r',

        Pieces::WQueen => 'Q',
        Pieces::BQueen => 'q',

        Pieces::WKing => 'K',
        Pieces::BKing => 'k',
    }
}
