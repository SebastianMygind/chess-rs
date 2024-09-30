/* This submodule implements moving in chess, this includes the actual move and also checking
 * For legal moves
 */
mod legal_moves;
pub mod meta_data;
mod piece_logic;

use crate::chess::chess_errors::IllegalMove;
use crate::chess::{BoardPiece, ChessBoard, Move, Pieces, SquarePosition, ARR_SIZE};

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

impl SquarePosition {
    pub fn pos_to_arr_index(&self) -> usize {
        if self.rank == 0 || self.file == 0 {
            panic!(
                "ERROR: square position {}, {}. Should be above 0",
                self.rank, self.file
            );
        }

        (self.rank - 1) * 8 + self.file - 1
    }

    pub fn new(arr_pos: usize) -> SquarePosition {
        let rank = (arr_pos / 8) + 1;
        let file = (arr_pos % 8) + 1;

        SquarePosition { rank, file }
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
        board: &[BoardPiece; ARR_SIZE],
        friendly_pieces: &[Pieces; 6],
        board_position: &usize,
    ) -> bool {
        let mut piece_square = SquarePosition::new(*board_position);

        let x_pos: i8 = (piece_square.file as i8) + self.dx;
        let y_pos: i8 = (piece_square.rank as i8) + self.dy;

        if x_pos < 1 || y_pos < 1 || x_pos > 8 || y_pos > 8 {
            return false;
        } else {
            piece_square.file = x_pos as usize;
            piece_square.rank = y_pos as usize;
        }

        let target_piece = board[piece_square.pos_to_arr_index()].piece_type;

        for friend in friendly_pieces {
            if *friend == target_piece {
                return false;
            }
        }
        true
    }

    /**
      given a position this function returns a new position after traveling the direction given by
      self.
    */
    pub fn walk_from_position(&self, position: usize) -> usize {
        let new_position: usize;
        let mut move_is_valid = true;

        let delta_position: i8 = self.dx + (self.dy * 8);

        let square_position = SquarePosition::new(position);

        if (square_position.file as i8 + self.dx) < 1 || (square_position.rank as i8 + self.dy) < 1
        {
            move_is_valid = false;
        }

        if !move_is_valid {
            panic!(
                "Error walking from position: Check that a move is within bounds before \
            calling this function!"
            )
        } else {
            new_position = (position as i8 + delta_position) as usize;
        }

        new_position
    }
}
