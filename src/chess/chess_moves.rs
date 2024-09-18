/* This submodule implements moving in chess, this includes the actual move and also checking
 * For legal moves
 */

mod legal_moves;
mod piece_logic;
mod meta_data;
mod king_check;

use iced::Border;
use crate::chess::chess_errors::IllegalMove;
use crate::chess::{BoardPiece, ChessBoard, Move, Pieces, SquarePosition, ARR_SIZE};

impl ChessBoard {
    pub fn make_move(&mut self, move_to_make: Move) -> Result<Move, IllegalMove> {
        return Ok(move_to_make);
    }
}

impl SquarePosition {
    pub fn pos_to_arr_index(&self) -> usize {

        (self.rank - 1) * 8 + self.file - 1
    }

    pub fn new(arr_pos: usize) -> SquarePosition {
        let rank = (arr_pos / 8) + 1;
        let file = (arr_pos % 8) + 1;

        SquarePosition {
            rank,
            file,
        }
    }
}

pub fn find_first_matching_chess_piece(
    board: &[BoardPiece; ARR_SIZE],
    piece_to_find: Pieces,
) -> Option<usize> {
    for (pos, square) in board.iter().enumerate() {
        if square.piece_type == piece_to_find {
            return Some(pos);
        }
    }
    None
}

pub struct BoardDirection {
    dx: i8,
    dy: i8,
}

impl BoardDirection {
    pub fn piece_can_travel(&self,
                            board: &[BoardPiece; ARR_SIZE],
                            friendly_pieces: &[Pieces; 6],
                            board_position: &usize) -> bool {
        let mut piece_square = SquarePosition::new(*board_position);

        let x_pos: i8 = (piece_square.file as i8) + self.dx;
        let y_pos: i8 = (piece_square.rank as i8) + self.dy;

        if x_pos < 0 || y_pos < 0 {
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

        let delta_position: i8 = self.dx + (self.dy * 8);

        if (delta_position.unsigned_abs() > position as u8) {
            panic!("Error walking from position: Check that a move is within bounds before \
            calling this function!")
        } else {
            new_position = position + delta_position as usize;
        }

        new_position
    }
}

/** A generic function that takes a chess board, a board position, a vector of directions
   and a depth, it will then check all directions for any non-empty piece

*/
pub fn check_board_directions(
    board: &[BoardPiece; ARR_SIZE],
    position: usize,
    directions: &[BoardDirection],
    move_depth: Option<u8>, // None means no depth limit, i.e. searches till collision or end of board
) -> Option<Vec<usize>> {
    let target_square = SquarePosition::new(position);

    let x_pos: i8 = target_square.file as i8;
    let y_pos: i8 = target_square.rank as i8;

    let mut vector: Vec<usize> = Vec::new();

    match move_depth {
        Some(depth) => {
            for direction in directions {
                let mut x = direction.dx;
                let mut y = direction.dy;
                let mut current_depth: u8 = 1;
                while (x_pos + x > 0 && x_pos + x <= 8)
                    && (y_pos + y > 0 && y_pos + y <= 8)
                    && (current_depth < depth)
                {
                    x += direction.dx;
                    y += direction.dy;

                    let loop_square = SquarePosition {
                        file: (x_pos + x) as usize,
                        rank: (y_pos + y) as usize,
                    };
                    let loop_pos = loop_square.pos_to_arr_index();

                    if board[loop_pos].piece_type != Pieces::Empty {
                        vector.push(loop_pos);
                        break;
                    }
                    current_depth += 1;
                }
            }
        }

        None => {
            for direction in directions {
                let mut x = direction.dx;
                let mut y = direction.dy;

                while (x_pos + x > 0 && x_pos + x <= 8) && (y_pos + y > 0 && y_pos + y <= 8) {
                    x += direction.dx;
                    y += direction.dy;

                    let loop_square = SquarePosition {
                        file: (x_pos + x) as usize,
                        rank: (y_pos + y) as u8,
                    };
                    let loop_pos = loop_square.pos_to_arr_index();

                    if board[loop_pos].piece_type != Pieces::Empty {
                        vector.push(loop_pos);
                        break;
                    }
                }
            }
        }
    }
    if vector.len() > 0 {
        return Some(vector);
    }

    None
}
