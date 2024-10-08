use crate::chess::chess_moves::legal_moves::generic_piece::{
    check_multi_step_for_piece_exists, check_single_step_for_piece_exists, get_single_step_moves,
};

use crate::chess::chess_moves::piece_logic::{
    BISHOP_DIRECTION, BLACK_PAWN_ATTACK_DIRECTION, KING_AND_QUEEN_DIRECTION, KNIGHT_DIRECTION,
    ROOK_DIRECTION, WHITE_PAWN_ATTACK_DIRECTION,
};
use crate::chess::chess_moves::MoveDirection;
use crate::chess::Color::{Black, White};
use crate::chess::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::chess::{Board, ChessBoard, Color, MetaData, Move, Piece, Position};

pub fn get_king_moves(
    chess_board: &ChessBoard,
    friendly_color: &Color,
    piece_position: &Position,
) -> Vec<Move> {
    let mut king_moves: Vec<Move> = get_single_step_moves(
        chess_board,
        piece_position,
        friendly_color,
        KING_AND_QUEEN_DIRECTION.as_slice(),
    );

    for king_move in &mut king_moves {
        king_move.meta_data = MetaData::KingMove;
    }

    let (kingside_castle_index, queenside_castle_index): (usize, usize) =
        if chess_board.white_is_side_to_move {
            (0, 1)
        } else {
            (2, 3)
        };

    if chess_board.castling_ability[kingside_castle_index] {
        if let Some(castling_move) =
            check_for_castling_move(&chess_board.board, piece_position, friendly_color, true)
        {
            king_moves.push(castling_move);
        }
    }

    if chess_board.castling_ability[queenside_castle_index] {
        if let Some(castling_move) =
            check_for_castling_move(&chess_board.board, piece_position, friendly_color, false)
        {
            king_moves.push(castling_move);
        }
    }

    king_moves
}

fn check_for_castling_move(
    board: &Board,
    current_position: &Position,
    king_color: &Color,
    is_king_side_castle: bool,
) -> Option<Move> {
    let king_end_position: Position;
    let rook_start_position: Position;

    if is_king_side_castle {
        rook_start_position = (current_position.0 + 3, current_position.1);

        king_end_position = (current_position.0 + 2, current_position.1);

        if !all_given_positions_are_empty(
            [
                (current_position.0 + 1, current_position.1),
                (current_position.0 + 2, current_position.1),
            ]
            .as_slice(),
            board,
        ) {
            return None;
        }

        if !all_given_positions_not_in_check(
            [
                *current_position,
                (current_position.0 + 1, current_position.1),
                (current_position.0 + 2, current_position.1),
                rook_start_position,
            ]
            .as_slice(),
            board,
            king_color,
        ) {
            return None;
        }

        Some(Move {
            start_pos: *current_position,
            end_pos: king_end_position,
            meta_data: MetaData::Castling,
        })
    } else {
        rook_start_position = (current_position.0 - 4, current_position.1);
        king_end_position = (current_position.0 - 2, current_position.1);

        if !all_given_positions_are_empty(
            [
                (current_position.0 - 1, current_position.1),
                (current_position.0 - 2, current_position.1),
                (current_position.0 - 3, current_position.1),
            ]
            .as_slice(),
            board,
        ) {
            return None;
        }

        if !all_given_positions_not_in_check(
            [
                *current_position,
                (current_position.0 - 1, current_position.1),
                (current_position.0 - 2, current_position.1),
                (current_position.0 - 3, current_position.1),
                rook_start_position,
            ]
            .as_slice(),
            board,
            king_color,
        ) {
            return None;
        }

        Some(Move {
            start_pos: *current_position,
            end_pos: king_end_position,
            meta_data: MetaData::Castling,
        })
    }
}

/** This function assumes that a king is unique, i.e. there only exists one king of each color. */
pub fn king_is_checked(board: &Board, king_position: &Position, king_color: &Color) -> bool {
    let enemy_color: Color = if *king_color == White { Black } else { White };

    // Check for king attacks.
    //   - This can be removed if you check when generating the pseudolegal moves.

    if check_single_step_for_piece_exists(
        Piece::new(enemy_color, King),
        board,
        KING_AND_QUEEN_DIRECTION.as_slice(),
        king_position,
    ) {
        return true;
    }

    // Check for queen attacks

    if check_multi_step_for_piece_exists(
        Piece::new(enemy_color, Queen),
        board,
        KING_AND_QUEEN_DIRECTION.as_slice(),
        king_position,
    ) {
        return true;
    }

    // Check for rook attacks

    if check_multi_step_for_piece_exists(
        Piece::new(enemy_color, Rook),
        board,
        ROOK_DIRECTION.as_slice(),
        king_position,
    ) {
        return true;
    }

    // Check for Bishop attacks

    if check_multi_step_for_piece_exists(
        Piece::new(enemy_color, Bishop),
        board,
        BISHOP_DIRECTION.as_slice(),
        king_position,
    ) {
        return true;
    }

    // Check for Knight attacks

    if check_multi_step_for_piece_exists(
        Piece::new(enemy_color, Knight),
        board,
        KNIGHT_DIRECTION.as_slice(),
        king_position,
    ) {
        return true;
    }

    // Check for pawn attacks

    let pawn_attack_direction: [MoveDirection; 2] = if *king_color == White {
        WHITE_PAWN_ATTACK_DIRECTION
    } else {
        BLACK_PAWN_ATTACK_DIRECTION
    };

    if check_single_step_for_piece_exists(
        Piece::new(enemy_color, Pawn),
        board,
        pawn_attack_direction.as_slice(),
        king_position,
    ) {
        return true;
    }

    false
}

fn all_given_positions_not_in_check(
    positions: &[Position],
    board: &Board,
    king_color: &Color,
) -> bool {
    for position in positions {
        if king_is_checked(board, position, king_color) {
            return false;
        }
    }

    true
}

/** Returns true if all given positions are empty */
fn all_given_positions_are_empty(positions: &[Position], board: &Board) -> bool {
    for position in positions {
        if board[position.1][position.0] != None {
            return false;
        }
    }
    true
}
