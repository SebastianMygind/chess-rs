/* This submodule implements moving in chess, this includes the actual move and also checking
 * For legal moves
 */
mod king_check;
mod meta_data;

use crate::chess::chess_errors::IllegalMove;
use crate::chess::chess_moves::meta_data::{update_capture, update_move};
use crate::chess::{
    BoardPiece, CaptureMove, CastlingMove, ChessBoard, Move, MoveInfo, MoveTypes,
    PawnPromotionMove, PieceMove, Pieces, Square, ARR_SIZE, EMPTY_PIECE,
};

impl ChessBoard {
    pub fn legal_moves(&self) -> Vec<Move> {
        let chess_move: Move = Move {
            move_type: MoveTypes::Move,
            move_specific: MoveInfo {
                capture: {
                    CaptureMove {
                        starting_square: Square { rank: 2, file: 5 },
                        target_square: Square { rank: 4, file: 5 },
                    }
                },
            },
        };

        return vec![chess_move];
    }

    pub fn make_move(&mut self, move_to_make: Move) -> Result<(), IllegalMove> {
        let legal_moves = Self::legal_moves(self);

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
                self.board[starting_square].piece_type = Pieces::Empty;
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

                let mut king_file: u32;
                let mut rook_file: u32;

                let mut rook_to_set_empty_file: u32;

                let mut king_piece: BoardPiece;
                let mut rook_piece: BoardPiece;

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
                let mut pawn: BoardPiece;

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

    return Square {
        rank: rank as u32,
        file: file as u32,
    };
}

/** This function returns all possible moves, but does not check for pinned pieces,
checks and other special moves **/
fn pseudo_legal_moves(chessboard: ChessBoard) -> Vec<Move> {
    for piece in chessboard.board {}

    return vec![];
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

/**
    This function takes a board and an array position in the board and check all diagonals for
    pieces. The function will only return the first collision in the diagonal, so it does not check
    for pinned pieces or piece color/type.
**/
pub fn check_diagonal_for_pieces(
    board: &[BoardPiece; ARR_SIZE],
    arr_pos: usize,
) -> Option<Vec<usize>> {
    let square = arr_pos_to_square(arr_pos);

    let mut vector: Vec<usize> = Vec::new();

    let check_up: &u32 = &(8 - square.rank);
    let check_down: &u32 = &(square.rank - 1);

    let check_right: &u32 = &(8 - square.file);
    let check_left: &u32 = &(square.file - 1);

    match check_up_right(&arr_pos, check_up, check_right, board) {
        Some(pos) => vector.push(pos),
        None => {}
    }

    match check_up_left(&arr_pos, check_up, check_left, board) {
        Some(pos) => vector.push(pos),
        None => {}
    }

    match check_down_right(&arr_pos, check_down, check_right, board) {
        Some(pos) => vector.push(pos),
        None => {}
    }

    match check_down_left(&arr_pos, check_down, check_left, board) {
        Some(pos) => vector.push(pos),
        None => {}
    }

    if vector.len() == 0 {
        return None;
    }

    return Some(vector);
}

fn check_up_right(
    start_pos: &usize,
    up: &u32,
    right: &u32,
    board: &[BoardPiece; ARR_SIZE],
) -> Option<usize> {
    let mut position: usize = *start_pos;

    while *up > 0 && *right > 0 {
        position += 9;

        if board[position].piece_type != Pieces::Empty {
            return Some(position);
        }
    }

    return None;
}

fn check_up_left(
    start_pos: &usize,
    up: &u32,
    left: &u32,
    board: &[BoardPiece; ARR_SIZE],
) -> Option<usize> {
    let mut position: usize = *start_pos;

    while *up > 0 && *left > 0 {
        position += 7;

        if board[position].piece_type != Pieces::Empty {
            return Some(position);
        }
    }

    return None;
}

fn check_down_right(
    start_pos: &usize,
    down: &u32,
    right: &u32,
    board: &[BoardPiece; ARR_SIZE],
) -> Option<usize> {
    let mut position: usize = *start_pos;

    while *down > 0 && *right > 0 {
        position -= 7;

        if board[position].piece_type != Pieces::Empty {
            return Some(position);
        }
    }

    return None;
}

fn check_down_left(
    start_pos: &usize,
    down: &u32,
    left: &u32,
    board: &[BoardPiece; ARR_SIZE],
) -> Option<usize> {
    let mut position: usize = *start_pos;

    while *down > 0 && *left > 0 {
        position -= 9;

        if board[position].piece_type != Pieces::Empty {
            return Some(position);
        }
    }

    return None;
}
