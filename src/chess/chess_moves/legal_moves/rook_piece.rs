use crate::chess::chess_moves::legal_moves::generic_piece::get_multi_step_moves;
use crate::chess::chess_moves::piece_logic::ROOK_DIRECTION;
use crate::chess::{ChessBoard, Move};

pub fn get_rook_moves(chess_board: &ChessBoard, piece_position: &usize) -> Vec<Move> {
    get_multi_step_moves(chess_board, piece_position, ROOK_DIRECTION.as_slice())
}
