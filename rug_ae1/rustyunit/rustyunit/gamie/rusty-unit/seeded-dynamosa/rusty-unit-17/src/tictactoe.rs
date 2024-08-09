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
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_104() {
//    rusty_monitor::set_test_id(104);
    let mut usize_0: usize = 3usize;
    let mut usize_1: usize = 1usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9_ref_0: &tictactoe::Player = &mut player_9;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_11);
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_12: tictactoe::Player = std::clone::Clone::clone(player_9_ref_0);
    let mut player_12_ref_0: &tictactoe::Player = &mut player_12;
    let mut player_13: tictactoe::Player = std::clone::Clone::clone(player_12_ref_0);
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_2: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut connectfourerror_2_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_2;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut option_9: &std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::get(tictactoe_0_ref_0, usize_1, usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_464() {
//    rusty_monitor::set_test_id(464);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut tictactoeerror_2: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_2_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_2;
    let mut tictactoeerror_3: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_3_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_3;
    let mut tictactoeerror_4: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_4_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_4;
    let mut tictactoeerror_5: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_5_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_5;
    let mut tictactoeerror_6: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_6_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_6;
    let mut tictactoeerror_7: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_7_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_7;
    let mut tictactoeerror_8: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_8_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_8;
    let mut tictactoeerror_9: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_9_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_9;
    let mut tictactoeerror_10: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_10_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_10;
    let mut tictactoeerror_11: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_11_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_11;
    let mut tictactoeerror_12: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_12_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_12;
    let mut tictactoeerror_13: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_13_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_13;
    let mut bool_0: bool = std::cmp::PartialEq::eq(tictactoeerror_13_ref_0, tictactoeerror_12_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(tictactoeerror_11_ref_0, tictactoeerror_10_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(tictactoeerror_9_ref_0, tictactoeerror_8_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(tictactoeerror_7_ref_0, tictactoeerror_6_ref_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(tictactoeerror_5_ref_0, tictactoeerror_4_ref_0);
    let mut bool_5: bool = std::cmp::PartialEq::eq(tictactoeerror_3_ref_0, tictactoeerror_2_ref_0);
    let mut bool_6: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5436() {
//    rusty_monitor::set_test_id(5436);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut usize_0: usize = 16usize;
    let mut bool_4: bool = false;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::Win(player_2);
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_4, mine_adjacent: usize_0, is_revealed: bool_3, is_flagged: bool_2};
    let mut gamestate_5_ref_0: &tictactoe::GameState = &mut gamestate_5;
    let mut gamestate_6: tictactoe::GameState = std::clone::Clone::clone(gamestate_5_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_462() {
//    rusty_monitor::set_test_id(462);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_5_ref_0: &tictactoe::GameState = &mut gamestate_5;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_6: tictactoe::GameState = crate::tictactoe::GameState::Win(player_2);
    let mut gamestate_6_ref_0: &tictactoe::GameState = &mut gamestate_6;
    let mut gamestate_7: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_7_ref_0: &tictactoe::GameState = &mut gamestate_7;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut gamestate_8: tictactoe::GameState = crate::tictactoe::GameState::Win(player_4);
    let mut gamestate_8_ref_0: &tictactoe::GameState = &mut gamestate_8;
    let mut gamestate_9: tictactoe::GameState = std::clone::Clone::clone(gamestate_8_ref_0);
    let mut gamestate_10: tictactoe::GameState = std::clone::Clone::clone(gamestate_7_ref_0);
    let mut gamestate_11: tictactoe::GameState = std::clone::Clone::clone(gamestate_6_ref_0);
    let mut gamestate_12: tictactoe::GameState = std::clone::Clone::clone(gamestate_5_ref_0);
    let mut gamestate_13: tictactoe::GameState = std::clone::Clone::clone(gamestate_4_ref_0);
    let mut gamestate_14: tictactoe::GameState = std::clone::Clone::clone(gamestate_3_ref_0);
    let mut gamestate_15: tictactoe::GameState = std::clone::Clone::clone(gamestate_2_ref_0);
    let mut gamestate_16: tictactoe::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut gamestate_17: tictactoe::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8238() {
//    rusty_monitor::set_test_id(8238);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::other(player_12);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_2, status: gamestate_1};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::other(player_14);
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::other(player_16);
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_17);
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_19: tictactoe::Player = crate::tictactoe::Player::other(player_18);
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_19);
    let mut player_20: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_21: tictactoe::Player = crate::tictactoe::Player::other(player_20);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_21);
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_22: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_23: tictactoe::Player = crate::tictactoe::Player::other(player_22);
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_23);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut player_24: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_24);
    let mut player_25: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_25);
    let mut player_26: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_27: tictactoe::Player = crate::tictactoe::Player::other(player_26);
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_27);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut player_28: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_29: tictactoe::Player = crate::tictactoe::Player::other(player_28);
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_29);
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    let mut player_30: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_6: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_7: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_7_ref_0: &minesweeper::GameState = &mut gamestate_7;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_410() {
//    rusty_monitor::set_test_id(410);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut tictactoeerror_2: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_2_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_2;
    let mut tictactoeerror_3: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_3_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_3;
    let mut tictactoeerror_4: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_4_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_4;
    let mut tictactoeerror_5: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_5_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_5;
    let mut tictactoeerror_6: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_6_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_6;
    let mut tictactoeerror_7: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_7_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_7;
    let mut tictactoeerror_8: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_8_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_8;
    let mut tictactoeerror_9: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_9_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_9;
    let mut tictactoeerror_10: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_10_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_10;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_10_ref_0);
    let mut option_1: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_9_ref_0);
    let mut option_2: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_8_ref_0);
    let mut option_3: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_7_ref_0);
    let mut option_4: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_6_ref_0);
    let mut option_5: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_5_ref_0);
    let mut option_6: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_4_ref_0);
    let mut option_7: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_3_ref_0);
    let mut option_8: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_2_ref_0);
    let mut option_9: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_1_ref_0);
    let mut option_10: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1720() {
//    rusty_monitor::set_test_id(1720);
    let mut usize_0: usize = 0usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut usize_1: usize = 15usize;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut result_0: std::result::Result<(), tictactoe::TicTacToeError> = crate::tictactoe::TicTacToe::place(tictactoe_0_ref_0, player_0, usize_0, usize_1);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_107() {
//    rusty_monitor::set_test_id(107);
    let mut usize_0: usize = 5usize;
    let mut usize_1: usize = 66usize;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
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
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_10);
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_11);
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_2: usize = 1usize;
    let mut usize_3: usize = 2usize;
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_5: tictactoe::GameState = std::clone::Clone::clone(gamestate_3_ref_0);
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut gomoku_0: crate::gomoku::Gomoku = std::result::Result::unwrap(result_0);
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut option_9: &std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::get(tictactoe_0_ref_0, usize_1, usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1866() {
//    rusty_monitor::set_test_id(1866);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::other(player_12);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_14);
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::InProgress;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4968() {
//    rusty_monitor::set_test_id(4968);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_8_ref_0: &tictactoe::Player = &mut player_8;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_1_ref_0);
    let mut gomoku_0: crate::gomoku::Gomoku = std::result::Result::unwrap(result_0);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Win(player_9);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut gomoku_0_ref_0: &crate::gomoku::Gomoku = &mut gomoku_0;
    let mut gamestate_4: &gomoku::GameState = crate::gomoku::Gomoku::status(gomoku_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_772() {
//    rusty_monitor::set_test_id(772);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_7, option_6, option_5];
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4177() {
//    rusty_monitor::set_test_id(4177);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::other(player_12);
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_14);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_16);
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut player_18: reversi::Player = crate::reversi::Player::other(player_17);
    let mut player_19: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_20: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_21: gomoku::Player = crate::gomoku::Player::other(player_20);
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::Win(player_21);
    let mut player_22: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_23: gomoku::Player = crate::gomoku::Player::other(player_22);
    let mut option_18: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_24: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_19: std::option::Option<gomoku::Player> = std::option::Option::Some(player_24);
    let mut player_25: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_20: std::option::Option<gomoku::Player> = std::option::Option::Some(player_25);
    let mut option_21: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_26: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_22: std::option::Option<gomoku::Player> = std::option::Option::Some(player_26);
    let mut option_23: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_27: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_28: gomoku::Player = crate::gomoku::Player::other(player_27);
    let mut option_24: std::option::Option<gomoku::Player> = std::option::Option::Some(player_28);
    let mut option_25: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_26: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_29: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_27: std::option::Option<gomoku::Player> = std::option::Option::Some(player_29);
    let mut player_30: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_28: std::option::Option<gomoku::Player> = std::option::Option::Some(player_30);
    let mut player_31: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_29: std::option::Option<gomoku::Player> = std::option::Option::Some(player_31);
    let mut option_30: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_32: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_33: gomoku::Player = crate::gomoku::Player::other(player_32);
    let mut option_31: std::option::Option<gomoku::Player> = std::option::Option::Some(player_33);
    let mut player_34: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_35: gomoku::Player = crate::gomoku::Player::other(player_34);
    let mut option_32: std::option::Option<gomoku::Player> = std::option::Option::Some(player_35);
    let mut option_array_5: [std::option::Option<gomoku::Player>; 15] = [option_32, option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24, option_23, option_22, option_21, option_20, option_19, option_18];
    let mut player_36: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_33: std::option::Option<gomoku::Player> = std::option::Option::Some(player_36);
    let mut player_37: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_34: std::option::Option<gomoku::Player> = std::option::Option::Some(player_37);
    let mut player_38: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_35: std::option::Option<gomoku::Player> = std::option::Option::Some(player_38);
    let mut player_39: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_36: std::option::Option<gomoku::Player> = std::option::Option::Some(player_39);
    let mut option_37: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_40: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_38: std::option::Option<gomoku::Player> = std::option::Option::Some(player_40);
    let mut player_41: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_39: std::option::Option<gomoku::Player> = std::option::Option::Some(player_41);
    let mut option_40: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_42: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_41: std::option::Option<gomoku::Player> = std::option::Option::Some(player_42);
    let mut player_43: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_42: std::option::Option<gomoku::Player> = std::option::Option::Some(player_43);
    let mut player_44: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_43: std::option::Option<gomoku::Player> = std::option::Option::Some(player_44);
    let mut option_44: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_45: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_46: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_47: reversi::Player = crate::reversi::Player::Player1;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_46);
    let mut player_48: reversi::Player = crate::reversi::Player::Player1;
    let mut player_49: reversi::Player = crate::reversi::Player::other(player_47);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_48);
    let mut player_50: reversi::Player = crate::reversi::Player::Player1;
    let mut player_51: reversi::Player = crate::reversi::Player::other(player_49);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_50);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_52: reversi::Player = crate::reversi::Player::Player1;
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_51);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_53: reversi::Player = crate::reversi::Player::Player1;
    let mut player_54: reversi::Player = crate::reversi::Player::other(player_52);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_53);
    let mut player_55: reversi::Player = crate::reversi::Player::Player1;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_54);
    let mut player_56: reversi::Player = crate::reversi::Player::Player0;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_55);
    let mut player_57: reversi::Player = crate::reversi::Player::Player1;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_56);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_58: reversi::Player = crate::reversi::Player::Player0;
    let mut player_59: reversi::Player = crate::reversi::Player::other(player_57);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_58);
    let mut player_60: reversi::Player = crate::reversi::Player::Player1;
    let mut player_61: reversi::Player = crate::reversi::Player::other(player_59);
    let mut option_64: std::option::Option<reversi::Player> = std::option::Option::Some(player_60);
    let mut option_65: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_62: reversi::Player = crate::reversi::Player::Player1;
    let mut option_66: std::option::Option<reversi::Player> = std::option::Option::Some(player_61);
    let mut option_67: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_68: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_69: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_70: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_71: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_72: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_73: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_63: reversi::Player = crate::reversi::Player::Player1;
    let mut option_74: std::option::Option<reversi::Player> = std::option::Option::Some(player_62);
    let mut player_64: reversi::Player = crate::reversi::Player::Player0;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_60, option_50, option_67, option_54, option_59, option_46, option_53, option_66];
    let mut option_75: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_76: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_77: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_78: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_79: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_65: reversi::Player = crate::reversi::Player::Player1;
    let mut player_66: reversi::Player = crate::reversi::Player::other(player_64);
    let mut option_80: std::option::Option<reversi::Player> = std::option::Option::Some(player_65);
    let mut player_67: reversi::Player = crate::reversi::Player::Player0;
    let mut option_81: std::option::Option<reversi::Player> = std::option::Option::Some(player_66);
    let mut option_82: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_68: reversi::Player = crate::reversi::Player::Player0;
    let mut player_69: reversi::Player = crate::reversi::Player::other(player_67);
    let mut option_83: std::option::Option<reversi::Player> = std::option::Option::Some(player_68);
    let mut option_84: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_70: reversi::Player = crate::reversi::Player::Player1;
    let mut option_85: std::option::Option<reversi::Player> = std::option::Option::Some(player_69);
    let mut player_71: reversi::Player = crate::reversi::Player::Player0;
    let mut option_86: std::option::Option<reversi::Player> = std::option::Option::Some(player_70);
    let mut option_87: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_72: reversi::Player = crate::reversi::Player::Player0;
    let mut option_88: std::option::Option<reversi::Player> = std::option::Option::Some(player_71);
    let mut player_73: reversi::Player = crate::reversi::Player::Player0;
    let mut option_89: std::option::Option<reversi::Player> = std::option::Option::Some(player_72);
    let mut option_90: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_74: reversi::Player = crate::reversi::Player::Player0;
    let mut option_91: std::option::Option<reversi::Player> = std::option::Option::Some(player_73);
    let mut player_75: reversi::Player = crate::reversi::Player::Player1;
    let mut option_92: std::option::Option<reversi::Player> = std::option::Option::Some(player_74);
    let mut player_76: reversi::Player = crate::reversi::Player::Player1;
    let mut option_93: std::option::Option<reversi::Player> = std::option::Option::Some(player_75);
    let mut option_94: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_77: reversi::Player = crate::reversi::Player::Player1;
    let mut option_95: std::option::Option<reversi::Player> = std::option::Option::Some(player_76);
    let mut option_96: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_78: reversi::Player = crate::reversi::Player::Player0;
    let mut option_97: std::option::Option<reversi::Player> = std::option::Option::Some(player_77);
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4187() {
//    rusty_monitor::set_test_id(4187);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_9_ref_0: &tictactoe::Player = &mut player_9;
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut gomoku_0: crate::gomoku::Gomoku = std::result::Result::unwrap(result_0);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Win(player_10);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut gomoku_0_ref_0: &crate::gomoku::Gomoku = &mut gomoku_0;
    let mut gamestate_4: &gomoku::GameState = crate::gomoku::Gomoku::status(gomoku_0_ref_0);
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1648() {
//    rusty_monitor::set_test_id(1648);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_2);
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_4, status: gamestate_1};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_12: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    let mut gomoku_0: crate::gomoku::Gomoku = std::result::Result::unwrap(result_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut gomoku_0_ref_0: &crate::gomoku::Gomoku = &mut gomoku_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6246() {
//    rusty_monitor::set_test_id(6246);
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut bool_0: bool = crate::tictactoe::TicTacToe::is_ended(tictactoe_0_ref_0);
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut player_11: gomoku::Player = crate::gomoku::Player::other(player_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8417() {
//    rusty_monitor::set_test_id(8417);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_0_ref_0: &tictactoe::Player = &mut player_0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1_ref_0: &tictactoe::Player = &mut player_1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut bool_0: bool = std::cmp::PartialEq::eq(player_1_ref_0, player_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5433() {
//    rusty_monitor::set_test_id(5433);
    let mut usize_0: usize = 0usize;
    let mut usize_1: usize = 2usize;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::other(player_11);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::other(player_13);
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_14);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_16);
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::other(player_17);
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_18);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_6, status: gamestate_2};
    let mut tictactoe_1_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut player_19: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_20: tictactoe::Player = crate::tictactoe::Player::other(player_19);
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_20);
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut player_21: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_6: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_6_ref_0: &minesweeper::GameState = &mut gamestate_6;
    let mut gamestate_7: tictactoe::GameState = crate::tictactoe::GameState::Win(player_21);
    let mut gamestate_7_ref_0: &tictactoe::GameState = &mut gamestate_7;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_7_ref_0, gamestate_3_ref_0);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_1_ref_0);
    let mut gamestate_5_ref_0: &minesweeper::GameState = &mut gamestate_5;
    let mut option_18: &std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::get(tictactoe_0_ref_0, usize_1, usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_698() {
//    rusty_monitor::set_test_id(698);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_7, option_6, option_5];
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut bool_0: bool = std::cmp::PartialEq::eq(tictactoeerror_1_ref_0, tictactoeerror_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_685() {
//    rusty_monitor::set_test_id(685);
    let mut usize_0: usize = 2usize;
    let mut usize_1: usize = 26usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut result_0: std::result::Result<(), tictactoe::TicTacToeError> = crate::tictactoe::TicTacToe::place(tictactoe_0_ref_0, player_0, usize_1, usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8265() {
//    rusty_monitor::set_test_id(8265);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut result_0: std::result::Result<(), tictactoe::TicTacToeError> = std::result::Result::Err(tictactoeerror_0);
    let mut usize_0: usize = 15usize;
    let mut usize_1: usize = 16usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut result_1: std::result::Result<(), tictactoe::TicTacToeError> = crate::tictactoe::TicTacToe::place(tictactoe_0_ref_0, player_0, usize_1, usize_0);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8212() {
//    rusty_monitor::set_test_id(8212);
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5_ref_0: &tictactoe::Player = &mut player_5;
    let mut player_9: tictactoe::Player = std::clone::Clone::clone(player_5_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_131() {
//    rusty_monitor::set_test_id(131);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_2};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut usize_0: usize = 16usize;
    let mut usize_1: usize = 5usize;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut option_9: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_1_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7695() {
//    rusty_monitor::set_test_id(7695);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_0_ref_0: &tictactoe::Player = &mut player_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_0_ref_0);
    let mut player_1_ref_0: &tictactoe::Player = &mut player_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3118() {
//    rusty_monitor::set_test_id(3118);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_3, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut option_9: std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::winner(tictactoe_0_ref_0);
    let mut player_12: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::other(player_14);
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_9, option_11, option_10];
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_0);
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_17);
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_16);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_15, option_14, option_13];
    let mut player_19: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_20: tictactoe::Player = crate::tictactoe::Player::other(player_19);
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_20);
    let mut player_21: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_21);
    let mut player_22: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_18: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_22);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_18, option_17, option_16];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_13, status: gamestate_1};
    let mut tictactoe_1_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut bool_0: bool = crate::tictactoe::TicTacToe::is_ended(tictactoe_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7774() {
//    rusty_monitor::set_test_id(7774);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_238() {
//    rusty_monitor::set_test_id(238);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
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
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_3, status: gamestate_1};
    let mut tictactoe_1_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_1;
    let mut gamestate_2: &tictactoe::GameState = crate::tictactoe::TicTacToe::status(tictactoe_1_ref_0);
    let mut gamestate_3: &tictactoe::GameState = crate::tictactoe::TicTacToe::status(tictactoe_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8881() {
//    rusty_monitor::set_test_id(8881);
    let mut usize_0: usize = 15usize;
    let mut usize_1: usize = 16usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut result_0: std::result::Result<(), tictactoe::TicTacToeError> = crate::tictactoe::TicTacToe::place(tictactoe_0_ref_0, player_0, usize_1, usize_0);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_906() {
//    rusty_monitor::set_test_id(906);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::other(player_14);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_15);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut gamestate_5: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut option_9: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut gamestate_6: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_7: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut tictactoe_1: crate::tictactoe::TicTacToe = std::clone::Clone::clone(tictactoe_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_643() {
//    rusty_monitor::set_test_id(643);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_3);
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_4);
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::Win(player_5);
    let mut gamestate_5_ref_0: &tictactoe::GameState = &mut gamestate_5;
    let mut gamestate_6: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_6_ref_0: &tictactoe::GameState = &mut gamestate_6;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_7: tictactoe::GameState = crate::tictactoe::GameState::Win(player_6);
    let mut gamestate_7_ref_0: &tictactoe::GameState = &mut gamestate_7;
    let mut gamestate_8: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_8_ref_0: &tictactoe::GameState = &mut gamestate_8;
    let mut gamestate_9: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_9_ref_0: &tictactoe::GameState = &mut gamestate_9;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_9_ref_0, gamestate_8_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_7_ref_0, gamestate_6_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(gamestate_5_ref_0, gamestate_4_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8696() {
//    rusty_monitor::set_test_id(8696);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_9);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::other(player_11);
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::other(player_13);
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_14);
    let mut player_15: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_16: tictactoe::Player = crate::tictactoe::Player::other(player_15);
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_16);
    let mut player_17: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_18: tictactoe::Player = crate::tictactoe::Player::other(player_17);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_18);
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_19: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_20: tictactoe::Player = crate::tictactoe::Player::other(player_19);
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_20);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut player_21: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_21);
    let mut player_22: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_22);
    let mut player_23: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_24: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_24);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_12, status: gamestate_1};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_25: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_26: tictactoe::Player = crate::tictactoe::Player::other(player_25);
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_26);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_27: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_5_ref_0: &minesweeper::GameState = &mut gamestate_5;
    let mut gamestate_6: tictactoe::GameState = crate::tictactoe::GameState::Win(player_27);
    let mut gamestate_6_ref_0: &tictactoe::GameState = &mut gamestate_6;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_6_ref_0, gamestate_2_ref_0);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_312() {
//    rusty_monitor::set_test_id(312);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_9: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_10: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut option_11: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<tictactoe::Player>; 3] = [option_11, option_10, option_9];
    let mut option_12: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_13: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_14: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_array_4: [std::option::Option<tictactoe::Player>; 3] = [option_14, option_13, option_12];
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_15: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut option_16: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::other(player_12);
    let mut option_17: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut option_array_5: [std::option::Option<tictactoe::Player>; 3] = [option_17, option_16, option_15];
    let mut option_array_array_1: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_5, option_array_4, option_array_3];
    let mut tictactoe_1: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_1, next: player_7, status: gamestate_1};
    let mut tictactoe_1_ref_0: &mut crate::tictactoe::TicTacToe = &mut tictactoe_1;
    crate::tictactoe::TicTacToe::check_state(tictactoe_1_ref_0);
    crate::tictactoe::TicTacToe::check_state(tictactoe_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6349() {
//    rusty_monitor::set_test_id(6349);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut usize_0: usize = 16usize;
    let mut bool_4: bool = true;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_5_ref_0: &minesweeper::GameState = &mut gamestate_5;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_6: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_4, mine_adjacent: usize_0, is_revealed: bool_3, is_flagged: bool_2};
    let mut gamestate_6_ref_0: &tictactoe::GameState = &mut gamestate_6;
    let mut bool_5: bool = std::cmp::PartialEq::eq(gamestate_6_ref_0, gamestate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_128() {
//    rusty_monitor::set_test_id(128);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_1);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut usize_0: usize = 1usize;
    let mut usize_1: usize = 2usize;
    let mut usize_2: usize = 5usize;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<connect_four::Player>; 6] = [option_14, option_13, option_12, option_11, option_10, option_9];
    let mut usize_3: usize = 4usize;
    let mut usize_4: usize = 7usize;
    let mut usize_5: usize = 6usize;
    let mut usize_6: usize = 15usize;
    let mut usize_7: usize = 7usize;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Win(player_6);
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(tictactoeerror_0_ref_0);
    let mut player_7: tictactoe::Player = crate::tictactoe::TicTacToe::get_next_player(tictactoe_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_816() {
//    rusty_monitor::set_test_id(816);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut bool_0: bool = false;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut usize_0: usize = 7usize;
    let mut bool_3: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_3, mine_adjacent: usize_0, is_revealed: bool_2, is_flagged: bool_1};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8966() {
//    rusty_monitor::set_test_id(8966);
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::other(player_7);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_8);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_10);
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut bool_0: bool = crate::tictactoe::TicTacToe::is_ended(tictactoe_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6217() {
//    rusty_monitor::set_test_id(6217);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_776() {
//    rusty_monitor::set_test_id(776);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1_ref_0: &tictactoe::Player = &mut player_1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
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
    let mut tictactoe_0: crate::tictactoe::TicTacToe = crate::tictactoe::TicTacToe {board: option_array_array_0, next: player_4, status: gamestate_1};
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_7);
    let mut player_2_ref_0: &tictactoe::Player = &mut player_2;
    let mut player_8: tictactoe::Player = std::clone::Clone::clone(player_1_ref_0);
    let mut player_0_ref_0: &tictactoe::Player = &mut player_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_2_ref_0);
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_9: std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::winner(tictactoe_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_592() {
//    rusty_monitor::set_test_id(592);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_2);
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut gamestate_6: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_5_ref_0: &connect_four::GameState = &mut gamestate_5;
    let mut gamestate_7: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_6_ref_0: &connect_four::GameState = &mut gamestate_6;
    let mut gamestate_8: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_7_ref_0: &connect_four::GameState = &mut gamestate_7;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut gamestate_9: connect_four::GameState = crate::connect_four::GameState::Win(player_4);
    let mut gamestate_8_ref_0: &connect_four::GameState = &mut gamestate_8;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_10: connect_four::GameState = crate::connect_four::GameState::Win(player_5);
    let mut gamestate_9_ref_0: &connect_four::GameState = &mut gamestate_9;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(tictactoeerror_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_601() {
//    rusty_monitor::set_test_id(601);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1_ref_0: &gomoku::Player = &mut player_1;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut player_3: gomoku::Player = crate::gomoku::Player::other(player_2);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut result_1: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut player_4_ref_0: &tictactoe::Player = &mut player_4;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(tictactoeerror_0_ref_0);
//    panic!("From RustyUnit with love");
}
}