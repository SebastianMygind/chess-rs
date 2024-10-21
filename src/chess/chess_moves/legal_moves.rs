mod bishop_piece;
mod generic_piece;
mod king_piece;
mod knight_piece;
mod pawn_piece;
mod queen_piece;
mod rook_piece;

use crate::chess::chess_moves::legal_moves::bishop_piece::get_bishop_moves;
use crate::chess::chess_moves::legal_moves::generic_piece::find_first_matching_chess_piece;
use crate::chess::chess_moves::legal_moves::king_piece::{get_king_moves, king_is_checked};
use crate::chess::chess_moves::legal_moves::knight_piece::get_knight_moves;
use crate::chess::chess_moves::legal_moves::pawn_piece::get_pawn_moves;
use crate::chess::chess_moves::legal_moves::queen_piece::get_queen_moves;
use crate::chess::chess_moves::legal_moves::rook_piece::get_rook_moves;
use crate::chess::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::chess::{ChessBoard, Color, Move, Piece, PieceType, Position};

impl ChessBoard {
    pub fn legal_moves(&self) -> Vec<Move> {
        let mut legal_moves: Vec<Move> = Vec::with_capacity(50);

        let current_color: Color = if self.white_is_side_to_move {
            Color::White
        } else {
            Color::Black
        };

        let pseudo_legal_moves: Vec<Move> = self.pseudo_legal_moves();

        let king_position = find_first_matching_chess_piece(
            &self.board,
            &Piece::new(current_color, PieceType::King),
        )
        .expect("Both kings most exist on all boards!");

        for piece_move in pseudo_legal_moves {
            let position_to_check: Position =
                if piece_move.meta_data.piece_to_move == PieceType::King {
                    piece_move.end_pos
                } else {
                    king_position
                };

            let mut board_copy = *self;
            board_copy.make_move_on_board(&piece_move);

            if !king_is_checked(&board_copy.board, &position_to_check, &current_color) {
                legal_moves.push(piece_move);
            }
        }
        legal_moves
    }

    /** This function returns all possible moves, but does not check for pinned pieces,
    checks and other special moves related to king checks **/
    fn pseudo_legal_moves(&self) -> Vec<Move> {
        let mut pseudo_legal_moves: Vec<Move> = Vec::new();

        let friendly_color = if self.white_is_side_to_move {
            Color::White
        } else {
            Color::Black
        };

        for (row, squares) in self.board.iter().enumerate() {
            for (column, square) in squares.iter().enumerate() {
                match square {
                    None => continue,
                    Some(piece) => {
                        if piece.color == friendly_color {
                            match piece.piece_type {
                                King => {
                                    let mut moves =
                                        get_king_moves(self, &friendly_color, &(column, row));
                                    pseudo_legal_moves.append(&mut moves);
                                }

                                Queen => {
                                    let mut moves =
                                        get_queen_moves(self, &friendly_color, &(column, row));
                                    pseudo_legal_moves.append(&mut moves);
                                }

                                Rook => {
                                    let mut moves =
                                        get_rook_moves(self, &friendly_color, &(column, row));
                                    pseudo_legal_moves.append(&mut moves);
                                }

                                Bishop => {
                                    let mut moves =
                                        get_bishop_moves(self, &friendly_color, &(column, row));
                                    pseudo_legal_moves.append(&mut moves);
                                }

                                Knight => {
                                    let mut moves =
                                        get_knight_moves(self, &friendly_color, &(column, row));
                                    pseudo_legal_moves.append(&mut moves);
                                }

                                Pawn => {
                                    let mut moves =
                                        get_pawn_moves(self, &friendly_color, &(column, row));
                                    pseudo_legal_moves.append(&mut moves);
                                }
                            }
                        }
                    }
                }
            }
        }
        pseudo_legal_moves
    }

    pub fn make_move_on_board(&mut self, move_to_make: &Move) {
        if move_to_make.meta_data.is_castling_move {
            let (rook_start_position, rook_end_position): (Position, Position) =
                if move_to_make.start_pos.0 < move_to_make.end_pos.0 {
                    (
                        (move_to_make.start_pos.0 + 3, move_to_make.start_pos.1),
                        (move_to_make.start_pos.0 + 1, move_to_make.start_pos.1),
                    )
                } else {
                    (
                        (move_to_make.start_pos.0 - 4, move_to_make.start_pos.1),
                        (move_to_make.start_pos.0 - 1, move_to_make.start_pos.1),
                    )
                };

            self.board[move_to_make.end_pos.1][move_to_make.end_pos.0] =
                self.board[move_to_make.start_pos.1][move_to_make.start_pos.0];

            self.board[rook_end_position.1][rook_end_position.0] =
                self.board[rook_start_position.1][rook_start_position.0];

            self.board[move_to_make.start_pos.1][move_to_make.start_pos.0] = None;
            self.board[rook_start_position.1][rook_start_position.0] = None;
        } else if move_to_make.meta_data.is_en_passant_move {
            let target_square: Position = if move_to_make.start_pos.1 < move_to_make.end_pos.1 {
                (move_to_make.end_pos.0, move_to_make.end_pos.1 - 1)
            } else {
                (move_to_make.end_pos.0, move_to_make.end_pos.1 + 1)
            };

            self.board[move_to_make.end_pos.1][move_to_make.end_pos.0] =
                self.board[move_to_make.start_pos.1][move_to_make.start_pos.0];

            self.board[move_to_make.start_pos.1][move_to_make.start_pos.0] = None;
            self.board[target_square.1][target_square.0] = None;
        } else if let Some(promotion_piece_type) = move_to_make.meta_data.promotion_piece {
            let promotion_piece = if self.white_is_side_to_move {
                Piece::new(Color::White, promotion_piece_type)
            } else {
                Piece::new(Color::Black, promotion_piece_type)
            };
            self.board[move_to_make.start_pos.1][move_to_make.start_pos.0] = None;
            self.board[move_to_make.end_pos.1][move_to_make.end_pos.0] = Some(promotion_piece);
        } else {
            self.board[move_to_make.end_pos.1][move_to_make.end_pos.0] =
                self.board[move_to_make.start_pos.1][move_to_make.start_pos.0];
            self.board[move_to_make.start_pos.1][move_to_make.start_pos.0] = None;
        }
    }
}
