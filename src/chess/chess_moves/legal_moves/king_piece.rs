use crate::chess::chess_moves::piece_logic::{BLACK_PIECES, KING_AND_QUEEN_DIRECTION, WHITE_PIECES};
use crate::chess::{BoardPiece, ChessBoard, MetaData, Move, Pieces, ARR_SIZE, EMPTY_PIECE};

pub fn get_king_moves(chess_board: &ChessBoard, piece_position: &usize) -> Vec<Move> {
    let mut king_moves: Vec<Move> = Vec::new();

    let friendly_pieces: [Pieces; 6];
    let enemy_pieces: [Pieces; 6];

    if chess_board.white_is_side_to_move {
        friendly_pieces = WHITE_PIECES;
        enemy_pieces = BLACK_PIECES;
    } else {
        friendly_pieces = BLACK_PIECES;
        enemy_pieces = WHITE_PIECES;
    };

    for direction in KING_AND_QUEEN_DIRECTION {
        if direction.piece_can_travel(&chess_board.board, &friendly_pieces, piece_position ) {
            let piece_move = Move {start_pos: *piece_position,
                end_pos: direction.walk_from_position(*piece_position),
                meta_data: None};

            king_moves.push(piece_move);
        }
        let (kingside_castle_index, queenside_castle_index): (usize, usize) =
            if chess_board.white_is_side_to_move {
                (0,1)
            } else {
                (2,3)
            };

        if chess_board.castling_ability[kingside_castle_index] {
            let positions_to_check: [usize; 2] = [piece_position + 1, piece_position + 2];

            if check_board_for_empty_pieces(positions_to_check.as_slice(), &chess_board.board) {
                let castling_move: Move = Move {
                    start_pos: *piece_position,
                    end_pos: *piece_position + 2,
                    meta_data: Some(MetaData::Castling),
                };

                king_moves.push(castling_move);
            }
        }

        if chess_board.castling_ability[queenside_castle_index] {
            let positions_to_check: [usize; 3] = [
                piece_position - 1,
                piece_position - 2,
                piece_position -3];

            if check_board_for_empty_pieces(positions_to_check.as_slice(), &chess_board.board) {
                let castling_move: Move = Move {
                    start_pos: *piece_position,
                    end_pos: *piece_position - 2,
                    meta_data: Some(MetaData::Castling),
                };
                king_moves.push(castling_move);
            }
        }

    }

    king_moves
}

/** Returns true if all given positions are empty */
fn check_board_for_empty_pieces(positions: &[usize], board: &[BoardPiece; ARR_SIZE]) -> bool {
    for position in positions {
        if board[*position] != EMPTY_PIECE {
            return false;
        }
    }

    true
}