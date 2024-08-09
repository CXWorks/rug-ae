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
#[timeout(30000)]    #[no_coverage]
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
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_359() {
//    rusty_monitor::set_test_id(359);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_0_ref_0: &connect_four::Player = &mut player_0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut player_2_ref_0: &connect_four::Player = &mut player_2;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3_ref_0: &connect_four::Player = &mut player_3;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_4_ref_0: &connect_four::Player = &mut player_4;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut player_6_ref_0: &connect_four::Player = &mut player_6;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_7_ref_0: &connect_four::Player = &mut player_7;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_8_ref_0: &connect_four::Player = &mut player_8;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_9_ref_0: &connect_four::Player = &mut player_9;
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_10_ref_0: &connect_four::Player = &mut player_10;
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_11_ref_0: &connect_four::Player = &mut player_11;
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_12_ref_0: &connect_four::Player = &mut player_12;
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_13_ref_0: &connect_four::Player = &mut player_13;
    let mut bool_0: bool = std::cmp::PartialEq::eq(player_13_ref_0, player_12_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(player_11_ref_0, player_10_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(player_9_ref_0, player_8_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(player_7_ref_0, player_6_ref_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(player_4_ref_0, player_3_ref_0);
    let mut bool_5: bool = std::cmp::PartialEq::eq(player_2_ref_0, player_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_138() {
//    rusty_monitor::set_test_id(138);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut usize_0: usize = 0usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_3: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 35usize;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_6: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_0, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut bool_0: bool = crate::connect_four::ConnectFour::is_ended(connectfour_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_419() {
//    rusty_monitor::set_test_id(419);
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
    crate::connect_four::ConnectFour::get_connectable();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_612() {
//    rusty_monitor::set_test_id(612);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_0_ref_0: &connect_four::Player = &mut player_0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut player_2_ref_0: &connect_four::Player = &mut player_2;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_3_ref_0: &connect_four::Player = &mut player_3;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_4_ref_0: &connect_four::Player = &mut player_4;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5_ref_0: &connect_four::Player = &mut player_5;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_6_ref_0: &connect_four::Player = &mut player_6;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_7_ref_0: &connect_four::Player = &mut player_7;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_8_ref_0: &connect_four::Player = &mut player_8;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_9_ref_0: &connect_four::Player = &mut player_9;
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_10_ref_0: &connect_four::Player = &mut player_10;
    let mut player_11: connect_four::Player = std::clone::Clone::clone(player_10_ref_0);
    let mut player_12: connect_four::Player = std::clone::Clone::clone(player_9_ref_0);
    let mut player_13: connect_four::Player = std::clone::Clone::clone(player_8_ref_0);
    let mut player_14: connect_four::Player = std::clone::Clone::clone(player_7_ref_0);
    let mut player_15: connect_four::Player = std::clone::Clone::clone(player_6_ref_0);
    let mut player_16: connect_four::Player = std::clone::Clone::clone(player_5_ref_0);
    let mut player_17: connect_four::Player = std::clone::Clone::clone(player_4_ref_0);
    let mut player_18: connect_four::Player = std::clone::Clone::clone(player_3_ref_0);
    let mut player_19: connect_four::Player = std::clone::Clone::clone(player_2_ref_0);
    let mut player_20: connect_four::Player = std::clone::Clone::clone(player_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1195() {
//    rusty_monitor::set_test_id(1195);
    let mut usize_0: usize = 43usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 7usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_2: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_1};
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut usize_2: usize = 0usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_6: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_2};
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_2, status: gamestate_0};
    let mut connectfour_0_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut usize_3: usize = 7usize;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_12: crate::connect_four::Column = std::default::Default::default();
    let mut column_13: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_1: [crate::connect_four::Column; 7] = [column_13, column_12, column_11, column_10, column_9, column_8, column_7];
    let mut connectfour_1: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_1, next: player_10, status: gamestate_1};
    let mut connectfour_1_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_1;
    let mut result_0: std::result::Result<(), connect_four::ConnectFourError> = crate::connect_four::ConnectFour::put(connectfour_1_ref_0, player_9, usize_3);
    let mut result_1: std::result::Result<(), connect_four::ConnectFourError> = crate::connect_four::ConnectFour::put(connectfour_0_ref_0, player_1, usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_626() {
//    rusty_monitor::set_test_id(626);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_1_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_1;
    let mut connectfourerror_2: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_2_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_2;
    let mut connectfourerror_3: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_3_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_3;
    let mut connectfourerror_4: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_4_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_4;
    let mut connectfourerror_5: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_5_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_5;
    let mut connectfourerror_6: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_6_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_6;
    let mut connectfourerror_7: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_7_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_7;
    let mut connectfourerror_8: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_8_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_8;
    let mut connectfourerror_9: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_9_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_9;
    let mut connectfourerror_10: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_10_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_10;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_10_ref_0);
    let mut option_1: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_9_ref_0);
    let mut option_2: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_8_ref_0);
    let mut option_3: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_7_ref_0);
    let mut option_4: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_6_ref_0);
    let mut option_5: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_5_ref_0);
    let mut option_6: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_4_ref_0);
    let mut option_7: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_3_ref_0);
    let mut option_8: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_2_ref_0);
    let mut option_9: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_1_ref_0);
    let mut option_10: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(connectfourerror_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_264() {
//    rusty_monitor::set_test_id(264);
    let mut usize_0: usize = 8usize;
    let mut usize_1: usize = 16usize;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut usize_2: usize = 6usize;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_1: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_2};
    let mut usize_3: usize = 1usize;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_2: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_3};
    let mut usize_4: usize = 2usize;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_3: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_4};
    let mut usize_5: usize = 6usize;
    let mut option_18: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_20: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut option_21: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_22: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_13: connect_four::Player = crate::connect_four::Player::other(player_12);
    let mut option_23: std::option::Option<connect_four::Player> = std::option::Option::Some(player_13);
    let mut option_array_3: [std::option::Option<connect_four::Player>; 6] = [option_23, option_22, option_21, option_20, option_19, option_18];
    let mut column_4: crate::connect_four::Column = crate::connect_four::Column {column: option_array_3, occupied: usize_5};
    let mut usize_6: usize = 97usize;
    let mut option_24: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_14: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_25: std::option::Option<connect_four::Player> = std::option::Option::Some(player_14);
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_26: std::option::Option<connect_four::Player> = std::option::Option::Some(player_15);
    let mut player_16: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_17: connect_four::Player = crate::connect_four::Player::other(player_16);
    let mut option_27: std::option::Option<connect_four::Player> = std::option::Option::Some(player_17);
    let mut player_18: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_28: std::option::Option<connect_four::Player> = std::option::Option::Some(player_18);
    let mut player_19: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_29: std::option::Option<connect_four::Player> = std::option::Option::Some(player_19);
    let mut option_array_4: [std::option::Option<connect_four::Player>; 6] = [option_29, option_28, option_27, option_26, option_25, option_24];
    let mut column_5: crate::connect_four::Column = crate::connect_four::Column {column: option_array_4, occupied: usize_6};
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_0, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut option_30: &std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::get(connectfour_0_ref_0, usize_1, usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_107() {
//    rusty_monitor::set_test_id(107);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut usize_0: usize = 4usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_1: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 16usize;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_4: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut usize_2: usize = 8usize;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::Some(player_13);
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_5: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_2};
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut connectfour_1: crate::connect_four::ConnectFour = std::clone::Clone::clone(connectfour_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_60() {
//    rusty_monitor::set_test_id(60);
    let mut usize_0: usize = 6usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
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
    let mut option_1: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_12: crate::connect_four::Column = std::default::Default::default();
    let mut column_13: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_1: [crate::connect_four::Column; 7] = [column_13, column_12, column_11, column_10, column_9, column_8, column_7];
    let mut connectfour_1: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_1, next: player_2, status: gamestate_1};
    let mut connectfour_1_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_1;
    let mut option_2: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_1_ref_0);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_14: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_14_ref_0: &crate::connect_four::Column = &mut column_14;
    let mut bool_0: bool = crate::connect_four::Column::is_full(column_14_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_105() {
//    rusty_monitor::set_test_id(105);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_0_ref_0: &crate::connect_four::Column = &mut column_0;
    let mut usize_0: usize = 7usize;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_7, column_6, column_5, column_4, column_3, column_2, column_1];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut option_0: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_8: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_8_ref_0: &crate::connect_four::Column = &mut column_8;
    let mut bool_0: bool = std::cmp::PartialEq::ne(column_8_ref_0, column_0_ref_0);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_502() {
//    rusty_monitor::set_test_id(502);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_2_ref_0: &connect_four::GameState = &mut gamestate_2;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_4_ref_0: &connect_four::GameState = &mut gamestate_4;
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_5_ref_0: &connect_four::GameState = &mut gamestate_5;
    let mut gamestate_6: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_6_ref_0: &connect_four::GameState = &mut gamestate_6;
    let mut gamestate_7: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_7_ref_0: &connect_four::GameState = &mut gamestate_7;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_8: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut gamestate_8_ref_0: &connect_four::GameState = &mut gamestate_8;
    let mut gamestate_9: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_9_ref_0: &connect_four::GameState = &mut gamestate_9;
    let mut gamestate_10: connect_four::GameState = std::clone::Clone::clone(gamestate_9_ref_0);
    let mut gamestate_11: connect_four::GameState = std::clone::Clone::clone(gamestate_8_ref_0);
    let mut gamestate_12: connect_four::GameState = std::clone::Clone::clone(gamestate_7_ref_0);
    let mut gamestate_13: connect_four::GameState = std::clone::Clone::clone(gamestate_6_ref_0);
    let mut gamestate_14: connect_four::GameState = std::clone::Clone::clone(gamestate_5_ref_0);
    let mut gamestate_15: connect_four::GameState = std::clone::Clone::clone(gamestate_4_ref_0);
    let mut gamestate_16: connect_four::GameState = std::clone::Clone::clone(gamestate_3_ref_0);
    let mut gamestate_17: connect_four::GameState = std::clone::Clone::clone(gamestate_2_ref_0);
    let mut gamestate_18: connect_four::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut gamestate_19: connect_four::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5710() {
//    rusty_monitor::set_test_id(5710);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_2);
    let mut player_3_ref_0: &reversi::Player = &mut player_3;
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_4);
    let mut player_5_ref_0: &reversi::Player = &mut player_5;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_1_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(connectfourerror_1_ref_0, connectfourerror_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7451() {
//    rusty_monitor::set_test_id(7451);
    let mut usize_0: usize = 15usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 4usize;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_1: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_1};
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut usize_2: usize = 7usize;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_3: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_2};
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_4_ref_0: &mut crate::connect_four::Column = &mut column_4;
    let mut option_12: &mut std::option::Option<connect_four::Player> = std::ops::IndexMut::index_mut(column_4_ref_0, usize_0);
    let mut column_0_ref_0: &crate::connect_four::Column = &mut column_0;
    let mut column_5: crate::connect_four::Column = std::clone::Clone::clone(column_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_317() {
//    rusty_monitor::set_test_id(317);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Win(player_1);
    let mut gamestate_2_ref_0: &connect_four::GameState = &mut gamestate_2;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Win(player_2);
    let mut gamestate_4_ref_0: &connect_four::GameState = &mut gamestate_4;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::Win(player_4);
    let mut gamestate_5_ref_0: &connect_four::GameState = &mut gamestate_5;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut gamestate_6: connect_four::GameState = crate::connect_four::GameState::Win(player_6);
    let mut gamestate_6_ref_0: &connect_four::GameState = &mut gamestate_6;
    let mut gamestate_7: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_7_ref_0: &connect_four::GameState = &mut gamestate_7;
    let mut gamestate_8: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_8_ref_0: &connect_four::GameState = &mut gamestate_8;
    let mut gamestate_9: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_9_ref_0: &connect_four::GameState = &mut gamestate_9;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_9_ref_0, gamestate_8_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::ne(gamestate_7_ref_0, gamestate_6_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::ne(gamestate_5_ref_0, gamestate_4_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::ne(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut bool_4: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1082() {
//    rusty_monitor::set_test_id(1082);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_0_ref_0: &crate::connect_four::Column = &mut column_0;
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_1_ref_0: &crate::connect_four::Column = &mut column_1;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut u64_0: u64 = 17u64;
    let mut u64_1: u64 = 24u64;
    let mut steprng_0: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_0;
    let mut u64_2: u64 = 71u64;
    let mut u64_3: u64 = 26u64;
    let mut steprng_1: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_3, u64_2);
    let mut steprng_1_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_1;
    let mut u64_4: u64 = 33u64;
    let mut u64_5: u64 = 15u64;
    let mut steprng_2: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_5, u64_4);
    let mut steprng_2_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_2;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut player_5: reversi::Player = crate::reversi::Player::other(player_3);
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Win(player_4);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut bool_0: bool = std::cmp::PartialEq::ne(column_1_ref_0, column_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8904() {
//    rusty_monitor::set_test_id(8904);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_2_ref_0: &connect_four::Player = &mut player_2;
    let mut u64_0: u64 = 17u64;
    let mut u64_1: u64 = 24u64;
    let mut steprng_0: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_0;
    let mut usize_0: usize = 16usize;
    let mut usize_1: usize = 15usize;
    let mut usize_2: usize = 0usize;
    let mut u64_2: u64 = 33u64;
    let mut u64_3: u64 = 15u64;
    let mut steprng_1: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_3, u64_2);
    let mut steprng_1_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_1;
    let mut usize_3: usize = 99usize;
    let mut usize_4: usize = 6usize;
    let mut usize_5: usize = 5usize;
    let mut u64_4: u64 = 0u64;
    let mut u64_5: u64 = 97u64;
    let mut steprng_2: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_5, u64_4);
    let mut steprng_2_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_2;
    let mut usize_6: usize = 7usize;
    let mut usize_7: usize = 3usize;
    let mut usize_8: usize = 1usize;
    let mut result_0: std::result::Result<crate::minesweeper::Minesweeper, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::new(usize_8, usize_7, usize_6, steprng_2_ref_0);
    let mut result_1: std::result::Result<crate::minesweeper::Minesweeper, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::new(usize_5, usize_4, usize_3, steprng_1_ref_0);
    let mut result_2: std::result::Result<crate::minesweeper::Minesweeper, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::new(usize_2, usize_1, usize_0, steprng_0_ref_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_122() {
//    rusty_monitor::set_test_id(122);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut usize_0: usize = 6usize;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 0usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_10: connect_four::Player = crate::connect_four::Player::other(player_9);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_12: connect_four::Player = crate::connect_four::Player::other(player_11);
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_4: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_2, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut gamestate_1: &connect_four::GameState = crate::connect_four::ConnectFour::status(connectfour_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_559() {
//    rusty_monitor::set_test_id(559);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_0_ref_0: &crate::connect_four::Column = &mut column_0;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_149() {
//    rusty_monitor::set_test_id(149);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut usize_0: usize = 0usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 55usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_2: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut usize_2: usize = 6usize;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_8: connect_four::Player = crate::connect_four::Player::other(player_7);
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_11: connect_four::Player = crate::connect_four::Player::other(player_10);
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_5: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_2};
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut player_12: connect_four::Player = crate::connect_four::ConnectFour::get_next_player(connectfour_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_418() {
//    rusty_monitor::set_test_id(418);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_2_ref_0: &connect_four::GameState = &mut gamestate_2;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Win(player_2);
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_4_ref_0: &connect_four::GameState = &mut gamestate_4;
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_5_ref_0: &connect_four::GameState = &mut gamestate_5;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_6: connect_four::GameState = crate::connect_four::GameState::Win(player_3);
    let mut gamestate_6_ref_0: &connect_four::GameState = &mut gamestate_6;
    let mut gamestate_7: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_7_ref_0: &connect_four::GameState = &mut gamestate_7;
    let mut gamestate_8: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_8_ref_0: &connect_four::GameState = &mut gamestate_8;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_8_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_7_ref_0);
    let mut tuple_2: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_6_ref_0);
    let mut tuple_3: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_5_ref_0);
    let mut tuple_4: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_4_ref_0);
    let mut tuple_5: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_3_ref_0);
    let mut tuple_6: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_2_ref_0);
    let mut tuple_7: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_1_ref_0);
    let mut tuple_8: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_282() {
//    rusty_monitor::set_test_id(282);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_1_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_1;
    let mut connectfourerror_2: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_2_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_2;
    let mut connectfourerror_3: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_3_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_3;
    let mut connectfourerror_4: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_4_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_4;
    let mut connectfourerror_5: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_5_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_5;
    let mut connectfourerror_6: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_6_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_6;
    let mut connectfourerror_7: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_7_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_7;
    let mut connectfourerror_8: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_8_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_8;
    let mut connectfourerror_9: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_9_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_9;
    let mut connectfourerror_10: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_10_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_10;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_10_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_9_ref_0);
    let mut tuple_2: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_8_ref_0);
    let mut tuple_3: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_7_ref_0);
    let mut tuple_4: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_6_ref_0);
    let mut tuple_5: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_5_ref_0);
    let mut tuple_6: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_4_ref_0);
    let mut tuple_7: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_3_ref_0);
    let mut tuple_8: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_2_ref_0);
    let mut tuple_9: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_1_ref_0);
    let mut tuple_10: () = std::cmp::Eq::assert_receiver_is_total_eq(connectfourerror_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1124() {
//    rusty_monitor::set_test_id(1124);
    let mut usize_0: usize = 8usize;
    let mut usize_1: usize = 3usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut usize_2: usize = 15usize;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_8: connect_four::Player = crate::connect_four::Player::other(player_7);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_2};
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut usize_3: usize = 6usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_10: connect_four::Player = crate::connect_four::Player::other(player_9);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_12: connect_four::Player = crate::connect_four::Player::other(player_11);
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_2: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_3};
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut usize_4: usize = 4usize;
    let mut player_14: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::Some(player_14);
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::Some(player_15);
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_8: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_4};
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut usize_5: usize = 7usize;
    let mut player_16: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_18: std::option::Option<connect_four::Player> = std::option::Option::Some(player_16);
    let mut option_19: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_17: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_18: connect_four::Player = crate::connect_four::Player::other(player_17);
    let mut option_21: std::option::Option<connect_four::Player> = std::option::Option::Some(player_18);
    let mut player_19: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_22: std::option::Option<connect_four::Player> = std::option::Option::Some(player_19);
    let mut option_23: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<connect_four::Player>; 6] = [option_23, option_22, option_21, option_20, option_19, option_18];
    let mut column_10: crate::connect_four::Column = crate::connect_four::Column {column: option_array_3, occupied: usize_5};
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut usize_6: usize = 5usize;
    let mut player_20: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_24: std::option::Option<connect_four::Player> = std::option::Option::Some(player_20);
    let mut option_25: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_21: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_26: std::option::Option<connect_four::Player> = std::option::Option::Some(player_21);
    let mut option_27: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_28: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_22: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_29: std::option::Option<connect_four::Player> = std::option::Option::Some(player_22);
    let mut option_array_4: [std::option::Option<connect_four::Player>; 6] = [option_29, option_28, option_27, option_26, option_25, option_24];
    let mut column_12: crate::connect_four::Column = crate::connect_four::Column {column: option_array_4, occupied: usize_6};
    let mut column_13: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_1: [crate::connect_four::Column; 7] = [column_13, column_12, column_11, column_10, column_9, column_8, column_7];
    let mut connectfour_1: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_1, next: player_13, status: gamestate_1};
    let mut connectfour_1_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_1;
    crate::connect_four::ConnectFour::check_state(connectfour_1_ref_0);
    let mut option_30: &std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::get(connectfour_0_ref_0, usize_1, usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_657() {
//    rusty_monitor::set_test_id(657);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_2_ref_0: &connect_four::GameState = &mut gamestate_2;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Win(player_1);
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_4_ref_0: &connect_four::GameState = &mut gamestate_4;
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_5_ref_0: &connect_four::GameState = &mut gamestate_5;
    let mut gamestate_6: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_6_ref_0: &connect_four::GameState = &mut gamestate_6;
    let mut gamestate_7: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_7_ref_0: &connect_four::GameState = &mut gamestate_7;
    let mut gamestate_8: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_8_ref_0: &connect_four::GameState = &mut gamestate_8;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_9: connect_four::GameState = crate::connect_four::GameState::Win(player_2);
    let mut gamestate_9_ref_0: &connect_four::GameState = &mut gamestate_9;
    let mut gamestate_10: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_10_ref_0: &connect_four::GameState = &mut gamestate_10;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_10_ref_0, gamestate_9_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_7_ref_0, gamestate_6_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(gamestate_5_ref_0, gamestate_4_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_93() {
//    rusty_monitor::set_test_id(93);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_0_ref_0: &crate::connect_four::Column = &mut column_0;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut bool_0: bool = false;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_0: usize = 76usize;
    let mut usize_1: usize = 15usize;
    let mut usize_2: usize = 0usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_3: usize = 8usize;
    let mut usize_4: usize = 85usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::Win(player_2);
    let mut gamestate_3_ref_0: &gomoku::GameState = &mut gamestate_3;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_6: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut gamestate_6_ref_0: &gomoku::GameState = &mut gamestate_6;
    let mut gamestate_4_ref_0: &connect_four::GameState = &mut gamestate_4;
    let mut bool_1: bool = std::cmp::PartialEq::ne(gamestate_4_ref_0, gamestate_0_ref_0);
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut player_3_ref_0: &connect_four::Player = &mut player_3;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_3_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_314() {
//    rusty_monitor::set_test_id(314);
    let mut usize_0: usize = 94usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_0);
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_0_ref_0: &crate::connect_four::Column = &mut column_0;
    let mut usize_1: usize = 1usize;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_7, column_6, column_5, column_4, column_3, column_2, column_1];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_6, status: gamestate_0};
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut option_9: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_8: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_8_ref_0: &crate::connect_four::Column = &mut column_8;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(column_8_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(column_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1265() {
//    rusty_monitor::set_test_id(1265);
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_0_ref_0: &crate::connect_four::Column = &mut column_0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut usize_0: usize = 3usize;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_10, option_9, option_8, option_7, option_6, option_5];
    let mut column_7: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_7_ref_0: &crate::connect_four::Column = &mut column_7;
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_8_ref_0: &crate::connect_four::Column = &mut column_8;
    let mut bool_0: bool = std::cmp::PartialEq::eq(column_8_ref_0, column_7_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1929() {
//    rusty_monitor::set_test_id(1929);
    let mut usize_0: usize = 2usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut usize_1: usize = 6usize;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_0: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_1};
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut usize_2: usize = 3usize;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_7: connect_four::Player = crate::connect_four::Player::other(player_6);
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_9: connect_four::Player = crate::connect_four::Player::other(player_8);
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_2: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_2};
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut usize_3: usize = 8usize;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::Some(player_13);
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_5: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_3};
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_1, status: gamestate_0};
    let mut connectfour_0_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut player_18: reversi::Player = crate::reversi::Player::other(player_17);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_25, option_24, option_23, option_22, option_21, option_20, option_19, option_18];
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut player_21: reversi::Player = crate::reversi::Player::other(player_20);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut player_23: reversi::Player = crate::reversi::Player::Player0;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_33, option_32, option_31, option_30, option_29, option_28, option_27, option_26];
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut player_29: reversi::Player = crate::reversi::Player::other(player_28);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut player_31: reversi::Player = crate::reversi::Player::other(player_30);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_41, option_40, option_39, option_38, option_37, option_36, option_35, option_34];
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut player_33: reversi::Player = crate::reversi::Player::other(player_32);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_35: reversi::Player = crate::reversi::Player::Player1;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_49, option_48, option_47, option_46, option_45, option_44, option_43, option_42];
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_36: reversi::Player = crate::reversi::Player::Player0;
    let mut player_37: reversi::Player = crate::reversi::Player::other(player_36);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut player_39: reversi::Player = crate::reversi::Player::other(player_38);
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_40: reversi::Player = crate::reversi::Player::Player1;
    let mut player_41: reversi::Player = crate::reversi::Player::other(player_40);
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_57, option_56, option_55, option_54, option_53, option_52, option_51, option_50];
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_42: reversi::Player = crate::reversi::Player::Player1;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_43: reversi::Player = crate::reversi::Player::Player0;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut player_44: reversi::Player = crate::reversi::Player::Player0;
    let mut player_45: reversi::Player = crate::reversi::Player::other(player_44);
    let mut option_64: std::option::Option<reversi::Player> = std::option::Option::Some(player_45);
    let mut player_46: reversi::Player = crate::reversi::Player::Player1;
    let mut option_65: std::option::Option<reversi::Player> = std::option::Option::Some(player_46);
    let mut option_array_8: [std::option::Option<reversi::Player>; 8] = [option_65, option_64, option_63, option_62, option_61, option_60, option_59, option_58];
    let mut player_47: reversi::Player = crate::reversi::Player::Player1;
    let mut option_66: std::option::Option<reversi::Player> = std::option::Option::Some(player_47);
    let mut option_67: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_48: reversi::Player = crate::reversi::Player::Player1;
    let mut option_68: std::option::Option<reversi::Player> = std::option::Option::Some(player_48);
    let mut player_49: reversi::Player = crate::reversi::Player::Player1;
    let mut option_69: std::option::Option<reversi::Player> = std::option::Option::Some(player_49);
    let mut option_70: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_71: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_50: reversi::Player = crate::reversi::Player::Player1;
    let mut player_51: reversi::Player = crate::reversi::Player::other(player_50);
    let mut option_72: std::option::Option<reversi::Player> = std::option::Option::Some(player_51);
    let mut option_73: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_9: [std::option::Option<reversi::Player>; 8] = [option_73, option_72, option_71, option_70, option_69, option_68, option_67, option_66];
    let mut option_74: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_75: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_52: reversi::Player = crate::reversi::Player::Player0;
    let mut player_53: reversi::Player = crate::reversi::Player::other(player_52);
    let mut option_76: std::option::Option<reversi::Player> = std::option::Option::Some(player_53);
    let mut option_77: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_78: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_54: reversi::Player = crate::reversi::Player::Player1;
    let mut option_79: std::option::Option<reversi::Player> = std::option::Option::Some(player_54);
    let mut option_80: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut result_0: std::result::Result<(), connect_four::ConnectFourError> = crate::connect_four::ConnectFour::put(connectfour_0_ref_0, player_0, usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_383() {
//    rusty_monitor::set_test_id(383);
    let mut usize_0: usize = 3usize;
    let mut usize_1: usize = 84usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
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
    let mut option_0: std::option::Option<connect_four::Player> = crate::connect_four::ConnectFour::winner(connectfour_0_ref_0);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
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
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_14: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_1};
    let mut column_14_ref_0: &crate::connect_four::Column = &mut column_14;
    let mut usize_2: usize = 15usize;
    let mut column_15: crate::connect_four::Column = std::default::Default::default();
    let mut column_15_ref_0: &crate::connect_four::Column = &mut column_15;
    let mut option_6: &std::option::Option<connect_four::Player> = std::ops::Index::index(column_15_ref_0, usize_2);
    let mut option_7: &std::option::Option<connect_four::Player> = std::ops::Index::index(column_14_ref_0, usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8704() {
//    rusty_monitor::set_test_id(8704);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut usize_0: usize = 4usize;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_1: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_2: crate::connect_four::Column = std::default::Default::default();
    let mut usize_1: usize = 7usize;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut column_3: crate::connect_four::Column = crate::connect_four::Column {column: option_array_1, occupied: usize_1};
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut usize_2: usize = 5usize;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut column_5: crate::connect_four::Column = crate::connect_four::Column {column: option_array_2, occupied: usize_2};
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_6, column_5, column_4, column_3, column_2, column_1, column_0];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_0, status: gamestate_0};
    let mut connectfour_0_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_0;
    crate::connect_four::ConnectFour::check_state(connectfour_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_387() {
//    rusty_monitor::set_test_id(387);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut usize_0: usize = 6usize;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_1);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_2);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
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
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_7: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_7_ref_0: &mut crate::connect_four::Column = &mut column_7;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_8_ref_0: &mut crate::connect_four::Column = &mut column_8;
    crate::connect_four::Column::push(column_8_ref_0, player_6);
    crate::connect_four::Column::push(column_7_ref_0, player_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1176() {
//    rusty_monitor::set_test_id(1176);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut column_0: crate::connect_four::Column = std::default::Default::default();
    let mut column_1: crate::connect_four::Column = std::default::Default::default();
    let mut usize_0: usize = 7usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut column_2: crate::connect_four::Column = crate::connect_four::Column {column: option_array_0, occupied: usize_0};
    let mut column_3: crate::connect_four::Column = std::default::Default::default();
    let mut column_4: crate::connect_four::Column = std::default::Default::default();
    let mut column_5: crate::connect_four::Column = std::default::Default::default();
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut usize_1: usize = 7usize;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut column_6: crate::connect_four::Column = std::default::Default::default();
    let mut column_7: crate::connect_four::Column = std::default::Default::default();
    let mut column_8: crate::connect_four::Column = std::default::Default::default();
    let mut column_9: crate::connect_four::Column = std::default::Default::default();
    let mut column_10: crate::connect_four::Column = std::default::Default::default();
    let mut column_11: crate::connect_four::Column = std::default::Default::default();
    let mut column_12: crate::connect_four::Column = std::default::Default::default();
    let mut column_array_0: [crate::connect_four::Column; 7] = [column_12, column_11, column_10, column_9, column_8, column_7, column_6];
    let mut connectfour_0: crate::connect_four::ConnectFour = crate::connect_four::ConnectFour {board: column_array_0, next: player_10, status: gamestate_1};
    let mut connectfour_0_ref_0: &mut crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut result_0: std::result::Result<(), connect_four::ConnectFourError> = crate::connect_four::ConnectFour::put(connectfour_0_ref_0, player_9, usize_1);
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_370() {
//    rusty_monitor::set_test_id(370);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_1_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_1;
    let mut connectfourerror_2: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_2_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_2;
    let mut connectfourerror_3: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_3_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_3;
    let mut connectfourerror_4: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_4_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_4;
    let mut connectfourerror_5: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_5_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_5;
    let mut connectfourerror_6: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_6_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_6;
    let mut connectfourerror_7: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_7_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_7;
    let mut connectfourerror_8: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_8_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_8;
    let mut connectfourerror_9: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_9_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_9;
    let mut connectfourerror_10: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_10_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_10;
    let mut connectfourerror_11: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_11_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_11;
    let mut connectfourerror_12: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_12_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_12;
    let mut connectfourerror_13: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_13_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_13;
    let mut bool_0: bool = std::cmp::PartialEq::eq(connectfourerror_13_ref_0, connectfourerror_12_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(connectfourerror_11_ref_0, connectfourerror_10_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(connectfourerror_9_ref_0, connectfourerror_8_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(connectfourerror_7_ref_0, connectfourerror_6_ref_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(connectfourerror_5_ref_0, connectfourerror_4_ref_0);
    let mut bool_5: bool = std::cmp::PartialEq::eq(connectfourerror_3_ref_0, connectfourerror_2_ref_0);
    let mut bool_6: bool = std::cmp::PartialEq::eq(connectfourerror_1_ref_0, connectfourerror_0_ref_0);
//    panic!("From RustyUnit with love");
}
}