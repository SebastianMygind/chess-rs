use crate::array_engine::chess_moves::legal_moves::generic_piece::get_multi_step_moves;
use crate::array_engine::chess_moves::piece_logic::ROOK_DIRECTION;
use crate::array_engine::{ChessBoard, Color, Move, PieceType, Position};

pub fn get_rook_moves(
    chess_board: &ChessBoard,
    friendly_color: &Color,
    piece_position: &Position,
) -> Vec<Move> {
    get_multi_step_moves(
        chess_board,
        piece_position,
        PieceType::Rook,
        friendly_color,
        ROOK_DIRECTION.as_slice(),
    )
}
