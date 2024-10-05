use crate::chess::chess_moves::legal_moves::generic_piece::get_multi_step_moves;
use crate::chess::chess_moves::piece_logic::ROOK_DIRECTION;
use crate::chess::{ChessBoard, Color, Move, Position};

pub fn get_rook_moves(
    chess_board: &ChessBoard,
    friendly_color: &Color,
    piece_position: &Position,
) -> Vec<Move> {
    get_multi_step_moves(
        chess_board,
        piece_position,
        friendly_color,
        ROOK_DIRECTION.as_slice(),
    )
}
