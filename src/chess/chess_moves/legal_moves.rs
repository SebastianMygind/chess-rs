mod king_piece;
mod queen_piece;


use crate::chess::{BoardPiece, ChessBoard, Move, Pieces, SquarePosition, ARR_SIZE};
use crate::chess::chess_moves::legal_moves::king_piece::get_king_moves;
impl ChessBoard {
    pub fn legal_moves(&self) -> Vec<Move> {
        let mut legal_moves: Vec<Move> = Vec::new();
        todo!(" Check that all castling moves are allowed, specifically check that the squares
                between the rook and king are not attacked by any pieces.

                Make sure no move that is made leaves the king in check.
        ");

        let chess_move: Move = Move {
            start_pos: 12,
            end_pos: 20,
            meta_data: None,
        };

        return vec![chess_move];
    }

    /** This function returns all possible moves, but does not check for pinned pieces,
    checks and other special moves related to king checks **/
    fn pseudo_legal_moves(&self) -> Vec<Move> {
        let mut pseudo_legal_moves: Vec<Move> = Vec::new();

        for (index, piece) in self.board.iter().enumerate() {
            match piece.piece_type {
                Pieces::Empty => continue,

                Pieces::WKing | Pieces::BKing => {
                    let king_moves = get_king_moves(self, &index);
                }

                Pieces::WQueen | Pieces::BQueen => {}

                Pieces::WRook | Pieces::BRook => {}

                Pieces::WBishop | Pieces::BBishop => {}

                Pieces::WKnight | Pieces::BKnight => {}

                Pieces::WPawn => {}

                Pieces::BPawn => {}
            }
        }
        vec![]
    }
}

pub fn can_capture_piece(capturing_piece: &Pieces, piece_to_capture: &Pieces) -> bool {
    let capturing_is_white = piece_is_white(capturing_piece);
    let piece_to_capture_is_white = piece_is_white(piece_to_capture);

    if *capturing_piece == Pieces::Empty {
        panic!("illegal piece to capture with: Cannot capture with an empty piece");
    } else if *piece_to_capture == Pieces::Empty {
        return true;
    }

    capturing_is_white != piece_to_capture_is_white
}

fn piece_is_white(piece: &Pieces) -> bool {
    match *piece {
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

        Pieces::Empty => {false},
    }
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

    #[test]
    fn capture_empty() { assert_eq!(can_capture_piece(&Pieces::WPawn, &Pieces::Empty), true); }
}
