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
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use snafu::ErrorCompat;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2989() {
    rusty_monitor::set_test_id(2989);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_4, option_3, option_2];
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_7, option_6, option_5];
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut player_9_ref_0: &tictactoe::Player = &mut player_9;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut option_8: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_1_ref_0);
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut tictactoe_0: crate::tictactoe::TicTacToe = std::result::Result::unwrap(result_0);
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_4: tictactoe::GameState = std::clone::Clone::clone(gamestate_2_ref_0);
    let mut player_10: tictactoe::Player = std::clone::Clone::clone(player_9_ref_0);
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut bool_0: bool = crate::tictactoe::TicTacToe::is_ended(tictactoe_0_ref_0);
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2096() {
    rusty_monitor::set_test_id(2096);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_0);
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut bool_0: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7729() {
    rusty_monitor::set_test_id(7729);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut player_6: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_0_ref_0: &gomoku::Player = &mut player_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut player_5_ref_0: &reversi::Player = &mut player_5;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_913() {
    rusty_monitor::set_test_id(913);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_0);
    let mut gamestate_0: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut player_5_ref_0: &tictactoe::Player = &mut player_5;
    let mut player_8: tictactoe::Player = std::clone::Clone::clone(player_5_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2601() {
    rusty_monitor::set_test_id(2601);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_3, status: gamestate_4};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_12, option_11, option_10];
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_14);
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_15, option_14, option_13];
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::other(player_16);
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_17);
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_18: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_18);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_9, option_17, option_18];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_11, status: gamestate_5};
    let mut tictactoe_1_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_1;
    crate::tictactoe::TicTacToe::check_state(tictactoe_1_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_0);
    let mut gamestate_6: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut player_13_ref_0: &tictactoe::Player = &mut player_13;
    let mut player_19: tictactoe::Player = std::clone::Clone::clone(player_13_ref_0);
    let mut option_19: std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::winner(tictactoe_0_ref_0);
    let mut player_20: connect_four::Player = crate::connect_four::Player::Player1;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut result_1: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
    let mut tictactoeerror_2: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut bool_2: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7457() {
    rusty_monitor::set_test_id(7457);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 77usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_4, status: gamestate_1};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_0_ref_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(tictactoeerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_839() {
    rusty_monitor::set_test_id(839);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_12, option_11, option_10];
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_15, option_14, option_13];
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::other(player_12);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_0);
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut player_9_ref_0: &tictactoe::Player = &mut player_9;
    let mut player_14: tictactoe::Player = std::clone::Clone::clone(player_9_ref_0);
    let mut option_17: std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::winner(tictactoe_0_ref_0);
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2890() {
    rusty_monitor::set_test_id(2890);
    let mut usize_0: usize = 55usize;
    let mut usize_1: usize = 93usize;
    let mut usize_2: usize = 88usize;
    let mut usize_3: usize = 46usize;
    let mut u64_0: u64 = 3u64;
    let mut u64_1: u64 = 5u64;
    let mut steprng_0: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    let mut gamestate_5: tictactoe::GameState = std::clone::Clone::clone(gamestate_4_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_929() {
    rusty_monitor::set_test_id(929);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_0);
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut connectfour_0_ref_0: &crate::connect_four::ConnectFour = &mut connectfour_0;
    let mut gamestate_2: &connect_four::GameState = crate::connect_four::ConnectFour::status(connectfour_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_330() {
    rusty_monitor::set_test_id(330);
    let mut usize_0: usize = 79usize;
    let mut usize_1: usize = 3usize;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_12, option_11, option_10];
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_15, option_14, option_13];
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::other(player_12);
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_18: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_14);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_9, option_17, option_18];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_7, status: gamestate_1};
    let mut tictactoe_1_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_1;
    crate::tictactoe::TicTacToe::check_state(tictactoe_1_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_0);
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut option_19: &std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::get(tictactoe_0_ref_0, usize_1, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1291() {
    rusty_monitor::set_test_id(1291);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_4, option_3, option_2];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_7, option_6, option_5];
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_10, option_9, option_8];
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_15, option_14, option_13];
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut gomokuerror_1_ref_0: &gomoku::GomokuError = &mut gomokuerror_1;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut result_1: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2774() {
    rusty_monitor::set_test_id(2774);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_3, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_0);
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_208() {
    rusty_monitor::set_test_id(208);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::other(player_11);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_14);
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_16);
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_17);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_19: tictactoe::Player = crate::tictactoe::Player::other(player_18);
    let mut usize_0: usize = 74usize;
    let mut usize_1: usize = 56usize;
    let mut player_20: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_21: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_22: tictactoe::Player = crate::tictactoe::Player::other(player_21);
    let mut player_23: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_18: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_23);
    let mut option_19: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<tictactoe::Player>; 3] = [option_20, option_19, option_18];
    let mut option_21: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_24: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_22: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_24);
    let mut option_23: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<tictactoe::Player>; 3] = [option_23, option_22, option_21];
    let mut player_25: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_26: tictactoe::Player = crate::tictactoe::Player::other(player_25);
    let mut option_24: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_26);
    let mut option_25: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_26: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_8: [std::option::Option<tictactoe::Player>; 3] = [option_26, option_25, option_24];
    let mut option_array_array_2: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_8, option_array_7, option_array_6];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_22, status: gamestate_1};
    let mut tictactoe_1_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut result_0: std::result::Result<(), tictactoe::TicTacToeError> = crate::tictactoe::TicTacToe::place(tictactoe_1_ref_0, player_20, usize_1, usize_0);
    let mut player_27: tictactoe::Player = crate::tictactoe::Player::other(player_19);
    let mut player_27_ref_0: &tictactoe::Player = &mut player_27;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_27_ref_0);
    let mut player_28: gomoku::Player = crate::gomoku::Player::Player1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8370() {
    rusty_monitor::set_test_id(8370);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_0_ref_0: &tictactoe::Player = &mut player_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_3_ref_0: &tictactoe::Player = &mut player_3;
    let mut bool_0: bool = std::cmp::PartialEq::eq(player_3_ref_0, player_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3367() {
    rusty_monitor::set_test_id(3367);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_0);
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2427() {
    rusty_monitor::set_test_id(2427);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_0);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_5, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_0);
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1() {
    rusty_monitor::set_test_id(1);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
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
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_2, status: gamestate_3};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_5);
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3194() {
    rusty_monitor::set_test_id(3194);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1_ref_0: &gomoku::Player = &mut player_1;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_5_ref_0: &minesweeper::GameState = &mut gamestate_5;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut player_0_ref_0: &reversi::Player = &mut player_0;
    let mut player_2_ref_0: &tictactoe::Player = &mut player_2;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7249() {
    rusty_monitor::set_test_id(7249);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 53usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::other(player_12);
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_14);
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_16);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_17);
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_18);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_13, status: gamestate_1};
    let mut tictactoe_1_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut player_19: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_20: tictactoe::Player = crate::tictactoe::Player::other(player_19);
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut option_18: std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::winner(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_357() {
    rusty_monitor::set_test_id(357);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_3, status: gamestate_0};
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_12, option_11, option_10];
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_14);
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_15, option_14, option_13];
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::other(player_16);
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_17);
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_18: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_18);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_9, option_17, option_18];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_11, status: gamestate_1};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_0);
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_327() {
    rusty_monitor::set_test_id(327);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_3, status: gamestate_1};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_0);
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut gamestate_3: tictactoe::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_944() {
    rusty_monitor::set_test_id(944);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_0: usize = 65usize;
    let mut usize_1: usize = 1usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut u64_0: u64 = 38u64;
    let mut u64_1: u64 = 79u64;
    let mut steprng_0: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_0;
    let mut usize_2: usize = 4usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut result_0: std::result::Result<crate::minesweeper::Minesweeper, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::new(usize_0, usize_2, usize_1, steprng_0_ref_0);
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_26() {
    rusty_monitor::set_test_id(26);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_1: &tictactoe::GameState = crate::tictactoe::TicTacToe::status(tictactoe_0_ref_0);
    let mut option_9: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut result_1: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_32() {
    rusty_monitor::set_test_id(32);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut bool_0: bool = true;
    let mut usize_0: usize = 30usize;
    let mut usize_1: usize = 16usize;
    let mut usize_2: usize = 35usize;
    let mut usize_3: usize = 56usize;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_6_ref_0: &tictactoe::Player = &mut player_6;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut option_9: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
    let mut player_7: gomoku::Player = crate::gomoku::Player::Player0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_6_ref_0);
    let mut player_8: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut gamestate_1: &tictactoe::GameState = crate::tictactoe::TicTacToe::status(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1279() {
    rusty_monitor::set_test_id(1279);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut player_8_ref_0: &tictactoe::Player = &mut player_8;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut tictactoeerror_2: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_2_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_2;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut option_9: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_2_ref_0);
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut tictactoe_0: crate::tictactoe::TicTacToe = std::result::Result::unwrap(result_0);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_3: tictactoe::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut player_9: tictactoe::Player = std::clone::Clone::clone(player_8_ref_0);
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut bool_0: bool = crate::tictactoe::TicTacToe::is_ended(tictactoe_0_ref_0);
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1536() {
    rusty_monitor::set_test_id(1536);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut u64_0: u64 = 36u64;
    let mut u64_1: u64 = 97u64;
    let mut steprng_0: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_0;
    let mut usize_0: usize = 97usize;
    let mut usize_1: usize = 2usize;
    let mut usize_2: usize = 9usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut result_0: std::result::Result<crate::minesweeper::Minesweeper, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::new(usize_2, usize_1, usize_0, steprng_0_ref_0);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::InProgress;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3250() {
    rusty_monitor::set_test_id(3250);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_7, status: gamestate_5};
    let mut tictactoe_1_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut option_18: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut gamestate_6: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::other(player_14);
    let mut option_19: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_20: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_16);
    let mut option_21: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<tictactoe::Player>; 3] = [option_21, option_20, option_19];
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_22: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_23: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_18);
    let mut option_24: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<tictactoe::Player>; 3] = [option_24, option_23, option_22];
    let mut player_19: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_25: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_19);
    let mut player_20: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_21: tictactoe::Player = crate::tictactoe::Player::other(player_20);
    let mut option_26: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_21);
    let mut player_22: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_27: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_22);
    let mut option_array_8: [std::option::Option<tictactoe::Player>; 3] = [option_18, option_26, option_27];
    let mut option_array_array_2: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_8, option_array_7, option_array_6];
    let mut tictactoe_2: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_2, next: player_15, status: gamestate_6};
    let mut tictactoe_2_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_2;
    crate::tictactoe::TicTacToe::check_state(tictactoe_2_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_0);
    let mut gamestate_7: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut player_17_ref_0: &tictactoe::Player = &mut player_17;
    let mut player_23: tictactoe::Player = std::clone::Clone::clone(player_17_ref_0);
    let mut option_28: std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::winner(tictactoe_1_ref_0);
    let mut player_24: connect_four::Player = crate::connect_four::Player::Player1;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_4_ref_0, gamestate_3_ref_0);
    let mut result_1: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut tictactoe_3: crate::tictactoe::TicTacToe = std::clone::Clone::clone(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_566() {
    rusty_monitor::set_test_id(566);
    let mut usize_0: usize = 53usize;
    let mut usize_1: usize = 0usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::other(player_11);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::other(player_13);
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::other(player_14);
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_19: tictactoe::Player = crate::tictactoe::Player::other(player_18);
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_19);
    let mut player_20: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_20);
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut player_21: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_21);
    let mut player_22: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_22);
    let mut player_23: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_23);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut player_24: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_24);
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_25: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_25);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_17, status: gamestate_2};
    let mut tictactoe_1_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_26: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_18: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_27: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_20: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_16);
    let mut option_array_6: [std::option::Option<tictactoe::Player>; 3] = [option_20, option_19, option_18];
    let mut option_21: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_28: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_22: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_28);
    let mut player_29: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_23: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_29);
    let mut option_array_7: [std::option::Option<tictactoe::Player>; 3] = [option_23, option_22, option_21];
    let mut option_24: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_30: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_25: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_30);
    let mut option_26: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_8: [std::option::Option<tictactoe::Player>; 3] = [option_26, option_25, option_24];
    let mut option_27: &std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::get(tictactoe_0_ref_0, usize_1, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_42() {
    rusty_monitor::set_test_id(42);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut usize_0: usize = 74usize;
    let mut usize_1: usize = 56usize;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_4, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut result_0: std::result::Result<(), tictactoe::TicTacToeError> = crate::tictactoe::TicTacToe::place(tictactoe_0_ref_0, player_2, usize_1, usize_0);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut player_9_ref_0: &tictactoe::Player = &mut player_9;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_9_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3079() {
    rusty_monitor::set_test_id(3079);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::other(player_13);
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_14);
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_8, status: gamestate_1};
    let mut tictactoe_1_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_1;
    crate::tictactoe::TicTacToe::check_state(tictactoe_1_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_0);
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut player_16: tictactoe::Player = crate::tictactoe::TicTacToe::get_next_player(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6604() {
    rusty_monitor::set_test_id(6604);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_2, status: gamestate_1};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2775() {
    rusty_monitor::set_test_id(2775);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_4, option_3, option_2];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_7, option_6, option_5];
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_14);
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_16);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_16, option_17];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_4, option_array_3, option_array_2];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_10, status: gamestate_5};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_0);
    let mut gamestate_6: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut player_12_ref_0: &tictactoe::Player = &mut player_12;
    let mut player_17: tictactoe::Player = std::clone::Clone::clone(player_12_ref_0);
    let mut player_18: connect_four::Player = crate::connect_four::Player::Player1;
    let mut result_1: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}
}