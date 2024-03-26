/* Defines different piece types and color */
const PAWN : u8 = 0b0001;
const ROOK : u8 = 0b0010;
const BISHOP : u8 = 0b0011;
const KNIGHT : u8 = 0b0100;
const QUEEN : u8 = 0b0101;
const KING : u8 = 0b0111;
const BLACK : u8 = 0b0000;
const WHITE : u8 = 0b1000;
const EMPTY : u8 = 0b10101;


const FEN_START_POS : &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub struct ChessBoard {
    board: [u8; 64],
}




impl ChessBoard {
    const ROW_SIZE: usize = 8;

    pub fn new() -> ChessBoard {
        return Self{
            board : [EMPTY; 64],
        }
    }
    fn make_white(piece:u8) -> u8 {
        return (piece | WHITE);
    }
    fn make_black(piece:u8) -> u8 {
        return (piece | BLACK);
    }

    pub fn is_valid_fen(fen: &str) -> bool {
        let is_valid : bool = false;


        return is_valid;
    }
    pub fn set_fen_position(fen : &str) {
        if !Self::is_valid_fen(fen) {
            Err(fen).expect("NOT VALID FEN");
        }
    }
}