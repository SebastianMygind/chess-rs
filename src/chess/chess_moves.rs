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
            return Err(IllegalMove {
                attempted_move: move_to_make,
            });
        }

        self.make_move_on_board(&move_to_make);

        self.update_meta_data(&move_to_make);

        Ok(move_to_make)
    }
}

#[derive(Clone, Copy)]
pub struct MoveDirection {
    dx: i8,
    dy: i8,
}

impl Move {
    pub fn move_to_string(&self) -> String {
        let start_file: char = Self::file_to_char(self.start_pos.0);
        let end_file: char = Self::file_to_char(self.end_pos.0);

        format!(
            "{start_file}{}{end_file}{}",
            self.start_pos.1 + 1,
            self.end_pos.1 + 1
        )
    }

    fn file_to_char(file_num: usize) -> char {
        match file_num {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => {
                panic!("Illegal position file: {file_num}")
            }
        }
    }
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

        let new_i8_x: i8 = self.dx + casted_x;
        let new_i8_y: i8 = self.dy + casted_y;

        if new_i8_x > 7 || new_i8_x < 0 || new_i8_y > 7 || new_i8_y < 0 {
            return false;
        }

        let new_x: usize = new_i8_x as usize;
        let new_y: usize = new_i8_y as usize;
        let target_piece = board[new_y][new_x];

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

    pub fn move_is_within_bounds(&self, position: Position) -> bool {
        let casted_x: i8 = position.0 as i8;
        let casted_y: i8 = position.1 as i8;

        let new_i8_x: i8 = self.dx + casted_x;
        let new_i8_y: i8 = self.dy + casted_y;

        if new_i8_x > 7 || new_i8_x < 0 || new_i8_y > 7 || new_i8_y < 0 {
            return false;
        }

        true
    }
}
