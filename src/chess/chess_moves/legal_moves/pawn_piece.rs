use crate::chess::chess_moves::piece_logic::{
    BLACK_PAWN_ATTACK_DIRECTION, BLACK_PAWN_DIRECTION, PROMOTION_PIECES,
    WHITE_PAWN_ATTACK_DIRECTION, WHITE_PAWN_DIRECTION,
};
use crate::chess::chess_moves::MoveDirection;
use crate::chess::Color::White;
use crate::chess::{Board, ChessBoard, Color, Move, MoveMetaData, PieceType, Position};

pub fn get_pawn_moves(
    chess_board: &ChessBoard,
    friendly_color: &Color,
    piece_position: &Position,
) -> Vec<Move> {
    let (direction, attack_direction) = get_pawn_direction_and_attack(friendly_color);

    let mut pawn_moves: Vec<Move> = Vec::with_capacity(4);

    pawn_moves.append(&mut get_moves(
        direction,
        &chess_board.board,
        piece_position,
    ));

    let single_pawn_move_is_legal: bool = if pawn_moves.len() > 0 { true } else { false };

    let travelable_attack_direction: [bool; 2] = [
        attack_direction[0].piece_can_travel(&chess_board.board, friendly_color, piece_position),
        attack_direction[1].piece_can_travel(&chess_board.board, friendly_color, piece_position),
    ];

    if travelable_attack_direction[0] {
        pawn_moves.append(&mut get_attacks(
            attack_direction[0],
            &chess_board.board,
            friendly_color,
            piece_position,
        ));
    }

    if travelable_attack_direction[1] {
        pawn_moves.append(&mut get_attacks(
            attack_direction[1],
            &chess_board.board,
            friendly_color,
            piece_position,
        ));
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

fn get_moves(
    move_direction: MoveDirection,
    board: &Board,
    current_position: &Position,
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::with_capacity(4);

    if move_direction.move_is_within_bounds(*current_position) {
        let new_position = move_direction.walk_from_position(*current_position);

        if board[new_position.1][new_position.0] == None {
            if new_position.1 == 7 || new_position.1 == 0 {
                moves.append(&mut get_promotions(current_position, &new_position, None));
            } else {
                moves.push(Move {
                    start_pos: *current_position,
                    end_pos: new_position,
                    meta_data: MoveMetaData {
                        piece_to_move: PieceType::Pawn,
                        piece_to_capture: None,
                        promotion_piece: None,
                        is_castling_move: false,
                        generates_en_passant: false,
                        is_en_passant_move: false,
                    },
                })
            }
        }
    }
    moves
}

fn get_attacks(
    attack_direction: MoveDirection,
    board: &Board,
    friendly_color: &Color,
    current_position: &Position,
) -> Vec<Move> {
    let mut attack_moves: Vec<Move> = Vec::with_capacity(4);
    if attack_direction.piece_can_travel(board, friendly_color, current_position) {
        let new_position = attack_direction.walk_from_position(*current_position);

        if let Some(piece) = board[new_position.1][new_position.0] {
            if new_position.1 == 7 || new_position.1 == 0 {
                attack_moves.append(&mut get_promotions(
                    current_position,
                    &new_position,
                    Some(piece.piece_type),
                ));
            } else {
                attack_moves.push(Move {
                    start_pos: (0, 0),
                    end_pos: (0, 0),
                    meta_data: MoveMetaData {
                        piece_to_move: PieceType::Pawn,
                        piece_to_capture: Some(piece.piece_type),
                        promotion_piece: None,
                        is_castling_move: false,
                        generates_en_passant: false,
                        is_en_passant_move: false,
                    },
                });
            }
        }
    }

    attack_moves
}

fn get_promotions(
    start_position: &Position,
    end_position: &Position,
    capture_piece: Option<PieceType>,
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::with_capacity(4);

    for piece in PROMOTION_PIECES {
        moves.push(Move {
            start_pos: *start_position,
            end_pos: *end_position,
            meta_data: MoveMetaData {
                piece_to_move: PieceType::Pawn,
                piece_to_capture: capture_piece,
                promotion_piece: Some(piece),
                is_castling_move: false,
                generates_en_passant: false,
                is_en_passant_move: false,
            },
        });
    }
    moves
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

    if pawn_starting_rank == piece_position.1 {
        let new_position = double_move_direction.walk_from_position(*piece_position);

        if board[new_position.1][new_position.0] == None {
            let double_move: Move = Move {
                start_pos: *piece_position,
                end_pos: new_position,
                meta_data: MoveMetaData {
                    piece_to_move: PieceType::Pawn,
                    piece_to_capture: None,
                    promotion_piece: None,
                    is_castling_move: false,
                    generates_en_passant: true,
                    is_en_passant_move: false,
                },
            };
            return Some(double_move);
        }
    }
    None
}
