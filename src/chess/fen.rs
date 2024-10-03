use crate::chess::{BoardSquare, Pieces};

use super::ChessBoard;

const ARR_SIZE: usize = 64;
const ROW_SIZE: usize = 8;
const VALID_FEN_BOARD: [char; 21] = [
    'p', 'r', 'b', 'n', 'q', 'k', 'P', 'R', 'B', 'N', 'Q', 'K', '1', '2', '3', '4', '5', '6', '7',
    '8', '/',
];

pub const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/* FEN validation functions */
pub fn is_fen_valid(fen: &str) -> bool {
    let split_fen = split_at_space(fen);

    if split_fen.len() != 6 {
        return false;
    }

    if !(fen_check_board_validity_optimized(split_fen[0].as_str())
        && fen_check_side_to_move(split_fen[1].as_str())
        && fen_check_castling_ability(split_fen[2].as_str())
        && fen_check_en_passant(split_fen[3].as_str())
        && fen_check_halfmove(split_fen[4].as_str())
        && fen_check_fullmove(split_fen[5].as_str(), split_fen[4].as_str()))
    {
        return false;
    }
    true
}

pub fn split_at_space(fen: &str) -> Vec<String> {
    let mut split_fen: Vec<String> = Vec::with_capacity(4 * 90);
    let mut string_buffer: String = String::with_capacity(90);

    for c in fen.chars() {
        if c == ' ' {
            if string_buffer.len() > 0 {
                split_fen.push(string_buffer.clone());
                string_buffer.clear()
            } else {
                continue;
            }
        } else {
            string_buffer.push(c);
        }
    }

    if string_buffer.len() > 0 {
        split_fen.push(string_buffer);
    }

    split_fen
}

fn fen_check_board_validity_optimized(fen: &str) -> bool {
    let mut squares_on_rows: [u32; ROW_SIZE] = [0; ROW_SIZE];
    let mut current_row: usize = ROW_SIZE - 1;

    for c in fen.chars() {
        if c == ' ' {
            if (current_row != 0) && !squares_on_rows.iter().all(|&x| x == ROW_SIZE as u32) {
                return false;
            }
            break;
        };

        let result = VALID_FEN_BOARD.iter().find(|&&x| x == c);
        match result {
            Some(&found_char) => {
                if (found_char == '/') && (current_row > 0) && squares_on_rows[current_row] == 8 {
                    current_row -= 1;
                } else {
                    let squares_to_move: u32 = found_char.to_digit(10).unwrap_or(1);

                    if squares_on_rows[current_row] + squares_to_move <= ROW_SIZE as u32 {
                        squares_on_rows[current_row] += squares_to_move;
                    } else {
                        return false;
                    }
                }
            }
            None => {
                return false;
            }
        }
    }
    true
}

fn fen_check_side_to_move(side_to_move: &str) -> bool {
    if side_to_move.len() != 1 {
        return false;
    }

    let side_char = side_to_move.chars().next();

    match side_char {
        Some(c) => c == 'w' || c == 'b',
        None => false,
    }
}

fn fen_check_castling_ability(castling_string: &str) -> bool {
    let mut castling_ability: [bool; 4] = [false; 4];
    let mut current: usize;

    if fen_check_hyphen(castling_string) {
        return true;
    }

    for c in castling_string.chars() {
        match c {
            'K' => {
                current = 0;
                if castling_ability[current] == true {
                    return false;
                }
                castling_ability[current] = true;
            }
            'Q' => {
                current = 1;
                if castling_ability[current] == true {
                    return false;
                }
                castling_ability[current] = true;
            }
            'k' => {
                current = 2;
                if castling_ability[current] == true {
                    return false;
                }
                castling_ability[current] = true;
            }
            'q' => {
                current = 3;
                if castling_ability[current] == true {
                    return false;
                }
                castling_ability[current] = true;
            }
            _ => {
                return false;
            }
        }
    }
    true
}

