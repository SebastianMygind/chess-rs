/* This submodule implements moving in chess, this includes the actual move and also checking
 * For legal moves
 */
use crate::chess::chess_errors::IllegalMove;
use crate::chess::{CaptureMove, ChessBoard, Move, MoveInfo, MoveTypes, Pieces, Square};

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
            MoveTypes::Capture => {
                let capture: CaptureMove = unsafe { move_to_make.move_specific.capture };
                let starting_square = capture.starting_square.pos_to_arr_index();
                let target_square = capture.target_square.pos_to_arr_index();

                let piece = self.board[starting_square];

                self.board[target_square] = piece;
                self.board[starting_square].piece_type = Pieces::Empty;
            }

            MoveTypes::PawnPromotion => {}

            MoveTypes::Castle => {}

            MoveTypes::EnPassant => {}
        }
        Ok(())
    }
}

impl Square {
    pub fn pos_to_arr_index(&self) -> usize {
        ((self.rank - 1) * 8 + self.file - 1) as usize
    }
}
