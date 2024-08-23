use crate::chess::chess_moves::BoardDirection;
use crate::chess::Pieces::{
    BBishop, BKnight, BPawn, BQueen, BRook, WBishop, WKnight, WPawn, WQueen, WRook,
};
use crate::chess::{BoardPiece, Pieces, ARR_SIZE};

pub static WHITE_PAWN_ATTACK_DIRECTION: &[BoardDirection] = &[
    BoardDirection { dx: 1, dy: 1 },
    BoardDirection { dx: 1, dy: -1 },
];

pub static BLACK_PAWN_ATTACK_DIRECTION: &[BoardDirection] = &[
    BoardDirection { dx: -1, dy: 1 },
    BoardDirection { dx: -1, dy: -1 },
];

pub static HORIZONTAL_AND_VERTICAL_ATTACK_DIRECTION: &[BoardDirection] = &[
    BoardDirection { dx: 1, dy: 0 },
    BoardDirection { dx: -1, dy: 0 },
    BoardDirection { dx: 0, dy: 1 },
    BoardDirection { dx: 0, dy: -1 },
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
    return attackers;
}

pub enum Directions {
    Diagonal,
    HorizontalAndVertical,
    WhitePawn,
    BlackPawn,
    Knight,
}

pub fn get_potential_attacking_pieces(
    direction: Directions,
    white_is_side_to_move: &bool,
) -> Vec<Pieces> {
    let is_white_bool = *white_is_side_to_move;

    match direction {
        Directions::Diagonal => {
            if is_white_bool {
                vec![BQueen, BBishop]
            } else {
                vec![WQueen, WBishop]
            }
        }
        Directions::HorizontalAndVertical => {
            if is_white_bool {
                vec![BQueen, BRook]
            } else {
                vec![WQueen, WRook]
            }
        }
        Directions::Knight => {
            if is_white_bool {
                vec![BKnight]
            } else {
                vec![WKnight]
            }
        }
        Directions::WhitePawn => {
            vec![WPawn]
        }
        Directions::BlackPawn => {
            vec![BPawn]
        }
    }
}
