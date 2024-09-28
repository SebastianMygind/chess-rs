use crate::chess::chess_moves::legal_moves::generic_piece::{
    get_friendly_and_enemy_pieces, get_single_step_moves,
};
use crate::chess::chess_moves::piece_logic::{
    BLACK_PAWN_ATTACK_DIRECTION, BLACK_PAWN_DIRECTION, WHITE_PAWN_ATTACK_DIRECTION,
    WHITE_PAWN_DIRECTION,
};
use crate::chess::chess_moves::MoveDirection;
use crate::chess::{BoardPiece, ChessBoard, MetaData, Move, Pieces, SquarePosition, ARR_SIZE};

pub fn get_pawn_moves(chess_board: &ChessBoard, piece_position: &usize) -> Vec<Move> {
    let (friendly_pieces, enemy_pieces) =
        get_friendly_and_enemy_pieces(chess_board.white_is_side_to_move);
    
    let (direction, attack_direction) =
        get_pawn_direction_and_attack(chess_board.white_is_side_to_move);

    let square_piece_position: SquarePosition = SquarePosition::new(*piece_position);

    let mut pawn_moves: Vec<Move> =
        get_single_step_moves(chess_board, piece_position, [direction].as_slice());

    let travelable_attack_direction: [bool; 2] = [
        attack_direction[0].piece_can_travel(&chess_board.board, &friendly_pieces, piece_position),
        attack_direction[1].piece_can_travel(&chess_board.board, &friendly_pieces, piece_position),
    ];

    let square = chess_board.en_passant_target_square;

    if travelable_attack_direction[0] {
        let possible_attack_position: usize =
            attack_direction[0].walk_from_position(*piece_position);
        for enemy in enemy_pieces {
            if chess_board.board[possible_attack_position].piece_type == enemy {
                let pawn_attack_move: Move = Move {
                    start_pos: *piece_position,
                    end_pos: possible_attack_position,
                    meta_data: MetaData::Capture,
                };
                pawn_moves.push(pawn_attack_move);
            }
        }
        if let Some(en_passant_move) =
            check_en_passant_move(*piece_position, square, possible_attack_position)
        {
            pawn_moves.push(en_passant_move);
        }
    }

    if travelable_attack_direction[1] {
        let possible_attack_position: usize =
            attack_direction[1].walk_from_position(*piece_position);
        for enemy in enemy_pieces {
            if chess_board.board[possible_attack_position].piece_type == enemy {
                let pawn_attack_move: Move = Move {
                    start_pos: *piece_position,
                    end_pos: possible_attack_position,
                    meta_data: MetaData::Capture,
                };
                pawn_moves.push(pawn_attack_move);
            }
        }
        if let Some(en_passant_move) =
            check_en_passant_move(*piece_position, square, possible_attack_position)
        {
            pawn_moves.push(en_passant_move);
        }
    }

    let mut promotion_moves = get_promotion_moves(
        chess_board.white_is_side_to_move,
        piece_position,
        &chess_board.board,
        &friendly_pieces,
        &square_piece_position,
    );

    pawn_moves.append(&mut promotion_moves);

    if let Some(move_exists) = get_pawn_double_move(
        chess_board.white_is_side_to_move,
        piece_position,
        &chess_board.board,
        &friendly_pieces,
        &square_piece_position,
        &direction,
    ) {
        pawn_moves.push(move_exists);
    };

    pawn_moves
}

fn get_pawn_direction_and_attack(
    white_is_side_to_move: bool,
) -> (MoveDirection, [MoveDirection; 2]) {
    if white_is_side_to_move {
        (WHITE_PAWN_DIRECTION, WHITE_PAWN_ATTACK_DIRECTION)
    } else {
        (BLACK_PAWN_DIRECTION, BLACK_PAWN_ATTACK_DIRECTION)
    }
}

fn check_en_passant_move(
    start_position: usize,
    en_passant_position: Option<usize>,
    attack_position: usize,
) -> Option<Move> {
    if let Some(en_passant) = en_passant_position {
        return if en_passant == attack_position {
            Some(Move {
                start_pos: start_position,
                end_pos: attack_position,
                meta_data: MetaData::EnPassant,
            })
        } else {
            None
        };
    }
    None
}

fn get_pawn_double_move(
    white_is_side_to_move: bool,
    piece_position: &usize,
    board: &[BoardPiece; ARR_SIZE],
    friendly_pieces: &[Pieces; 6],
    square_position: &SquarePosition,
    direction: &MoveDirection,
) -> Option<Move> {
    let (pawn_starting_rank, direction_change): (usize, i8) = if white_is_side_to_move {
        (2, 1)
    } else {
        (7, -1)
    };

    let mut double_move_direction = *direction;
    double_move_direction.dy += direction_change;

    if pawn_starting_rank == square_position.rank
        && double_move_direction.piece_can_travel(board, friendly_pieces, piece_position)
    {
        let double_move: Move = Move {
            start_pos: *piece_position,
            end_pos: double_move_direction.walk_from_position(*piece_position),
            meta_data: MetaData::Move,
        };
        return Some(double_move);
    }
    None
}
fn get_promotion_moves(
    white_is_side_to_move: bool,
    piece_position: &usize,
    board: &[BoardPiece; ARR_SIZE],
    friendly_pieces: &[Pieces; 6],
    square_position: &SquarePosition,
) -> Vec<Move> {
    let mut promotion_moves: Vec<Move> = Vec::new();

    let (direction, _): (MoveDirection, [MoveDirection; 2]) =
        get_pawn_direction_and_attack(white_is_side_to_move);

    let (promotion_rank, target_rank): (usize, usize) = if white_is_side_to_move {
        (7, 8)
    } else {
        (2, 1)
    };

    let promotion_pieces = if white_is_side_to_move {
        [
            Pieces::WQueen,
            Pieces::WKnight,
            Pieces::WRook,
            Pieces::WBishop,
        ]
    } else {
        [
            Pieces::BQueen,
            Pieces::BKnight,
            Pieces::BRook,
            Pieces::BBishop,
        ]
    };

    if square_position.rank == promotion_rank
        && direction.piece_can_travel(board, friendly_pieces, piece_position)
    {
        for piece in promotion_pieces {
            let mut moved_square_position = square_position.clone();

            moved_square_position.rank = target_rank;

            let promotion_move: Move = Move {
                start_pos: *piece_position,
                end_pos: moved_square_position.pos_to_arr_index(),
                meta_data: MetaData::Promotion(piece),
            };

            promotion_moves.push(promotion_move);
        }
    }

    promotion_moves
}
