use crate::chess::chess_moves::piece_logic::{
    BLACK_PAWN_ATTACK_DIRECTION, BLACK_PAWN_DIRECTION, WHITE_PAWN_ATTACK_DIRECTION,
    WHITE_PAWN_DIRECTION,
};
use crate::chess::chess_moves::MoveDirection;
use crate::chess::Color::{Black, White};
use crate::chess::PieceType::{Bishop, Knight, Queen, Rook};
use crate::chess::{Board, ChessBoard, Color, MetaData, Move, Piece, Position};

pub fn get_pawn_moves(
    chess_board: &ChessBoard,
    friendly_color: &Color,
    piece_position: &Position,
) -> Vec<Move> {
    let (direction, attack_direction) = get_pawn_direction_and_attack(friendly_color);

    let single_pawn_move_is_legal: bool;

    let mut pawn_moves: Vec<Move> = Vec::with_capacity(3);

    if let Some(pawn_move) = get_pawn_single_move(
        friendly_color,
        piece_position,
        &chess_board.board,
        direction,
    ) {
        pawn_moves.push(pawn_move);
        single_pawn_move_is_legal = true;
    } else {
        single_pawn_move_is_legal = false;
    }

    let travelable_attack_direction: [bool; 2] = [
        attack_direction[0].piece_can_travel(&chess_board.board, friendly_color, piece_position),
        attack_direction[1].piece_can_travel(&chess_board.board, friendly_color, piece_position),
    ];

    if travelable_attack_direction[0] {
        if let Some(pawn_move) = check_pawn_attack_and_en_passant_move(
            attack_direction[0],
            &chess_board.board,
            piece_position,
            friendly_color,
            chess_board.en_passant_target_square,
        ) {
            pawn_moves.push(pawn_move);
        }
    }

    if travelable_attack_direction[1] {
        if let Some(pawn_move) = check_pawn_attack_and_en_passant_move(
            attack_direction[1],
            &chess_board.board,
            piece_position,
            friendly_color,
            chess_board.en_passant_target_square,
        ) {
            pawn_moves.push(pawn_move);
        }
    }

    let mut promotion_moves =
        get_promotion_moves(friendly_color, piece_position, &chess_board.board);

    if promotion_moves.len() > 0 {
        pawn_moves.append(&mut promotion_moves);
    }

    if single_pawn_move_is_legal {
        if let Some(move_exists) = get_pawn_double_move(
            friendly_color,
            piece_position,
            &chess_board.board,
            &direction,
        ) {
            pawn_moves.push(move_exists);
        };
    }

    pawn_moves
}

fn check_pawn_attack_and_en_passant_move(
    attack_direction: MoveDirection,
    board: &Board,
    current_position: &Position,
    friendly_color: &Color,
    en_passant_position: Option<Position>,
) -> Option<Move> {
    let possible_attack_position: Position = attack_direction.walk_from_position(*current_position);

    if let Some(piece) = board[possible_attack_position.1][possible_attack_position.0] {
        if piece.color != *friendly_color {
            Some(Move {
                start_pos: *current_position,
                end_pos: possible_attack_position,
                meta_data: MetaData::Capture,
            })
        } else {
            None
        }
    } else {
        check_en_passant_move(
            *current_position,
            en_passant_position,
            possible_attack_position,
        )
    }
}

fn get_pawn_direction_and_attack(
    current_side_color: &Color,
) -> (MoveDirection, [MoveDirection; 2]) {
    if White == *current_side_color {
        (WHITE_PAWN_DIRECTION, WHITE_PAWN_ATTACK_DIRECTION)
    } else {
        (BLACK_PAWN_DIRECTION, BLACK_PAWN_ATTACK_DIRECTION)
    }
}

fn check_en_passant_move(
    start_position: Position,
    en_passant_position: Option<Position>,
    attack_position: Position,
) -> Option<Move> {
    if let Some(en_passant) = en_passant_position {
        if en_passant == attack_position {
            return Some(Move {
                start_pos: start_position,
                end_pos: attack_position,
                meta_data: MetaData::EnPassant,
            });
        }
    }
    None
}

fn get_pawn_single_move(
    current_piece_color: &Color,
    piece_position: &Position,
    board: &Board,
    direction: MoveDirection,
) -> Option<Move> {
    if !direction.piece_can_travel(board, current_piece_color, piece_position) {
        return None;
    }
    let new_position = direction.walk_from_position(*piece_position);

    if board[new_position.1][new_position.0] != None {
        return None;
    }

    Some(Move {
        start_pos: *piece_position,
        end_pos: new_position,
        meta_data: MetaData::PawnMove,
    })
}

fn get_pawn_double_move(
    current_piece_color: &Color,
    piece_position: &Position,
    board: &Board,
    direction: &MoveDirection,
) -> Option<Move> {
    let (pawn_starting_rank, direction_change): (usize, i8) = if *current_piece_color == White {
        (1, 1)
    } else {
        (6, -1)
    };

    let mut double_move_direction = *direction;
    double_move_direction.dy += direction_change;

    if pawn_starting_rank == piece_position.1
        && double_move_direction.piece_can_travel(board, current_piece_color, piece_position)
    {
        let new_position = double_move_direction.walk_from_position(*piece_position);

        if board[new_position.1][new_position.0] == None {
            let double_move: Move = Move {
                start_pos: *piece_position,
                end_pos: new_position,
                meta_data: MetaData::PawnDoubleMove,
            };
            return Some(double_move);
        }
    }
    None
}
fn get_promotion_moves(piece_color: &Color, piece_position: &Position, board: &Board) -> Vec<Move> {
    let mut promotion_moves: Vec<Move> = Vec::new();

    let (direction, _): (MoveDirection, [MoveDirection; 2]) =
        get_pawn_direction_and_attack(piece_color);

    let promotion_rank: usize = if *piece_color == White { 6 } else { 1 };

    let promotion_pieces = if *piece_color == White {
        [
            Piece::new(White, Queen),
            Piece::new(White, Rook),
            Piece::new(White, Bishop),
            Piece::new(White, Knight),
        ]
    } else {
        [
            Piece::new(Black, Queen),
            Piece::new(Black, Rook),
            Piece::new(Black, Bishop),
            Piece::new(Black, Knight),
        ]
    };

    if piece_position.1 == promotion_rank
        && direction.piece_can_travel(board, piece_color, piece_position)
    {
        for piece in promotion_pieces {
            let promotion_move: Move = Move {
                start_pos: *piece_position,
                end_pos: direction.walk_from_position(*piece_position),
                meta_data: MetaData::Promotion(piece),
            };

            promotion_moves.push(promotion_move);
        }
    }

    promotion_moves
}
