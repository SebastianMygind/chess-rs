use crate::chess::chess_moves::legal_moves::generic_piece::get_single_step_moves;
use crate::chess::chess_moves::piece_logic::KNIGHT_DIRECTION;
use crate::chess::{ChessBoard, Color, Move, PieceType, Position};

pub fn get_knight_moves(
    chess_board: &ChessBoard,
    friendly_color: &Color,
    piece_position: &Position,
) -> Vec<Move> {
    get_single_step_moves(
        chess_board,
        piece_position,
        PieceType::Knight,
        friendly_color,
        KNIGHT_DIRECTION.as_slice(),
    )
}
