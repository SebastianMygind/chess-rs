/* Sub-module for FEN helper functions */
pub mod fen;

/* Module that allows printing a chessboard to the CLI */
mod chess_display;

use crate::chess::fen::{is_fen_valid, parse_fen_castling_ability, parse_fen_piece_placement, parse_fen_side_to_move, split_at_space};

/* Defines different piece types and color */
#[derive(Debug, Clone, Copy)]
pub enum Pieces {
    Empty,
    WPawn,
    WRook,
    WBishop,
    WKnight,
    WQueen,
    WKing,
    BPawn,
    BRook,
    BBishop,
    BKnight,
    BQueen,
    BKing,
}

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

#[derive(Debug, Clone)]
pub struct ChessBoard {
    board: [BoardPiece; ARR_SIZE],
    white_is_side_to_move: bool,
    castling_ability: [bool; 4],
    en_passant_target_square: String,
    halfmove_clock: u32,
    fullmove_counter: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct BoardPiece {
    piece_type: Pieces,
}

// Implements chess functionality
impl ChessBoard {
    pub fn new() -> ChessBoard {
        return Self {
            board: [BoardPiece {
                piece_type: Pieces::Empty,
            }; ARR_SIZE],
            white_is_side_to_move: true,
            castling_ability: [false; 4],
            en_passant_target_square: String::new(),
            halfmove_clock: 0,
            fullmove_counter: 0,
        };
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
    pub fn set_fen_position_arr(&mut self, fen: &str) -> Result<(), &'static str> {
        if !is_fen_valid(fen) {
            return Err("NOT VALID FEN");
        }
        let split_fen = split_at_space(fen);

        /* Piece placement */
        let parsed_board = parse_fen_piece_placement(&split_fen[0].as_str());

        self.board = parsed_board;

        /* Side to move */
        let is_white_move = parse_fen_side_to_move(&split_fen[1].as_str());

        self.white_is_side_to_move = is_white_move;

        /* Castling ability*/
        let castling_ability = parse_fen_castling_ability(&split_fen[2].as_str());

        self.castling_ability = castling_ability;

        /* En Passant */


        /* Half move clock */

        /* Full move counter */

        Ok(())
    }
}
