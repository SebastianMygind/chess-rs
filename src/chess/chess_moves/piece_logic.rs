use crate::chess::chess_moves::BoardDirection;
use crate::chess::Pieces::{BBishop, BKing, BKnight, BPawn, BQueen, BRook, WBishop, WKing, WKnight, WPawn, WQueen, WRook};
use crate::chess::{BoardPiece, Pieces, ARR_SIZE};

pub static WHITE_PAWN_ATTACK_DIRECTION: &[BoardDirection] = &[
    BoardDirection { dx: 1, dy: 1 },
    BoardDirection { dx: 1, dy: -1 },
];

pub static BLACK_PAWN_ATTACK_DIRECTION: &[BoardDirection] = &[
    BoardDirection { dx: -1, dy: 1 },
    BoardDirection { dx: -1, dy: -1 },
];

pub static ROOK_DIRECTION: &[BoardDirection] = &[
    BoardDirection { dx: 1, dy: 0 },
    BoardDirection { dx: -1, dy: 0 },
    BoardDirection { dx: 0, dy: 1 },
    BoardDirection { dx: 0, dy: -1 },
];

pub const KING_AND_QUEEN_DIRECTION: [BoardDirection; 8] = [
    BoardDirection { dx: 1, dy: 0 },
    BoardDirection { dx: 1, dy: 1 },
    BoardDirection { dx: 0, dy: 1 },
    BoardDirection { dx: -1, dy: 0 },
    BoardDirection { dx: -1, dy: -1 },
    BoardDirection { dx: 0, dy: -1 },
    BoardDirection { dx: 1, dy: -1 },
    BoardDirection { dx: -1, dy: 1 },
];

pub static DIAGONAL_ATTACK_DIRECTION: &[BoardDirection] = &[
    BoardDirection { dx: 1, dy: 1 },
    BoardDirection { dx: -1, dy: 1 },
    BoardDirection { dx: 1, dy: -1 },
    BoardDirection { dx: -1, dy: -1 },
];

pub static KNIGHT_ATTACK_DIRECTION: &[BoardDirection] = &[
    BoardDirection { dx: 2, dy: 1 },
    BoardDirection { dx: 2, dy: -1 },
    BoardDirection { dx: -2, dy: 1 },
    BoardDirection { dx: -2, dy: -1 },
    BoardDirection { dx: 1, dy: 2 },
    BoardDirection { dx: 1, dy: -2 },
    BoardDirection { dx: -1, dy: 2 },
    BoardDirection { dx: -1, dy: -2 },
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
