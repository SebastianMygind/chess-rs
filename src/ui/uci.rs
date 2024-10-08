use crate::chess::{ChessBoard, Color, MetaData, Move, Piece, PieceType};
use std::io;
use std::str::SplitWhitespace;

enum Action {
    Quit,
    MakeMove(Move),
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

                Action::MakeMove(move_to_make) => match chess_board.make_move(move_to_make) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{e}")
                    }
                },

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
            println!("Invalid move string provided  after argument: move");
            Action::Continue
        }

        _ => {
            println!("Invalid Argument");
            Action::Continue
        }
    }
}

fn parse_move_string(move_string: &str) -> Option<Move> {
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
                Some(Move {
                    start_pos: (start_column, start_row),
                    end_pos: (end_column, end_row),
                    meta_data: MetaData::Promotion(Piece::new(Color::White, piece)),
                })
            } else {
                None
            }
        }

        None => Some(Move {
            start_pos: (start_column, start_row),
            end_pos: (end_column, end_row),
            meta_data: MetaData::Move,
        }),
    }
}

fn char_move_file_to_usize(char: char) -> Option<usize> {
    match char {
        'a' => Some(1),
        'b' => Some(2),
        'c' => Some(3),
        'd' => Some(4),
        'e' => Some(5),
        'f' => Some(6),
        'g' => Some(7),
        'h' => Some(8),
        _ => {
            println!("Invalid file, expected a-h, got: {char}");
            None
        }
    }
}

fn char_move_rank_to_usize(char: char) -> Option<usize> {
    match char {
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
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
