use crate::chess::chess_moves::legal_moves::generic_piece::get_single_step_moves;
use crate::chess::chess_moves::piece_logic::KNIGHT_ATTACK_DIRECTION;
use crate::chess::{ChessBoard, Move};

pub fn get_knight_moves(chess_board: &ChessBoard, piece_position: &usize) -> Vec<Move> {
    get_single_step_moves(
        chess_board,
        piece_position,
        KNIGHT_ATTACK_DIRECTION.as_slice(),
    )
}
