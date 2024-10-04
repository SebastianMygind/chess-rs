use crate::chess::chess_moves::legal_moves::generic_piece::get_multi_step_moves;
use crate::chess::chess_moves::piece_logic::BISHOP_DIRECTION;
use crate::chess::{ChessBoard, Color, Coordinate, Move};

pub fn get_bishop_moves(
    chess_board: &ChessBoard,
    friendly_color: &Color,
    piece_position: &Coordinate,
) -> Vec<Move> {
    get_multi_step_moves(
        chess_board,
        piece_position,
        friendly_color,
        BISHOP_DIRECTION.as_slice(),
    )
}
