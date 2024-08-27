use crate::chess::{
    CaptureMove, CastlingMove, ChessBoard, Move, MoveInfo, MoveTypes, PieceMove, Pieces, Square,
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

    /** This function returns all possible moves, but does not check for pinned pieces,
    checks and other special moves **/
    fn pseudo_legal_moves(&self) -> Vec<Move> {
        for (index, piece) in self.board.iter().enumerate() {
            match piece.piece_type {
                Pieces::Empty => {
                    continue;
                }

                Pieces::BBishop | Pieces::WBishop => {}

                Pieces::WQueen | Pieces::BQueen => {}

                Pieces::WRook | Pieces::BRook => {}

                Pieces::WKing | Pieces::BKing => {}

                Pieces::WKnight | Pieces::BKnight => {}

                Pieces::WPawn => {}

                Pieces::BPawn => {}
            }
        }

        return vec![];
    }
}
