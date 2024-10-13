/* This module has functions for updating metadata for a Chessboard struct */
use crate::chess::{ChessBoard, Move, PieceType, Position};

impl ChessBoard {
    pub fn update_meta_data(&mut self, move_to_make: &Move) {
        match move_to_make.meta_data.piece_to_move {
            PieceType::King => {
                self.update_castling_move_abilities();

                match move_to_make.meta_data.piece_to_capture {
                    Some(piece_to_capture) => {
                        self.handle_capture_target(piece_to_capture, *move_to_make);
                    }

                    None => {
                        self.increment_half_move_clock();
                    }
                }
            }

            PieceType::Rook => match move_to_make.meta_data.piece_to_capture {
                Some(piece_to_capture) => {
                    self.handle_capture_target(piece_to_capture, *move_to_make);
                }

                None => {
                    self.increment_half_move_clock();
                }
            },

            PieceType::Pawn => match move_to_make.meta_data.piece_to_capture {
                Some(piece_to_capture) => {
                    self.handle_capture_target(piece_to_capture, *move_to_make);
                }

                None => {
                    self.reset_half_move_clock();

                    if move_to_make.meta_data.generates_en_passant {
                        let en_passant_target_square: Position = if self.white_is_side_to_move {
                            (move_to_make.end_pos.0, move_to_make.end_pos.1 - 1)
                        } else {
                            (move_to_make.end_pos.0, move_to_make.end_pos.1 + 1)
                        };

                        self.en_passant_target_square = Some(en_passant_target_square);
                    }
                }
            },

            _ => match move_to_make.meta_data.piece_to_capture {
                Some(piece_to_capture) => {
                    self.handle_capture_target(piece_to_capture, *move_to_make);
                }

                None => {
                    self.increment_half_move_clock();
                }
            },
        }
        self.update_fullmove_counter();
        self.update_side_to_move();
    }

    fn update_fullmove_counter(&mut self) {
        if !self.white_is_side_to_move {
            self.full_move_counter += 1;
        }
    }

    fn update_side_to_move(&mut self) {
        self.white_is_side_to_move = !self.white_is_side_to_move;
    }

    fn set_no_en_passant(&mut self) {
        if self.en_passant_target_square.is_some() {
            self.en_passant_target_square = None;
        }
    }

    fn increment_half_move_clock(&mut self) {
        self.half_move_clock += 1;
    }
    fn reset_half_move_clock(&mut self) {
        self.half_move_clock = 0;
    }

    fn handle_capture_target(&mut self, capture_target: PieceType, move_to_make: Move) {
        self.set_no_en_passant();
        self.reset_half_move_clock();

        if capture_target == PieceType::Rook {
            self.handle_rook_capture_target(move_to_make.end_pos);
        }
    }

    fn handle_rook_capture_target(&mut self, rook_position: Position) {
        if self.white_is_side_to_move {
            if rook_position == (7, 7) {
                self.set_no_castling_ability(3);
            } else if rook_position == (0, 7) {
                self.set_no_castling_ability(2);
            }
        } else {
            if rook_position == (7, 0) {
                self.set_no_castling_ability(1);
            } else if rook_position == (0, 0) {
                self.set_no_castling_ability(0);
            }
        }
    }

    fn update_castling_move_abilities(&mut self) {
        if self.white_is_side_to_move {
            self.set_no_castling_ability(0);
            self.set_no_castling_ability(1);
        } else {
            self.set_no_castling_ability(2);
            self.set_no_castling_ability(3);
        }
    }

    fn set_no_castling_ability(&mut self, index: usize) {
        if self.castling_ability[index] {
            self.castling_ability[index] = false;
        }
    }
}
