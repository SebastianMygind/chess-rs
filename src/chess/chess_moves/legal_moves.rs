mod bishop_piece;
mod generic_piece;
mod king_piece;
mod knight_piece;
mod pawn_piece;
mod queen_piece;
mod rook_piece;

use crate::chess::chess_moves::legal_moves::bishop_piece::get_bishop_moves;
use crate::chess::chess_moves::legal_moves::generic_piece::{
    find_first_matching_chess_piece, Color,
};
use crate::chess::chess_moves::legal_moves::king_piece::{get_king_moves, king_is_checked};
use crate::chess::chess_moves::legal_moves::knight_piece::get_knight_moves;
use crate::chess::chess_moves::legal_moves::pawn_piece::get_pawn_moves;
use crate::chess::chess_moves::legal_moves::queen_piece::get_queen_moves;
use crate::chess::chess_moves::legal_moves::rook_piece::get_rook_moves;
use crate::chess::{Board, BoardSquare, ChessBoard, MetaData, Move, Pieces, ARR_SIZE, EMPTY_PIECE};

impl ChessBoard {
    pub fn legal_moves(&self) -> Vec<Move> {
        let mut legal_moves: Vec<Move> = Vec::new();

        let pseudo_legal_moves: Vec<Move> = self.pseudo_legal_moves();

        let king_position = find_first_matching_chess_piece(&self.board, king_piece)
            .expect("Both kings most exist on all boards!");

        for piece_move in pseudo_legal_moves {
            let mut board_copy = self.board;
            Self::make_move_on_board(&mut board_copy, &piece_move);

            if board_copy[piece_move.end_pos].piece_type == Pieces::WKing
                || board_copy[piece_move.end_pos].piece_type == Pieces::BKing
            {
                if !king_is_checked(&board_copy, &piece_move.end_pos, &current_color) {
                    legal_moves.push(piece_move);
                }

                continue;
            }

            if !king_is_checked(&board_copy, &king_position, &current_color) {
                legal_moves.push(piece_move);
            }
        }
        legal_moves
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

    pub fn make_move_on_board(board: &mut Board, move_to_make: &Move) {
        match move_to_make.meta_data {
            MetaData::Move | MetaData::PawnMove | MetaData::Capture | MetaData::PawnDoubleMove => {
                board[move_to_make.end_pos] = board[move_to_make.start_pos];
                board[move_to_make.start_pos] = EMPTY_PIECE;
            }

            MetaData::Promotion(piece_type) => {
                board[move_to_make.end_pos] = BoardSquare { piece_type };
                board[move_to_make.start_pos] = EMPTY_PIECE;
            }

            MetaData::EnPassant => {
                let delta_position: i8 = if move_to_make.start_pos < move_to_make.end_pos {
                    8
                } else {
                    -8
                };

                board[move_to_make.end_pos] = board[move_to_make.start_pos];

                board[move_to_make.start_pos] = EMPTY_PIECE;
                board[(move_to_make.end_pos as i8 + delta_position) as usize] = EMPTY_PIECE;
            }

            MetaData::Castling => {
                let (rook_start_position, rook_end_position): (usize, usize) =
                    if move_to_make.start_pos < move_to_make.end_pos {
                        (move_to_make.start_pos + 3, move_to_make.start_pos + 1)
                    } else {
                        (move_to_make.start_pos - 4, move_to_make.start_pos - 1)
                    };

                board[move_to_make.end_pos] = board[move_to_make.start_pos];
                board[rook_end_position] = board[rook_start_position];

                board[move_to_make.start_pos] = EMPTY_PIECE;
                board[rook_start_position] = EMPTY_PIECE;
            }
        }
    }
}
