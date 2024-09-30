use crate::chess::chess_moves::MoveDirection;
use crate::chess::Pieces;
use crate::chess::Pieces::{
    BBishop, BKing, BKnight, BPawn, BQueen, BRook, WBishop, WKing, WKnight, WPawn, WQueen, WRook,
};

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

pub(crate) const WHITE_PIECES: [Pieces; 6] = [WPawn, WRook, WBishop, WKnight, WQueen, WKing];
pub(crate) const BLACK_PIECES: [Pieces; 6] = [BPawn, BRook, BBishop, BKnight, BQueen, BKing];
