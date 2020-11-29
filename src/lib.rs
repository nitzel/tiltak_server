//! A library implementing the rules for Tak, including a fairly strong AI.
//!
//! # Examples
//!
//! Generate legal moves for the start position
//!
//! ```
//! use taik::board::Board;
//! use board_game_traits::board::Board as BoardTrait;
//!
//! let board = Board::start_board();
//! let mut moves = vec![];
//! board.generate_moves(&mut moves);
//! assert_eq!(moves.len(), 25);
//! ```
//!
//! Run Monte Carlo Tree Search for the start position
//!
//! ```rust,no_run
//! use taik::board::Board;
//! use taik::mcts;
//!
//! let board = Board::default();
//! let (best_move, score) = mcts(board, 100_000);
//! println!("Played {} with score {}", best_move, score);
//! ```

extern crate arrayvec;
extern crate board_game_traits;
extern crate pgn_traits;

#[cfg(feature = "aws-lambda")]
pub mod aws;
mod bitboard;
pub mod board;
pub mod minmax;
pub mod move_gen;
pub mod search;
mod tests;
#[cfg(feature = "constant-tuning")]
pub mod tune;

pub use search::mcts;

pub mod pgn_parser;
pub mod pgn_writer;
