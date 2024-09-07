/* This submodule implements moving in chess, this includes the actual move and also checking
 * For legal moves
 */
mod attack_logic;
mod king_check;
mod legal_moves;
mod meta_data;

use crate::chess::chess_errors::IllegalMove;
use crate::chess::chess_moves::meta_data::{update_capture, update_move};
use crate::chess::{
    BoardPiece, CaptureMove, CastlingMove, ChessBoard, Move, MoveInfo, MoveTypes,
    PawnPromotionMove, PieceMove, Pieces, Square, ARR_SIZE, EMPTY_PIECE,
};

impl ChessBoard {
    pub fn make_move(&mut self, move_to_make: Move) -> Result<(), IllegalMove> {
        let legal_moves = self.legal_moves();

        let mut is_legal_move = false;

        for possible_move in legal_moves {
            if possible_move == move_to_make {
                is_legal_move = true;
                break;
            }
        }

        if !is_legal_move {
            return Err(IllegalMove);
        }

        match move_to_make.move_type {
            MoveTypes::Move => {
                let piece_move: PieceMove = unsafe { move_to_make.move_specific.piece_move };
                let starting_square = piece_move.starting_square.pos_to_arr_index();
                let target_square = piece_move.ending_square.pos_to_arr_index();

                let piece = self.board[starting_square];

                update_move(self, &piece, &move_to_make);

                self.board[target_square] = piece;
                self.board[starting_square] = EMPTY_PIECE;
            }

            MoveTypes::Capture => {
                let capture: CaptureMove = unsafe { move_to_make.move_specific.capture };
                let starting_square = capture.starting_square.pos_to_arr_index();
                let target_square = capture.target_square.pos_to_arr_index();

                let piece = self.board[starting_square];

                update_capture(self);

                self.board[target_square] = piece;
                self.board[starting_square] = EMPTY_PIECE;
            }

            MoveTypes::PawnPromotion => {
                let promotion: PawnPromotionMove = unsafe { move_to_make.move_specific.promotion };

                let target_pos = promotion.target_square.pos_to_arr_index();

                let promoted_piece = BoardPiece {
                    piece_type: promotion.promotion_piece,
                };

                self.board[target_pos] = promoted_piece;
                self.board[target_pos - 8] = EMPTY_PIECE;
            }

            MoveTypes::Castle => {
                let castle: CastlingMove = unsafe { move_to_make.move_specific.castle };

                let king_file: u32;
                let rook_file: u32;

                let rook_to_set_empty_file: u32;

                let king_piece: BoardPiece;
                let rook_piece: BoardPiece;

                if castle.is_king_side {
                    king_file = 7;
                    rook_file = 6;

                    rook_to_set_empty_file = 8;
                } else {
                    king_file = 3;
                    rook_file = 4;

                    rook_to_set_empty_file = 0;
                }

                if castle.rank == 1 {
                    king_piece = BoardPiece::new(Pieces::WKing);
                    rook_piece = BoardPiece::new(Pieces::WRook);
                } else {
                    king_piece = BoardPiece::new(Pieces::BKing);
                    rook_piece = BoardPiece::new(Pieces::BRook);
                }

                let king_pos_arr = ((castle.rank - 1) * 8 + (king_file - 1)) as usize;
                let rook_pos_arr = ((castle.rank - 1) * 8 + (rook_file - 1)) as usize;

                let set_empty_1_arr = ((castle.rank - 1) * 8 + (5 - 1)) as usize;
                let set_empty_2_arr =
                    ((castle.rank - 1) * 8 + (rook_to_set_empty_file - 1)) as usize;

                self.board[set_empty_1_arr] = EMPTY_PIECE;
                self.board[set_empty_2_arr] = EMPTY_PIECE;

                self.board[king_pos_arr] = king_piece;
                self.board[rook_pos_arr] = rook_piece;
            }

            MoveTypes::EnPassant => {
                let en_passant = unsafe { move_to_make.move_specific.en_passant };

                let set_empty_1_arr = en_passant.pawn_to_move.pos_to_arr_index();
                let set_empty_2_arr = en_passant.pawn_to_capture.pos_to_arr_index();

                let mut target_square = set_empty_1_arr.clone();
                let pawn: BoardPiece;

                // Offset array position
                match en_passant.is_white_move {
                    true => {
                        pawn = BoardPiece::new(Pieces::WPawn);

                        if en_passant.pawn_to_move.file < en_passant.pawn_to_capture.file {
                            target_square += 9;
                        } else {
                            target_square += 7;
                        }
                    }
                    false => {
                        pawn = BoardPiece::new(Pieces::BPawn);

                        if en_passant.pawn_to_move.file > en_passant.pawn_to_capture.file {
                            target_square -= 9
                        } else {
                            target_square -= 7
                        }
                    }
                };

                self.board[set_empty_1_arr] = EMPTY_PIECE;
                self.board[set_empty_2_arr] = EMPTY_PIECE;

                self.board[target_square] = pawn;
            }
        }
        Ok(())
    }
}

impl Square {
    pub fn pos_to_arr_index(&self) -> usize {
        ((self.rank - 1) * 8 + self.file - 1) as usize
    }
}

pub fn arr_pos_to_square(arr_pos: usize) -> Square {
    let rank = (arr_pos / 8) + 1;
    let file = (arr_pos % 8) + 1;

    Square {
        rank: rank as u32,
        file: file as u32,
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
    return None;
}

pub struct BoardDirection {
    dx: i32,
    dy: i32,
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

    let x_pos: i32 = target_square.file as i32;
    let y_pos: i32 = target_square.rank as i32;

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

                    let loop_square = Square {
                        file: (x_pos + x) as u32,
                        rank: (y_pos + y) as u32,
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

                    let loop_square = Square {
                        file: (x_pos + x) as u32,
                        rank: (y_pos + y) as u32,
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

    return None;
}
