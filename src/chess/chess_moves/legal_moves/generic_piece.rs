use crate::chess::chess_moves::MoveDirection;
use crate::chess::{Board, ChessBoard, Color, MetaData, Move, Piece, Position};

pub fn get_single_step_moves(
    chess_board: &ChessBoard,
    piece_position: &Position,
    piece_color: &Color,
    directions: &[MoveDirection],
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    for direction in directions {
        if direction.piece_can_travel(&chess_board.board, piece_color, piece_position) {
            let new_position: Position = direction.walk_from_position(*piece_position);

            let meta_data: MetaData = if chess_board.board[new_position.1][new_position.0] == None {
                MetaData::Move
            } else {
                MetaData::Capture
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

            let meta_data: MetaData =
                if chess_board.board[current_position.1][current_position.0] == None {
                    MetaData::Move
                } else {
                    MetaData::Capture
                };

            let current_move: Move = Move {
                start_pos: *piece_position,
                end_pos: current_position,
                meta_data,
            };
            moves.push(current_move);
        }
    }

    moves
}

pub fn check_single_step_for_piece_exists(
    piece_to_check_for: Piece,
    board: &Board,
    directions: &[MoveDirection],
    friendly_color: &Color,
    starting_position: &Position,
) -> bool {
    for direction in directions {
        if direction.piece_can_travel(board, friendly_color, starting_position) {
            let new_position = direction.walk_from_position(*starting_position);

            if let Some(piece) = board[new_position.1][new_position.0] {
                return if piece == piece_to_check_for {
                    true
                } else {
                    false
                };
            }
        }
    }
    false
}

pub fn check_multi_step_for_piece_exists(
    piece_to_check_for: Piece,
    board: &Board,
    directions: &[MoveDirection],
    friendly_color: &Color,
    starting_position: &Position,
) -> bool {
    for direction in directions {
        let mut current_position = *starting_position;
        while direction.piece_can_travel(board, friendly_color, &current_position) {
            current_position = direction.walk_from_position(current_position);

            match board[current_position.1][current_position.0] {
                Some(piece) => {
                    if piece == piece_to_check_for {
                        return true;
                    } else {
                        break;
                    }
                }
                None => continue,
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
