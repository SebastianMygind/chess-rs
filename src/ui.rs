use crate::chess::{PieceType, Position};

pub mod gui;
pub mod uci;

struct UserMove {
    start_position: Position,
    end_position: Position,
    promotion_piece: Option<PieceType>,
}
