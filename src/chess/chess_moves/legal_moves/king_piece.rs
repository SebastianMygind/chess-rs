use crate::chess::chess_moves::legal_moves::generic_piece::{
    check_multi_step_for_piece_exists, check_single_step_for_piece_exists, get_single_step_moves,
};

use crate::chess::chess_moves::piece_logic::{
    BISHOP_DIRECTION, BLACK_PAWN_ATTACK_DIRECTION, KING_AND_QUEEN_DIRECTION, KNIGHT_DIRECTION,
    ROOK_DIRECTION, WHITE_PAWN_ATTACK_DIRECTION,
};
use crate::chess::chess_moves::MoveDirection;
use crate::chess::Color::White;
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
    let king_color: Color;

    let (kingside_castle_index, queenside_castle_index): (usize, usize) =
        if chess_board.white_is_side_to_move {
            king_color = Color::White;
            (0, 1)
        } else {
            king_color = Color::Black;
            (2, 3)
        };

    if chess_board.castling_ability[kingside_castle_index] {
        let positions_to_check: [Position; 2] = [
            (piece_position.0 + 1, piece_position.1),
            (piece_position.0 + 2, piece_position.1),
        ];

        let is_valid_castling_move = check_king_through_rook_positions_not_in_check(
            positions_to_check.as_slice(),
            &chess_board.board,
            &king_color,
        );

        if is_valid_castling_move
            || check_board_for_empty_pieces(positions_to_check.as_slice(), &chess_board.board)
        {
            let castling_move: Move = Move {
                start_pos: *piece_position,
                end_pos: (piece_position.0 + 2, piece_position.1),
                meta_data: MetaData::Castling,
            };

            king_moves.push(castling_move);
        }
    }

    if chess_board.castling_ability[queenside_castle_index] {
        let positions_to_check: [Position; 3] = [
            (piece_position.0 - 1, piece_position.1),
            (piece_position.0 - 2, piece_position.1),
            (piece_position.0 - 2, piece_position.1),
        ];

        let is_valid_castling_move = check_king_through_rook_positions_not_in_check(
            positions_to_check.as_slice(),
            &chess_board.board,
            &king_color,
        );

        if is_valid_castling_move
            || check_board_for_empty_pieces(positions_to_check.as_slice(), &chess_board.board)
        {
            let castling_move: Move = Move {
                start_pos: *piece_position,
                end_pos: (piece_position.0 - 2, piece_position.1),
                meta_data: MetaData::Castling,
            };
            king_moves.push(castling_move);
        }
    }

    king_moves
}

/** This function assumes that a king is unique, i.e. there only exists one king of each color. */
pub fn king_is_checked(board: &Board, king_position: &Position, king_color: &Color) -> bool {
    let attacking_piece_color: Color = match king_color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    // Check for king attacks.
    //   - This can be removed if you check when generating the pseudolegal moves.

    if check_single_step_for_piece_exists(
        Piece::new(attacking_piece_color, King),
        board,
        KING_AND_QUEEN_DIRECTION.as_slice(),
        king_color,
        king_position,
    ) {
        return true;
    }

    // Check for queen attacks

    if check_multi_step_for_piece_exists(
        Piece::new(attacking_piece_color, Queen),
        board,
        KING_AND_QUEEN_DIRECTION.as_slice(),
        king_color,
        king_position,
    ) {
        return true;
    }

    // Check for rook attacks

    if check_multi_step_for_piece_exists(
        Piece::new(attacking_piece_color, Rook),
        board,
        ROOK_DIRECTION.as_slice(),
        king_color,
        king_position,
    ) {
        return true;
    }

    // Check for Bishop attacks

    if check_multi_step_for_piece_exists(
        Piece::new(attacking_piece_color, Bishop),
        board,
        BISHOP_DIRECTION.as_slice(),
        king_color,
        king_position,
    ) {
        return true;
    }

    // Check for Knight attacks

    if check_multi_step_for_piece_exists(
        Piece::new(attacking_piece_color, Knight),
        board,
        KNIGHT_DIRECTION.as_slice(),
        king_color,
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
        Piece::new(attacking_piece_color, Pawn),
        board,
        pawn_attack_direction.as_slice(),
        king_color,
        king_position,
    ) {
        return true;
    }

    false
}

fn check_king_through_rook_positions_not_in_check(
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
fn check_board_for_empty_pieces(positions: &[Position], board: &Board) -> bool {
    for position in positions {
        if board[position.1][position.0] != None {
            return false;
        }
    }
    true
}
