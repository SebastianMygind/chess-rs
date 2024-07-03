use std::fmt;

use crate::chess::{BoardPiece, ChessBoard, Pieces, ARR_SIZE};

const T_LINE: &str = "┌—————┬—————┬—————┬—————┬—————┬—————┬—————┬—————┐\n";
const H_LINE: &str = "|—————|—————|—————|—————|—————|—————|—————|—————|\n";
const B_LINE: &str = "└—————┴—————┴—————┴—————┴—————┴—————┴—————┴—————┘\n";

/* This printing function will only print from the perspective of white. This means that the board
 * goes from top to bottom, respectively rank 8 to 1 */
impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ltp: Vec<String> = parse_chessboard_to_string(&self.board);

        if ltp.len() != 8 {
            return write!(f, "Error parsing chessboard!");
        }
        // Prints the 8 ranks of a chessboard, zero indexed.
        write!(
            f,
            "{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}",
            T_LINE,
            ltp[7],
            H_LINE,
            ltp[6],
            H_LINE,
            ltp[5],
            H_LINE,
            ltp[4],
            H_LINE,
            ltp[3],
            H_LINE,
            ltp[2],
            H_LINE,
            ltp[1],
            H_LINE,
            ltp[0],
            B_LINE
        )
    }
}

fn parse_chessboard_to_string(board_struct: &[BoardPiece; 64]) -> Vec<String> {
    let mut printable_board = Vec::new();

    let board = board_struct.clone();

    let ranks: [usize; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    for rank in ranks {
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

    return printable_board;
}

fn piece_type_to_char(square_type: Pieces) -> char {
    return match square_type {
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
    };
}
