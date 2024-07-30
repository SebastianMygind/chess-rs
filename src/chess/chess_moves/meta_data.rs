/* This module has functions for updating metadata for a Chessboard struct */
use crate::chess::{BoardPiece, ChessBoard, EnPassant, Move, PieceMove, Pieces};

pub fn update_move(chessboard: &mut ChessBoard, piece: &BoardPiece, move_to_make: &Move) {
    match piece.piece_type {
        Pieces::WQueen
        | Pieces::BQueen
        | Pieces::WBishop
        | Pieces::BBishop
        | Pieces::WKnight
        | Pieces::BKnight => {
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
                if chessboard.board[0] != BoardPiece::new(Pieces::WRook) {
                    chessboard.castling_ability[0] = false;
                }
            }
            if chessboard.castling_ability[1] == true {
                if chessboard.board[7] != BoardPiece::new(Pieces::WRook) {
                    chessboard.castling_ability[1] = false;
                }
            }

            increment_half_move_clock(chessboard);

            update_fullmove_counter(chessboard);

            update_side_to_move(chessboard);

            set_no_en_passant(chessboard);
        }

        Pieces::BRook => {
            if chessboard.castling_ability[2] == true {
                if chessboard.board[56] != BoardPiece::new(Pieces::BRook) {
                    chessboard.castling_ability[2] = false;
                }
            }
            if chessboard.castling_ability[3] == true {
                if chessboard.board[63] != BoardPiece::new(Pieces::BRook) {
                    chessboard.castling_ability[3] = false;
                }
            }

            increment_half_move_clock(chessboard);

            update_fullmove_counter(chessboard);

            update_side_to_move(chessboard);

            set_no_en_passant(chessboard);
        }

        Pieces::WPawn => {
            let pawn_move: PieceMove = unsafe { move_to_make.move_specific.piece_move };

            if pawn_move.ending_square.pos_to_arr_index()
                - pawn_move.starting_square.pos_to_arr_index()
                == 16
            {
                chessboard.en_passant_target_square = EnPassant {
                    is_valid: true,
                    arr_pos: (pawn_move.starting_square.pos_to_arr_index() + 8) as u32,
                }
            } else {
                set_no_en_passant(chessboard);
            }

            reset_half_move_clock(chessboard);

            update_fullmove_counter(chessboard);

            update_side_to_move(chessboard);
        }

        Pieces::BPawn => {
            let pawn_move: PieceMove = unsafe { move_to_make.move_specific.piece_move };

            if pawn_move.starting_square.pos_to_arr_index()
                - pawn_move.ending_square.pos_to_arr_index()
                == 16
            {
                chessboard.en_passant_target_square = EnPassant {
                    is_valid: true,
                    arr_pos: (pawn_move.starting_square.pos_to_arr_index() - 8) as u32,
                }
            } else {
                set_no_en_passant(chessboard);
            }

            reset_half_move_clock(chessboard);

            update_fullmove_counter(chessboard);

            update_side_to_move(chessboard);
        }

        Pieces::Empty => {
            panic!("ERROR: Invalid piece. Cannot move an empty piece");
        }
    }
}

pub fn update_capture(chessboard: &mut ChessBoard) {
    reset_half_move_clock(chessboard);

    set_no_en_passant(chessboard);

    update_fullmove_counter(chessboard);

    update_side_to_move(chessboard);
}

pub fn update_castle(chessboard: &mut ChessBoard) {}

pub fn update_en_passant(chessboard: &mut ChessBoard) {}

pub fn update_pawn_promotion(chessboard: &mut ChessBoard) {}

pub fn update_game_state(chessboard: &mut ChessBoard) {
    let legal_moves: Vec<Move> = chessboard.legal_moves();

    if legal_moves.len() == 0 {}
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
            arr_pos: 0,
        }
    }
}

fn increment_half_move_clock(chessboard: &mut ChessBoard) {
    chessboard.halfmove_clock += 1;
}

fn reset_half_move_clock(chessboard: &mut ChessBoard) {
    chessboard.halfmove_clock = 0;
}
