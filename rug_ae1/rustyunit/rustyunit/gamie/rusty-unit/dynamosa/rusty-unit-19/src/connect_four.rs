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
fn rusty_test_5630() {
    rusty_monitor::set_test_id(5630);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_4_ref_0: &connect_four::GameState = &mut gamestate_4;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_4_ref_0, gamestate_3_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_399() {
    rusty_monitor::set_test_id(399);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_0_ref_0: &connect_four::Player = &mut player_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 89usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_6: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut result_1: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut player_2_ref_0: &connect_four::Player = &mut player_2;
    let mut bool_3: bool = std::cmp::PartialEq::eq(player_2_ref_0, player_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4204() {
    rusty_monitor::set_test_id(4204);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut usize_0: usize = 93usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
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
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_12: crate::connect_four::Column = std::default::Default::default();
    let mut column_13: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_1: [crate::connect_four::Column; 7] = [column_13, column_12, column_11, column_10, column_9, column_8, column_7];
    let mut connectfour_1: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_1, next: player_3, status: gamestate_1};
    let mut connectfour_1_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_1;
    let mut option_1: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_1_ref_0);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut column_14: crate::connect_four::Column = std::default::Default::default();
    let mut column_15: crate::connect_four::Column = std::default::Default::default();
    let mut column_16: crate::connect_four::Column = std::default::Default::default();
    let mut column_17: crate::connect_four::Column = std::default::Default::default();
    let mut column_18: crate::connect_four::Column = std::default::Default::default();
    let mut column_19: crate::connect_four::Column = std::default::Default::default();
    let mut column_20: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_2: [crate::connect_four::Column; 7] = [column_20, column_19, column_18, column_17, column_16, column_15, column_14];
    let mut connectfour_2: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_2, next: player_5, status: gamestate_2};
    let mut connectfour_2_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_2;
    let mut option_2: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_2_ref_0);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_21: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_21_ref_0: &crate::connect_four::Column = &mut column_21;
    let mut column_22: crate::connect_four::Column = std::default::Default::default();
    let mut column_22_ref_0: &crate::connect_four::Column = &mut column_22;
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_3_ref_0: &gomoku::GameState = &mut gamestate_3;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_0: bool = std::cmp::PartialEq::ne(column_22_ref_0, column_21_ref_0);
    let mut option_6: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7158() {
    rusty_monitor::set_test_id(7158);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_2_ref_0: &connect_four::GameState = &mut gamestate_2;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_278() {
    rusty_monitor::set_test_id(278);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_2, status: gamestate_1};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut option_0: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_260() {
    rusty_monitor::set_test_id(260);
    let mut usize_0: usize = 23usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_2);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 76usize;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_4: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_1};
    let mut usize_2: usize = 55usize;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_5: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_2};
    let mut usize_3: usize = 14usize;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_14: connect_four::Player = crate::connect_four::Player::other(player_13);
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::Some(player_14);
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_16: connect_four::Player = crate::connect_four::Player::other(player_15);
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::Some(player_16);
    let mut player_17: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::Some(player_17);
    let mut player_18: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_19: connect_four::Player = crate::connect_four::Player::other(player_18);
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::Some(player_19);
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_6: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_3};
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_4, status: gamestate_0};
    let mut connectfour_0_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut player_20: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_18: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_21: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_19: std::option::Option<connect_four::Player> = std::option::Option::Some(player_21);
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_22: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_23: connect_four::Player = crate::connect_four::Player::other(player_22);
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_12: crate::connect_four::Column = std::default::Default::default();
    let mut option_20: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_24: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_13: crate::connect_four::Column = std::default::Default::default();
    let mut column_14: crate::connect_four::Column = std::default::Default::default();
    let mut column_15: crate::connect_four::Column = std::default::Default::default();
    let mut column_16: crate::connect_four::Column = std::default::Default::default();
    let mut column_17: crate::connect_four::Column = std::default::Default::default();
    let mut column_18: crate::connect_four::Column = std::default::Default::default();
    let mut column_19: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_1: [crate::connect_four::Column; 7] = [column_19, column_18, column_17, column_16, column_15, column_14, column_13];
    let mut connectfour_1: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_1, next: player_24, status: gamestate_2};
    let mut connectfour_1_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_1;
    let mut option_21: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_1_ref_0);
    let mut player_25: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_22: std::option::Option<connect_four::Player> = std::option::Option::Some(player_25);
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_4_ref_0: &connect_four::GameState = &mut gamestate_4;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_4_ref_0, gamestate_3_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut player_26: connect_four::Player = std::option::Option::unwrap(option_22);
    let mut result_1: std::result::Result<(), connect_four::ConnectFourError> = crate::connect_four::ConnectFour::put(connectfour_0_ref_0, player_0, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4168() {
    rusty_monitor::set_test_id(4168);
    let mut usize_0: usize = 42usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_2_ref_0: &connect_four::GameState = &mut gamestate_2;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut column_6_ref_0: &mut crate::connect_four::Column = &mut column_6;
    let mut option_4: &mut std::option::Option<connect_four::Player> = std::ops::IndexMut::index_mut(column_6_ref_0, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_86() {
    rusty_monitor::set_test_id(86);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut usize_0: usize = 25usize;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut usize_1: usize = 84usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_8: connect_four::Player = crate::connect_four::Player::other(player_7);
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_10: connect_four::Player = crate::connect_four::Player::other(player_9);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_1: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut usize_2: usize = 50usize;
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_14: connect_four::Player = crate::connect_four::Player::other(player_13);
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::Some(player_14);
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_3: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_2};
    let mut usize_3: usize = 74usize;
    let mut option_18: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_16: connect_four::Player = crate::connect_four::Player::other(player_15);
    let mut option_19: std::option::Option<connect_four::Player> = std::option::Option::Some(player_16);
    let mut option_20: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_17: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_21: std::option::Option<connect_four::Player> = std::option::Option::Some(player_17);
    let mut option_22: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_18: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_23: std::option::Option<connect_four::Player> = std::option::Option::Some(player_18);
    let mut option_array_3: [std::option::Option<connect_four::Player>; 6] = [option_23, option_22, option_21, option_20, option_19, option_18];
    let mut column_4: crate::connect_four::Column = crate::connect_four::Column {column: option_array_3, occupied: usize_3};
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut usize_4: usize = 95usize;
    let mut option_24: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_25: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_19: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_20: connect_four::Player = crate::connect_four::Player::other(player_19);
    let mut option_26: std::option::Option<connect_four::Player> = std::option::Option::Some(player_20);
    let mut player_21: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_22: connect_four::Player = crate::connect_four::Player::other(player_21);
    let mut option_27: std::option::Option<connect_four::Player> = std::option::Option::Some(player_22);
    let mut player_23: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_24: connect_four::Player = crate::connect_four::Player::other(player_23);
    let mut option_28: std::option::Option<connect_four::Player> = std::option::Option::Some(player_24);
    let mut player_25: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_29: std::option::Option<connect_four::Player> = std::option::Option::Some(player_25);
    let mut option_array_4: [std::option::Option<connect_four::Player>; 6] = [option_29, option_28, option_27, option_26, option_25, option_24];
    let mut column_6: crate::connect_four::Column = crate::connect_four::Column {column: option_array_4, occupied: usize_4};
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_0, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_0_ref_0);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4535() {
    rusty_monitor::set_test_id(4535);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_3);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_4_ref_0: &connect_four::Player = &mut player_4;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut player_5_ref_0: &connect_four::Player = &mut player_5;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_5_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_518() {
    rusty_monitor::set_test_id(518);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut usize_0: usize = 98usize;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_7: connect_four::Player = crate::connect_four::Player::other(player_6);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_6: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_0, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut bool_0: bool = crate::connect_four::ConnectFour::is_ended(connectfour_0_ref_0);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4517() {
    rusty_monitor::set_test_id(4517);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut usize_0: usize = 95usize;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_2: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 90usize;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_6: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut usize_2: usize = 93usize;
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Win(player_11);
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_13: connect_four::Player = crate::connect_four::Player::other(player_12);
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_12: crate::connect_four::Column = std::default::Default::default();
    let mut column_13: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_1: [crate::connect_four::Column; 7] = [column_13, column_12, column_11, column_10, column_9, column_8, column_7];
    let mut connectfour_1: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_1, next: player_13, status: gamestate_1};
    let mut connectfour_1_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_1;
    let mut option_12: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_1_ref_0);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_14: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_14: crate::connect_four::Column = std::default::Default::default();
    let mut column_15: crate::connect_four::Column = std::default::Default::default();
    let mut column_16: crate::connect_four::Column = std::default::Default::default();
    let mut column_17: crate::connect_four::Column = std::default::Default::default();
    let mut column_18: crate::connect_four::Column = std::default::Default::default();
    let mut column_19: crate::connect_four::Column = std::default::Default::default();
    let mut column_20: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_2: [crate::connect_four::Column; 7] = [column_20, column_19, column_18, column_17, column_16, column_15, column_14];
    let mut connectfour_2: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_2, next: player_14, status: gamestate_2};
    let mut connectfour_2_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_2;
    let mut option_13: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_2_ref_0);
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_16: connect_four::Player = crate::connect_four::Player::other(player_15);
    let mut column_21: crate::connect_four::Column = std::default::Default::default();
    let mut column_22: crate::connect_four::Column = std::default::Default::default();
    let mut column_23: crate::connect_four::Column = std::default::Default::default();
    let mut column_24: crate::connect_four::Column = std::default::Default::default();
    let mut column_25: crate::connect_four::Column = std::default::Default::default();
    let mut column_26: crate::connect_four::Column = std::default::Default::default();
    let mut column_27: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_3: [crate::connect_four::Column; 7] = [column_27, column_26, column_25, column_24, column_23, column_22, column_21];
    let mut connectfour_3: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_3, next: player_16, status: gamestate_3};
    let mut connectfour_3_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_3;
    let mut option_14: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_3_ref_0);
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_17: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::Some(player_17);
    let mut player_18: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::Some(player_18);
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_28: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_2};
    let mut column_28_ref_0: &crate::connect_four::Column = &mut column_28;
    let mut column_29: crate::connect_four::Column = std::default::Default::default();
    let mut column_29_ref_0: &crate::connect_four::Column = &mut column_29;
    let mut gamestate_4: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_4_ref_0: &gomoku::GameState = &mut gamestate_4;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_0: bool = std::cmp::PartialEq::ne(column_29_ref_0, column_28_ref_0);
    let mut gamestate_5: &connect_four::GameState = crate::connect_four::ConnectFour::status(connectfour_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4484() {
    rusty_monitor::set_test_id(4484);
    let mut usize_0: usize = 10usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
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
    let mut option_1: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Win(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_12: crate::connect_four::Column = std::default::Default::default();
    let mut column_13: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_1: [crate::connect_four::Column; 7] = [column_13, column_12, column_11, column_10, column_9, column_8, column_7];
    let mut connectfour_1: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_1, next: player_5, status: gamestate_1};
    let mut connectfour_1_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_1;
    let mut option_3: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_1_ref_0);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_14: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_14_ref_0: &crate::connect_four::Column = &mut column_14;
    let mut usize_1: usize = 88usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_9: connect_four::Player = crate::connect_four::Player::other(player_8);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Win(player_9);
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_15: crate::connect_four::Column = std::default::Default::default();
    let mut column_16: crate::connect_four::Column = std::default::Default::default();
    let mut column_17: crate::connect_four::Column = std::default::Default::default();
    let mut column_18: crate::connect_four::Column = std::default::Default::default();
    let mut column_19: crate::connect_four::Column = std::default::Default::default();
    let mut column_20: crate::connect_four::Column = std::default::Default::default();
    let mut column_21: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_2: [crate::connect_four::Column; 7] = [column_21, column_20, column_19, column_18, column_17, column_16, column_15];
    let mut connectfour_2: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_2, next: player_10, status: gamestate_2};
    let mut connectfour_2_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_2;
    let mut option_10: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_2_ref_0);
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_22: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_22_ref_0: &crate::connect_four::Column = &mut column_22;
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Win(player_12);
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_14: connect_four::Player = crate::connect_four::Player::other(player_13);
    let mut column_23: crate::connect_four::Column = std::default::Default::default();
    let mut column_24: crate::connect_four::Column = std::default::Default::default();
    let mut column_25: crate::connect_four::Column = std::default::Default::default();
    let mut column_26: crate::connect_four::Column = std::default::Default::default();
    let mut column_27: crate::connect_four::Column = std::default::Default::default();
    let mut column_28: crate::connect_four::Column = std::default::Default::default();
    let mut column_29: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_3: [crate::connect_four::Column; 7] = [column_29, column_28, column_27, column_26, column_25, column_24, column_23];
    let mut connectfour_3: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_3, next: player_14, status: gamestate_3};
    let mut connectfour_3_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_3;
    let mut option_12: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_3_ref_0);
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_30: crate::connect_four::Column = std::default::Default::default();
    let mut column_31: crate::connect_four::Column = std::default::Default::default();
    let mut column_32: crate::connect_four::Column = std::default::Default::default();
    let mut column_33: crate::connect_four::Column = std::default::Default::default();
    let mut column_34: crate::connect_four::Column = std::default::Default::default();
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_16: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_17: connect_four::Player = crate::connect_four::Player::other(player_16);
    let mut column_35: crate::connect_four::Column = std::default::Default::default();
    let mut column_36: crate::connect_four::Column = std::default::Default::default();
    let mut column_37: crate::connect_four::Column = std::default::Default::default();
    let mut column_38: crate::connect_four::Column = std::default::Default::default();
    let mut column_39: crate::connect_four::Column = std::default::Default::default();
    let mut column_40: crate::connect_four::Column = std::default::Default::default();
    let mut column_41: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_4: [crate::connect_four::Column; 7] = [column_41, column_40, column_39, column_38, column_37, column_36, column_35];
    let mut connectfour_4: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_4, next: player_17, status: gamestate_5};
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_18: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::Some(player_18);
    let mut player_19: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::Some(player_19);
    let mut column_42: crate::connect_four::Column = std::default::Default::default();
    let mut column_42_ref_0: &crate::connect_four::Column = &mut column_42;
    let mut gamestate_6: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_6_ref_0: &gomoku::GameState = &mut gamestate_6;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_0: bool = std::cmp::PartialEq::eq(column_22_ref_0, column_14_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5871() {
    rusty_monitor::set_test_id(5871);
    let mut usize_0: usize = 93usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
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
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_12: crate::connect_four::Column = std::default::Default::default();
    let mut column_13: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_1: [crate::connect_four::Column; 7] = [column_13, column_12, column_11, column_10, column_9, column_8, column_7];
    let mut connectfour_1: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_1, next: player_3, status: gamestate_1};
    let mut connectfour_1_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_1;
    let mut option_1: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_1_ref_0);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut column_14: crate::connect_four::Column = std::default::Default::default();
    let mut column_15: crate::connect_four::Column = std::default::Default::default();
    let mut column_16: crate::connect_four::Column = std::default::Default::default();
    let mut column_17: crate::connect_four::Column = std::default::Default::default();
    let mut column_18: crate::connect_four::Column = std::default::Default::default();
    let mut column_19: crate::connect_four::Column = std::default::Default::default();
    let mut column_20: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_2: [crate::connect_four::Column; 7] = [column_20, column_19, column_18, column_17, column_16, column_15, column_14];
    let mut connectfour_2: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_2, next: player_5, status: gamestate_2};
    let mut connectfour_2_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_2;
    let mut option_2: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_2_ref_0);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_21: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_21_ref_0: &crate::connect_four::Column = &mut column_21;
    let mut column_22: crate::connect_four::Column = std::default::Default::default();
    let mut column_22_ref_0: &crate::connect_four::Column = &mut column_22;
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_3_ref_0: &gomoku::GameState = &mut gamestate_3;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_0: bool = std::cmp::PartialEq::ne(column_22_ref_0, column_21_ref_0);
    crate::connect_four::ConnectFour::get_connectable();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_750() {
    rusty_monitor::set_test_id(750);
    let mut usize_0: usize = 58usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_1);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
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
    let mut option_1: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Win(player_4);
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
    let mut option_2: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_1_ref_0);
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Win(player_7);
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_9: connect_four::Player = crate::connect_four::Player::other(player_8);
    let mut column_14: crate::connect_four::Column = std::default::Default::default();
    let mut column_15: crate::connect_four::Column = std::default::Default::default();
    let mut column_16: crate::connect_four::Column = std::default::Default::default();
    let mut column_17: crate::connect_four::Column = std::default::Default::default();
    let mut column_18: crate::connect_four::Column = std::default::Default::default();
    let mut column_19: crate::connect_four::Column = std::default::Default::default();
    let mut column_20: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_2: [crate::connect_four::Column; 7] = [column_20, column_19, column_18, column_17, column_16, column_15, column_14];
    let mut connectfour_2: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_2, next: player_9, status: gamestate_2};
    let mut connectfour_2_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_2;
    let mut option_3: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_2_ref_0);
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Win(player_10);
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_12: connect_four::Player = crate::connect_four::Player::other(player_11);
    let mut column_21: crate::connect_four::Column = std::default::Default::default();
    let mut column_22: crate::connect_four::Column = std::default::Default::default();
    let mut column_23: crate::connect_four::Column = std::default::Default::default();
    let mut column_24: crate::connect_four::Column = std::default::Default::default();
    let mut column_25: crate::connect_four::Column = std::default::Default::default();
    let mut column_26: crate::connect_four::Column = std::default::Default::default();
    let mut column_27: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_3: [crate::connect_four::Column; 7] = [column_27, column_26, column_25, column_24, column_23, column_22, column_21];
    let mut connectfour_3: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_3, next: player_12, status: gamestate_3};
    let mut connectfour_3_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_3;
    let mut option_4: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_3_ref_0);
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_14: connect_four::Player = crate::connect_four::Player::other(player_13);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_14);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_28: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_28_ref_0: &crate::connect_four::Column = &mut column_28;
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut player_16: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_17: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_17);
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_18: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_19: connect_four::Player = crate::connect_four::Player::other(player_18);
    let mut column_29: crate::connect_four::Column = std::default::Default::default();
    let mut column_30: crate::connect_four::Column = std::default::Default::default();
    let mut column_31: crate::connect_four::Column = std::default::Default::default();
    let mut column_32: crate::connect_four::Column = std::default::Default::default();
    let mut column_33: crate::connect_four::Column = std::default::Default::default();
    let mut column_34: crate::connect_four::Column = std::default::Default::default();
    let mut column_35: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_4: [crate::connect_four::Column; 7] = [column_35, column_34, column_33, column_32, column_31, column_30, column_29];
    let mut connectfour_4: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_4, next: player_19, status: gamestate_4};
    let mut connectfour_4_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_4;
    let mut option_8: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_4_ref_0);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_20: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_36: crate::connect_four::Column = std::default::Default::default();
    let mut column_37: crate::connect_four::Column = std::default::Default::default();
    let mut column_38: crate::connect_four::Column = std::default::Default::default();
    let mut column_39: crate::connect_four::Column = std::default::Default::default();
    let mut column_40: crate::connect_four::Column = std::default::Default::default();
    let mut column_41: crate::connect_four::Column = std::default::Default::default();
    let mut player_21: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_21);
    let mut gamestate_6: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_6_ref_0: &connect_four::GameState = &mut gamestate_6;
    let mut gamestate_7: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_7_ref_0: &connect_four::GameState = &mut gamestate_7;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_7_ref_0, gamestate_6_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut player_22: reversi::Player = crate::reversi::Player::other(player_15);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(column_28_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1788() {
    rusty_monitor::set_test_id(1788);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 89usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut result_1: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_130() {
    rusty_monitor::set_test_id(130);
    let mut usize_0: usize = 13usize;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_0, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut option_0: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_7: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_7_ref_0: &crate::connect_four::Column = &mut column_7;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Win(player_4);
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut column_8: crate::connect_four::Column = std::clone::Clone::clone(column_7_ref_0);
    let mut result_1: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_387() {
    rusty_monitor::set_test_id(387);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut usize_0: usize = 14usize;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8229() {
    rusty_monitor::set_test_id(8229);
    let mut usize_0: usize = 30usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_11, column_10, column_9, column_8, column_7, column_6, column_5];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_0, status: gamestate_1};
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_2_ref_0: &connect_four::GameState = &mut gamestate_2;
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut column_3_ref_0: &crate::connect_four::Column = &mut column_3;
    let mut option_4: &std::option::Option<connect_four::Player> = std::ops::Index::index(column_3_ref_0, usize_0);
    let mut player_5: connect_four::Player = std::option::Option::unwrap(option_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1331() {
    rusty_monitor::set_test_id(1331);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut usize_0: usize = 79usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_7: connect_four::Player = crate::connect_four::Player::other(player_6);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut usize_1: usize = 23usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_1: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut usize_2: usize = 69usize;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_10: connect_four::Player = crate::connect_four::Player::other(player_9);
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::Some(player_13);
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_3: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_2};
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut usize_3: usize = 42usize;
    let mut option_18: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_14: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_20: std::option::Option<connect_four::Player> = std::option::Option::Some(player_14);
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_21: std::option::Option<connect_four::Player> = std::option::Option::Some(player_15);
    let mut option_22: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_23: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<connect_four::Player>; 6] = [option_23, option_22, option_21, option_20, option_19, option_18];
    let mut column_5: crate::connect_four::Column = crate::connect_four::Column {column: option_array_3, occupied: usize_3};
    let mut usize_4: usize = 25usize;
    let mut player_16: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_17: connect_four::Player = crate::connect_four::Player::other(player_16);
    let mut option_24: std::option::Option<connect_four::Player> = std::option::Option::Some(player_17);
    let mut option_25: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_26: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_18: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_27: std::option::Option<connect_four::Player> = std::option::Option::Some(player_18);
    let mut option_28: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_29: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<connect_four::Player>; 6] = [option_29, option_28, option_27, option_26, option_25, option_24];
    let mut column_6: crate::connect_four::Column = crate::connect_four::Column {column: option_array_4, occupied: usize_4};
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_7_ref_0: &mut crate::connect_four::Column = &mut column_7;
    let mut player_19: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Win(player_19);
    let mut player_20: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_21: connect_four::Player = crate::connect_four::Player::other(player_20);
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut usize_5: usize = 61usize;
    let mut option_30: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_22: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_31: std::option::Option<connect_four::Player> = std::option::Option::Some(player_22);
    let mut player_23: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_32: std::option::Option<connect_four::Player> = std::option::Option::Some(player_23);
    let mut option_33: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_34: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_35: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<connect_four::Player>; 6] = [option_35, option_34, option_33, option_32, option_31, option_30];
    let mut column_10: crate::connect_four::Column = crate::connect_four::Column {column: option_array_5, occupied: usize_5};
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_12: crate::connect_four::Column = std::default::Default::default();
    let mut column_13: crate::connect_four::Column = std::default::Default::default();
    let mut column_14: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_1: [crate::connect_four::Column; 7] = [column_14, column_13, column_12, column_11, column_10, column_9, column_8];
    let mut connectfour_1: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_1, next: player_21, status: gamestate_1};
    let mut connectfour_1_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_1;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_24: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_25: connect_four::Player = crate::connect_four::Player::other(player_24);
    let mut player_26: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_36: std::option::Option<connect_four::Player> = std::option::Option::Some(player_26);
    let mut option_37: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_27: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_28: connect_four::Player = crate::connect_four::Player::other(player_27);
    let mut option_38: std::option::Option<connect_four::Player> = std::option::Option::Some(player_28);
    let mut option_39: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_29: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_40: std::option::Option<connect_four::Player> = std::option::Option::Some(player_29);
    let mut player_30: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_41: std::option::Option<connect_four::Player> = std::option::Option::Some(player_30);
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_2_ref_0: &connect_four::GameState = &mut gamestate_2;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    crate::connect_four::Column::push(column_7_ref_0, player_25);
    let mut connectfour_2: crate::connect_four::ConnectFour = std::clone::Clone::clone(connectfour_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1809() {
    rusty_monitor::set_test_id(1809);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_0_ref_0: &crate::connect_four::Column = &mut column_0;
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_1_ref_0: &crate::connect_four::Column = &mut column_1;
    let mut usize_0: usize = 93usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_8, column_7, column_6, column_5, column_4, column_3, column_2];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_2, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut option_0: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_12: crate::connect_four::Column = std::default::Default::default();
    let mut column_13: crate::connect_four::Column = std::default::Default::default();
    let mut column_14: crate::connect_four::Column = std::default::Default::default();
    let mut column_15: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_1: [crate::connect_four::Column; 7] = [column_15, column_14, column_13, column_12, column_11, column_10, column_9];
    let mut connectfour_1: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_1, next: player_3, status: gamestate_1};
    let mut connectfour_1_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_1;
    let mut option_1: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_1_ref_0);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut column_16: crate::connect_four::Column = std::default::Default::default();
    let mut column_17: crate::connect_four::Column = std::default::Default::default();
    let mut column_18: crate::connect_four::Column = std::default::Default::default();
    let mut column_19: crate::connect_four::Column = std::default::Default::default();
    let mut column_20: crate::connect_four::Column = std::default::Default::default();
    let mut column_21: crate::connect_four::Column = std::default::Default::default();
    let mut column_22: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_2: [crate::connect_four::Column; 7] = [column_22, column_21, column_20, column_19, column_18, column_17, column_16];
    let mut connectfour_2: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_2, next: player_5, status: gamestate_2};
    let mut connectfour_2_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_2;
    let mut option_2: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_2_ref_0);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_23: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_23_ref_0: &crate::connect_four::Column = &mut column_23;
    let mut column_24: crate::connect_four::Column = std::default::Default::default();
    let mut column_24_ref_0: &crate::connect_four::Column = &mut column_24;
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_3_ref_0: &gomoku::GameState = &mut gamestate_3;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_0: bool = std::cmp::PartialEq::ne(column_24_ref_0, column_23_ref_0);
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut bool_1: bool = std::cmp::PartialEq::eq(column_1_ref_0, column_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5572() {
    rusty_monitor::set_test_id(5572);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut usize_0: usize = 61usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_2: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_2, status: gamestate_0};
    let mut connectfour_0_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_9: connect_four::Player = crate::connect_four::Player::other(player_8);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut usize_1: usize = 57usize;
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::Some(player_13);
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_14: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::Some(player_14);
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_16, option_15, option_14, option_13, option_12, option_11];
    let mut column_7: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut usize_2: usize = 3usize;
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::Some(player_15);
    let mut player_16: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_18: std::option::Option<connect_four::Player> = std::option::Option::Some(player_16);
    let mut player_17: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_19: std::option::Option<connect_four::Player> = std::option::Option::Some(player_17);
    let mut player_18: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_20: std::option::Option<connect_four::Player> = std::option::Option::Some(player_18);
    let mut player_19: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_20: connect_four::Player = crate::connect_four::Player::other(player_19);
    let mut option_21: std::option::Option<connect_four::Player> = std::option::Option::Some(player_20);
    let mut option_22: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_22, option_21, option_20, option_19, option_18, option_17];
    let mut column_10: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_2};
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut player_21: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_23: std::option::Option<connect_four::Player> = std::option::Option::Some(player_21);
    let mut option_24: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_22: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_25: std::option::Option<connect_four::Player> = std::option::Option::Some(player_22);
    let mut player_23: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_26: std::option::Option<connect_four::Player> = std::option::Option::Some(player_23);
    let mut player_24: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_27: std::option::Option<connect_four::Player> = std::option::Option::Some(player_24);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    crate::connect_four::ConnectFour::check_state(connectfour_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2273() {
    rusty_monitor::set_test_id(2273);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_0_ref_0: &mut crate::connect_four::Column = &mut column_0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1_ref_0: &connect_four::Player = &mut player_1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_7, option_6, option_5];
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_12: connect_four::Player = std::clone::Clone::clone(player_1_ref_0);
    crate::connect_four::Column::push(column_0_ref_0, player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3309() {
    rusty_monitor::set_test_id(3309);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 89usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut result_1: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut gamestate_5_ref_0: &connect_four::GameState = &mut gamestate_5;
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_5_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3009() {
    rusty_monitor::set_test_id(3009);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
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
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_3, status: gamestate_1};
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_12: crate::connect_four::Column = std::default::Default::default();
    let mut column_13: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_1: [crate::connect_four::Column; 7] = [column_13, column_12, column_11, column_10, column_9, column_8, column_7];
    let mut connectfour_1: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_1, next: player_4, status: gamestate_2};
    let mut connectfour_1_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_1;
    let mut option_3: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_1_ref_0);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_4_ref_0: &connect_four::GameState = &mut gamestate_4;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_4_ref_0, gamestate_3_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut gamestate_5: connect_four::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7778() {
    rusty_monitor::set_test_id(7778);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 89usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut result_1: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut gamestate_6: connect_four::GameState = std::clone::Clone::clone(gamestate_3_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1137() {
    rusty_monitor::set_test_id(1137);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_3, status: gamestate_1};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut option_2: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_170() {
    rusty_monitor::set_test_id(170);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_5_ref_0: &minesweeper::GameState = &mut gamestate_5;
    let mut gamestate_6: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_7: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_7_ref_0: &reversi::GameState = &mut gamestate_7;
    let mut gamestate_6_ref_0: &minesweeper::GameState = &mut gamestate_6;
    let mut gamestate_8: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut player_1_ref_0: &connect_four::Player = &mut player_1;
    let mut player_2: connect_four::Player = std::clone::Clone::clone(player_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_24() {
    rusty_monitor::set_test_id(24);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut usize_0: usize = 4usize;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut usize_1: usize = 8usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_1: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut usize_2: usize = 3usize;
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::Some(player_13);
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_2: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_2};
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut usize_3: usize = 94usize;
    let mut player_14: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_18: std::option::Option<connect_four::Player> = std::option::Option::Some(player_14);
    let mut option_19: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_16: connect_four::Player = crate::connect_four::Player::other(player_15);
    let mut option_20: std::option::Option<connect_four::Player> = std::option::Option::Some(player_16);
    let mut player_17: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_21: std::option::Option<connect_four::Player> = std::option::Option::Some(player_17);
    let mut player_18: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_19: connect_four::Player = crate::connect_four::Player::other(player_18);
    let mut option_22: std::option::Option<connect_four::Player> = std::option::Option::Some(player_19);
    let mut player_20: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_23: std::option::Option<connect_four::Player> = std::option::Option::Some(player_20);
    let mut option_array_3: [std::option::Option<connect_four::Player>; 6] = [option_23, option_22, option_21, option_20, option_19, option_18];
    let mut column_5: crate::connect_four::Column = crate::connect_four::Column {column: option_array_3, occupied: usize_3};
    let mut usize_4: usize = 21usize;
    let mut option_24: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_21: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_22: connect_four::Player = crate::connect_four::Player::other(player_21);
    let mut option_25: std::option::Option<connect_four::Player> = std::option::Option::Some(player_22);
    let mut option_26: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_23: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_27: std::option::Option<connect_four::Player> = std::option::Option::Some(player_23);
    let mut player_24: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_28: std::option::Option<connect_four::Player> = std::option::Option::Some(player_24);
    let mut option_29: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<connect_four::Player>; 6] = [option_29, option_28, option_27, option_26, option_25, option_24];
    let mut column_6: crate::connect_four::Column = crate::connect_four::Column {column: option_array_4, occupied: usize_4};
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut player_25: connect_four::Player = crate::connect_four::ConnectFour::get_next_player(connectfour_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7746() {
    rusty_monitor::set_test_id(7746);
    let mut usize_0: usize = 52usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 55usize;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_2: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_1};
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut usize_2: usize = 59usize;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_4: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_2};
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut usize_3: usize = 53usize;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_9: connect_four::Player = crate::connect_four::Player::other(player_8);
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_12: connect_four::Player = crate::connect_four::Player::other(player_11);
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_14: connect_four::Player = crate::connect_four::Player::other(player_13);
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::Some(player_14);
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_6: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_3};
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut usize_4: usize = 93usize;
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Win(player_15);
    let mut player_16: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_17: connect_four::Player = crate::connect_four::Player::other(player_16);
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_12: crate::connect_four::Column = std::default::Default::default();
    let mut column_13: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_1: [crate::connect_four::Column; 7] = [column_13, column_12, column_11, column_10, column_9, column_8, column_7];
    let mut connectfour_1: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_1, next: player_17, status: gamestate_1};
    let mut connectfour_1_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_1;
    let mut option_18: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_1_ref_0);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_18: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_14: crate::connect_four::Column = std::default::Default::default();
    let mut column_15: crate::connect_four::Column = std::default::Default::default();
    let mut column_16: crate::connect_four::Column = std::default::Default::default();
    let mut column_17: crate::connect_four::Column = std::default::Default::default();
    let mut column_18: crate::connect_four::Column = std::default::Default::default();
    let mut column_19: crate::connect_four::Column = std::default::Default::default();
    let mut column_20: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_2: [crate::connect_four::Column; 7] = [column_20, column_19, column_18, column_17, column_16, column_15, column_14];
    let mut connectfour_2: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_2, next: player_18, status: gamestate_2};
    let mut connectfour_2_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_2;
    let mut option_19: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_2_ref_0);
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_19: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_20: connect_four::Player = crate::connect_four::Player::other(player_19);
    let mut column_21: crate::connect_four::Column = std::default::Default::default();
    let mut column_22: crate::connect_four::Column = std::default::Default::default();
    let mut column_23: crate::connect_four::Column = std::default::Default::default();
    let mut column_24: crate::connect_four::Column = std::default::Default::default();
    let mut column_25: crate::connect_four::Column = std::default::Default::default();
    let mut column_26: crate::connect_four::Column = std::default::Default::default();
    let mut column_27: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_3: [crate::connect_four::Column; 7] = [column_27, column_26, column_25, column_24, column_23, column_22, column_21];
    let mut connectfour_3: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_3, next: player_20, status: gamestate_3};
    let mut connectfour_3_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_3;
    let mut option_20: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_3_ref_0);
    let mut option_21: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_21: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_22: std::option::Option<connect_four::Player> = std::option::Option::Some(player_21);
    let mut player_22: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_23: std::option::Option<connect_four::Player> = std::option::Option::Some(player_22);
    let mut option_array_3: [std::option::Option<connect_four::Player>; 6] = [option_23, option_22, option_21, option_20, option_19, option_18];
    let mut column_28: crate::connect_four::Column = crate::connect_four::Column {column: option_array_3, occupied: usize_4};
    let mut column_28_ref_0: &crate::connect_four::Column = &mut column_28;
    let mut column_29: crate::connect_four::Column = std::default::Default::default();
    let mut column_29_ref_0: &crate::connect_four::Column = &mut column_29;
    let mut gamestate_4: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_4_ref_0: &gomoku::GameState = &mut gamestate_4;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_0: bool = std::cmp::PartialEq::ne(column_29_ref_0, column_28_ref_0);
    let mut result_0: std::result::Result<(), connect_four::ConnectFourError> = crate::connect_four::ConnectFour::put(connectfour_0_ref_0, player_0, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1584() {
    rusty_monitor::set_test_id(1584);
    let mut usize_0: usize = 93usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
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
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_12: crate::connect_four::Column = std::default::Default::default();
    let mut column_13: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_1: [crate::connect_four::Column; 7] = [column_13, column_12, column_11, column_10, column_9, column_8, column_7];
    let mut connectfour_1: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_1, next: player_3, status: gamestate_1};
    let mut connectfour_1_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_1;
    let mut option_1: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_1_ref_0);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut column_14: crate::connect_four::Column = std::default::Default::default();
    let mut column_15: crate::connect_four::Column = std::default::Default::default();
    let mut column_16: crate::connect_four::Column = std::default::Default::default();
    let mut column_17: crate::connect_four::Column = std::default::Default::default();
    let mut column_18: crate::connect_four::Column = std::default::Default::default();
    let mut column_19: crate::connect_four::Column = std::default::Default::default();
    let mut column_20: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_2: [crate::connect_four::Column; 7] = [column_20, column_19, column_18, column_17, column_16, column_15, column_14];
    let mut connectfour_2: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_2, next: player_5, status: gamestate_2};
    let mut connectfour_2_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_2;
    let mut option_2: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_2_ref_0);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_21: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_21_ref_0: &crate::connect_four::Column = &mut column_21;
    let mut column_22: crate::connect_four::Column = std::default::Default::default();
    let mut column_22_ref_0: &crate::connect_four::Column = &mut column_22;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_0: bool = std::cmp::PartialEq::ne(column_22_ref_0, column_21_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6924() {
    rusty_monitor::set_test_id(6924);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_0_ref_0: &crate::connect_four::Column = &mut column_0;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_4, option_3, option_2];
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_7, option_6, option_5];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut bool_0: bool = crate::connect_four::Column::is_full(column_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_793() {
    rusty_monitor::set_test_id(793);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}
}