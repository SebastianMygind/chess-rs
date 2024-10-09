use crate::chess::chess_moves::MoveDirection;
use crate::chess::{Board, ChessBoard, Color, Move, MoveMetaData, Piece, PieceType, Position};

pub fn get_single_step_moves(
    chess_board: &ChessBoard,
    piece_position: &Position,
    piece_type: PieceType,
    piece_color: &Color,
    directions: &[MoveDirection],
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    for direction in directions {
        if direction.piece_can_travel(&chess_board.board, piece_color, piece_position) {
            let new_position: Position = direction.walk_from_position(*piece_position);

            let optional_piece_to_capture: Option<PieceType> =
                if let Some(piece) = chess_board.board[new_position.1][new_position.0] {
                    Some(piece.piece_type)
                } else {
                    None
                };

            let meta_data: MoveMetaData = MoveMetaData {
                piece_to_move: piece_type,
                piece_to_capture: optional_piece_to_capture,
                promotion_piece: None,
                is_castling_move: false,
                generates_en_passant: false,
                is_en_passant_move: false,
            };

            let current_move: Move = Move {
                start_pos: *piece_position,
                end_pos: new_position,
                meta_data,
            };

            moves.push(current_move);
        }
    }

    moves
}

pub fn get_multi_step_moves(
    chess_board: &ChessBoard,
    piece_position: &Position,
    piece_color: &Color,
    directions: &[MoveDirection],
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    for direction in directions {
        let mut current_position: Position = *piece_position;

        while direction.piece_can_travel(&chess_board.board, piece_color, &current_position) {
            current_position = direction.walk_from_position(current_position);
            let move_obstructed: bool;
            let meta_data: MetaData =
                if chess_board.board[current_position.1][current_position.0] == None {
                    move_obstructed = false;
                    MetaData::Move
                } else {
                    move_obstructed = true;
                    MetaData::Capture
                };

            let current_move: Move = Move {
                start_pos: *piece_position,
                end_pos: current_position,
                meta_data,
            };
            moves.push(current_move);

            if move_obstructed {
                break;
            }
        }
    }

    moves
}

pub fn check_single_step_for_piece_exists(
    piece_to_check_for: Piece,
    board: &Board,
    directions: &[MoveDirection],
    starting_position: &Position,
) -> bool {
    for direction in directions {
        if direction.move_is_within_bounds(*starting_position) {
            let new_position = direction.walk_from_position(*starting_position);

            if let Some(piece) = board[new_position.1][new_position.0] {
                if piece == piece_to_check_for {
                    return true;
                }
            }
        }
    }
    false
}

pub fn check_multi_step_for_piece_exists(
    piece_to_check_for: Piece,
    board: &Board,
    directions: &[MoveDirection],
    starting_position: &Position,
) -> bool {
    for direction in directions {
        let mut current_position = *starting_position;
        while direction.move_is_within_bounds(current_position) {
            let new_position = direction.walk_from_position(current_position);

            if let Some(piece) = board[new_position.1][new_position.0] {
                if piece.color == piece_to_check_for.color {
                    if piece.piece_type == piece_to_check_for.piece_type {
                        return true;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                current_position = new_position;
            }
        }
    }
    false
}

pub fn find_first_matching_chess_piece(board: &Board, piece_to_find: &Piece) -> Option<Position> {
    for (i, file) in board.iter().enumerate() {
        for (j, square) in file.iter().enumerate() {
            if let Some(piece) = square {
                if *piece == *piece_to_find {
                    return Some((j, i));
                }
            }
        }
    }
    None
}
