//! Connect Four
//!
//! Check struct [`ConnectFour`](https://docs.rs/gamie/*/gamie/connect_four/struct.ConnectFour.html) for more information
//!
//! # Examples
//!
//! ```rust
//! # fn connect_four() {
//! use gamie::connect_four::{ConnectFour, Player as ConnectFourPlayer};
//!
//! let mut game = ConnectFour::new().unwrap();
//! game.put(ConnectFourPlayer::Player0, 3).unwrap();
//! game.put(ConnectFourPlayer::Player1, 2).unwrap();
//! // ...
//! # }
//! ```

use crate::std_lib::{iter, Box, Index, IndexMut, Infallible};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use snafu::Snafu;

/// Connect Four
///
/// Passing an invalid position to a method will cause panic. Check the target position validity first when dealing with user input
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConnectFour {
    board: [Column; 7],
    next: Player,
    status: GameState,
}

/// The column of the game board.
///
/// This is a vector-like struct. Inner elements can be accessed by using index
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct Column {
    column: [Option<Player>; 6],
    occupied: usize,
}

impl Column {
    fn is_full(&self) -> bool {
        self.occupied == 6
    }

    fn push(&mut self, player: Player) {
        self.column[self.occupied] = Some(player);
        self.occupied += 1;
    }
}

impl Default for Column {
    fn default() -> Self {
        Self {
            column: [None; 6],
            occupied: 0,
        }
    }
}

impl Index<usize> for Column {
    type Output = Option<Player>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.column[index]
    }
}

impl IndexMut<usize> for Column {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.column[index]
    }
}

/// Players
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Player {
    Player0,
    Player1,
}

impl Player {
    /// Get the opposite player
    pub fn other(self) -> Self {
        match self {
            Player::Player0 => Player::Player1,
            Player::Player1 => Player::Player0,
        }
    }
}

/// Game status
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GameState {
    Win(Player),
    Tie,
    InProgress,
}

impl ConnectFour {
    /// Create a new Connect Four game
    pub fn new() -> Result<Self, Infallible> {
        Ok(Self {
            board: Default::default(),
            next: Player::Player0,
            status: GameState::InProgress,
        })
    }

    /// Get a cell reference from the game board
    /// Panic when target position out of bounds
    pub fn get(&self, row: usize, col: usize) -> &Option<Player> {
        &self.board[5 - row][col]
    }

    /// Check if the game was end
    pub fn is_ended(&self) -> bool {
        self.status != GameState::InProgress
    }

    /// Get the winner of the game. Return `None` when the game is tied or not end yet
    pub fn winner(&self) -> Option<Player> {
        if let GameState::Win(player) = self.status {
            Some(player)
        } else {
            None
        }
    }

    /// Get the game status
    pub fn status(&self) -> &GameState {
        &self.status
    }

    /// Get the next player
    pub fn get_next_player(&self) -> Player {
        self.next
    }

    /// Put a piece into the game board
    /// Panic when target position out of bounds
    pub fn put(&mut self, player: Player, col: usize) -> Result<(), ConnectFourError> {
        if self.is_ended() {
            return Err(ConnectFourError::GameEnded);
        }

        if player != self.next {
            return Err(ConnectFourError::WrongPlayer);
        }

        if self.board[col].is_full() {
            return Err(ConnectFourError::ColumnFilled);
        }

        self.board[col].push(player);
        self.next = self.next.other();

        self.check_state();

        Ok(())
    }

    fn check_state(&mut self) {
        for connectable in Self::get_connectable() {
            let mut last = None;
            let mut count = 0u8;

            for cell in connectable.map(|(row, col)| self.board[col][row]) {
                if cell != last {
                    last = cell;
                    count = 1;
                } else {
                    count += 1;
                    if count == 4 && cell.is_some() {
                        self.status = GameState::Win(cell.unwrap());
                        return;
                    }
                }
            }
        }

        if (0..7).all(|col| self.board[col][5].is_some()) {
            self.status = GameState::Tie;
        }
    }

