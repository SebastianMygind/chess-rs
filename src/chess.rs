/* Sub-module for FEN helper functions */
use std::{error, fmt};
use crate::chess::fen::{
    is_fen_valid, parse_fen_castling_ability, parse_fen_epawn, parse_fen_full_move_counter,
    parse_fen_half_move_clock, parse_fen_piece_placement, parse_fen_side_to_move, split_at_space,
};

pub mod fen;

/* Module that allows printing a chessboard to the CLI */
mod chess_display;

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

pub(crate) const ARR_SIZE: usize = 64;
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
    en_passant_target_square: EnPassant,
    halfmove_clock: u32,
    fullmove_counter: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct BoardPiece {
    piece_type: Pieces,
}

#[derive(Debug, Clone)]
pub struct EnPassant {
    is_valid: bool,
    rank: u32, // x-axis
    file: u32, // y-axis
}

#[derive(Clone, Debug, PartialEq)]
enum MoveTypes {
    Capture,
    PawnPromotion,
    EnPassant,
    Castle
}

#[derive(Clone, Debug, PartialEq)]
pub struct Square {
    rank: u32, // y-position
    file: u32 // x-position
}
#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    move_type: MoveTypes,
    move_specific: MoveInfo
}

#[derive(Debug, Clone)]
struct IllegalMove;

impl error::Error for IllegalMove {}
impl fmt::Display for IllegalMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "illegal move")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CaptureMove {
    starting_square: Square,
    target_square: Square
}

#[derive(Clone, Debug, PartialEq)]
pub struct PawnPromotionMove {
    target_square: Square,
    promotion_piece: Pieces
}

#[derive(Clone, Debug, PartialEq)]
pub struct CastlingMove {
    is_king_side: bool, // If false, the move is castling queen side.
    rank: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnPassantMove {
    pawn_to_move: Square,
    pawn_to_capture: Square
}

#[derive(Clone, Debug, PartialEq)]
pub union MoveInfo {
    capture: CaptureMove,
    promotion: PawnPromotionMove,
    castle: CastlingMove,
    en_passant: EnPassantMove,
}

// Implements chess functionality
impl ChessBoard {
    pub fn new() -> ChessBoard {
        return Self {
            board: [BoardPiece {
                piece_type: Pieces::Empty,
            }; ARR_SIZE],
            white_is_side_to_move: true,
            castling_ability: [true; 4],
            en_passant_target_square: EnPassant {
                is_valid: false,
                rank: 0,
                file: 0,
            },
            halfmove_clock: 0,
            fullmove_counter: 0,
        };
    }

    pub fn legal_moves() -> Vec<Move> {

    }

    pub fn make_move(&mut self ,move_to_make: Move) -> Result<(), IllegalMove> {

        let legal_moves = Self::legal_moves();

        let mut is_legal_move = false;

        for possible_move in legal_moves {
            if possible_move == move_to_make {
                is_legal_move = true;
                break;
            }
        }

        if !is_legal_move {
            return Err(IllegalMove);
        }

        match move_to_make.move_type {
            MoveTypes::Capture => {


            }

            MoveTypes::PawnPromotion => {

            }

            MoveTypes::Castle => {

            }

            MoveTypes::EnPassant => {

            }
        }
        Ok(())
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
