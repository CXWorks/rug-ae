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
fn rusty_test_1811() {
    rusty_monitor::set_test_id(1811);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player0;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut tictactoeerror_2: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut player_7: gomoku::Player = crate::gomoku::Player::other(player_1);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut player_8: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_7_ref_0: &gomoku::Player = &mut player_7;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5894() {
    rusty_monitor::set_test_id(5894);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_11);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_13: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_14);
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut gamestate_4: tictactoe::GameState = std::clone::Clone::clone(gamestate_3_ref_0);
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_5: gomoku::GameState = crate::gomoku::GameState::Win(player_13);
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut option_9: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_1_ref_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(tictactoeerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_760() {
    rusty_monitor::set_test_id(760);
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_2);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player1;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut bool_0: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
    let mut player_4: gomoku::Player = crate::gomoku::Player::other(player_3);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut bool_1: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_4_ref_0: &gomoku::Player = &mut player_4;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_7: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    let mut gamestate_6: tictactoe::GameState = std::clone::Clone::clone(gamestate_4_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6646() {
    rusty_monitor::set_test_id(6646);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut option_9: std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::winner(tictactoe_0_ref_0);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::other(player_13);
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_16);
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_18: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_19: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_17);
    let mut option_20: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<tictactoe::Player>; 3] = [option_20, option_19, option_18];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_6, option_array_5, option_array_3];
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_14, status: gamestate_1};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_445() {
    rusty_monitor::set_test_id(445);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_1_ref_0: &tictactoe::Player = &mut player_1;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    let mut connectfourerror_1_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_1;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_0_ref_0, gamestate_4_ref_0);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_204() {
    rusty_monitor::set_test_id(204);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_1_ref_0: &tictactoe::Player = &mut player_1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2_ref_0: &tictactoe::Player = &mut player_2;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3_ref_0: &tictactoe::Player = &mut player_3;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_8);
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut player_12: gomoku::Player = crate::gomoku::Player::Player1;
    let mut bool_1: bool = std::cmp::PartialEq::eq(player_3_ref_0, player_2_ref_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3459() {
    rusty_monitor::set_test_id(3459);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_10, status: gamestate_1};
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_155() {
    rusty_monitor::set_test_id(155);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut player_3_ref_0: &tictactoe::Player = &mut player_3;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4_ref_0: &tictactoe::Player = &mut player_4;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5_ref_0: &tictactoe::Player = &mut player_5;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_10);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_0_ref_0);
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut player_14: gomoku::Player = crate::gomoku::Player::Player1;
    let mut bool_1: bool = std::cmp::PartialEq::eq(player_5_ref_0, player_4_ref_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8100() {
    rusty_monitor::set_test_id(8100);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_14);
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::other(player_16);
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_18);
    let mut player_19: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_19);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_20: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_21: tictactoe::Player = crate::tictactoe::Player::other(player_20);
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_21);
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut player_22: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_23: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_24: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_24);
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    let mut gamestate_5: tictactoe::GameState = std::clone::Clone::clone(gamestate_4_ref_0);
    let mut player_25: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_6: gomoku::GameState = crate::gomoku::GameState::Win(player_23);
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut option_17: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
    let mut tictactoe_1: crate::tictactoe::TicTacToe = std::clone::Clone::clone(tictactoe_0_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7640() {
    rusty_monitor::set_test_id(7640);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut option_9: std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::winner(tictactoe_0_ref_0);
    let mut player_5: tictactoe::Player = std::option::Option::unwrap(option_9);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2516() {
    rusty_monitor::set_test_id(2516);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_2: gomoku::Player = crate::gomoku::Player::other(player_1);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut gamestate_5_ref_0: &minesweeper::GameState = &mut gamestate_5;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_3_ref_0: &tictactoe::Player = &mut player_3;
    let mut player_4: tictactoe::Player = std::clone::Clone::clone(player_3_ref_0);
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_817() {
    rusty_monitor::set_test_id(817);
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player1;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut bool_0: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
    let mut player_4: gomoku::Player = crate::gomoku::Player::other(player_3);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_4_ref_0: &gomoku::Player = &mut player_4;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_7: gomoku::Player = crate::gomoku::Player::other(player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7268() {
    rusty_monitor::set_test_id(7268);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_6, option_5, option_4];
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_0_ref_0, gamestate_1_ref_0);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7243() {
    rusty_monitor::set_test_id(7243);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_4: &tictactoe::GameState = crate::tictactoe::TicTacToe::status(tictactoe_0_ref_0);
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1025() {
    rusty_monitor::set_test_id(1025);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_2);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_3: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut connectfourerror_2: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut gamestate_6: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut connectfourerror_2_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_2;
    let mut gamestate_7: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut connectfourerror_3: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut gamestate_5_ref_0: &tictactoe::GameState = &mut gamestate_5;
    let mut gamestate_8: tictactoe::GameState = std::clone::Clone::clone(gamestate_5_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_487() {
    rusty_monitor::set_test_id(487);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::Win(player_3);
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_6: gomoku::Player = crate::gomoku::Player::other(player_5);
    let mut option_0: std::option::Option<gomoku::Player> = std::option::Option::Some(player_6);
    let mut player_7: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_1: std::option::Option<gomoku::Player> = std::option::Option::Some(player_7);
    let mut option_2: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_8: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_5: std::option::Option<gomoku::Player> = std::option::Option::Some(player_8);
    let mut option_6: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_9: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_8: std::option::Option<gomoku::Player> = std::option::Option::Some(player_9);
    let mut option_9: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_10: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_13: std::option::Option<gomoku::Player> = std::option::Option::Some(player_10);
    let mut option_14: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<gomoku::Player>; 15] = [option_14, option_13, option_12, option_11, option_10, option_9, option_8, option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_11: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_12: gomoku::Player = crate::gomoku::Player::other(player_11);
    let mut option_15: std::option::Option<gomoku::Player> = std::option::Option::Some(player_12);
    let mut player_13: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_14: gomoku::Player = crate::gomoku::Player::other(player_13);
    let mut option_16: std::option::Option<gomoku::Player> = std::option::Option::Some(player_14);
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut player_16: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_17: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_15);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_4: tictactoe::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
    let mut option_17: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_954() {
    rusty_monitor::set_test_id(954);
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player1;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut player_4: gomoku::Player = crate::gomoku::Player::other(player_3);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_7: gomoku::Player = crate::gomoku::Player::other(player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4174() {
    rusty_monitor::set_test_id(4174);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut bool_1: bool = crate::tictactoe::TicTacToe::is_ended(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2322() {
    rusty_monitor::set_test_id(2322);
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_3);
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player1;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut gamestate_4: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut bool_0: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
    let mut player_5: gomoku::Player = crate::gomoku::Player::other(player_4);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut bool_1: bool = std::cmp::PartialEq::ne(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut player_6: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_5_ref_0: &gomoku::Player = &mut player_5;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_1_ref_0);
    let mut tictactoeerror_2: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut gamestate_5: gomoku::GameState = crate::gomoku::GameState::Win(player_0);
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_41() {
    rusty_monitor::set_test_id(41);
    let mut usize_0: usize = 26usize;
    let mut usize_1: usize = 29usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
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
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut option_9: &std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::get(tictactoe_0_ref_0, usize_1, usize_0);
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2536() {
    rusty_monitor::set_test_id(2536);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_13, option_12, option_11];
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_16, option_15, option_14];
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::other(player_13);
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::other(player_15);
    let mut option_18: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_16);
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::other(player_17);
    let mut option_19: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_18);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_19, option_18, option_17];
    let mut option_20: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_19: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_20: tictactoe::Player = crate::tictactoe::Player::other(player_19);
    let mut option_21: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_20);
    let mut option_22: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<tictactoe::Player>; 3] = [option_22, option_21, option_20];
    let mut option_23: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_24: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_25: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<tictactoe::Player>; 3] = [option_25, option_24, option_23];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_7, option_array_6, option_array_5];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_14, status: gamestate_2};
    let mut tictactoe_1_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut player_21: tictactoe::Player = crate::tictactoe::Player::Player0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6984() {
    rusty_monitor::set_test_id(6984);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_5: tictactoe::Player = crate::tictactoe::TicTacToe::get_next_player(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5288() {
    rusty_monitor::set_test_id(5288);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_1: std::option::Option<gomoku::Player> = std::option::Option::Some(player_1);
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_3: gomoku::Player = crate::gomoku::Player::other(player_2);
    let mut option_2: std::option::Option<gomoku::Player> = std::option::Option::Some(player_3);
    let mut option_3: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_5: std::option::Option<gomoku::Player> = std::option::Option::Some(player_4);
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_6: std::option::Option<gomoku::Player> = std::option::Option::Some(player_5);
    let mut player_6: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_7: std::option::Option<gomoku::Player> = std::option::Option::Some(player_6);
    let mut player_7: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_8: std::option::Option<gomoku::Player> = std::option::Option::Some(player_7);
    let mut option_9: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_8: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_9: gomoku::Player = crate::gomoku::Player::other(player_8);
    let mut option_10: std::option::Option<gomoku::Player> = std::option::Option::Some(player_9);
    let mut option_11: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut backtrace_0: &snafu::Backtrace = std::option::Option::unwrap(option_0);
    let mut player_0_ref_0: &gomoku::Player = &mut player_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3434() {
    rusty_monitor::set_test_id(3434);
    let mut usize_0: usize = 56usize;
    let mut usize_1: usize = 9usize;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_14);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_9, status: gamestate_1};
    let mut tictactoe_1_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::other(player_15);
    let mut option_18: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_19: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_17);
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_20: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_18);
    let mut option_array_6: [std::option::Option<tictactoe::Player>; 3] = [option_20, option_19, option_18];
    let mut option_21: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_19: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_22: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_19);
    let mut player_20: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_23: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_20);
    let mut option_array_7: [std::option::Option<tictactoe::Player>; 3] = [option_23, option_22, option_21];
    let mut player_21: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_22: tictactoe::Player = crate::tictactoe::Player::other(player_21);
    let mut option_24: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_22);
    let mut player_23: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_25: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_23);
    let mut player_24: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_26: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_24);
    let mut option_array_8: [std::option::Option<tictactoe::Player>; 3] = [option_26, option_25, option_24];
    let mut option_array_array_2: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_8, option_array_7, option_array_6];
    let mut tictactoe_2: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_2, next: player_16, status: gamestate_2};
    let mut tictactoe_2_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_2;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut player_25: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_26: tictactoe::Player = crate::tictactoe::Player::other(player_25);
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_26);
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    let mut player_27: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_28: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_29: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::Win(player_29);
    let mut gamestate_5_ref_0: &tictactoe::GameState = &mut gamestate_5;
    let mut gamestate_6: tictactoe::GameState = std::clone::Clone::clone(gamestate_5_ref_0);
    let mut player_30: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_7: gomoku::GameState = crate::gomoku::GameState::Win(player_28);
    let mut player_31: reversi::Player = crate::reversi::Player::Player1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_4_ref_0, gamestate_3_ref_0);
    let mut option_27: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
    crate::tictactoe::TicTacToe::check_state(tictactoe_1_ref_0);
    let mut option_28: &std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::get(tictactoe_0_ref_0, usize_1, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1810() {
    rusty_monitor::set_test_id(1810);
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_2);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player1;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut bool_0: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
    let mut player_4: gomoku::Player = crate::gomoku::Player::other(player_3);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut bool_1: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut player_4_ref_0: &gomoku::Player = &mut player_4;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_6: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3389() {
    rusty_monitor::set_test_id(3389);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut gamestate_4: gomoku::GameState = crate::gomoku::GameState::Tie;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1656() {
    rusty_monitor::set_test_id(1656);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut usize_0: usize = 34usize;
    let mut usize_1: usize = 42usize;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_3, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9_ref_0: &tictactoe::Player = &mut player_9;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut player_10: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_11: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_9: std::option::Option<gomoku::Player> = std::option::Option::Some(player_11);
    let mut player_12: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_13: gomoku::Player = crate::gomoku::Player::other(player_12);
    let mut option_10: std::option::Option<gomoku::Player> = std::option::Option::Some(player_13);
    let mut option_11: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_14: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_13: std::option::Option<gomoku::Player> = std::option::Option::Some(player_14);
    let mut player_15: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_14: std::option::Option<gomoku::Player> = std::option::Option::Some(player_15);
    let mut player_16: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_15: std::option::Option<gomoku::Player> = std::option::Option::Some(player_16);
    let mut player_17: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_16: std::option::Option<gomoku::Player> = std::option::Option::Some(player_17);
    let mut option_17: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_18: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_19: gomoku::Player = crate::gomoku::Player::other(player_18);
    let mut option_18: std::option::Option<gomoku::Player> = std::option::Option::Some(player_19);
    let mut option_19: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_21: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_22: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_23: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<gomoku::Player>; 15] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16, option_15, option_14, option_13, option_12, option_11, option_10, option_9];
    let mut option_24: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_25: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_26: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_27: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_20: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_28: std::option::Option<gomoku::Player> = std::option::Option::Some(player_20);
    let mut option_29: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_21: gomoku::Player = crate::gomoku::Player::other(player_10);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_22: tictactoe::Player = std::clone::Clone::clone(player_9_ref_0);
    let mut result_0: std::result::Result<(), tictactoe::TicTacToeError> = crate::tictactoe::TicTacToe::place(tictactoe_0_ref_0, player_2, usize_1, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6268() {
    rusty_monitor::set_test_id(6268);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_0: std::option::Option<gomoku::Player> = std::option::Option::Some(player_3);
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_5: gomoku::Player = crate::gomoku::Player::other(player_4);
    let mut option_1: std::option::Option<gomoku::Player> = std::option::Option::Some(player_5);
    let mut option_2: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_6: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_7);
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut player_9: gomoku::Player = crate::gomoku::Player::Player1;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_2: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut gamestate_5: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut player_10: gomoku::Player = crate::gomoku::Player::other(player_2);
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_3_ref_0, gamestate_0_ref_0);
    let mut player_1_ref_0: &gomoku::Player = &mut player_1;
    let mut gamestate_6: tictactoe::GameState = crate::tictactoe::GameState::Win(player_8);
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    let mut option_4: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5081() {
    rusty_monitor::set_test_id(5081);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_7, status: gamestate_1};
    let mut tictactoe_1_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut gamestate_5: &tictactoe::GameState = crate::tictactoe::TicTacToe::status(tictactoe_1_ref_0);
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2687() {
    rusty_monitor::set_test_id(2687);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
    let mut gamestate_0: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_1: std::option::Option<gomoku::Player> = std::option::Option::Some(player_1);
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_3: gomoku::Player = crate::gomoku::Player::other(player_2);
    let mut option_2: std::option::Option<gomoku::Player> = std::option::Option::Some(player_3);
    let mut option_3: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_5: std::option::Option<gomoku::Player> = std::option::Option::Some(player_4);
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_6: std::option::Option<gomoku::Player> = std::option::Option::Some(player_5);
    let mut player_6: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_7: std::option::Option<gomoku::Player> = std::option::Option::Some(player_6);
    let mut option_8: std::option::Option<gomoku::Player> = std::option::Option::Some(player_0);
    let mut option_9: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_7: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_8: gomoku::Player = crate::gomoku::Player::other(player_7);
    let mut option_10: std::option::Option<gomoku::Player> = std::option::Option::Some(player_8);
    let mut option_11: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut backtrace_0: &snafu::Backtrace = std::option::Option::unwrap(option_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1814() {
    rusty_monitor::set_test_id(1814);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_3);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player1;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut bool_0: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
    let mut player_5: gomoku::Player = crate::gomoku::Player::other(player_4);
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut bool_1: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut player_6: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_5_ref_0: &gomoku::Player = &mut player_5;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_2);
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_8: gomoku::Player = crate::gomoku::Player::other(player_1);
    panic!("From RustyUnit with love");
}
}