use crate::chess::{ChessBoard, Move, PieceType};
use std::io;
use std::str::SplitWhitespace;

use super::UserMove;

enum Action {
    Quit,
    MakeMove(UserMove),
    RunPerft(i64),
    Continue,
    PrintBoard,
}

pub struct UniversalChessInterface {}

impl UniversalChessInterface {
    pub fn run(_args: Vec<String>) {
        let mut chess_board: ChessBoard = ChessBoard::new();

        println!("{INTRO_STRING}");

        loop {
            let mut input = String::new();

            match io::stdin().read_line(&mut input) {
                Ok(_) => {}
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            }
            let args = input.split_whitespace();

            let action = handle_args(args);

            match action {
                Action::Quit => break,

                Action::MakeMove(parsed_move) => {
                    let mut legal_move: Option<Move> = None;
                    for chess_move in chess_board.legal_moves() {
                        if parsed_move.promotion_piece.is_some() {
                            if chess_move.start_pos == parsed_move.start_position
                                && chess_move.end_pos == parsed_move.end_position
                            {
                                if parsed_move.promotion_piece
                                    == chess_move.meta_data.promotion_piece
                                {
                                    legal_move = Some(chess_move);
                                    break;
                                }
                            }
                        } else {
                            if chess_move.start_pos == parsed_move.start_position
                                && chess_move.end_pos == parsed_move.end_position
                            {
                                legal_move = Some(chess_move);
                                break;
                            }
                        }
                    }
                    if let Some(move_to_make) = legal_move {
                        match chess_board.make_move(move_to_make) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("{e}")
                            }
                        }
                    } else {
                    }
                }

                Action::Continue => {}

                Action::RunPerft(depth) => {
                    chess_board.perft(depth);
                }

                Action::PrintBoard => {
                    println!("{chess_board}");
                }
            }
        }
    }
}

fn handle_args(mut args: SplitWhitespace) -> Action {
    let argument = match args.next() {
        Some(arg) => arg,
        None => return Action::Continue,
    };

    match argument {
        "quit" | "exit" => Action::Quit,

        "print" => Action::PrintBoard,

        "perft" => {
            if let Some(depth) = args.next() {
                if let Ok(num) = depth.parse::<i64>() {
                    return Action::RunPerft(num);
                }
            }
            println!("You must provide a depth from 1 and up!");
            Action::Continue
        }

        "move" => {
            if let Some(move_string) = args.next() {
                if let Some(move_to_make) = parse_move_string(move_string) {
                    return Action::MakeMove(move_to_make);
                }
            }
            println!("Invalid move string provided after argument: move");
            Action::Continue
        }

        _ => {
            println!("Invalid Argument");
            Action::Continue
        }
    }
}

fn parse_move_string(move_string: &str) -> Option<UserMove> {
    let mut move_chars = move_string.chars();

    let start_column: usize = if let Some(char) = move_chars.next() {
        match char_move_file_to_usize(char) {
            Some(num) => num,
            None => return None,
        }
    } else {
        return None;
    };

    let start_row: usize = if let Some(char) = move_chars.next() {
        match char_move_rank_to_usize(char) {
            Some(num) => num,
            None => return None,
        }
    } else {
        return None;
    };

    let end_column: usize = if let Some(char) = move_chars.next() {
        match char_move_file_to_usize(char) {
            Some(num) => num,
            None => return None,
        }
    } else {
        return None;
    };

    let end_row: usize = if let Some(char) = move_chars.next() {
        match char_move_rank_to_usize(char) {
            Some(num) => num,
            None => return None,
        }
    } else {
        return None;
    };

    match move_chars.next() {
        Some(promotion_char) => {
            if let Some(piece) = char_move_promotion_to_piece(promotion_char) {
                Some(UserMove {
                    start_position: (start_column, start_row),
                    end_position: (end_column, end_row),
                    promotion_piece: Some(piece),
                })
            } else {
                None
            }
        }

        None => Some(UserMove {
            start_position: (start_column, start_row),
            end_position: (end_column, end_row),
            promotion_piece: None,
        }),
    }
}

fn char_move_file_to_usize(char: char) -> Option<usize> {
    match char {
        'a' => Some(0),
        'b' => Some(1),
        'c' => Some(2),
        'd' => Some(3),
        'e' => Some(4),
        'f' => Some(5),
        'g' => Some(6),
        'h' => Some(7),
        _ => {
            println!("Invalid file, expected a-h, got: {char}");
            None
        }
    }
}

fn char_move_rank_to_usize(char: char) -> Option<usize> {
    match char {
        '1' => Some(0),
        '2' => Some(1),
        '3' => Some(2),
        '4' => Some(3),
        '5' => Some(4),
        '6' => Some(5),
        '7' => Some(6),
        '8' => Some(7),
        _ => None,
    }
}

fn char_move_promotion_to_piece(char: char) -> Option<PieceType> {
    match char {
        'q' => Some(PieceType::Queen),
        'n' => Some(PieceType::Knight),
        'r' => Some(PieceType::Rook),
        'b' => Some(PieceType::Bishop),
        _ => None,
    }
}

const INTRO_STRING: &str = "\n\nWelcome to Chess-rs CLI tool!!!\n\n";
