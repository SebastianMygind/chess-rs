mod chess;
mod ui;

use iced::{Application, Settings};

use crate::chess::fen::FEN_START_POS;
use crate::chess::{CaptureMove, ChessBoard, Move, MoveInfo, MoveTypes, Pieces, Square};
use crate::ui::game_state::GameState;

fn main() -> iced::Result {
    let mut board = ChessBoard::new();

    let chess_move: Move = Move {
        move_type: MoveTypes::Capture,
        move_specific: MoveInfo {
            capture: {
                CaptureMove {
                    starting_square: Square { rank: 2, file: 5 },
                    target_square: Square { rank: 4, file: 5 },
                }
            },
        },
    };

    match board.set_fen_position_arr(FEN_START_POS) {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}", e)
        }
    }

    match board.make_move(chess_move) {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}", e)
        }
    }

    print!("{}", board);
    GameState::run(Settings::default())
}
