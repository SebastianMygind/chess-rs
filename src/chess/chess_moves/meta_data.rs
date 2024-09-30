/* This module has functions for updating metadata for a Chessboard struct */
use crate::chess::{ChessBoard, MetaData, Move};

impl ChessBoard {
    pub fn update_meta_data(&mut self, move_to_make: &Move) {
        match move_to_make.meta_data {
            MetaData::Move => {
                self.increment_half_move_clock();
            }

            MetaData::Capture => {
                self.reset_half_move_clock();
                self.set_no_en_passant();
            }

            MetaData::Castling => {
                self.increment_half_move_clock();
                self.set_no_en_passant();
            }

            MetaData::PawnMove => {
                self.reset_half_move_clock();
            }

            MetaData::PawnDoubleMove => {
                if move_to_make.start_pos < move_to_make.end_pos {
                    self.en_passant_target_square = Some(move_to_make.start_pos + 8);
                } else if move_to_make.start_pos > move_to_make.end_pos {
                    self.en_passant_target_square = Some(move_to_make.start_pos - 8);
                } else {
                    panic!(
                        "PawnDoubleMoves not generated correctly, start position = end position!"
                    );
                }
            }

            MetaData::EnPassant => {
                self.reset_half_move_clock();
                self.set_no_en_passant();
            }

            MetaData::Promotion(_) => {
                self.reset_half_move_clock();
                self.set_no_en_passant();
            }
        }
        self.update_fullmove_counter();
        self.update_side_to_move();
    }

    fn update_fullmove_counter(&mut self) {
        if !self.white_is_side_to_move {
            self.fullmove_counter += 1;
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
        self.halfmove_clock += 1;
    }
    fn reset_half_move_clock(&mut self) {
        self.halfmove_clock = 0;
    }
}
