/* This module implements a function that checks if the king of "side to move" is in check  */

use crate::chess::chess_moves::{
    arr_pos_to_square, check_diagonal_for_pieces, find_first_matching_chess_piece,
};
use crate::chess::{ChessBoard, Pieces, Square};

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

        let mut diagonal_pieces_pos: Vec<usize> = Vec::new();

        match check_diagonal_for_pieces(&self.board, king_pos) {
            Some(pieces) => diagonal_pieces_pos = pieces,
            None => {}
        }

        let mut attackers: Vec<usize> = Vec::new();

        for piece in diagonal_pieces_pos {
            match self.board[piece].piece_type {
                Pieces::WQueen | Pieces::WBishop => {
                    if !self.white_is_side_to_move {
                        attackers.push(piece);
                    }
                }

                Pieces::BQueen | Pieces::BBishop => {
                    if self.white_is_side_to_move {
                        attackers.push(piece);
                    }
                }

                Pieces::WPawn => {
                    if !self.white_is_side_to_move
                        && (piece + 7 == king_pos || piece + 9 == king_pos)
                    {
                        attackers.push(piece);
                    }
                }

                Pieces::BPawn => {
                    if self.white_is_side_to_move
                        && (piece as i64 - 7 == king_pos as i64
                            || piece as i64 - 9 == king_pos as i64)
                    {
                        attackers.push(piece);
                    }
                }

                _ => {}
            }
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
