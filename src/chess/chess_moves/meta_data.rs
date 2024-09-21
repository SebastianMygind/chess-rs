/* This module has functions for updating metadata for a Chessboard struct */
use crate::chess::{BoardPiece, ChessBoard, Move, Pieces};

pub fn update_move(chessboard: &mut ChessBoard, piece: &BoardPiece, move_to_make: &Move) {}

pub fn update_capture(chessboard: &mut ChessBoard) {
    reset_half_move_clock(chessboard);

    set_no_en_passant(chessboard);

    update_fullmove_counter(chessboard);
}

pub fn update_castle(chessboard: &mut ChessBoard) {}

pub fn update_en_passant(chessboard: &mut ChessBoard) {}

pub fn update_pawn_promotion(chessboard: &mut ChessBoard) {}

/* Helper functions */

fn update_fullmove_counter(chessboard: &mut ChessBoard) {
    if !chessboard.white_is_side_to_move {
        chessboard.fullmove_counter += 1;
    }
}

fn update_side_to_move(chessboard: &mut ChessBoard) {
    chessboard.white_is_side_to_move = !chessboard.white_is_side_to_move;
}

fn set_no_en_passant(chessboard: &mut ChessBoard) {
    match chessboard.en_passant_target_square {
        Some(_) => {
            chessboard.en_passant_target_square = None;
        }

        None => return,
    }
}

fn increment_half_move_clock(chessboard: &mut ChessBoard) {
    chessboard.halfmove_clock += 1;
}

fn reset_half_move_clock(chessboard: &mut ChessBoard) {
    chessboard.halfmove_clock = 0;
}
