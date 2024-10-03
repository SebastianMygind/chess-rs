use crate::chess::chess_moves::piece_logic::{BLACK_PIECES, WHITE_PIECES};
use crate::chess::chess_moves::MoveDirection;
use crate::chess::Pieces::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::chess::{Board, BoardSquare, ChessBoard, MetaData, Move};

pub fn get_single_step_moves(
    chess_board: &ChessBoard,
    piece_position: &(usize, usize),
    directions: &[MoveDirection],
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let (friendly_pieces, _) = get_friendly_and_enemy_pieces(chess_board.white_is_side_to_move);

    for direction in directions {
        if direction.piece_can_travel(&chess_board.board, &friendly_pieces, piece_position) {
            let new_position: usize = direction.walk_from_position(*piece_position);

            let meta_data: MetaData = if chess_board.board[new_position].piece_type == Pieces::Empty
            {
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
    piece_position: &usize,
    directions: &[MoveDirection],
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let (friendly_pieces, _) = get_friendly_and_enemy_pieces(chess_board.white_is_side_to_move);

    for direction in directions {
        let mut current_position: usize = *piece_position;

        while direction.piece_can_travel(&chess_board.board, &friendly_pieces, &current_position) {
            current_position = direction.walk_from_position(current_position);

            let meta_data: MetaData =
                if chess_board.board[current_position].piece_type == Pieces::Empty {
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
    piece_to_check_for: &Pieces,
    board: &[BoardSquare; ARR_SIZE],
    directions: &[MoveDirection],
    starting_position: &usize,
) -> bool {
    let enemy_pieces: [Pieces; 6];

    match get_enemy_pieces_from_color(get_color_from_piece(*piece_to_check_for)) {
        Some(enemies) => enemy_pieces = enemies,
        None => return false,
    }

    for direction in directions {
        if direction.piece_can_travel(board, &enemy_pieces, starting_position) {
            let new_position = direction.walk_from_position(*starting_position);

            if board[new_position].piece_type == *piece_to_check_for {
                return true;
            }
        }
    }
    false
}

pub fn check_multi_step_for_piece_exists(
    piece_to_check_for: &Pieces,
    board: &[BoardSquare; ARR_SIZE],
    directions: &[MoveDirection],
    starting_position: &usize,
) -> bool {
    let enemy_pieces: [Pieces; 6];

    match get_enemy_pieces_from_color(get_color_from_piece(*piece_to_check_for)) {
        Some(enemies) => enemy_pieces = enemies,
        None => return false,
    }

    for direction in directions {
        let mut current_position = *starting_position;
        while direction.piece_can_travel(board, &enemy_pieces, &current_position) {
            current_position = direction.walk_from_position(current_position);

            if board[current_position].piece_type == *piece_to_check_for {
                return true;
            }
            if board[current_position].piece_type == Pieces::Empty {
                continue;
            } else {
                break;
            }
        }
    }
    false
}

pub fn find_first_matching_chess_piece(board: &Board, piece_to_find: Pieces) -> Option<usize> {
    for (pos, square) in board.iter().enumerate() {
        if square.piece_type == piece_to_find {
            return Some(pos);
        }
    }
    None
}

fn get_color_from_piece(piece: Pieces) -> Color {
    match piece {
        WKing | WKnight | WRook | WQueen | WBishop | WPawn => Color::White,

        BKing | BKnight | BRook | BQueen | BBishop | BPawn => Color::Black,

        Pieces::Empty => Color::Empty,
    }
}

pub fn get_enemy_pieces_from_color(color: Color) -> Option<[Pieces; 6]> {
    match color {
        Color::Black => Some([WKing, WQueen, WRook, WBishop, WKnight, WPawn]),

        Color::White => Some([BKing, BQueen, BRook, BBishop, BKnight, BPawn]),

        Color::Empty => None,
    }
}