fn fen_check_en_passant(en_passant: &str) -> bool {
    if fen_check_hyphen(en_passant) {
        return true;
    }

    if !en_passant.len() == 2 {
        return false;
    } else {
        let mut square_iter = en_passant.chars();
        let file_letter = square_iter.next();

        match file_letter {
            Some(c) => {
                if c == 'a'
                    || c == 'b'
                    || c == 'c'
                    || c == 'd'
                    || c == 'e'
                    || c == 'f'
                    || c == 'g'
                    || c == 'h'
                {
                    let eprank = square_iter.next();

                    match eprank {
                        Some(n) => {
                            let number = n.to_digit(10).unwrap_or(1);
                            if number != 3 || number != 6 {
                                return false;
                            }
                        }
                        None => return false,
                    }
                } else {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

fn fen_check_halfmove(halfmove: &str) -> bool {
    if halfmove.len() > 2 {
        return false;
    }

    let parsed_halfmove = halfmove.parse::<u32>();

    match parsed_halfmove {
        Ok(number) => {
            if number > 50 {
                return false;
            }
        }
        Err(_e) => return false,
    }
    true
}

fn fen_check_fullmove(fullmove: &str, halfmove: &str) -> bool {
    let halfmove_result = halfmove.parse::<u32>();
    let halfmove_parsed: u32 = match halfmove_result {
        Ok(number) => number,
        Err(_e) => return false,
    };

    let fullmove_result = fullmove.parse::<u32>();
    let fullmove_parsed: u32 = match fullmove_result {
        Ok(number) => number,
        Err(_e) => return false,
    };

    {
        if fullmove_parsed > halfmove_parsed {
            true
        } else {
            false
        }
    }
}

fn fen_check_hyphen(fen_slice: &str) -> bool {
    if fen_slice.len() == 1 {
        if let Some(letter) = fen_slice.chars().next() {
            return letter == '-';
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_fen1() {
        assert_eq!(is_fen_valid(FEN_START_POSITION), true);
    }

    #[test]
    fn test_bad_fen1() {
        assert_eq!(
            is_fen_valid("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR z KQkq - 0 1"),
            false
        );
    }

    #[test]
    fn test_bad_fen2() {
        assert_eq!(
            is_fen_valid("rnbqkbnr/pppppppp/8/8/8/8/PPP3PPPP/RNBQKBNR w KQkq - 0 1"),
            false
        );
    }
}

/* FEN parsing functions */

pub fn parse_fen_piece_placement(fen_string: &str) ->  {
    let fen_ranks: Vec<&str> = fen_string.split('/').collect();

    let mut parsed_ranks: Vec<Vec<Pieces>> = Vec::new();

    for fen_rank in fen_ranks {
        parsed_ranks.push(parse_fen_piece_rank(fen_rank));
    }
    /* Make rank 1 the 0th element instead of the 7th */
    parsed_ranks.reverse();

    let mut board = [BoardSquare {
        piece_type: Pieces::Empty,
    }; ARR_SIZE];

    let mut index = 0;

    for rank in parsed_ranks {
        for element in rank {
            board[index].piece_type = element;
            index += 1;
        }
    }

    if index != 64 {
        panic!("ERROR: not 64 elements in parse_fen_piece_placement!")
    }

    board
}

fn parse_fen_piece_rank(rank_string: &str) -> Vec<Pieces> {
    let mut parsed_rank: Vec<Pieces> = Vec::new();

    for c in rank_string.chars() {
        let char_to_num = c.to_digit(10);
        match char_to_num {
            Some(mut num) => {
                while num > 0 {
                    parsed_rank.push(Pieces::Empty);
                    num -= 1;
                }
            }

            None => parsed_rank.push(parse_fen_piece(c)),
        }
    }
    if parsed_rank.len() > 8 {
        panic!("ERROR: parse_fen_piece_rank returns more than 8 pieces")
    }

    return parsed_rank;
}

fn parse_fen_piece(c: char) -> Pieces {
    match c {
        'K' => Pieces::WKing,
        'k' => Pieces::BKing,

        'Q' => Pieces::WQueen,
        'q' => Pieces::BQueen,

        'R' => Pieces::WRook,
        'r' => Pieces::BRook,

        'B' => Pieces::WBishop,
        'b' => Pieces::BBishop,

        'N' => Pieces::WKnight,
        'n' => Pieces::BKnight,

        'P' => Pieces::WPawn,
        'p' => Pieces::BPawn,

        _ => {
            panic!("Not valid FEN: err in parse_fen_piece")
        }
    }
}

pub fn parse_fen_side_to_move(fen: &str) -> bool {
    if fen.len() != 1 {
        panic!("ERROR: parse_fen_side_to_move");
    }

    let char = fen.chars().next();

    match char {
        Some(c) => match c {
            'w' => true,
            'b' => false,
            _ => {
                panic!("ERROR: side to move char not allowed!")
            }
        },
        None => {
            panic!("ERROR: Could not parse fen: side_to_move char")
        }
    }
}

/* Parses FEN castling ability. Returns an array of booleans where KQkq is K:1 Q:1 k:1 q:1*/
pub fn parse_fen_castling_ability(fen: &str) -> [bool; 4] {
    let mut c_ability: [bool; 4] = [false; 4];

    for c in fen.chars() {
        match c {
            'K' => {
                c_ability[0] = true;
            }
            'Q' => {
                c_ability[1] = true;
            }
            'k' => {
                c_ability[2] = true;
            }
            'q' => {
                c_ability[3] = true;
            }
            '-' => {
                if fen.len() == 1 {
                    return [false; 4];
                } else {
                    panic!("ERROR: '-' where more than one character is given")
                }
            }
            _ => {
                panic!("ERROR: unknown castle")
            }
        }
    }
    return c_ability;
}

pub fn parse_fen_epawn(fen: &str) -> Option<usize> {
    let mut fen_iter = fen.chars();

    if fen == "-" {
        return None;
    }

    let parsed_epawn: usize;

    let epawn_file: usize;
    let epawn_rank: usize;

    let file = fen_iter.next().unwrap();

    match file {
        'a' => epawn_file = 1,
        'b' => epawn_file = 2,
        'c' => epawn_file = 3,
        'd' => epawn_file = 4,
        'e' => epawn_file = 5,
        'f' => epawn_file = 6,
        'g' => epawn_file = 7,
        'h' => epawn_file = 8,
        _ => {
            panic!("unkown file character: parse_fen_epawn")
        }
    }

    let rank_char = fen_iter.next().unwrap();

    let rank: usize = match rank_char.to_digit(10) {
        Some(num) => num as usize,
        None => {
            panic!("could not parse rank number: parse_fen_epawn")
        }
    };

    if 0 < rank && rank <= 8 {
        epawn_rank = rank;
    } else {
        panic!("rank not valid, must have value between 1 and 8");
    }

    parsed_epawn = (epawn_rank - 1) * 8 + epawn_file - 1;

    Some(parsed_epawn)
}

pub fn parse_fen_half_move_clock(fen: &str) -> u64 {
    fen.parse::<u64>().unwrap()
}

pub fn parse_fen_full_move_counter(fen: &str) -> u64 {
    fen.parse::<u64>().unwrap()
}
