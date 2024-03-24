/* Defines different piece types and color */
use crate::chess_logic::chessboard::ChessBoardAssignments::Empty;

#[derive(Debug)]
pub enum ChessBoardAssignments {
    Empty,
    White(WhitePieces),
    Black(BlackPieces),
}

#[derive(Debug, Clone, Copy)]
pub enum WhitePieces {
    WPawn,
    WRook,
    WBishop,
    WKnight,
    WQueen,
    WKing,
}

#[derive(Debug)]
pub enum BlackPieces {
    BPawn,
    BRook,
    BBishop,
    BKnight,
    BQueen,
    BKing,
}

const WHITE_MAJOR_SIGNATURE: [WhitePieces; 8] = [
    WhitePieces::WRook, WhitePieces::WKnight, WhitePieces::WBishop, WhitePieces::WQueen
    , WhitePieces::WKing, WhitePieces::WBishop, WhitePieces::WKnight, WhitePieces::WRook
];

pub struct ChessBoard {
    pub board: [ChessBoardAssignments; 64],
}

/** Implementation for setting the position of the chess_logic
 * TODO: Add FEN converter and set_fen_position
 **/

const ARRAY_REPEAT_VALUE: ChessBoardAssignments = Empty;
impl ChessBoard {
    pub fn new() -> ChessBoard {
        Self{
            board: [ARRAY_REPEAT_VALUE; 64],
        }
    }
    pub fn set_start_position(&mut self) {
        let mut index: usize = 0;

        for &piece in WHITE_MAJOR_SIGNATURE.iter() {
            self.board[index] = ChessBoardAssignments::White(piece);
            index += 1;
        }
        println!("The square reached is {}", index);
        for i in 0..8 {
            print!("{:?}, ", self.board[i])
        }
    }
}

