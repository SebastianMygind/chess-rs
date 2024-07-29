/* This submodule implements moving in chess, this includes the actual move and also checking
 * For legal moves
 */
mod meta_data;

use crate::chess::chess_errors::IllegalMove;
use crate::chess::{BoardPiece, CaptureMove, CastlingMove, ChessBoard, Move, MoveInfo, MoveTypes, PawnPromotionMove, Pieces, Square, EMPTY_PIECE, PieceMove};

impl ChessBoard {
    pub fn legal_moves() -> Vec<Move> {
        let chess_move: Move = Move {
            move_type: MoveTypes::Capture,
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
        let legal_moves = Self::legal_moves();

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

                if piece.piece_type == Pieces::WPawn || piece.piece_type == Pieces::BPawn {

                    // Update metadata for 50 move rule.
                    self.halfmove_clock = 0;

                }

                self.board[target_square] = piece;
                self.board[starting_square] = EMPTY_PIECE;
            }

            MoveTypes::Capture => {
                let capture: CaptureMove = unsafe { move_to_make.move_specific.capture };
                let starting_square = capture.starting_square.pos_to_arr_index();
                let target_square = capture.target_square.pos_to_arr_index();

                let piece = self.board[starting_square];

                self.board[target_square] = piece;
                self.board[starting_square].piece_type = Pieces::Empty;

                // Update metadata for 50 move rule.
                self.halfmove_clock = 0;
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

/** This function returns all possible moves, but does not check for pinned pieces,
  * checks and other special moves **/
fn pseudo_legal_moves(chessboard: ChessBoard) -> Vec<Move> {
    for piece in chessboard.board {

    }

    return;
}
