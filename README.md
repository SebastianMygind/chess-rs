# unified-chess
unified-chess is a project that implements a chess game in the Rust programming language.
This is a hobby project to learn Rust and Chess.

The programs architecture is split up in three subprojects, 
or cargo workspaces.

In the unified-chess-engine you will also find the reason for the name "unified".
That is that this chess engine library, should expose an api for multiple chess engine 
implementations, this should simplify using the engines through a unified trait "ChessEngine".

## Goals
- [ ] Api for connecting and using a chess engine
- [ ] Array-based chess engine
- [ ] Bitboard-based chess engine
- [ ] GUI for chess using the iced rust crate
- [ ] Multiplayer through a server built for chess (See unified-chess-server)

## Dependencies
- [Iced](https://crates.io/crates/iced)
- [Tokio](https://crates.io/crates/tokio)
