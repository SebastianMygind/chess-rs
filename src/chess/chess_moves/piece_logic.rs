use crate::chess::chess_moves::MoveDirection;
use crate::chess::Pieces::{
    BBishop, BKing, BKnight, BPawn, BQueen, BRook, WBishop, WKing, WKnight, WPawn, WQueen, WRook,
};
use crate::chess::{BoardPiece, Pieces, ARR_SIZE};

pub struct KingInCheck {
    // Vector of all squares that check the king
    attackers: Vec<usize>,
    king_pos: usize,
}

pub const WHITE_PAWN_DIRECTION: MoveDirection = MoveDirection { dx: 0, dy: 1 };

pub const WHITE_PAWN_ATTACK_DIRECTION: [MoveDirection; 2] = [
    MoveDirection { dx: 1, dy: 1 },
    MoveDirection { dx: -1, dy: 1 },
];

pub const BLACK_PAWN_DIRECTION: MoveDirection = MoveDirection { dx: 0, dy: -1 };
pub const BLACK_PAWN_ATTACK_DIRECTION: [MoveDirection; 2] = [
    MoveDirection { dx: -1, dy: -1 },
    MoveDirection { dx: 1, dy: -1 },
];

pub const ROOK_DIRECTION: [MoveDirection; 4] = [
    MoveDirection { dx: 1, dy: 0 },
    MoveDirection { dx: -1, dy: 0 },
    MoveDirection { dx: 0, dy: 1 },
    MoveDirection { dx: 0, dy: -1 },
];

pub const KING_AND_QUEEN_DIRECTION: [MoveDirection; 8] = [
    MoveDirection { dx: 1, dy: 0 },
    MoveDirection { dx: 1, dy: 1 },
    MoveDirection { dx: 0, dy: 1 },
    MoveDirection { dx: -1, dy: 0 },
    MoveDirection { dx: -1, dy: -1 },
    MoveDirection { dx: 0, dy: -1 },
    MoveDirection { dx: 1, dy: -1 },
    MoveDirection { dx: -1, dy: 1 },
];

pub const BISHOP_DIRECTION: [MoveDirection; 4] = [
    MoveDirection { dx: 1, dy: 1 },
    MoveDirection { dx: -1, dy: 1 },
    MoveDirection { dx: 1, dy: -1 },
    MoveDirection { dx: -1, dy: -1 },
];

pub const KNIGHT_DIRECTION: [MoveDirection; 8] = [
    MoveDirection { dx: 2, dy: 1 },
    MoveDirection { dx: 2, dy: -1 },
    MoveDirection { dx: -2, dy: 1 },
    MoveDirection { dx: -2, dy: -1 },
    MoveDirection { dx: 1, dy: 2 },
    MoveDirection { dx: 1, dy: -2 },
    MoveDirection { dx: -1, dy: 2 },
    MoveDirection { dx: -1, dy: -2 },
];

pub fn check_for_attackers(
    board: &[BoardPiece; ARR_SIZE],
    collisions: Option<Vec<usize>>,
    attacking_pieces: Vec<Pieces>,
) -> Vec<usize> {
    let mut attackers: Vec<usize> = Vec::new();

    match collisions {
        Some(collisions) => {
            for collision in collisions {
                for piece in &attacking_pieces {
                    if board[collision].piece_type == *piece {
                        attackers.push(collision);
                    }
                }
            }
        }
        None => {}
    }
    attackers
}

pub(crate) const WHITE_PIECES: [Pieces; 6] = [WPawn, WRook, WBishop, WKnight, WQueen, WKing];
pub(crate) const BLACK_PIECES: [Pieces; 6] = [BPawn, BRook, BBishop, BKnight, BQueen, BKing];
