/* Defines different piece types and color */

const PAWN: u8 = 0b0001;
const ROOK: u8 = 0b0010;
const BISHOP: u8 = 0b0011;
const KNIGHT: u8 = 0b0100;
const QUEEN: u8 = 0b0101;
const KING: u8 = 0b0111;
const BLACK: u8 = 0b10000;
const WHITE: u8 = 0b1000;
const EMPTY: u8 = 0b0;

const ARR_SIZE: usize = 64;
const ROW_SIZE: usize = 8;
const COL_SIZE: usize = 8;
const NUM_CHAR: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
const VALID_FEN_BOARD: [char; 21] = ['p', 'r', 'b', 'n', 'q', 'k', 'P', 'R', 'B', 'N',
    'Q', 'K', '1', '2', '3', '4', '5', '6', '7', '8', '/'];

const FEN_START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct ChessBoard {
    board: [u8; ARR_SIZE],
}


impl ChessBoard {
    pub fn new() -> ChessBoard {
        return Self {
            board: [EMPTY; 64],
        };
    }
    fn make_white(piece: u8) -> u8 {
        return (piece | WHITE);
    }
    fn make_black(piece: u8) -> u8 {
        return (piece | BLACK);
    }
    pub fn set_fen_position(fen: &str) -> Result<(), &'static str> {
        if !is_valid_fen(fen) {
            return Err("NOT VALID FEN");
        }

        let mut fen_index: i32 = (ARR_SIZE as i32) - (ROW_SIZE as i32);
        let mut col_index: i32 = 0;

        // Additional logic for valid FEN goes here...

        // Return Ok(()) to signify success
        Ok(())
    }
}

pub fn is_valid_fen(fen: &str) -> bool {
    /* Piece count and syntax check */
    let fen_string: Vec<char> = fen.chars().collect();
    let mut fen_index: usize = 0;

    let is_board_valid = fen_check_board_validity(&fen_string, &mut fen_index);

    if !is_board_valid {
        return false;
    }

    if fen.len() <= (fen_index + 1) {
        return false;
    }

    /* Checking for side to move */
    match fen_string[fen_index] {
        'w' => { fen_index += 2; }

        'b' => { fen_index += 2; }

        _ => { return false; }
    }

    if fen.len() <= (fen_index + 1) {
        return false;
    }

    /* Castling syntax check */
    if fen_string[fen_index] == '-' {
        fen_index += 1;
    } else {
        let mut castle_len: usize = 0;
        let next_space = find_space_from_index(&fen_string, fen_index.clone());
        match next_space {
            Some(index) => {
                castle_len = index - fen_index;
            }
            None => { return false; }
        }

        if castle_len < 5 && castle_len > 0 {
            let mut castle_ability: [bool; 4] = [false; 4];
            let mut current: usize = 0;

            let castle_index_end: usize = fen_index + castle_len;
            while fen_index < castle_index_end {
                match fen_string[fen_index] {
                    'K' => {
                        current = 0;
                        if castle_ability[current] == true {
                            return false;
                        }
                        castle_ability[current] = true;
                    }
                    'Q' => {
                        current = 1;
                        if castle_ability[current] == true {
                            return false;
                        }
                        castle_ability[current] = true;
                    }
                    'k' => {
                        current = 2;
                        if castle_ability[current] == true {
                            return false;
                        }
                        castle_ability[current] = true;
                    }
                    'q' => {
                        current = 3;
                        if castle_ability[current] == true {
                            return false;
                        }
                        castle_ability[current] = true;
                    }
                    _ => { return false; }
                }
                fen_index += 1;
            }
        } else {
            return false;
        }
    }

    /* En Peassant check */


    return true;
}

fn fen_check_board_validity(fen: &Vec<char>, index: &mut usize) -> bool {
    let mut squares_on_rows: [u32; ROW_SIZE] = [0; ROW_SIZE];
    let mut current_row: usize = ROW_SIZE - 1;

    for c in fen.iter() {
        if *c == ' ' {
            if (current_row != 0) && !squares_on_rows.iter().all(|&x| x == ROW_SIZE as u32) {
                return false;
            }
            *index += 1;
            break;
        };

        let result = VALID_FEN_BOARD.iter().find(|&&x| x == *c);
        match result {
            Some(&found_char) => {
                if (found_char == '/') && (current_row > 0) &&
                    squares_on_rows[current_row] == 8 {
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
            None => { return false; }
        }
        *index += 1;
    }
    return true;
}

fn find_space_from_index(fen: &Vec<char>, start_index: usize) -> Option<usize> {
    fen.iter().enumerate()
        .skip(start_index)
        .find_map(|(i, c)| if *c == ' ' { Some(i) } else { None })
}