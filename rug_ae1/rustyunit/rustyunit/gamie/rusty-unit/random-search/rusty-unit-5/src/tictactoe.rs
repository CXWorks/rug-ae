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
fn rusty_test_1592() {
    rusty_monitor::set_test_id(1592);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut usize_0: usize = 45usize;
    let mut usize_1: usize = 24usize;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_1_ref_0: &tictactoe::Player = &mut player_1;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_2: usize = 35usize;
    let mut bool_2: bool = false;
    let mut usize_3: usize = 25usize;
    let mut usize_4: usize = 32usize;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut tictactoeerror_2: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_2_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_2;
    let mut tictactoeerror_3: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_3_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_3;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_3_ref_0);
    let mut backtrace_0: &snafu::Backtrace = std::option::Option::unwrap(option_0);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_3);
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_2, is_revealed: bool_1, is_flagged: bool_0};
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_1_ref_0: &gomoku::GomokuError = &mut gomokuerror_1;
    let mut bool_3: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4238() {
    rusty_monitor::set_test_id(4238);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 42usize;
    let mut bool_2: bool = true;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_11: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_12: gomoku::Player = crate::gomoku::Player::other(player_11);
    let mut player_12_ref_0: &gomoku::Player = &mut player_12;
    let mut gamestate_1: &tictactoe::GameState = crate::tictactoe::TicTacToe::status(tictactoe_0_ref_0);
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2921() {
    rusty_monitor::set_test_id(2921);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut usize_0: usize = 54usize;
    let mut usize_1: usize = 15usize;
    let mut usize_2: usize = 60usize;
    let mut usize_3: usize = 62usize;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_6, status: gamestate_2};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut result_0: std::result::Result<(), tictactoe::TicTacToeError> = crate::tictactoe::TicTacToe::place(tictactoe_0_ref_0, player_3, usize_3, usize_2);
    let mut result_1: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut reversi_0: crate::reversi::Reversi = std::result::Result::unwrap(result_1);
    let mut player_12_ref_0: &connect_four::Player = &mut player_12;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut result_2: std::result::Result<(), reversi::ReversiError> = crate::reversi::Reversi::check_position_validity(reversi_0_ref_0, usize_1, usize_0, player_1);
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut player_13: reversi::Player = crate::reversi::Player::Player0;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4525() {
    rusty_monitor::set_test_id(4525);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut player_10: tictactoe::Player = crate::tictactoe::TicTacToe::get_next_player(tictactoe_0_ref_0);
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4124() {
    rusty_monitor::set_test_id(4124);
    let mut usize_0: usize = 88usize;
    let mut usize_1: usize = 44usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut option_9: &std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::get(tictactoe_0_ref_0, usize_1, usize_0);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2988() {
    rusty_monitor::set_test_id(2988);
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut player_1_ref_0: &gomoku::Player = &mut player_1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_6, status: gamestate_1};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut player_8: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_9: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    let mut player_8_ref_0: &gomoku::Player = &mut player_8;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2147() {
    rusty_monitor::set_test_id(2147);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_9: gomoku::Player = crate::gomoku::Player::Player1;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(tictactoeerror_0_ref_0);
    let mut player_10: gomoku::Player = crate::gomoku::Player::other(player_9);
    let mut player_11: tictactoe::Player = crate::tictactoe::TicTacToe::get_next_player(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2268() {
    rusty_monitor::set_test_id(2268);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_1_ref_0: &tictactoe::Player = &mut player_1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut player_3_ref_0: &tictactoe::Player = &mut player_3;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut tictactoeerror_2: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_2_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_2;
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_5: gomoku::Player = crate::gomoku::Player::other(player_4);
    let mut usize_0: usize = 64usize;
    let mut usize_1: usize = 63usize;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_6);
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut player_7: gomoku::Player = crate::gomoku::Player::other(player_5);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut bool_1: bool = std::cmp::PartialEq::eq(tictactoeerror_2_ref_0, tictactoeerror_1_ref_0);
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut bool_2: bool = std::cmp::PartialEq::eq(player_3_ref_0, player_1_ref_0);
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1847() {
    rusty_monitor::set_test_id(1847);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_2, status: gamestate_2};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut bool_0: bool = crate::tictactoe::TicTacToe::is_ended(tictactoe_0_ref_0);
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1593() {
    rusty_monitor::set_test_id(1593);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut usize_0: usize = 18usize;
    let mut usize_1: usize = 63usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1_ref_0: &reversi::Player = &mut player_1;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2_ref_0: &reversi::Player = &mut player_2;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut player_3_ref_0: &reversi::Player = &mut player_3;
    let mut usize_2: usize = 18usize;
    let mut usize_3: usize = 69usize;
    let mut usize_4: usize = 64usize;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player1;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut result_1: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut tictactoeerror_2: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_3: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_2_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_2;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_2_ref_0);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_1);
    let mut bool_0: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
    let mut player_6: gomoku::Player = crate::gomoku::Player::Player1;
    let mut bool_1: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1679() {
    rusty_monitor::set_test_id(1679);
    let mut usize_0: usize = 29usize;
    let mut usize_1: usize = 1usize;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_1_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_1;
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7_ref_0: &reversi::Player = &mut player_7;
    let mut player_8_ref_0: &tictactoe::Player = &mut player_8;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_8_ref_0);
    let mut tictactoe_1: crate::tictactoe::TicTacToe = std::clone::Clone::clone(tictactoe_0_ref_0);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut tictactoe_1_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut option_9: &std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::get(tictactoe_1_ref_0, usize_1, usize_0);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1658() {
    rusty_monitor::set_test_id(1658);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_11);
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut bool_0: bool = false;
    let mut usize_0: usize = 9usize;
    let mut usize_1: usize = 90usize;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_2_ref_0);
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3220() {
    rusty_monitor::set_test_id(3220);
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player1;
    let mut usize_0: usize = 98usize;
    let mut usize_1: usize = 7usize;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_5, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut player_12: reversi::Player = crate::reversi::Player::other(player_3);
    let mut player_13: gomoku::Player = crate::gomoku::Player::other(player_2);
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::Win(player_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3651() {
    rusty_monitor::set_test_id(3651);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3891() {
    rusty_monitor::set_test_id(3891);
    let mut usize_0: usize = 8usize;
    let mut usize_1: usize = 76usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut bool_0: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
    let mut result_0: std::result::Result<(), tictactoe::TicTacToeError> = crate::tictactoe::TicTacToe::place(tictactoe_0_ref_0, player_0, usize_1, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1295() {
    rusty_monitor::set_test_id(1295);
    let mut bool_0: bool = false;
    let mut usize_0: usize = 64usize;
    let mut usize_1: usize = 31usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3641() {
    rusty_monitor::set_test_id(3641);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_11_ref_0: &tictactoe::Player = &mut player_11;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::other(player_12);
    let mut player_13_ref_0: &tictactoe::Player = &mut player_13;
    let mut player_14: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut usize_0: usize = 11usize;
    let mut usize_1: usize = 2usize;
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_17);
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_19: tictactoe::Player = crate::tictactoe::Player::other(player_18);
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_19);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_20: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_21: tictactoe::Player = crate::tictactoe::Player::other(player_20);
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_21);
    let mut player_22: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_22);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut player_23: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_24: tictactoe::Player = crate::tictactoe::Player::other(player_23);
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_24);
    let mut player_25: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_26: tictactoe::Player = crate::tictactoe::Player::other(player_25);
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_26);
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_16, status: gamestate_2};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_27: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_27);
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut result_0: std::result::Result<(), tictactoe::TicTacToeError> = crate::tictactoe::TicTacToe::place(tictactoe_0_ref_0, player_15, usize_1, usize_0);
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_3_ref_0, gamestate_1_ref_0);
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Win(player_14);
    let mut bool_1: bool = std::cmp::PartialEq::eq(player_13_ref_0, player_11_ref_0);
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut option_18: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
    let mut player_28: gomoku::Player = crate::gomoku::Player::Player0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4515() {
    rusty_monitor::set_test_id(4515);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut player_6_ref_0: &tictactoe::Player = &mut player_6;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_0: usize = 52usize;
    let mut usize_1: usize = 87usize;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_6_ref_0);
    let mut player_7: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_4: tictactoe::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut option_9: std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::winner(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2018() {
    rusty_monitor::set_test_id(2018);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut usize_0: usize = 20usize;
    let mut usize_1: usize = 84usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 39usize;
    let mut usize_3: usize = 98usize;
    let mut option_9: std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::winner(tictactoe_0_ref_0);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2635() {
    rusty_monitor::set_test_id(2635);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::other(player_11);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player1;
    let mut usize_0: usize = 24usize;
    let mut usize_1: usize = 8usize;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_2: usize = 7usize;
    let mut usize_3: usize = 46usize;
    let mut usize_4: usize = 72usize;
    let mut isize_0: isize = 83isize;
    let mut isize_1: isize = -100isize;
    let mut isize_2: isize = 55isize;
    let mut isize_3: isize = 0isize;
    let mut tuple_0: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = -8isize;
    let mut isize_5: isize = 93isize;
    let mut tuple_1: (isize, isize) = (isize_5, isize_4);
    let mut isize_6: isize = -9isize;
    let mut isize_7: isize = 40isize;
    let mut tuple_2: (isize, isize) = (isize_7, isize_6);
    let mut isize_8: isize = -13isize;
    let mut isize_9: isize = -265isize;
    let mut tuple_3: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = -55isize;
    let mut isize_11: isize = 105isize;
    let mut tuple_4: (isize, isize) = (isize_11, isize_10);
    let mut isize_12: isize = -35isize;
    let mut isize_13: isize = -176isize;
    let mut tuple_5: (isize, isize) = (isize_13, isize_12);
    let mut isize_14: isize = 75isize;
    let mut isize_15: isize = 118isize;
    let mut tuple_6: (isize, isize) = (isize_15, isize_14);
    let mut isize_16: isize = 48isize;
    let mut isize_17: isize = 1isize;
    let mut tuple_7: (isize, isize) = (isize_17, isize_16);
    let mut tuple_array_0: [(isize, isize); 8] = [tuple_7, tuple_6, tuple_5, tuple_4, tuple_3, tuple_2, tuple_1, tuple_0];
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::other(player_14);
    let mut player_16_ref_0: &tictactoe::Player = &mut player_16;
    let mut player_17: tictactoe::Player = std::clone::Clone::clone(player_16_ref_0);
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_18: connect_four::Player = crate::connect_four::Player::other(player_13);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1797() {
    rusty_monitor::set_test_id(1797);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_5, status: gamestate_2};
    let mut tictactoe_1_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_1;
    crate::tictactoe::TicTacToe::check_state(tictactoe_1_ref_0);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_5: tictactoe::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut bool_0: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4962() {
    rusty_monitor::set_test_id(4962);
    let mut usize_0: usize = 87usize;
    let mut usize_1: usize = 34usize;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_8);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut option_9: &std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::get(tictactoe_0_ref_0, usize_1, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1454() {
    rusty_monitor::set_test_id(1454);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut usize_0: usize = 68usize;
    let mut usize_1: usize = 37usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_3, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut result_0: std::result::Result<(), tictactoe::TicTacToeError> = crate::tictactoe::TicTacToe::place(tictactoe_0_ref_0, player_1, usize_1, usize_0);
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(tictactoeerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5007() {
    rusty_monitor::set_test_id(5007);
    let mut usize_0: usize = 26usize;
    let mut usize_1: usize = 32usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
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
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut usize_2: usize = 97usize;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut option_9: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
    let mut backtrace_0: &snafu::Backtrace = std::option::Option::unwrap(option_9);
    let mut result_0: std::result::Result<(), tictactoe::TicTacToeError> = crate::tictactoe::TicTacToe::place(tictactoe_0_ref_0, player_0, usize_1, usize_0);
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut result_1: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gomoku_0: crate::gomoku::Gomoku = std::result::Result::unwrap(result_1);
    let mut result_2: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_890() {
    rusty_monitor::set_test_id(890);
    let mut usize_0: usize = 4usize;
    let mut usize_1: usize = 39usize;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_7_ref_0: &connect_four::Player = &mut player_7;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_6);
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut bool_0: bool = crate::tictactoe::TicTacToe::is_ended(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3141() {
    rusty_monitor::set_test_id(3141);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 78usize;
    let mut bool_2: bool = true;
    let mut usize_1: usize = 9usize;
    let mut usize_2: usize = 83usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut bool_3: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut result_0: std::result::Result<(), tictactoe::TicTacToeError> = crate::tictactoe::TicTacToe::place(tictactoe_0_ref_0, player_0, usize_2, usize_1);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3024() {
    rusty_monitor::set_test_id(3024);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut usize_0: usize = 6usize;
    let mut usize_1: usize = 42usize;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut tictactoe_1: crate::tictactoe::TicTacToe = std::clone::Clone::clone(tictactoe_0_ref_0);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3388() {
    rusty_monitor::set_test_id(3388);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_0: usize = 23usize;
    let mut usize_1: usize = 13usize;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_2: usize = 13usize;
    let mut bool_2: bool = false;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_2, is_revealed: bool_1, is_flagged: bool_0};
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::InProgress;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut player_10: gomoku::Player = crate::gomoku::Player::Player1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3153() {
    rusty_monitor::set_test_id(3153);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
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
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut usize_0: usize = 75usize;
    let mut usize_1: usize = 29usize;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::other(player_12);
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::other(player_14);
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_16);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_8, status: gamestate_1};
    let mut tictactoe_1_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut result_0: std::result::Result<(), tictactoe::TicTacToeError> = crate::tictactoe::TicTacToe::place(tictactoe_1_ref_0, player_6, usize_1, usize_0);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut bool_0: bool = crate::tictactoe::TicTacToe::is_ended(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4582() {
    rusty_monitor::set_test_id(4582);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_8, status: gamestate_1};
    let mut tictactoe_1_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_13);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut usize_0: usize = 17usize;
    let mut player_14: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_15: connect_four::Player = crate::connect_four::Player::other(player_14);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut usize_1: usize = 25usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_1, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gomoku_0: crate::gomoku::Gomoku = std::result::Result::unwrap(result_0);
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut gamestate_4: tictactoe::GameState = std::clone::Clone::clone(gamestate_3_ref_0);
    let mut gomoku_0_ref_0: &crate::gomoku::Gomoku = &mut gomoku_0;
    let mut option_18: std::option::Option<gomoku::Player> = crate::gomoku::Gomoku::winner(gomoku_0_ref_0);
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    let mut bool_5: bool = std::cmp::PartialEq::eq(gamestate_4_ref_0, gamestate_2_ref_0);
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::other(player_12);
    let mut tictactoe_2: crate::tictactoe::TicTacToe = std::clone::Clone::clone(tictactoe_1_ref_0);
    let mut gamestate_5_ref_0: &connect_four::GameState = &mut gamestate_5;
    let mut bool_6: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
    let mut tictactoe_3: crate::tictactoe::TicTacToe = std::clone::Clone::clone(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}
}