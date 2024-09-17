/* This module implements a function that checks if the king of "side to move" is in check  */

use crate::chess::chess_moves::attack_logic::{
    check_for_attackers, get_potential_attacking_pieces, Directions, BLACK_PAWN_ATTACK_DIRECTION,
    DIAGONAL_ATTACK_DIRECTION, KNIGHT_ATTACK_DIRECTION, ROOK_DIRECTION,
    WHITE_PAWN_ATTACK_DIRECTION,
};
use crate::chess::chess_moves::{
    arr_pos_to_square, check_board_directions, find_first_matching_chess_piece, BoardDirection,
};
use crate::chess::{BoardPiece, ChessBoard, Pieces, SquarePosition, ARR_SIZE};

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

        // Check the diagonal
        let diagonal_collision = check_board_directions(
            &self.board,
            king_pos.clone(),
            DIAGONAL_ATTACK_DIRECTION,
            None,
        );

        attackers.extend(check_for_attackers(
            &self.board,
            diagonal_collision,
            get_potential_attacking_pieces(Directions::Diagonal, &self.white_is_side_to_move),
        ));

        // Check vertical and horizontal
        let ver_hor_collisions =
            check_board_directions(&self.board, king_pos.clone(), ROOK_DIRECTION, None);

        attackers.extend(check_for_attackers(
            &self.board,
            ver_hor_collisions,
            get_potential_attacking_pieces(
                Directions::HorizontalAndVertical,
                &self.white_is_side_to_move,
            ),
        ));

        // Check for knights
        let knight_collisions = check_board_directions(
            &self.board,
            king_pos.clone(),
            KNIGHT_ATTACK_DIRECTION,
            Some(1),
        );

        attackers.extend(check_for_attackers(
            &self.board,
            knight_collisions,
            get_potential_attacking_pieces(Directions::Knight, &self.white_is_side_to_move),
        ));

        // Check Pawn collisions

        let mut attacking_pawn: Vec<Pieces> = Vec::new();

        let pawn_collisions = if self.white_is_side_to_move {
            attacking_pawn.push(Pieces::BPawn);

            check_board_directions(
                &self.board,
                king_pos.clone(),
                BLACK_PAWN_ATTACK_DIRECTION,
                Some(1),
            )
        } else {
            attacking_pawn.push(Pieces::WPawn);
            check_board_directions(
                &self.board,
                king_pos.clone(),
                WHITE_PAWN_ATTACK_DIRECTION,
                Some(1),
            )
        };

        attackers.extend(check_for_attackers(
            &self.board,
            pawn_collisions,
            attacking_pawn,
        ));

        if attackers.len() == 0 {
            return None;
        }

        Some(KingInCheck {
            attackers,
            king_pos,
        })
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_pos_1() {
        assert_eq!(arr_pos_to_square(0), SquarePosition { file: 1, rank: 1 })
    }

    #[test]
    fn test_pos_2() {
        assert_eq!(arr_pos_to_square(8), SquarePosition { file: 1, rank: 2 })
    }

    #[test]
    fn test_pos_3() {
        assert_eq!(arr_pos_to_square(7), SquarePosition { file: 8, rank: 1 })
    }

    #[test]
    fn test_pos_4() {
        assert_eq!(arr_pos_to_square(63), SquarePosition { file: 8, rank: 8 })
    }

    #[test]
    fn test_pos_5() {
        assert_eq!(arr_pos_to_square(60), SquarePosition { file: 5, rank: 8 })
    }
}

fn check_board_type_eq(
    board: &[BoardPiece; ARR_SIZE],
    position_to_check: &usize,
    matches: &Vec<Pieces>,
) -> bool {
    for type_match in matches {
        if board[*position_to_check].piece_type == *type_match {
            return true;
        }
    }
    false
}
