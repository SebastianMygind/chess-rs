use crate::chess::chess_moves::legal_moves::generic_piece::{
    check_multi_step_for_piece_exists, check_single_step_for_piece_exists, get_single_step_moves,
    Color,
};

use crate::chess::chess_moves::piece_logic::{
    BISHOP_DIRECTION, BLACK_PAWN_ATTACK_DIRECTION, KING_AND_QUEEN_DIRECTION, KNIGHT_DIRECTION,
    ROOK_DIRECTION, WHITE_PAWN_ATTACK_DIRECTION,
};
use crate::chess::chess_moves::MoveDirection;
use crate::chess::Pieces::{
    BBishop, BKing, BKnight, BPawn, BQueen, BRook, WBishop, WKing, WKnight, WPawn, WQueen, WRook,
};
use crate::chess::{BoardPiece, ChessBoard, MetaData, Move, ARR_SIZE, EMPTY_PIECE};

pub fn get_king_moves(chess_board: &ChessBoard, piece_position: &usize) -> Vec<Move> {
    let mut king_moves: Vec<Move> = get_single_step_moves(
        chess_board,
        piece_position,
        KING_AND_QUEEN_DIRECTION.as_slice(),
    );

    let (kingside_castle_index, queenside_castle_index): (usize, usize) =
        if chess_board.white_is_side_to_move {
            (0, 1)
        } else {
            (2, 3)
        };

    if chess_board.castling_ability[kingside_castle_index] {
        let positions_to_check: [usize; 2] = [piece_position + 1, piece_position + 2];

        if check_board_for_empty_pieces(positions_to_check.as_slice(), &chess_board.board) {
            let castling_move: Move = Move {
                start_pos: *piece_position,
                end_pos: *piece_position + 2,
                meta_data: MetaData::Castling,
            };

            king_moves.push(castling_move);
        }
    }

    if chess_board.castling_ability[queenside_castle_index] {
        let positions_to_check: [usize; 3] =
            [piece_position - 1, piece_position - 2, piece_position - 3];

        if check_board_for_empty_pieces(positions_to_check.as_slice(), &chess_board.board) {
            let castling_move: Move = Move {
                start_pos: *piece_position,
                end_pos: *piece_position - 2,
                meta_data: MetaData::Castling,
            };
            king_moves.push(castling_move);
        }
    }

    king_moves
}

/** This function assumes that a king is unique, i.e. there only exists one king of each color. */
pub fn king_is_checked(
    board: &[BoardPiece; ARR_SIZE],
    king_position: &usize,
    king_color: &Color,
) -> bool {
    let pawn_attack: [MoveDirection; 2];
    let attacking_pieces = if *king_color == Color::White {
        pawn_attack = WHITE_PAWN_ATTACK_DIRECTION;
        [BKing, BQueen, BRook, BBishop, BKnight, BPawn]
    } else {
        pawn_attack = BLACK_PAWN_ATTACK_DIRECTION;
        [WKing, WQueen, WRook, WBishop, WKnight, WPawn]
    };

    // Check for king attacks.
    //   - This can be removed if you check when generating the pseudolegal moves.

    if check_single_step_for_piece_exists(
        &attacking_pieces[0],
        board,
        KING_AND_QUEEN_DIRECTION.as_slice(),
        king_position,
    ) {
        return true;
    }

    // Check for queen attacks

    if check_multi_step_for_piece_exists(
        &attacking_pieces[1],
        board,
        KING_AND_QUEEN_DIRECTION.as_slice(),
        king_position,
    ) {
        return true;
    }

    // Check for rook attacks

    if check_multi_step_for_piece_exists(
        &attacking_pieces[2],
        board,
        ROOK_DIRECTION.as_slice(),
        king_position,
    ) {
        return true;
    }

    // Check for Bishop attacks

    if check_multi_step_for_piece_exists(
        &attacking_pieces[3],
        board,
        BISHOP_DIRECTION.as_slice(),
        king_position,
    ) {
        return true;
    }

    // Check for Knight attacks

    if check_multi_step_for_piece_exists(
        &attacking_pieces[4],
        board,
        KNIGHT_DIRECTION.as_slice(),
        king_position,
    ) {
        return true;
    }

    // Check for pawn attacks

    if check_multi_step_for_piece_exists(
        &attacking_pieces[5],
        board,
        pawn_attack.as_slice(),
        king_position,
    ) {
        return true;
    }

    false
}

/** Returns true if all given positions are empty */
fn check_board_for_empty_pieces(positions: &[usize], board: &[BoardPiece; ARR_SIZE]) -> bool {
    for position in positions {
        if board[*position] != EMPTY_PIECE {
            return false;
        }
    }
    true
}
