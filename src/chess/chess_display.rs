use std::fmt;

use crate::chess::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::chess::{Board, ChessBoard, Color, Move, Square};

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

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "start_pos: ({}, {}), end_pos: ({}, {})\n",
            self.start_pos.0, self.start_pos.1, self.end_pos.0, self.end_pos.1
        )
    }
}

fn parse_chessboard_to_string(board: &Board) -> Vec<String> {
    let mut printable_board = Vec::new();

    for rank in 0..=7 {
        let mut pieces: Vec<char> = Vec::new();

        for file in 0..=7 {
            pieces.push(piece_type_to_char(board[rank][file]));
        }

        let rank_string: String = format!(
            "|  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |",
            pieces[0], pieces[1], pieces[2], pieces[3], pieces[4], pieces[5], pieces[6], pieces[7],
        );

        printable_board.push(rank_string);
    }

    printable_board
}

fn piece_type_to_char(square: Square) -> char {
    match square {
        None => ' ',

        Some(piece) => match piece.color {
            Color::White => match piece.piece_type {
                King => 'K',
                Queen => 'Q',
                Rook => 'R',
                Bishop => 'B',
                Knight => 'N',
                Pawn => 'P',
            },
            Color::Black => match piece.piece_type {
                King => 'k',
                Queen => 'q',
                Rook => 'r',
                Bishop => 'b',
                Knight => 'n',
                Pawn => 'p',
            },
        },
    }
}