    fn get_connectable() -> impl Iterator<Item = Box<dyn Iterator<Item = (usize, usize)>>> {
        let horizontal = (0usize..6).map(move |row| {
            Box::new((0usize..7).map(move |col| (row, col)))
                as Box<dyn Iterator<Item = (usize, usize)>>
        });

        let vertical = (0usize..7).map(move |col| {
            Box::new((0usize..6).map(move |row| (row, col)))
                as Box<dyn Iterator<Item = (usize, usize)>>
        });

        let horizontal_upper_left_to_lower_right = (0usize..7).map(move |col| {
            Box::new(
                iter::successors(Some((0usize, col)), |(row, col)| Some((row + 1, col + 1)))
                    .take((7 - col).min(6)),
            ) as Box<dyn Iterator<Item = (usize, usize)>>
        });

        let vertical_upper_left_to_lower_right = (0usize..6).map(move |row| {
            Box::new(
                iter::successors(Some((row, 0usize)), |(row, col)| Some((row + 1, col + 1)))
                    .take(6 - row),
            ) as Box<dyn Iterator<Item = (usize, usize)>>
        });

        let horizontal_upper_right_to_lower_left = (0usize..7).map(move |col| {
            Box::new(
                iter::successors(Some((0usize, col)), |(row, col)| {
                    col.checked_sub(1).map(|new_col| (row + 1, new_col))
                })
                .take((1 + col).min(6)),
            ) as Box<dyn Iterator<Item = (usize, usize)>>
        });

        let vertical_upper_right_to_lower_left = (0usize..6).map(move |row| {
            Box::new(
                iter::successors(Some((row, 6usize)), |(row, col)| Some((row + 1, col - 1)))
                    .take(6 - row),
            ) as Box<dyn Iterator<Item = (usize, usize)>>
        });

        horizontal
            .chain(vertical)
            .chain(horizontal_upper_left_to_lower_right)
            .chain(vertical_upper_left_to_lower_right)
            .chain(horizontal_upper_right_to_lower_left)
            .chain(vertical_upper_right_to_lower_left)
    }
}

/// Errors that can occur when putting a piece into the board
#[derive(Debug, Eq, PartialEq, Snafu)]
pub enum ConnectFourError {
    #[snafu(display("Wrong player"))]
    WrongPlayer,
    #[snafu(display("Filled Column"))]
    ColumnFilled,
    #[snafu(display("The game was already end"))]
    GameEnded,
}

