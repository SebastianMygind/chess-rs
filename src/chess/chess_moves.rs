/* This submodule implements moving in chess, this includes the actual move and also checking
 * For legal moves
 */
pub mod attack_logic;
mod king_check;
mod legal_moves;
mod meta_data;

use crate::chess::chess_errors::IllegalMove;
use crate::chess::chess_moves::meta_data::{update_capture, update_move};
use crate::chess::{BoardPiece, ChessBoard, Move, Pieces, SquarePosition, ARR_SIZE, EMPTY_PIECE};

impl ChessBoard {
    pub fn make_move(&mut self, move_to_make: Move) -> Result<Move, IllegalMove> {
        return Ok(move_to_make);
    }
}

impl SquarePosition {
    pub fn pos_to_arr_index(&self) -> usize {
        ((self.rank - 1) * 8 + self.file - 1) as usize
    }
}

pub fn arr_pos_to_square(arr_pos: usize) -> SquarePosition {
    let rank = (arr_pos / 8) + 1;
    let file = (arr_pos % 8) + 1;

    SquarePosition {
        rank: rank as u8,
        file: file as u8,
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

/** A generic function that takes a chess board, a board position, a vector of directions
 * and a depth, it will then check all directions for any non-empty piece
 *
**/
pub fn check_board_directions(
    board: &[BoardPiece; ARR_SIZE],
    position: usize,
    directions: &[BoardDirection],
    move_depth: Option<u8>, // None means no depth limit, i.e. searches till collision or end of board
) -> Option<Vec<usize>> {
    let target_square = arr_pos_to_square(position);

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
                        file: (x_pos + x) as u8,
                        rank: (y_pos + y) as u8,
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
                        file: (x_pos + x) as u8,
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
