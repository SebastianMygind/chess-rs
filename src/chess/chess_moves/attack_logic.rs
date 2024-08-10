use crate::chess::chess_moves::BoardDirection;
use crate::chess::Pieces::{
    BBishop, BKnight, BPawn, BQueen, BRook, WBishop, WKnight, WPawn, WQueen, WRook,
};
use crate::chess::{BoardPiece, Pieces, ARR_SIZE};

pub const WHITE_PAWN_ATTACK_DIRECTION: Vec<BoardDirection> = vec![
    BoardDirection { dx: 1, dy: 1 },
    BoardDirection { dx: 1, dy: -1 },
];

pub const BLACK_PAWN_ATTACK_DIRECTION: Vec<BoardDirection> = vec![
    BoardDirection { dx: -1, dy: 1 },
    BoardDirection { dx: -1, dy: -1 },
];

pub const HORIZONTAL_AND_VERTICAL_ATTACK_DIRECTION: Vec<BoardDirection> = vec![
    BoardDirection { dx: 1, dy: 0 },
    BoardDirection { dx: -1, dy: 0 },
    BoardDirection { dx: 0, dy: 1 },
    BoardDirection { dx: 0, dy: -1 },
];

pub const DIAGONAL_ATTACK_DIRECTION: Vec<BoardDirection> = vec![
    BoardDirection { dx: 1, dy: 1 },
    BoardDirection { dx: -1, dy: 1 },
    BoardDirection { dx: 1, dy: -1 },
    BoardDirection { dx: -1, dy: -1 },
];

pub const KNIGHT_ATTACK_DIRECTION: Vec<BoardDirection> = vec![
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
                for piece in attacking_pieces {
                    if board[collision].piece_type == piece {
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
    is_white_to_move: &bool,
) -> Vec<Pieces> {
    return match direction {
        Directions::Diagonal => {
            if is_white_to_move {
                vec![BQueen, BBishop]
            } else {
                vec![WQueen, WBishop]
            }
        }
        Directions::HorizontalAndVertical => {
            if is_white_to_move {
                vec![BQueen, BRook]
            } else {
                vec![WQueen, WRook]
            }
        }
        Directions::Knight => {
            if is_white_to_move {
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
    };
}
