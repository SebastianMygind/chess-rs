mod bishop_piece;
pub mod generic_piece;
mod king_piece;
mod knight_piece;
mod pawn_piece;
mod queen_piece;
mod rook_piece;

use crate::chess::chess_moves::legal_moves::bishop_piece::get_bishop_moves;
use crate::chess::chess_moves::legal_moves::king_piece::get_king_moves;
use crate::chess::chess_moves::legal_moves::knight_piece::get_knight_moves;
use crate::chess::chess_moves::legal_moves::pawn_piece::get_pawn_moves;
use crate::chess::chess_moves::legal_moves::queen_piece::get_queen_moves;
use crate::chess::chess_moves::legal_moves::rook_piece::get_rook_moves;
use crate::chess::{ChessBoard, MetaData, Move, Pieces};

impl ChessBoard {
    pub fn legal_moves(&self) -> Vec<Move> {
        let mut legal_moves: Vec<Move> = Vec::new();

        let pseudo_legal_moves: Vec<Move> = self.pseudo_legal_moves();

        todo!(
            " Check that all castling moves are allowed, specifically check that the squares
                between the rook and king are not attacked by any pieces.

                Make sure no move that is made leaves the king in check.
        "
        );

        let chess_move: Move = Move {
            start_pos: 12,
            end_pos: 20,
            meta_data: MetaData::Move,
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
                    let mut king_moves = get_king_moves(self, &index);
                    pseudo_legal_moves.append(&mut king_moves);
                }

                Pieces::WQueen | Pieces::BQueen => {
                    let mut queen_moves = get_queen_moves(self, &index);
                    pseudo_legal_moves.append(&mut queen_moves);
                }

                Pieces::WRook | Pieces::BRook => {
                    let mut rook_moves = get_rook_moves(self, &index);
                    pseudo_legal_moves.append(&mut rook_moves);
                }

                Pieces::WBishop | Pieces::BBishop => {
                    let mut bishop_moves = get_bishop_moves(self, &index);
                    pseudo_legal_moves.append(&mut bishop_moves);
                }

                Pieces::WKnight | Pieces::BKnight => {
                    let mut knight_moves = get_knight_moves(self, &index);
                    pseudo_legal_moves.append(&mut knight_moves);
                }

                Pieces::WPawn | Pieces::BPawn => {
                    let mut pawn_moves = get_pawn_moves(self, &index);
                    pseudo_legal_moves.append(&mut pawn_moves);
                }
            }
        }
        pseudo_legal_moves
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

        Pieces::Empty => false,
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
    fn capture_empty() {
        assert_eq!(can_capture_piece(&Pieces::WPawn, &Pieces::Empty), true);
    }
}
