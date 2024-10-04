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
use crate::chess::{Board, ChessBoard, Color, MetaData, Move, Piece, PieceType, Position};
use iced::widget::shader::wgpu::naga::back::msl::sampler::Coord;

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
            let mut board_copy = self.board;
            Self::make_move_on_board(&mut board_copy, &piece_move);

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

        let friendly_color = if self.white_is_side_to_move {
            Color::White
        } else {
            Color::Black
        };

        for (row, squares) in self.board.iter().enumerate() {
            for (column, square) in squares.iter().enumerate() {
                match square {
                    None => continue,
                    Some(piece) => match piece.piece_type {
                        King => {
                            let mut moves = get_king_moves(self, &friendly_color, &(column, row));
                            pseudo_legal_moves.append(&mut moves);
                        }

                        Queen => {
                            let mut moves = get_queen_moves(self, &friendly_color, &(column, row));
                            pseudo_legal_moves.append(&mut moves);
                        }

                        Rook => {
                            let mut moves = get_rook_moves(self, &friendly_color, &(column, row));
                            pseudo_legal_moves.append(&mut moves);
                        }

                        Bishop => {
                            let mut moves = get_bishop_moves(self, &friendly_color, &(column, row));
                            pseudo_legal_moves.append(&mut moves);
                        }

                        Knight => {
                            let mut moves = get_knight_moves(self, &friendly_color, &(column, row));
                            pseudo_legal_moves.append(&mut moves);
                        }

                        Pawn => {
                            let mut moves = get_pawn_moves(self, &friendly_color, &(column, row));
                            pseudo_legal_moves.append(&mut moves);
                        }
                    },
                }
            }
        }
        pseudo_legal_moves
    }

    pub fn make_move_on_board(board: &mut Board, move_to_make: &Move) {
        let start_pos_x = move_to_make.start_pos.0;
        let start_pos_y = move_to_make.start_pos.1;

        let end_pos_x = move_to_make.end_pos.0;
        let end_pos_y = move_to_make.end_pos.1;

        match move_to_make.meta_data {
            MetaData::Move | MetaData::PawnMove | MetaData::Capture | MetaData::PawnDoubleMove => {
                board[end_pos_y][end_pos_x] = board[start_pos_y][start_pos_x];
                board[start_pos_y][start_pos_x] = None;
            }

            MetaData::Promotion(piece) => {
                board[end_pos_y][end_pos_x] = Some(piece);
                board[start_pos_y][start_pos_x] = None;
            }

            MetaData::EnPassant => {
                let target_square: Position = if move_to_make.start_pos.1 < move_to_make.end_pos.1 {
                    (end_pos_x, end_pos_y + 1)
                } else {
                    (end_pos_x, end_pos_y - 1)
                };

                board[end_pos_y][end_pos_x] = board[start_pos_y][start_pos_x];

                board[start_pos_y][start_pos_x] = None;
                board[target_square.1][target_square.0] = None;
            }

            MetaData::Castling => {
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

                board[end_pos_y][end_pos_x] = board[start_pos_y][start_pos_x];
                board[rook_end_position.1][rook_end_position.0] =
                    board[rook_start_position.1][rook_start_position.0];

                board[start_pos_y][start_pos_x] = None;
                board[rook_start_position.1][rook_start_position.0] = None;
            }
        }
    }
}
