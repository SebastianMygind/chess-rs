/* Sub-module for FEN helper functions */

use core::fmt;

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
use fen::FEN_START_POSITION;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Square {
    Empty,
    Piece(Piece),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    White(PieceType),
    Black(PieceType),
}

/* Defines different piece types and color */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

/* Defines enums to use with the move struct */
#[derive(PartialEq)]
pub enum MetaData {
    Move,
    PawnMove,
    PawnDoubleMove,
    Capture,
    EnPassant,
    Castling,
    Promotion(Piece),
}

pub const ROW_SIZE: usize = 8;
pub const COL_SIZE: usize = 8;
type Board = [[BoardSquare; COL_SIZE]; ROW_SIZE];
type Coordinates = (usize, usize);

const EMPTY_PIECE: BoardSquare = BoardSquare {
    piece_type: Square::Empty,
};

/* Chessboard specific implementations */
#[derive(Debug, Clone, Copy)]
pub struct ChessBoard {
    board: Board,
    white_is_side_to_move: bool,
    castling_ability: [bool; 4], // WKingside, WQueenside, BKingside, BQueenside
    en_passant_target_square: Option<Coordinates>,
    halfmove_clock: u64,
    fullmove_counter: u64,
    is_checked: bool,
    is_checkmate: bool,
    is_stalemate: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoardSquare {
    piece_type: Square,
}

#[derive(PartialEq)]
pub(crate) struct Move {
    pub start_pos: Coordinates,
    pub end_pos: Coordinates,
    pub meta_data: MetaData,
}

// Implements chess functionality
impl ChessBoard {
    pub fn new() -> ChessBoard {
        let mut new_board: ChessBoard = ChessBoard {
            board: [[EMPTY_PIECE; COL_SIZE]; ROW_SIZE],
            white_is_side_to_move: true,
            castling_ability: [true; 4],
            en_passant_target_square: None,
            halfmove_clock: 0,
            fullmove_counter: 0,
            is_checked: false,
            is_checkmate: false,
            is_stalemate: false,
        };

        new_board.set_fen_position_arr(FEN_START_POSITION).unwrap();

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
