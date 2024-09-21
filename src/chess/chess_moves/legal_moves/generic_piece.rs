use crate::chess::chess_moves::piece_logic::{BLACK_PIECES, WHITE_PIECES};
use crate::chess::chess_moves::MoveDirection;
use crate::chess::{ChessBoard, MetaData, Move, Pieces};

pub fn get_single_step_moves(
    chess_board: &ChessBoard,
    piece_position: &usize,
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

pub fn get_friendly_and_enemy_pieces(white_is_side_to_move: bool) -> ([Pieces; 6], [Pieces; 6]) {
    if white_is_side_to_move {
        (WHITE_PIECES, BLACK_PIECES)
    } else {
        (BLACK_PIECES, WHITE_PIECES)
    }
}
