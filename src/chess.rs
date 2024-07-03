//
pub mod fen;

//
mod chess_display;
/* Defines different piece types and color */

use crate::chess::fen::{is_fen_valid, split_at_space};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Pieces {Empty, WPawn, WRook, WBishop, WKnight, WQueen, WKing,
                 BPawn, BRook, BBishop, BKnight, BQueen, BKing}

const PAWN: u8 = 0b0001;
const ROOK: u8 = 0b0010;
const BISHOP: u8 = 0b0011;
const KNIGHT: u8 = 0b0100;
const QUEEN: u8 = 0b0101;
const KING: u8 = 0b0111;
const BLACK: u8 = 0b10000;
const WHITE: u8 = 0b1000;
const EMPTY: u8 = 0b0;

const ARR_SIZE: usize = 64;
const ROW_SIZE: usize = 8;
const COL_SIZE: usize = 8;
const COL_LETTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const NUM_CHAR: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
const VALID_FEN_BOARD: [char; 21] = [
    'p', 'r', 'b', 'n', 'q', 'k', 'P', 'R', 'B', 'N', 'Q', 'K', '1', '2', '3', '4', '5', '6', '7',
    '8', '/',
];

#[derive(Debug, Copy, Clone)]
pub struct ChessBoard {
    board: [BoardPiece; ARR_SIZE],
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BoardPiece {
    piece_type: Pieces,
    pawn_double_move: bool,
    can_castle: bool
}

// Implements chess functionality
impl ChessBoard {
    pub fn new() -> ChessBoard {
        return Self { board: [BoardPiece {piece_type: Pieces::Empty, pawn_double_move: false, can_castle: false}; ARR_SIZE] };
    }
    fn make_white(piece: u8) -> u8 {
        return (piece | WHITE);
    }
    fn make_black(piece: u8) -> u8 {
        return (piece | BLACK);
    }
}

// Implements FEN functionality
impl ChessBoard {
    pub fn set_fen_position_arr(&self, fen: &str) -> Result<(), &'static str> {
        if !is_fen_valid(fen) {
            return Err("NOT VALID FEN");
        }
        let split_fen = split_at_space(fen);
        let mut board_index: usize = 63;
        let fen_pos = split_fen[0].clone();

        for c in fen.chars() {
            match c {
                _ => {}
            }
        }

        Ok(())
    }

    pub fn set_arr_pos(&mut self, new_piece: Pieces, arr_pos: usize) {
        self.board[arr_pos] = BoardPiece {piece_type: new_piece, pawn_double_move: false, can_castle: false };
    }

}