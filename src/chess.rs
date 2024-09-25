/* Sub-module for FEN helper functions */

use crate::chess::fen::{
    is_fen_valid, parse_fen_castling_ability, parse_fen_epawn, parse_fen_full_move_counter,
    parse_fen_half_move_clock, parse_fen_piece_placement, parse_fen_side_to_move, split_at_space,
};
pub mod fen;

/* Module that allows printing a chessboard to the CLI */
mod chess_display;
pub mod chess_errors;
pub mod chess_moves;

use chess_errors::InvalidFen;
use fen::FEN_START_POS;

/* Defines different piece types and color */
#[derive(Debug, Clone, Copy, PartialEq)]
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

/* Defines enums to use with the move struct */
#[derive(PartialEq)]
pub enum MetaData {
    Move,
    Capture,
    EnPassant,
    Castling,
    Promotion(Pieces),
}

pub const ARR_SIZE: usize = ROW_SIZE * COL_SIZE;
const ROW_SIZE: usize = 8;
const COL_SIZE: usize = 8;

const EMPTY_PIECE: BoardPiece = BoardPiece {
    piece_type: Pieces::Empty,
};

/* Chessboard specific implementations */
#[derive(Debug, Clone)]
pub struct ChessBoard {
    board: [BoardPiece; ARR_SIZE],
    white_is_side_to_move: bool,
    castling_ability: [bool; 4], // WKingside, WQueenside, BKingside, BQueenside
    en_passant_target_square: Option<usize>,
    halfmove_clock: u64,
    fullmove_counter: u64,
    is_checked: bool,
    is_checkmate: bool,
    is_stalemate: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoardPiece {
    piece_type: Pieces,
}

#[derive(PartialEq)]
pub(crate) struct Move {
    pub start_pos: usize,
    pub end_pos: usize,
    pub meta_data: MetaData,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SquarePosition {
    pub file: usize, // x-position
    pub rank: usize, // y-position
}

/*
trait Piece {
    fn make_move(&self, piece_move: Move,
                 board: [BoardPiece; ARR_SIZE],
                 board_position: usize) -> Result<Move, IllegalMove>;
    fn get_moves(&self, board: [BoardPiece; ARR_SIZE], board_position: usize) -> Vec<Move>;
}
struct SquareV2 {
    piece: Option<PieceV2>
}
struct PieceV2 {
    piece_specific: Box<dyn Piece>
}
*/

// Implements chess functionality
impl ChessBoard {
    pub fn new() -> ChessBoard {
        let mut new_board: ChessBoard = ChessBoard {
            board: [EMPTY_PIECE; ARR_SIZE],
            white_is_side_to_move: true,
            castling_ability: [true; 4],
            en_passant_target_square: None,
            halfmove_clock: 0,
            fullmove_counter: 0,
            is_checked: false,
            is_checkmate: false,
            is_stalemate: false,
        };

        new_board.set_fen_position_arr(FEN_START_POS).unwrap();

        new_board
    }

    // Implements FEN functionality
    pub fn set_fen_position_arr(&mut self, fen: &str) -> Result<(), InvalidFen> {
        if !is_fen_valid(fen) {
            return Err(InvalidFen);
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
        let en_passant = parse_fen_epawn(&split_fen[3].as_str());

        self.en_passant_target_square = en_passant;

        /* Half move clock */
        let half_moves = parse_fen_half_move_clock(&split_fen[4].as_str());

        self.halfmove_clock = half_moves;

        /* Full move counter */
        let full_moves = parse_fen_full_move_counter(&split_fen[5].as_str());

        self.fullmove_counter = full_moves;

        Ok(())
    }
}