#[cfg(test)]
mod tests {
    use crate::connect_four::*;
    use ntest::timeout;
    #[test]
    #[timeout(3000)]
    #[no_coverage]
    fn test() {
        let mut game = ConnectFour::new().unwrap();
        game.put(Player::Player0, 3).unwrap();
        game.put(Player::Player1, 2).unwrap();
        game.put(Player::Player0, 2).unwrap();
        game.put(Player::Player1, 1).unwrap();
        game.put(Player::Player0, 1).unwrap();
        game.put(Player::Player1, 0).unwrap();
        game.put(Player::Player0, 3).unwrap();
        game.put(Player::Player1, 0).unwrap();
        game.put(Player::Player0, 1).unwrap();
        game.put(Player::Player1, 6).unwrap();
        game.put(Player::Player0, 2).unwrap();
        game.put(Player::Player1, 6).unwrap();
        game.put(Player::Player0, 3).unwrap();
        game.put(Player::Player1, 5).unwrap();
        game.put(Player::Player0, 0).unwrap();
        assert_eq!(Some(Player::Player0), game.winner());
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::ops::IndexMut;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use snafu::ErrorCompat;
	use std::ops::Index;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4330() {
    rusty_monitor::set_test_id(4330);
    let mut usize_0: usize = 36usize;
    let mut usize_1: usize = 57usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_0);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut option_2: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Win(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_12: crate::connect_four::Column = std::default::Default::default();
    let mut column_13: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_1: [crate::connect_four::Column; 7] = [column_13, column_12, column_11, column_10, column_9, column_8, column_7];
    let mut connectfour_1: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_1, next: player_6, status: gamestate_1};
    let mut connectfour_1_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_1;
    let mut option_5: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_1_ref_0);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_14: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_1};
    let mut column_14_ref_0: &mut crate::connect_four::Column = &mut column_14;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_1_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_7);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_9);
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut bool_0: bool = false;
    let mut usize_2: usize = 52usize;
    let mut usize_3: usize = 72usize;
    let mut usize_4: usize = 38usize;
    let mut usize_5: usize = 17usize;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::other(player_14);
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_3, option_array_2, option_array_1];
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut reversi_0: crate::reversi::Reversi = std::result::Result::unwrap(result_0);
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut option_15: std::option::Option<reversi::Player> = crate::reversi::Reversi::winner(reversi_0_ref_0);
    let mut connectfourerror_2: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_3: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_2_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_2;
    let mut bool_1: bool = std::cmp::PartialEq::eq(connectfourerror_2_ref_0, connectfourerror_1_ref_0);
    let mut player_10_ref_0: &tictactoe::Player = &mut player_10;
    let mut player_16: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_16: &mut std::option::Option<connect_four::Player> = std::ops::IndexMut::index_mut(column_14_ref_0, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3188() {
    rusty_monitor::set_test_id(3188);
    let mut usize_0: usize = 70usize;
    let mut usize_1: usize = 44usize;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_2: usize = 31usize;
    let mut bool_2: bool = false;
    let mut usize_3: usize = 79usize;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_0_ref_0: &crate::connect_four::Column = &mut column_0;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_0_ref_0: &connect_four::Player = &mut player_0;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut usize_4: usize = 18usize;
    let mut bool_5: bool = false;
    let mut usize_5: usize = 64usize;
    let mut usize_6: usize = 39usize;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_1_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_1;
    let mut bool_6: bool = std::cmp::PartialEq::eq(connectfourerror_1_ref_0, connectfourerror_0_ref_0);
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_5, mine_adjacent: usize_4, is_revealed: bool_4, is_flagged: bool_3};
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_0_ref_0);
    let mut result_1: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut option_0: &std::option::Option<connect_four::Player> = std::ops::Index::index(column_0_ref_0, usize_3);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    crate::connect_four::ConnectFour::get_connectable();
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_2, is_revealed: bool_1, is_flagged: bool_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4058() {
    rusty_monitor::set_test_id(4058);
    let mut usize_0: usize = 49usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut usize_1: usize = 94usize;
    let mut usize_2: usize = 25usize;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut player_0_ref_0: &connect_four::Player = &mut player_0;
    let mut player_11: connect_four::Player = std::clone::Clone::clone(player_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_967() {
    rusty_monitor::set_test_id(967);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 30usize;
    let mut bool_2: bool = false;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut usize_1: usize = 56usize;
    let mut usize_2: usize = 47usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut option_9: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1460() {
    rusty_monitor::set_test_id(1460);
    let mut bool_0: bool = false;
    let mut usize_0: usize = 47usize;
    let mut usize_1: usize = 13usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_0_ref_0: &connect_four::Player = &mut player_0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut usize_2: usize = 34usize;
    let mut usize_3: usize = 19usize;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_0_ref_0: &mut crate::connect_four::Column = &mut column_0;
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_1_ref_0: &crate::connect_four::Column = &mut column_1;
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_2_ref_0: &crate::connect_four::Column = &mut column_2;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_1_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_1;
    let mut usize_4: usize = 5usize;
    let mut usize_5: usize = 75usize;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3_ref_0: &connect_four::Player = &mut player_3;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_6: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_6_ref_0: &gomoku::Player = &mut player_6;
    let mut player_7: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_7_ref_0: &gomoku::Player = &mut player_7;
    let mut player_8: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut player_9: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut player_9_ref_0: &connect_four::Player = &mut player_9;
    let mut bool_1: bool = std::cmp::PartialEq::eq(player_9_ref_0, player_3_ref_0);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut player_8_ref_0: &connect_four::Player = &mut player_8;
    let mut player_10: connect_four::Player = std::clone::Clone::clone(player_8_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::ne(column_2_ref_0, column_1_ref_0);
    let mut player_11: gomoku::Player = crate::gomoku::Player::Player0;
    crate::connect_four::Column::push(column_0_ref_0, player_2);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_0_ref_0);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2732() {
    rusty_monitor::set_test_id(2732);
    let mut usize_0: usize = 76usize;
    let mut usize_1: usize = 59usize;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_0_ref_0: &crate::connect_four::Column = &mut column_0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_1_ref_0: &crate::connect_four::Column = &mut column_1;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_1_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_1;
    let mut connectfourerror_2: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_2_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_2;
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_2_ref_0: &crate::connect_four::Column = &mut column_2;
    let mut usize_2: usize = 75usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_0);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_3: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_2};
    let mut column_3_ref_0: &crate::connect_four::Column = &mut column_3;
    let mut bool_0: bool = std::cmp::PartialEq::ne(column_3_ref_0, column_2_ref_0);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut gamestate_1: connect_four::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_2_ref_0);
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut gamestate_2: connect_four::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_0_ref_0);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut tuple_2: () = std::cmp::Eq::assert_receiver_is_total_eq(column_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3473() {
    rusty_monitor::set_test_id(3473);
    let mut usize_0: usize = 0usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut usize_1: usize = 97usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_1};
    let mut usize_2: usize = 92usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_7: connect_four::Player = crate::connect_four::Player::other(player_6);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_1: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_2};
    let mut usize_3: usize = 44usize;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_2: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_3};
    let mut usize_4: usize = 15usize;
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_11: connect_four::Player = crate::connect_four::Player::other(player_10);
    let mut option_18: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut option_19: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_21: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_22: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_23: std::option::Option<connect_four::Player> = std::option::Option::Some(player_13);
    let mut option_array_3: [std::option::Option<connect_four::Player>; 6] = [option_23, option_22, option_21, option_20, option_19, option_18];
    let mut column_3: crate::connect_four::Column = crate::connect_four::Column {column: option_array_3, occupied: usize_4};
    let mut usize_5: usize = 87usize;
    let mut player_14: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_24: std::option::Option<connect_four::Player> = std::option::Option::Some(player_14);
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_25: std::option::Option<connect_four::Player> = std::option::Option::Some(player_15);
    let mut option_26: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_16: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_17: connect_four::Player = crate::connect_four::Player::other(player_16);
    let mut option_27: std::option::Option<connect_four::Player> = std::option::Option::Some(player_17);
    let mut option_28: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_18: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_29: std::option::Option<connect_four::Player> = std::option::Option::Some(player_18);
    let mut option_array_4: [std::option::Option<connect_four::Player>; 6] = [option_29, option_28, option_27, option_26, option_25, option_24];
    let mut column_4: crate::connect_four::Column = crate::connect_four::Column {column: option_array_4, occupied: usize_5};
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut player_19: gomoku::Player = crate::gomoku::Player::Player0;
    let mut result_0: std::result::Result<(), connect_four::ConnectFourError> = crate::connect_four::ConnectFour::put(connectfour_0_ref_0, player_0, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5112() {
    rusty_monitor::set_test_id(5112);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut usize_0: usize = 46usize;
    let mut usize_1: usize = 80usize;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_3: gomoku::Player = crate::gomoku::Player::other(player_2);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_2: usize = 15usize;
    let mut bool_2: bool = true;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Win(player_4);
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut usize_3: usize = 39usize;
    let mut usize_4: usize = 9usize;
    let mut usize_5: usize = 35usize;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_2_ref_0: &connect_four::GameState = &mut gamestate_2;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut gomokuerror_1_ref_0: &gomoku::GomokuError = &mut gomokuerror_1;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_6: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut bool_4: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_2, is_revealed: bool_1, is_flagged: bool_0};
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2593() {
    rusty_monitor::set_test_id(2593);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut player_5_ref_0: &connect_four::Player = &mut player_5;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_7: connect_four::Player = crate::connect_four::Player::other(player_6);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_7);
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut usize_0: usize = 79usize;
    let mut usize_1: usize = 79usize;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_1_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_1;
    let mut connectfourerror_2: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_2_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_2;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_2_ref_0);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_1_ref_0);
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut option_1: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_0_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut player_10_ref_0: &tictactoe::Player = &mut player_10;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut player_12: connect_four::Player = std::clone::Clone::clone(player_5_ref_0);
    let mut player_13: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut player_14: reversi::Player = crate::reversi::Player::other(player_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1776() {
    rusty_monitor::set_test_id(1776);
    let mut usize_0: usize = 26usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut player_7_ref_0: &tictactoe::Player = &mut player_7;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_8_ref_0: &tictactoe::Player = &mut player_8;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_10: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut player_0_ref_0: &connect_four::Player = &mut player_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3498() {
    rusty_monitor::set_test_id(3498);
    let mut usize_0: usize = 0usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_3, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut option_2: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_12: crate::connect_four::Column = std::default::Default::default();
    let mut column_13: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_1: [crate::connect_four::Column; 7] = [column_13, column_12, column_11, column_10, column_9, column_8, column_7];
    let mut connectfour_1: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_1, next: player_6, status: gamestate_1};
    let mut connectfour_1_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_1;
    let mut option_5: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_1_ref_0);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_14: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_14_ref_0: &crate::connect_four::Column = &mut column_14;
    let mut bool_0: bool = crate::connect_four::Column::is_full(column_14_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4414() {
    rusty_monitor::set_test_id(4414);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut usize_0: usize = 46usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_3: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut usize_1: usize = 58usize;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_7: connect_four::Player = crate::connect_four::Player::other(player_6);
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_4: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut player_10: gomoku::Player = crate::gomoku::Player::Player0;
    crate::connect_four::ConnectFour::check_state(connectfour_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3302() {
    rusty_monitor::set_test_id(3302);
    let mut usize_0: usize = 74usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_0);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_0_ref_0: &crate::connect_four::Column = &mut column_0;
    let mut usize_1: usize = 8usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_1: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_1_ref_0: &crate::connect_four::Column = &mut column_1;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_4);
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut gamestate_1: connect_four::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_0_ref_0);
    let mut bool_0: bool = std::cmp::PartialEq::eq(column_1_ref_0, column_0_ref_0);
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_1_ref_0);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4150() {
    rusty_monitor::set_test_id(4150);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut player_1_ref_0: &connect_four::Player = &mut player_1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut usize_0: usize = 69usize;
    let mut usize_1: usize = 0usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_2: usize = 86usize;
    let mut usize_3: usize = 57usize;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut usize_4: usize = 9usize;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_5: usize = 12usize;
    let mut usize_6: usize = 70usize;
    let mut usize_7: usize = 34usize;
    let mut usize_8: usize = 32usize;
    let mut usize_9: usize = 59usize;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut minesweepererror_3: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_3_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_3;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_5);
    let mut player_2_ref_0: &connect_four::Player = &mut player_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(player_2_ref_0, player_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1429() {
    rusty_monitor::set_test_id(1429);
    let mut usize_0: usize = 80usize;
    let mut usize_1: usize = 29usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut usize_2: usize = 60usize;
    let mut usize_3: usize = 88usize;
    let mut usize_4: usize = 26usize;
    let mut usize_5: usize = 39usize;
    let mut usize_6: usize = 12usize;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut usize_7: usize = 38usize;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_7};
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut usize_8: usize = 63usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_4: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_8};
    let mut usize_9: usize = 63usize;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_8: connect_four::Player = crate::connect_four::Player::other(player_7);
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_10: connect_four::Player = crate::connect_four::Player::other(player_9);
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_5: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_9};
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_4, status: gamestate_0};
    let mut connectfour_0_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut result_0: std::result::Result<(), connect_four::ConnectFourError> = crate::connect_four::ConnectFour::put(connectfour_0_ref_0, player_2, usize_6);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2095() {
    rusty_monitor::set_test_id(2095);
    let mut usize_0: usize = 21usize;
    let mut usize_1: usize = 25usize;
    let mut usize_2: usize = 4usize;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_0_ref_0: &mut crate::connect_four::Column = &mut column_0;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut player_1_ref_0: &connect_four::Player = &mut player_1;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player1;
    let mut usize_3: usize = 82usize;
    let mut usize_4: usize = 16usize;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut usize_5: usize = 38usize;
    let mut usize_6: usize = 28usize;
    let mut u64_0: u64 = 10u64;
    let mut u64_1: u64 = 96u64;
    let mut steprng_0: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_7: usize = 91usize;
    let mut usize_8: usize = 89usize;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::Win(player_2);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_0_ref_0);
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut option_0: &mut std::option::Option<connect_four::Player> = std::ops::IndexMut::index_mut(column_0_ref_0, usize_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1195() {
    rusty_monitor::set_test_id(1195);
    let mut usize_0: usize = 66usize;
    let mut usize_1: usize = 56usize;
    let mut usize_2: usize = 64usize;
    let mut usize_3: usize = 89usize;
    let mut usize_4: usize = 70usize;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut option_0: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_7: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_4};
    let mut column_7_ref_0: &crate::connect_four::Column = &mut column_7;
    let mut option_6: &std::option::Option<connect_four::Player> = std::ops::Index::index(column_7_ref_0, usize_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1229() {
    rusty_monitor::set_test_id(1229);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut usize_0: usize = 31usize;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_6, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut option_4: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_7: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_7_ref_0: &crate::connect_four::Column = &mut column_7;
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_8_ref_0: &crate::connect_four::Column = &mut column_8;
    let mut bool_0: bool = std::cmp::PartialEq::ne(column_8_ref_0, column_7_ref_0);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2377() {
    rusty_monitor::set_test_id(2377);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut usize_0: usize = 64usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut usize_1: usize = 14usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_1: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut usize_2: usize = 54usize;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_7: connect_four::Player = crate::connect_four::Player::other(player_6);
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_6: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_2};
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_0, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut connectfour_1: crate::connect_four::ConnectFour = std::clone::Clone::clone(connectfour_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2257() {
    rusty_monitor::set_test_id(2257);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_1_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_1;
    let mut usize_0: usize = 79usize;
    let mut usize_1: usize = 53usize;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut usize_2: usize = 53usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_6: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_2};
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_0, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut option_6: &std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::get(connectfour_0_ref_0, usize_1, usize_0);
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(connectfourerror_1_ref_0, connectfourerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3145() {
    rusty_monitor::set_test_id(3145);
    let mut usize_0: usize = 56usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 68usize;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_2: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_1};
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut usize_2: usize = 66usize;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_8: connect_four::Player = crate::connect_four::Player::other(player_7);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_4: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_2};
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_2, status: gamestate_0};
    let mut connectfour_0_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_7_ref_0: &crate::connect_four::Column = &mut column_7;
    let mut bool_0: bool = crate::connect_four::Column::is_full(column_7_ref_0);
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut result_0: std::result::Result<(), connect_four::ConnectFourError> = crate::connect_four::ConnectFour::put(connectfour_0_ref_0, player_0, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3757() {
    rusty_monitor::set_test_id(3757);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut usize_0: usize = 26usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_2: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 36usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_6: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_0, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2: &connect_four::GameState = crate::connect_four::ConnectFour::status(connectfour_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3916() {
    rusty_monitor::set_test_id(3916);
    let mut usize_0: usize = 91usize;
    let mut usize_1: usize = 51usize;
    let mut usize_2: usize = 32usize;
    let mut usize_3: usize = 2usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_1);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut usize_4: usize = 63usize;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_7: connect_four::Player = crate::connect_four::Player::other(player_6);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_9: connect_four::Player = crate::connect_four::Player::other(player_8);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_4: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_4};
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_2, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut player_10: connect_four::Player = crate::connect_four::ConnectFour::get_next_player(connectfour_0_ref_0);
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::InProgress;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2120() {
    rusty_monitor::set_test_id(2120);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut player_1_ref_0: &connect_four::Player = &mut player_1;
    let mut usize_0: usize = 80usize;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_6, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut option_4: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_7: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_7_ref_0: &crate::connect_four::Column = &mut column_7;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut column_8: crate::connect_four::Column = std::clone::Clone::clone(column_7_ref_0);
    let mut player_8: connect_four::Player = std::clone::Clone::clone(player_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_269() {
    rusty_monitor::set_test_id(269);
    let mut usize_0: usize = 49usize;
    let mut usize_1: usize = 70usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut usize_2: usize = 93usize;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut usize_3: usize = 96usize;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_3};
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut usize_4: usize = 4usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_3: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_4};
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut usize_5: usize = 53usize;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_5: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_5};
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_2, status: gamestate_0};
    let mut connectfour_0_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut result_0: std::result::Result<(), connect_four::ConnectFourError> = crate::connect_four::ConnectFour::put(connectfour_0_ref_0, player_1, usize_2);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2623() {
    rusty_monitor::set_test_id(2623);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut usize_0: usize = 72usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_1: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 81usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_8: connect_four::Player = crate::connect_four::Player::other(player_7);
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_3: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_0, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_9_ref_0: &connect_four::Player = &mut player_9;
    let mut usize_2: usize = 16usize;
    let mut usize_3: usize = 58usize;
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut reversi_0: crate::reversi::Reversi = std::result::Result::unwrap(result_0);
    let mut reversi_0_ref_0: &mut crate::reversi::Reversi = &mut reversi_0;
    let mut result_1: std::result::Result<(), reversi::ReversiError> = crate::reversi::Reversi::place(reversi_0_ref_0, player_10, usize_3, usize_2);
    let mut result_2: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut tuple_0: () = std::result::Result::unwrap(result_1);
    let mut reversi_1: crate::reversi::Reversi = std::result::Result::unwrap(result_2);
    let mut player_11: connect_four::Player = std::clone::Clone::clone(player_9_ref_0);
    let mut option_12: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3747() {
    rusty_monitor::set_test_id(3747);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut usize_0: usize = 31usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_1: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut usize_1: usize = 42usize;
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_7_ref_0: &crate::connect_four::Column = &mut column_7;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_1_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(connectfourerror_1_ref_0, connectfourerror_0_ref_0);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_6: &std::option::Option<connect_four::Player> = std::ops::Index::index(column_7_ref_0, usize_1);
    let mut option_7: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4925() {
    rusty_monitor::set_test_id(4925);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut usize_0: usize = 25usize;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut usize_1: usize = 22usize;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_7: connect_four::Player = crate::connect_four::Player::other(player_6);
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_9: connect_four::Player = crate::connect_four::Player::other(player_8);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_11: connect_four::Player = crate::connect_four::Player::other(player_10);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_1: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut usize_2: usize = 44usize;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_4: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_2};
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_0, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut u64_0: u64 = 81u64;
    let mut u64_1: u64 = 61u64;
    let mut steprng_0: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_0;
    let mut usize_3: usize = 27usize;
    let mut usize_4: usize = 5usize;
    let mut usize_5: usize = 86usize;
    let mut result_0: std::result::Result<crate::minesweeper::Minesweeper, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::new(usize_5, usize_4, usize_3, steprng_0_ref_0);
    let mut gamestate_1: &connect_four::GameState = crate::connect_four::ConnectFour::status(connectfour_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4611() {
    rusty_monitor::set_test_id(4611);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut usize_0: usize = 0usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_2, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut option_0: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_7: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_7_ref_0: &crate::connect_four::Column = &mut column_7;
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_8_ref_0: &crate::connect_four::Column = &mut column_8;
    crate::connect_four::ConnectFour::get_connectable();
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut bool_0: bool = std::cmp::PartialEq::ne(column_8_ref_0, column_7_ref_0);
    let mut connectfourerror_1_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_1;
    let mut bool_1: bool = std::cmp::PartialEq::eq(connectfourerror_1_ref_0, connectfourerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2579() {
    rusty_monitor::set_test_id(2579);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut usize_0: usize = 8usize;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_7: connect_four::Player = crate::connect_four::Player::other(player_6);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 80usize;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_2: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut usize_2: usize = 24usize;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::Some(player_13);
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_6: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_2};
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_0, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut player_14: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_7_ref_0: &mut crate::connect_four::Column = &mut column_7;
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_8_ref_0: &crate::connect_four::Column = &mut column_8;
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_9_ref_0: &crate::connect_four::Column = &mut column_9;
    let mut bool_0: bool = std::cmp::PartialEq::eq(column_9_ref_0, column_8_ref_0);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::InProgress;
    crate::connect_four::Column::push(column_7_ref_0, player_14);
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut connectfour_1: crate::connect_four::ConnectFour = std::clone::Clone::clone(connectfour_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4645() {
    rusty_monitor::set_test_id(4645);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut usize_0: usize = 40usize;
    let mut usize_1: usize = 63usize;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player1;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut player_4_ref_0: &connect_four::Player = &mut player_4;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_0_ref_0: &mut crate::connect_four::Column = &mut column_0;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 29usize;
    let mut usize_3: usize = 31usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    crate::connect_four::Column::push(column_0_ref_0, player_5);
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut player_6: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut gamestate_4_ref_0: &connect_four::GameState = &mut gamestate_4;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_4_ref_0, gamestate_0_ref_0);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4968() {
    rusty_monitor::set_test_id(4968);
    let mut usize_0: usize = 87usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 5usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_3: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_1};
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_7_ref_0: &crate::connect_four::Column = &mut column_7;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(column_7_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_0_ref_0);
    let mut player_7: connect_four::Player = crate::connect_four::Player::other(player_6);
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut result_0: std::result::Result<(), connect_four::ConnectFourError> = crate::connect_four::ConnectFour::put(connectfour_0_ref_0, player_0, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_135() {
    rusty_monitor::set_test_id(135);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut usize_0: usize = 47usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 35usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_3: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut usize_2: usize = 19usize;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_11: connect_four::Player = crate::connect_four::Player::other(player_10);
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_14: connect_four::Player = crate::connect_four::Player::other(player_13);
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::Some(player_14);
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_4: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_2};
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_0;
    crate::connect_four::ConnectFour::check_state(connectfour_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4169() {
    rusty_monitor::set_test_id(4169);
    let mut usize_0: usize = 49usize;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut option_0: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_7: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_7_ref_0: &crate::connect_four::Column = &mut column_7;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut u64_0: u64 = 51u64;
    let mut u64_1: u64 = 46u64;
    let mut steprng_0: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_0;
    let mut usize_1: usize = 36usize;
    let mut usize_2: usize = 47usize;
    let mut usize_3: usize = 40usize;
    let mut result_0: std::result::Result<crate::minesweeper::Minesweeper, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::new(usize_3, usize_2, usize_1, steprng_0_ref_0);
    let mut bool_0: bool = crate::connect_four::Column::is_full(column_7_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3416() {
    rusty_monitor::set_test_id(3416);
    let mut usize_0: usize = 44usize;
    let mut usize_1: usize = 4usize;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut usize_2: usize = 94usize;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_7: connect_four::Player = crate::connect_four::Player::other(player_6);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_2};
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut usize_3: usize = 30usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_11: connect_four::Player = crate::connect_four::Player::other(player_10);
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_4: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_3};
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_2_ref_0: &connect_four::GameState = &mut gamestate_2;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut option_12: &std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::get(connectfour_0_ref_0, usize_1, usize_0);
    panic!("From RustyUnit with love");
}
}