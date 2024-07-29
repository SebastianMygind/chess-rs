/* This module has functions for updating metadata for a Chessboard struct */
use crate::chess::{BoardPiece, ChessBoard, EnPassant, PieceMove, Pieces};

pub fn update_move(chessboard: &mut ChessBoard, piece: &BoardPiece) {
    match piece.piece_type {

        Pieces::WQueen | Pieces::BQueen   |
        Pieces::WBishop | Pieces::BBishop |
        Pieces::WKnight | Pieces::BKnight => {
            increment_half_move_clock(chessboard);

            update_fullmove_counter(chessboard);

            update_side_to_move(chessboard);

            set_no_en_passant(chessboard);
        }

        Pieces::WKing => {
            increment_half_move_clock(chessboard);

            update_fullmove_counter(chessboard);

            if chessboard.castling_ability[0] == true {
                chessboard.castling_ability[0] = false;
            }
            if chessboard.castling_ability[1] == true {
                chessboard.castling_ability[1] = false;
            }

            update_side_to_move(chessboard);

            set_no_en_passant(chessboard);
        }

        Pieces::BKing => {

            if chessboard.castling_ability[2] == true {
                chessboard.castling_ability[2] = false;
            }
            if chessboard.castling_ability[3] == true {
                chessboard.castling_ability[3] = false;
            }

            increment_half_move_clock(chessboard);

            update_fullmove_counter(chessboard);

            update_side_to_move(chessboard);

            set_no_en_passant(chessboard);
        }

        Pieces::WRook => {
            if chessboard.castling_ability[0] == true {

            }
            if chessboard.castling_ability[1] == true {

            }

            increment_half_move_clock(chessboard);

            update_fullmove_counter(chessboard);

            update_side_to_move(chessboard);

            set_no_en_passant(chessboard);
        }

        Pieces::BRook => {
            if chessboard.castling_ability[2] == true {

            }
            if chessboard.castling_ability[3] == true {

            }

            increment_half_move_clock(chessboard);

            update_fullmove_counter(chessboard);

            update_side_to_move(chessboard);

            set_no_en_passant(chessboard);
        }

        Pieces::WPawn | Pieces::BPawn => {

        }

        Pieces::Empty => {
            panic!("ERROR: Invalid piece. Cannot move an empty piece");
        }
    }
}

pub fn update_capture(chessboard: &mut ChessBoard) {

}

pub fn update_castle(chessboard: &mut ChessBoard) {

}

pub fn update_en_passant(chessboard: &mut ChessBoard) {

}

pub fn update_pawn_promotion(chessboard: &mut ChessBoard) {

}


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
    if chessboard.en_passant_target_square.is_valid {
        chessboard.en_passant_target_square = EnPassant {
            is_valid: false,
            rank: 0,
            file: 0,
        }
    }
}

fn increment_half_move_clock(chessboard: &mut ChessBoard) {
    chessboard.halfmove_clock += 1;
}