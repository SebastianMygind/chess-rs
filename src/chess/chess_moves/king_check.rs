/* This module implements a function that checks if the king of "side to move" is in check  */

use crate::chess::chess_moves::{
    arr_pos_to_square, check_board_directions, find_first_matching_chess_piece, BoardDirection,
};
use crate::chess::{BoardPiece, ChessBoard, Pieces, Square, ARR_SIZE};

pub struct KingInCheck {
    // Vector of all squares that check the king
    attackers: Vec<usize>,
    king_pos: usize,
}

impl ChessBoard {
    pub fn king_in_check(&self) -> Option<KingInCheck> {
        let king_pos: usize;

        if self.white_is_side_to_move {
            king_pos = find_first_matching_chess_piece(&self.board, Pieces::WKing)
                .expect("ERROR: No white king on board!");
        } else {
            king_pos = find_first_matching_chess_piece(&self.board, Pieces::BKing)
                .expect("ERROR: No black king on board!");
        }

        let king_square = arr_pos_to_square(king_pos);

        let mut attackers: Vec<usize> = Vec::new();

        let diagonal_direction = vec![
            BoardDirection { dx: 1, dy: 1 },
            BoardDirection { dx: -1, dy: 1 },
            BoardDirection { dx: 1, dy: -1 },
            BoardDirection { dx: -1, dy: -1 },
        ];
        let white_pawn_direction = vec![
            BoardDirection { dx: 1, dy: 1 },
            BoardDirection { dx: 1, dy: -1 },
        ];
        let black_pawn_direction = vec![
            BoardDirection { dx: -1, dy: 1 },
            BoardDirection { dx: -1, dy: -1 },
        ];
        let horizontal_and_vertical_direction = vec![
            BoardDirection { dx: 1, dy: 0 },
            BoardDirection { dx: -1, dy: 0 },
            BoardDirection { dx: 0, dy: 1 },
            BoardDirection { dx: 0, dy: -1 },
        ];

        let knight_direction = vec![
            BoardDirection { dx: 2, dy: 1 },
            BoardDirection { dx: 2, dy: -1 },
            BoardDirection { dx: -2, dy: 1 },
            BoardDirection { dx: -2, dy: -1 },
            BoardDirection { dx: 1, dy: 2 },
            BoardDirection { dx: 1, dy: -2 },
            BoardDirection { dx: -1, dy: 2 },
            BoardDirection { dx: -1, dy: -2 },
        ];

        let diagonal_collision =
            check_board_directions(&self.board, king_pos.clone(), diagonal_direction, None);

        match diagonal_collision {
            Some(collisions) => {
                let attacking_pieces: Vec<Pieces> = if self.white_is_side_to_move {
                    vec![Pieces::BQueen, Pieces::BBishop]
                } else {
                    vec![Pieces::WQueen, Pieces::WBishop]
                };

                for collision in collisions {
                    if check_board_type_eq(&self.board, &collision, &attacking_pieces) {
                        attackers.push(collision);
                    }
                }
            }

            None => {}
        }

        let king_in_check = KingInCheck {
            attackers,
            king_pos,
        };

        return Some(KingInCheck {
            attackers: vec![],
            king_pos,
        });
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_pos_1() {
        assert_eq!(arr_pos_to_square(0), Square { file: 1, rank: 1 })
    }

    #[test]
    fn test_pos_2() {
        assert_eq!(arr_pos_to_square(8), Square { file: 1, rank: 2 })
    }

    #[test]
    fn test_pos_3() {
        assert_eq!(arr_pos_to_square(7), Square { file: 8, rank: 1 })
    }

    #[test]
    fn test_pos_4() {
        assert_eq!(arr_pos_to_square(63), Square { file: 8, rank: 8 })
    }

    #[test]
    fn test_pos_5() {
        assert_eq!(arr_pos_to_square(60), Square { file: 5, rank: 8 })
    }
}

fn check_board_type_eq(
    board: &[BoardPiece; ARR_SIZE],
    position_to_check: &usize,
    matches: &Vec<Pieces>,
) -> bool {
}
