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

use chess_errors::{IllegalMove, InvalidFen};
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

pub const ARR_SIZE: usize = 64;
const ROW_SIZE: usize = 8;
const COL_SIZE: usize = 8;
const COL_LETTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const NUM_CHAR: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
const VALID_FEN_BOARD: [char; 21] = [
    'p', 'r', 'b', 'n', 'q', 'k', 'P', 'R', 'B', 'N', 'Q', 'K', '1', '2', '3', '4', '5', '6', '7',
    '8', '/',
];

const EMPTY_PIECE: BoardPiece = BoardPiece {
    piece_type: Pieces::Empty,
};

/* Chessboard specific implementations */
#[derive(Debug, Clone)]
pub struct ChessBoard {
    board: [BoardPiece; ARR_SIZE],
    white_is_side_to_move: bool,
    castling_ability: [bool; 4],
    en_passant_target_square: EnPassant,
    halfmove_clock: u32,
    fullmove_counter: u64,
    is_checked: bool,
    is_checkmate: bool,
    is_stalemate: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoardPiece {
    piece_type: Pieces,
}

impl BoardPiece {
    pub fn new(chess_piece: Pieces) -> BoardPiece {
        return Self {
            piece_type: chess_piece,
        };
    }
}

#[derive(Debug, Clone)]
pub struct EnPassant {
    is_valid: bool,
    arr_pos: u32,
}

impl EnPassant {}

/* Move specific implementations */
#[derive(Clone)]
pub struct Move {
    pub(crate) move_type: MoveTypes,
    pub(crate) move_specific: MoveInfo,
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        if self.move_type == other.move_type {
            match self.move_type {
                MoveTypes::Move => {
                    let self_move = unsafe { self.move_specific.piece_move.clone() };
                    let other_move = unsafe { other.move_specific.piece_move.clone() };

                    self_move == other_move
                }

                MoveTypes::Capture => {
                    let self_move = unsafe { self.move_specific.capture.clone() };
                    let other_move = unsafe { other.move_specific.capture.clone() };

                    self_move == other_move
                }

                MoveTypes::PawnPromotion => {
                    let self_move = unsafe { self.move_specific.promotion.clone() };
                    let other_move = unsafe { other.move_specific.promotion.clone() };

                    self_move == other_move
                }

                MoveTypes::Castle => {
                    let self_move = unsafe { self.move_specific.castle.clone() };
                    let other_move = unsafe { other.move_specific.castle.clone() };

                    self_move == other_move
                }

                MoveTypes::EnPassant => {
                    let self_move = unsafe { self.move_specific.en_passant.clone() };
                    let other_move = unsafe { other.move_specific.en_passant.clone() };

                    self_move == other_move
                }
            }
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MoveTypes {
    Move,
    Capture,
    PawnPromotion,
    EnPassant,
    Castle,
}

#[derive(Clone, Copy)]
pub union MoveInfo {
    pub(crate) piece_move: PieceMove,
    pub(crate) capture: CaptureMove,
    pub(crate) promotion: PawnPromotionMove,
    pub(crate) castle: CastlingMove,
    pub(crate) en_passant: EnPassantMove,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square {
    pub(crate) file: u32, // x-position
    pub(crate) rank: u32, // y-position
}

/** No captures are allowed when moving, use CaptureMove instead **/
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PieceMove {
    pub(crate) starting_square: Square,
    pub(crate) ending_square: Square,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CaptureMove {
    pub(crate) starting_square: Square,
    pub(crate) target_square: Square,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PawnPromotionMove {
    pub(crate) target_square: Square,
    pub(crate) promotion_piece: Pieces,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CastlingMove {
    pub(crate) is_king_side: bool, // If false, the move is castling queen side.
    pub(crate) rank: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnPassantMove {
    pub(crate) pawn_to_move: Square,
    pub(crate) pawn_to_capture: Square,
    pub(crate) is_white_move: bool,
}

// Implements chess functionality
impl ChessBoard {
    pub fn new() -> ChessBoard {
        let mut new_board: ChessBoard = ChessBoard {
            board: [EMPTY_PIECE; ARR_SIZE],
            white_is_side_to_move: true,
            castling_ability: [true; 4],
            en_passant_target_square: EnPassant {
                is_valid: false,
                arr_pos: 0,
            },
            halfmove_clock: 0,
            fullmove_counter: 0,
            is_checked: false,
            is_checkmate: false,
            is_stalemate: false,
        };

        new_board.set_fen_position_arr(FEN_START_POS).unwrap();

        return new_board;
    }
}

// Implements FEN functionality
impl ChessBoard {
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
