/* This submodule implements moving in chess, this includes the actual move and also checking
 * For legal moves
 */
mod legal_moves;
pub mod meta_data;
mod piece_logic;

use crate::chess::chess_errors::IllegalMove;
use crate::chess::{Board, ChessBoard, Color, Move, Position};

impl ChessBoard {
    pub fn make_move(&mut self, move_to_make: Move) -> Result<Move, IllegalMove> {
        let legal_moves = self.legal_moves();
        let mut move_is_legal: bool = false;
        for legal_move in legal_moves {
            if move_to_make == legal_move {
                move_is_legal = true;

                break;
            }
        }

        if !move_is_legal {
            return Err(IllegalMove);
        }

        Self::make_move_on_board(&mut self.board, &move_to_make);

        self.update_meta_data(&move_to_make);

        Ok(move_to_make)
    }
}

#[derive(Clone, Copy)]
pub struct MoveDirection {
    dx: i8,
    dy: i8,
}

impl MoveDirection {
    pub fn piece_can_travel(
        &self,
        board: &Board,
        friendly_piece_color: &Color,
        board_position: &Position,
    ) -> bool {
        let casted_x: i8 = board_position.0 as i8;
        let casted_y: i8 = board_position.1 as i8;
        if self.dx + casted_x > 7
            || self.dx + casted_x < 0
            || self.dy + casted_y > 7
            || self.dy + casted_y < 0
        {
            return false;
        }

        let target_piece = board[board_position.1][board_position.0];

        match target_piece {
            None => true,

            Some(piece) => {
                if piece.color == *friendly_piece_color {
                    false
                } else {
                    true
                }
            }
        }
    }

    /**
      given a position this function returns a new position after traveling the direction given by
      self.
    */
    pub fn walk_from_position(&self, position: Position) -> Position {
        (
            (position.0 as i8 + self.dx) as usize,
            (position.1 as i8 + self.dy) as usize,
        )
    }
}
