use super::ChessBoard;

pub const TEST_POS_1_FEN: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
pub const TEST_POS_2_FEN: &str =
    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ";
pub const TEST_POS_3_FEN: &str = "4k2r/6r1/8/8/8/8/3R4/R3K3 w Qk - 0 1";
pub const ROOK_END_GAME_FEN: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
pub const KIWIPETE_FEN_POSITION: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

impl ChessBoard {
    pub fn perft(&self, depth: i64) {
        let mut count: i64 = 0;

        let legal_moves = &self.legal_moves();

        if depth == 1 {
            for chess_move in legal_moves {
                println!("move: {}, count: {}", chess_move.move_to_string(), 1);
                count += 1;
            }
        } else {
            for chess_move in legal_moves {
                let mut board_copy = *self;

                board_copy
                    .make_move(*chess_move)
                    .expect("PERFT ERROR: Error when iterating through legal moves!");

                let perft_count = Self::count_legal_move_leaves(board_copy, depth - 1);
                println!(
                    "move: {}, count: {}",
                    chess_move.move_to_string(),
                    perft_count
                );
                count += perft_count;
            }
        }

        println!("perft({depth}) = {count}");
    }

    fn count_legal_move_leaves(chess_board: Self, depth: i64) -> i64 {
        let mut count: i64 = 0;

        let legal_moves = chess_board.legal_moves();

        if depth > 1 {
            for legal_move in legal_moves {
                let mut new_chessboard = chess_board.clone();

                let _move_result = match new_chessboard.make_move(legal_move) {
                    Ok(_) => {}

                    Err(_error) => {
                        println!("{new_chessboard}");
                        println!("Illegal move: {}", legal_move);
                        return -1;
                    }
                };

                count += Self::count_legal_move_leaves(new_chessboard, depth - 1);
            }
        } else if depth == 1 {
            return chess_board.legal_moves().len() as i64;
        } else if depth < 1 {
            return -1;
        }
        count
    }
}
