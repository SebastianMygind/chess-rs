use crate::chess::{
    BoardPiece, CaptureMove, CastlingMove, ChessBoard, Move, MoveInfo, MoveTypes, PieceMove,
    Pieces, Square, ARR_SIZE,
};

use super::attack_logic::{get_potential_attacking_pieces, DIAGONAL_ATTACK_DIRECTION};

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
        let mut moves: Vec<Move> = Vec::new();

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

fn singe_piece_legal_moves(piece: Pieces, board: &[BoardPiece; ARR_SIZE]) -> Vec<u8> {
    return Vec::new();
}

fn can_capture_piece(capturing_piece: &Pieces, piece_to_capture: &Pieces) -> bool {
    let capturing_is_white = piece_is_white(capturing_piece);
    let piece_to_capture_is_white = piece_is_white(piece_to_capture);

    return capturing_is_white != piece_to_capture_is_white;
}

fn piece_is_white(piece: &Pieces) -> bool {
    return match *piece {
        Pieces::WKing
        | Pieces::WKnight
        | Pieces::WRook
        | Pieces::WQueen
        | Pieces::WBishop
        | Pieces::WPawn => true,

        Pieces::BKing
        | Pieces::BKnight
        | Pieces::BRook
        | Pieces::BQueen
        | Pieces::BBishop
        | Pieces::BPawn => false,

        Pieces::Empty => false,
    };
}

// TESTS!!

// Testing function: can_capture_piece
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capture_test1() {
        assert_eq!(can_capture_piece(&Pieces::WKing, &Pieces::BKing), true);
    }

    #[test]
    fn capture_test2() {
        assert_eq!(can_capture_piece(&Pieces::WPawn, &Pieces::WRook), false);
    }
}
