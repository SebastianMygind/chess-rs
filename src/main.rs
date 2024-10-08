mod chess;
mod ui;

use std::env;
use ui::gui::ChessApplication;
use ui::uci::UniversalChessInterface;

/** Given no arguments the application will run*/
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "uci-mode" => {
                UniversalChessInterface::run(args);
            }
            _ => {
                println!("Unknown argument {}", args[1]);
            }
        }
    } else {
        iced::run(
            ChessApplication::title,
            ChessApplication::update,
            ChessApplication::view,
        )
        .expect("Error from iced.");
    }
}
