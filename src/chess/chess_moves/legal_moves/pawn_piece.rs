use crate::chess::chess_moves::legal_moves::generic_piece::{
    get_friendly_and_enemy_pieces, get_single_step_moves,
};
use crate::chess::chess_moves::piece_logic::{
    BLACK_PAWN_ATTACK_DIRECTION, BLACK_PAWN_DIRECTION, WHITE_PAWN_ATTACK_DIRECTION,
    WHITE_PAWN_DIRECTION,
};
use crate::chess::chess_moves::MoveDirection;
use crate::chess::{ChessBoard, MetaData, Move};

pub fn get_pawn_moves(chess_board: &ChessBoard, piece_position: &usize) -> Vec<Move> {
    let (friendly_pieces, _) = get_friendly_and_enemy_pieces(chess_board.white_is_side_to_move);
    let (direction, attack_direction) =
        get_pawn_direction_and_attack(chess_board.white_is_side_to_move);

    let mut pawn_moves: Vec<Move> = get_single_step_moves(
        chess_board,
        piece_position,
        [direction, attack_direction[0], attack_direction[1]].as_slice(),
    );

    if let Some(square) = chess_board.en_passant_target_square {
        let en_passant_direction: [bool; 2] = [
            attack_direction[0].piece_can_travel(
                &chess_board.board,
                &friendly_pieces,
                piece_position,
            ),
            attack_direction[1].piece_can_travel(
                &chess_board.board,
                &friendly_pieces,
                piece_position,
            ),
        ];

        if en_passant_direction[0] {
            let possible_en_passant_move = attack_direction[0].walk_from_position(*piece_position);
            if square == possible_en_passant_move {
                let en_passant_move = Move {
                    start_pos: *piece_position,
                    end_pos: square,
                    meta_data: MetaData::EnPassant,
                };
                pawn_moves.push(en_passant_move);
            }
        }
        if en_passant_direction[1] {
            let possible_en_passant_move = attack_direction[1].walk_from_position(*piece_position);
            if square == possible_en_passant_move {
                let en_passant_move = Move {
                    start_pos: *piece_position,
                    end_pos: square,
                    meta_data: MetaData::EnPassant,
                };
                pawn_moves.push(en_passant_move);
            }
        }
    }

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
