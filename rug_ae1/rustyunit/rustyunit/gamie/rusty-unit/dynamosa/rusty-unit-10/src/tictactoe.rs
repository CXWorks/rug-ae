//! Tic-Tac-Toe
//!
//! Check struct [`TicTacToe`](https://docs.rs/gamie/*/gamie/tictactoe/struct.TicTacToe.html) for more information
//!
//! # Examples
//!
//! ```rust
//! use gamie::tictactoe::{TicTacToe, Player as TicTacToePlayer};
//!
//! # fn tictactoe() {
//! let mut game = TicTacToe::new().unwrap();
//!
//! game.place(TicTacToePlayer::Player0, 1, 1).unwrap();
//! game.place(TicTacToePlayer::Player1, 0, 0).unwrap();
//!
//! // ...
//!
//! println!("{:?}", game.status());
//! # }
//! ```

use crate::std_lib::Infallible;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use snafu::Snafu;

/// Tic-Tac-Toe
///
/// Passing an invalid position to a method will cause panic. Check the target position validity first when dealing with user input
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TicTacToe {
    board: [[Option<Player>; 3]; 3],
    next: Player,
    status: GameState,
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

impl TicTacToe {
    /// Create a new Tic-Tac-Toe game
    pub fn new() -> Result<Self, Infallible> {
        Ok(Self {
            board: [[None; 3]; 3],
            next: Player::Player0,
            status: GameState::InProgress,
        })
    }

    /// Get a cell reference from the game board
    /// Panic when target position out of bounds
    pub fn get(&self, row: usize, col: usize) -> &Option<Player> {
        &self.board[row][col]
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

    /// Place a piece on the board
    /// Panic when target position out of bounds
    pub fn place(&mut self, player: Player, row: usize, col: usize) -> Result<(), TicTacToeError> {
        if self.is_ended() {
            return Err(TicTacToeError::GameEnded);
        }

        if player != self.next {
            return Err(TicTacToeError::WrongPlayer);
        }

        if self.board[row][col].is_some() {
            return Err(TicTacToeError::OccupiedPosition);
        }

        self.board[row][col] = Some(player);
        self.next = self.next.other();

        self.check_state();

        Ok(())
    }

    fn check_state(&mut self) {
        for row in 0..3 {
            if self.board[row][0].is_some()
                && self.board[row][0] == self.board[row][1]
                && self.board[row][1] == self.board[row][2]
            {
                self.status = GameState::Win(self.board[row][0].unwrap());
                return;
            }
        }

        for col in 0..3 {
            if self.board[0][col].is_some()
                && self.board[0][col] == self.board[1][col]
                && self.board[1][col] == self.board[2][col]
            {
                self.status = GameState::Win(self.board[0][col].unwrap());
                return;
            }
        }

        if self.board[0][0].is_some()
            && self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
        {
            self.status = GameState::Win(self.board[0][0].unwrap());
            return;
        }

        if self.board[0][0].is_some()
            && self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
        {
            self.status = GameState::Win(self.board[0][2].unwrap());
            return;
        }

        self.status = if self.board.iter().flatten().all(|p| p.is_some()) {
            GameState::Tie
        } else {
            GameState::InProgress
        };
    }
}

/// Errors that can occur when placing a piece on the board
#[derive(Debug, Eq, PartialEq, Snafu)]
pub enum TicTacToeError {
    #[snafu(display("Wrong player"))]
    WrongPlayer,
    #[snafu(display("Occupied position"))]
    OccupiedPosition,
    #[snafu(display("The game was already end"))]
    GameEnded,
}

#[cfg(test)]
mod tests {
    use crate::tictactoe::*;

    #[test]
    fn test() {
        let mut game = TicTacToe::new().unwrap();

        assert_eq!(game.get_next_player(), Player::Player0,);

        assert_eq!(game.place(Player::Player0, 1, 1), Ok(()));

        assert_eq!(game.get_next_player(), Player::Player1,);

        assert_eq!(
            game.place(Player::Player0, 0, 0),
            Err(TicTacToeError::WrongPlayer)
        );

        assert_eq!(game.place(Player::Player1, 1, 0), Ok(()));

        assert_eq!(game.get_next_player(), Player::Player0,);

        assert!(!game.is_ended());

        assert_eq!(
            game.place(Player::Player0, 1, 1),
            Err(TicTacToeError::OccupiedPosition)
        );

        assert_eq!(game.place(Player::Player0, 2, 2), Ok(()));

        assert_eq!(game.status(), &GameState::InProgress);

        assert_eq!(game.place(Player::Player1, 2, 0), Ok(()));

        assert_eq!(game.place(Player::Player0, 0, 0), Ok(()));

        assert!(game.is_ended());

        assert_eq!(game.winner(), Some(Player::Player0));

        assert_eq!(
            game.place(Player::Player0, 0, 2),
            Err(TicTacToeError::GameEnded)
        );

        assert_eq!(game.winner(), Some(Player::Player0));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use snafu::ErrorCompat;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6235() {
    rusty_monitor::set_test_id(6235);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_4, option_3, option_2];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8_ref_0: &tictactoe::Player = &mut player_8;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut player_10_ref_0: &tictactoe::Player = &mut player_10;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_13: connect_four::Player = crate::connect_four::Player::other(player_12);
    let mut player_13_ref_0: &connect_four::Player = &mut player_13;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(player_10_ref_0, player_8_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_15() {
    rusty_monitor::set_test_id(15);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_3, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut usize_0: usize = 51usize;
    let mut usize_1: usize = 84usize;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_12);
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_14);
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_16);
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_17);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_13, status: gamestate_1};
    let mut tictactoe_1_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut option_18: &std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::get(tictactoe_1_ref_0, usize_1, usize_0);
    let mut gamestate_2: &tictactoe::GameState = crate::tictactoe::TicTacToe::status(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2532() {
    rusty_monitor::set_test_id(2532);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_7, option_6, option_5];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7_ref_0: &tictactoe::Player = &mut player_7;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut player_9_ref_0: &tictactoe::Player = &mut player_9;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_12: connect_four::Player = crate::connect_four::Player::other(player_11);
    let mut player_12_ref_0: &connect_four::Player = &mut player_12;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(player_9_ref_0, player_7_ref_0);
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_3: tictactoe::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_298() {
    rusty_monitor::set_test_id(298);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_1_ref_0: &tictactoe::Player = &mut player_1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_3, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::other(player_14);
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::other(player_16);
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_17);
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_19: tictactoe::Player = crate::tictactoe::Player::other(player_18);
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_19);
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_20: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_20);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_12, status: gamestate_1};
    let mut tictactoe_1_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut player_21: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_21_ref_0: &tictactoe::Player = &mut player_21;
    let mut player_22: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_23: tictactoe::Player = crate::tictactoe::Player::other(player_22);
    let mut player_23_ref_0: &tictactoe::Player = &mut player_23;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_24: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_25: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_26: connect_four::Player = crate::connect_four::Player::other(player_25);
    let mut player_26_ref_0: &connect_four::Player = &mut player_26;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(player_23_ref_0, player_21_ref_0);
    let mut bool_2: bool = crate::tictactoe::TicTacToe::is_ended(tictactoe_1_ref_0);
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut player_24_ref_0: &tictactoe::Player = &mut player_24;
    let mut bool_3: bool = std::cmp::PartialEq::eq(player_24_ref_0, player_1_ref_0);
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::InProgress;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1418() {
    rusty_monitor::set_test_id(1418);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9_ref_0: &tictactoe::Player = &mut player_9;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut player_11_ref_0: &tictactoe::Player = &mut player_11;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_14: connect_four::Player = crate::connect_four::Player::other(player_13);
    let mut player_14_ref_0: &connect_four::Player = &mut player_14;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(player_11_ref_0, player_9_ref_0);
    let mut bool_2: bool = crate::tictactoe::TicTacToe::is_ended(tictactoe_0_ref_0);
    let mut option_9: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_52() {
    rusty_monitor::set_test_id(52);
    let mut gamestate_0: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_0_ref_0: &tictactoe::Player = &mut player_0;
    let mut tictactoeerror_2: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_2_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_2;
    let mut tictactoeerror_3: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_3_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_3;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut gamestate_4: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_0_ref_0: &gomoku::GameState = &mut gamestate_0;
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_5_ref_0: &tictactoe::GameState = &mut gamestate_5;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_5_ref_0, gamestate_1_ref_0);
    let mut gamestate_6: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut bool_1: bool = std::cmp::PartialEq::eq(tictactoeerror_3_ref_0, tictactoeerror_2_ref_0);
    let mut gamestate_7: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut bool_2: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8355() {
    rusty_monitor::set_test_id(8355);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7_ref_0: &tictactoe::Player = &mut player_7;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut player_9_ref_0: &tictactoe::Player = &mut player_9;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_12: connect_four::Player = crate::connect_four::Player::other(player_11);
    let mut player_12_ref_0: &connect_four::Player = &mut player_12;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(player_9_ref_0, player_7_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1404() {
    rusty_monitor::set_test_id(1404);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_1_ref_0);
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut option_9: std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::winner(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7195() {
    rusty_monitor::set_test_id(7195);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut isize_0: isize = 15isize;
    let mut isize_1: isize = 97isize;
    let mut tuple_0: (isize, isize) = (isize_1, isize_0);
    let mut isize_2: isize = 108isize;
    let mut isize_3: isize = 90isize;
    let mut tuple_1: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = 19isize;
    let mut isize_5: isize = 64isize;
    let mut tuple_2: (isize, isize) = (isize_5, isize_4);
    let mut isize_6: isize = -69isize;
    let mut isize_7: isize = -26isize;
    let mut tuple_3: (isize, isize) = (isize_7, isize_6);
    let mut isize_8: isize = 157isize;
    let mut isize_9: isize = 159isize;
    let mut tuple_4: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = 101isize;
    let mut isize_11: isize = -131isize;
    let mut tuple_5: (isize, isize) = (isize_11, isize_10);
    let mut isize_12: isize = -88isize;
    let mut isize_13: isize = -76isize;
    let mut tuple_6: (isize, isize) = (isize_13, isize_12);
    let mut isize_14: isize = -24isize;
    let mut isize_15: isize = -114isize;
    let mut tuple_7: (isize, isize) = (isize_15, isize_14);
    let mut tuple_array_0: [(isize, isize); 8] = [tuple_7, tuple_6, tuple_5, tuple_4, tuple_3, tuple_2, tuple_1, tuple_0];
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 4usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut usize_1: usize = 36usize;
    let mut bool_5: bool = false;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_5, mine_adjacent: usize_1, is_revealed: bool_4, is_flagged: bool_3};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut bool_6: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_0_ref_0);
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_38() {
    rusty_monitor::set_test_id(38);
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_10: tictactoe::Player = crate::tictactoe::TicTacToe::get_next_player(tictactoe_0_ref_0);
    let mut player_10_ref_0: &tictactoe::Player = &mut player_10;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_10_ref_0);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::Win(player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_114() {
    rusty_monitor::set_test_id(114);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_2, status: gamestate_2};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut gamestate_3: &tictactoe::GameState = crate::tictactoe::TicTacToe::status(tictactoe_0_ref_0);
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4829() {
    rusty_monitor::set_test_id(4829);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9_ref_0: &tictactoe::Player = &mut player_9;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut player_11_ref_0: &tictactoe::Player = &mut player_11;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_14: connect_four::Player = crate::connect_four::Player::other(player_13);
    let mut player_14_ref_0: &connect_four::Player = &mut player_14;
    let mut bool_0: bool = std::cmp::PartialEq::eq(player_11_ref_0, player_9_ref_0);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut option_9: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_31() {
    rusty_monitor::set_test_id(31);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_5_ref_0: &gomoku::Player = &mut player_5;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_0: usize = 89usize;
    let mut usize_1: usize = 87usize;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7_ref_0: &tictactoe::Player = &mut player_7;
    let mut player_8: tictactoe::Player = std::clone::Clone::clone(player_7_ref_0);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut player_9: tictactoe::Player = crate::tictactoe::TicTacToe::get_next_player(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_786() {
    rusty_monitor::set_test_id(786);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut tictactoeerror_2: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_2_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_2;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut bool_0: bool = std::cmp::PartialEq::eq(tictactoeerror_2_ref_0, tictactoeerror_1_ref_0);
    let mut option_8: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_127() {
    rusty_monitor::set_test_id(127);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut isize_0: isize = 15isize;
    let mut isize_1: isize = 97isize;
    let mut tuple_0: (isize, isize) = (isize_1, isize_0);
    let mut isize_2: isize = 108isize;
    let mut isize_3: isize = 90isize;
    let mut tuple_1: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = 19isize;
    let mut isize_5: isize = 64isize;
    let mut tuple_2: (isize, isize) = (isize_5, isize_4);
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::Win(player_1);
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut player_2_ref_0: &gomoku::Player = &mut player_2;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut gamestate_1_ref_0: &gomoku::GameState = &mut gamestate_1;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut tuple_3: () = std::cmp::Eq::assert_receiver_is_total_eq(tictactoeerror_0_ref_0);
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut gamestate_5: tictactoe::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
    let mut gamestate_6: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut tuple_4: () = std::cmp::Eq::assert_receiver_is_total_eq(tictactoeerror_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_187() {
    rusty_monitor::set_test_id(187);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_0_ref_0: &tictactoe::Player = &mut player_0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut vec_1: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut player_3_ref_0: &tictactoe::Player = &mut player_3;
    let mut bool_0: bool = std::cmp::PartialEq::eq(player_3_ref_0, player_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6267() {
    rusty_monitor::set_test_id(6267);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut usize_0: usize = 20usize;
    let mut usize_1: usize = 36usize;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_6, status: gamestate_1};
    let mut tictactoe_1_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_18: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::other(player_16);
    let mut option_19: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_17);
    let mut option_20: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_19: tictactoe::Player = crate::tictactoe::Player::other(player_13);
    let mut option_21: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_19);
    let mut option_22: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_20: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_21: tictactoe::Player = crate::tictactoe::Player::other(player_20);
    let mut option_23: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_21);
    let mut player_22: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_24: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_22);
    let mut player_23: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_23_ref_0: &tictactoe::Player = &mut player_23;
    let mut player_24: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_25: tictactoe::Player = crate::tictactoe::Player::other(player_24);
    let mut player_25_ref_0: &tictactoe::Player = &mut player_25;
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_5_ref_0: &tictactoe::GameState = &mut gamestate_5;
    let mut gamestate_6: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_26: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_27: connect_four::Player = crate::connect_four::Player::other(player_26);
    let mut player_27_ref_0: &connect_four::Player = &mut player_27;
    let mut gamestate_6_ref_0: &tictactoe::GameState = &mut gamestate_6;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_6_ref_0, gamestate_5_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(player_25_ref_0, player_23_ref_0);
    let mut option_25: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::ne(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut result_0: std::result::Result<(), tictactoe::TicTacToeError> = crate::tictactoe::TicTacToe::place(tictactoe_1_ref_0, player_5, usize_1, usize_0);
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_933() {
    rusty_monitor::set_test_id(933);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::Win(player_1);
    let mut gamestate_1_ref_0: &gomoku::GameState = &mut gamestate_1;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_87() {
    rusty_monitor::set_test_id(87);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut player_4_ref_0: &reversi::Player = &mut player_4;
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_5);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut player_7: reversi::Player = crate::reversi::Player::other(player_6);
    let mut player_7_ref_0: &reversi::Player = &mut player_7;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut reversi_0: crate::reversi::Reversi = std::result::Result::unwrap(result_0);
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut reversi_0_ref_0: &mut crate::reversi::Reversi = &mut reversi_0;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1699() {
    rusty_monitor::set_test_id(1699);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut option_9: std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::winner(tictactoe_0_ref_0);
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_13, option_12, option_11];
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_16, option_15, option_14];
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_18: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_19, option_18, option_17];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_5, status: gamestate_1};
    let mut tictactoe_1_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut option_20: std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::winner(tictactoe_1_ref_0);
    let mut option_array_6: [std::option::Option<tictactoe::Player>; 3] = [option_20, option_10, option_9];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_21: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_22: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_23: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<tictactoe::Player>; 3] = [option_23, option_22, option_21];
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::other(player_12);
    let mut option_24: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_25: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_14);
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_26: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut option_array_8: [std::option::Option<tictactoe::Player>; 3] = [option_26, option_25, option_24];
    let mut option_27: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::other(player_16);
    let mut option_28: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_17);
    let mut option_29: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_9: [std::option::Option<tictactoe::Player>; 3] = [option_29, option_28, option_27];
    let mut option_array_array_2: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_6, option_array_8, option_array_7];
    let mut tictactoe_2: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_2, next: player_9, status: gamestate_2};
    let mut tictactoe_2_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_2;
    let mut player_18: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_3_ref_0);
    let mut gamestate_4: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut player_19: connect_four::Player = crate::connect_four::Player::other(player_18);
    let mut player_19_ref_0: &connect_four::Player = &mut player_19;
    let mut option_30: std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::winner(tictactoe_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1623() {
    rusty_monitor::set_test_id(1623);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_6, option_5, option_4];
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_1_ref_0);
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut player_12: connect_four::Player = crate::connect_four::Player::other(player_11);
    let mut player_12_ref_0: &connect_four::Player = &mut player_12;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_201() {
    rusty_monitor::set_test_id(201);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_0_ref_0: &tictactoe::Player = &mut player_0;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_2: gomoku::Player = crate::gomoku::Player::other(player_1);
    let mut gamestate_0: gomoku::GameState = crate::gomoku::GameState::Win(player_2);
    let mut gamestate_0_ref_0: &gomoku::GameState = &mut gamestate_0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_1_ref_0: &gomoku::GameState = &mut gamestate_1;
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::Win(player_4);
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut player_5_ref_0: &gomoku::Player = &mut player_5;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::Win(player_3);
    let mut gamestate_2_ref_0: &gomoku::GameState = &mut gamestate_2;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(tictactoeerror_1_ref_0);
    let mut gamestate_5_ref_0: &tictactoe::GameState = &mut gamestate_5;
    let mut gamestate_6: tictactoe::GameState = std::clone::Clone::clone(gamestate_5_ref_0);
    let mut gamestate_7: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_6: gomoku::Player = crate::gomoku::Player::Player0;
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(tictactoeerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2184() {
    rusty_monitor::set_test_id(2184);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9_ref_0: &tictactoe::Player = &mut player_9;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut player_11_ref_0: &tictactoe::Player = &mut player_11;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player0;
    let mut bool_0: bool = std::cmp::PartialEq::eq(player_11_ref_0, player_9_ref_0);
    let mut bool_1: bool = crate::tictactoe::TicTacToe::is_ended(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3458() {
    rusty_monitor::set_test_id(3458);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 34usize;
    let mut bool_2: bool = false;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::Win(player_1);
    let mut gamestate_1_ref_0: &gomoku::GameState = &mut gamestate_1;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3_ref_0: &tictactoe::Player = &mut player_3;
    let mut player_5: tictactoe::Player = std::clone::Clone::clone(player_3_ref_0);
    panic!("From RustyUnit with love");
}
}