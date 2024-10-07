use crate::chess::ChessBoard;

pub const ROOK_END_GAME_FEN: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
pub const KIWIPETE_FEN_POSITION: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

impl ChessBoard {
    pub fn perft(&self, depth: i64) -> i64 {
        let mut count: i64 = 0;

        let legal_moves = self.legal_moves();

        if depth > 1 {
            for legal_move in legal_moves {
                let mut new_chessboard = self.clone();

                let _move_result = match new_chessboard.make_move(legal_move) {
                    Ok(_) => {}

                    Err(_error) => {
                        println!("{new_chessboard}");
                        println!("Illegal move: {}", legal_move);
                        return -1;
                    }
                };

                count += new_chessboard.perft(depth - 1);
            }
        } else if depth == 1 {
            return self.legal_moves().len() as i64;
        } else if depth < 1 {
            return -1;
        }
        count
    }
    pub fn test_chessboard_perft_print_legal_moves(chess_board: Self, perft_depth: i64) {
        let perft_count = chess_board.perft(perft_depth);

        let legal_moves = chess_board.legal_moves();

        if legal_moves.len() <= 100 {
            for chess_move in legal_moves {
                println!("move: {}", chess_move.move_to_string());
            }
        } else {
            println!("More than 100 moves, will not print!");
        }

        println!("perft({perft_depth}) = {perft_count}");
    }
}
