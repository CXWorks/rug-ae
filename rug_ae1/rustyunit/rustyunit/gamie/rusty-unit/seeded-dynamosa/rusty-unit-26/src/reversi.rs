//! Reversi
//!
//! Check struct [`Reversi`](https://docs.rs/gamie/*/gamie/reversi/struct.Reversi.html) for more information
//!
//! # Examples
//!
//! ```rust
//! # fn reversi() {
//! use gamie::reversi::{Reversi, Player as ReversiPlayer};
//!
//! let mut game = Reversi::new().unwrap();
//!
//! game.place(ReversiPlayer::Player0, 2, 4).unwrap();
//!
//! // The next player may not be able to place the piece in any position, so check the output of `get_next_player()`
//! assert_eq!(game.get_next_player(), ReversiPlayer::Player1);
//!
//! game.place(ReversiPlayer::Player1, 2, 3).unwrap();
//!
//! // ...
//! # }
//! ```

use crate::std_lib::{iter, Infallible, Ordering};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use snafu::Snafu;

/// Reversi
///
/// Passing an invalid position to a method will cause panic. Check the target position validity first when dealing with user input
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Reversi {
    board: [[Option<Player>; 8]; 8],
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

impl Reversi {
    /// Create a new Reversi game
    pub fn new() -> Result<Self, Infallible> {
        let mut board = [[None; 8]; 8];
        board[3][3] = Some(Player::Player0);
        board[4][4] = Some(Player::Player0);
        board[3][4] = Some(Player::Player1);
        board[4][3] = Some(Player::Player1);

        Ok(Self {
            board,
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
    pub fn place(&mut self, player: Player, row: usize, col: usize) -> Result<(), ReversiError> {
        self.simple_check_position_validity(row, col, player)?;

        let mut flipped = false;

        for dir in Direction::iter() {
            if let Some((to_row, to_col)) =
                self.check_occupied_line_in_direction(row, col, dir, player)
            {
                self.flip(row, col, to_row, to_col, dir, player);
                flipped = true;
            }
        }

        if flipped {
            self.next = player.other();

            if !self.can_player_move(player.other()) {
                self.next = player;

                if !self.can_player_move(player) {
                    self.check_state();
                }
            }

            Ok(())
        } else {
            Err(ReversiError::InvalidPosition)
        }
    }

    /// Check if a position is valid for placing piece
    /// Panic when target position out of bounds
    pub fn check_position_validity(
        &self,
        row: usize,
        col: usize,
        player: Player,
    ) -> Result<(), ReversiError> {
        self.simple_check_position_validity(row, col, player)?;

        if Direction::iter()
            .map(|dir| self.check_occupied_line_in_direction(row, col, dir, player))
            .any(|o| o.is_some())
        {
            Ok(())
        } else {
            Err(ReversiError::InvalidPosition)
        }
    }

    fn simple_check_position_validity(
        &self,
        row: usize,
        col: usize,
        player: Player,
    ) -> Result<(), ReversiError> {
        if self.is_ended() {
            return Err(ReversiError::GameEnded);
        }

        if player != self.next {
            return Err(ReversiError::WrongPlayer);
        }

        if self.board[row][col].is_some() {
            return Err(ReversiError::OccupiedPosition);
        }

        Ok(())
    }

    fn can_player_move(&self, player: Player) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                if self.board[row][col].is_none()
                    && self.check_position_validity(row, col, player).is_ok()
                {
                    return true;
                }
            }
        }

        false
    }

    fn check_state(&mut self) {
        let mut black_count = 0;
        let mut white_count = 0;

        for cell in self.board.iter().flatten().flatten() {
            match cell {
                Player::Player0 => black_count += 1,
                Player::Player1 => white_count += 1,
            }
        }

        self.status = match black_count.cmp(&white_count) {
            Ordering::Less => GameState::Win(Player::Player1),
            Ordering::Equal => GameState::Tie,
            Ordering::Greater => GameState::Win(Player::Player0),
        };
    }

    fn flip(
        &mut self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
        dir: Direction,
        player: Player,
    ) {
        self.iter_positions_in_direction_from(from_row, from_col, dir)
            .take_while(|(row, col)| *row != to_row || *col != to_col)
            .for_each(|(row, col)| {
                self.board[row][col] = Some(player);
            });
    }

    fn check_occupied_line_in_direction(
        &self,
        row: usize,
        col: usize,
        dir: Direction,
        player: Player,
    ) -> Option<(usize, usize)> {
        let mut pos = self.iter_positions_in_direction_from(row, col, dir);

        pos.next();

        let first = if let Some(pos) = pos.next() {
            pos
        } else {
            return None;
        };

        if self.board[first.0][first.1] != Some(player.other()) {
            return None;
        }

        for (row, col) in pos {
            match self.board[row][col] {
                Some(piece) if piece == player.other() => continue,
                Some(_) => return Some((row, col)),
                None => return None,
            }
        }

        None
    }

    fn iter_positions_in_direction_from(
        &self,
        row: usize,
        col: usize,
        dir: Direction,
    ) -> impl Iterator<Item = (usize, usize)> {
        iter::successors(Some((row, col)), move |(row, col)| {
            let (offset_row, offset_col) = dir.as_offset();
            Some((
                (*row as i8 + offset_row) as usize,
                (*col as i8 + offset_col) as usize,
            ))
        })
        .take_while(|(row, col)| *row < 8 && *col < 8)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Upper,
    UpperRight,
    Right,
    LowerRight,
    Lower,
    LowerLeft,
    Left,
    UpperLeft,
}

impl Direction {
    fn as_offset(&self) -> (i8, i8) {
        match self {
            Direction::Upper => (-1, 0),
            Direction::UpperRight => (-1, 1),
            Direction::Right => (0, 1),
            Direction::LowerRight => (1, 1),
            Direction::Lower => (1, 0),
            Direction::LowerLeft => (1, -1),
            Direction::Left => (0, -1),
            Direction::UpperLeft => (-1, -1),
        }
    }

    fn iter() -> impl Iterator<Item = Self> {
        [
            Direction::Upper,
            Direction::UpperRight,
            Direction::Right,
            Direction::LowerRight,
            Direction::Lower,
            Direction::LowerLeft,
            Direction::Left,
            Direction::UpperLeft,
        ]
        .into_iter()
    }
}

/// Errors that can occur when placing a piece on the board
#[derive(Debug, Eq, PartialEq, Snafu)]
pub enum ReversiError {
    #[snafu(display("Wrong player"))]
    WrongPlayer,
    #[snafu(display("Position already occupied"))]
    OccupiedPosition,
    #[snafu(display("Invalid position"))]
    InvalidPosition,
    #[snafu(display("The game was already end"))]
    GameEnded,
}

#[cfg(test)]
mod tests {
    use crate::reversi::*;

    #[test]
    fn test() {
        let mut game = Reversi::new().unwrap();

        assert_eq!(game.place(Player::Player0, 2, 4), Ok(()));

        assert_eq!(game.place(Player::Player1, 2, 3), Ok(()));

        assert_eq!(
            game.place(Player::Player1, 2, 6),
            Err(ReversiError::WrongPlayer)
        );

        assert_eq!(
            game.place(Player::Player0, 2, 6),
            Err(ReversiError::InvalidPosition)
        );
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
#[timeout(30000)]fn rusty_test_5560() {
//    rusty_monitor::set_test_id(5560);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut direction_2: reversi::Direction = crate::reversi::Direction::Lower;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut gamestate_5: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_5_ref_0: &reversi::GameState = &mut gamestate_5;
    let mut gamestate_6: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_6_ref_0: &reversi::GameState = &mut gamestate_6;
    let mut gamestate_7: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_7_ref_0: &reversi::GameState = &mut gamestate_7;
    let mut gamestate_8: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_8_ref_0: &reversi::GameState = &mut gamestate_8;
    let mut gamestate_9: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_10: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_10_ref_0: &reversi::GameState = &mut gamestate_10;
    let mut gamestate_11: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_11_ref_0: &reversi::GameState = &mut gamestate_11;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_12: reversi::GameState = crate::reversi::GameState::Win(player_3);
    let mut gamestate_12_ref_0: &reversi::GameState = &mut gamestate_12;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_12_ref_0, gamestate_11_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_10_ref_0, gamestate_0_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(gamestate_8_ref_0, gamestate_7_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_6_ref_0, gamestate_5_ref_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(gamestate_4_ref_0, gamestate_3_ref_0);
    let mut bool_5: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut bool_6: bool = std::cmp::PartialEq::eq(direction_0_ref_0, direction_1_ref_0);
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_562() {
//    rusty_monitor::set_test_id(562);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut direction_2: reversi::Direction = crate::reversi::Direction::Upper;
    let mut direction_2_ref_0: &reversi::Direction = &mut direction_2;
    let mut direction_3: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut direction_3_ref_0: &reversi::Direction = &mut direction_3;
    let mut direction_4: reversi::Direction = crate::reversi::Direction::Right;
    let mut direction_4_ref_0: &reversi::Direction = &mut direction_4;
    let mut direction_5: reversi::Direction = crate::reversi::Direction::LowerLeft;
    let mut direction_5_ref_0: &reversi::Direction = &mut direction_5;
    let mut direction_6: reversi::Direction = crate::reversi::Direction::Left;
    let mut direction_6_ref_0: &reversi::Direction = &mut direction_6;
    let mut direction_7: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut direction_7_ref_0: &reversi::Direction = &mut direction_7;
    let mut direction_8: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut direction_8_ref_0: &reversi::Direction = &mut direction_8;
    let mut direction_9: reversi::Direction = crate::reversi::Direction::LowerLeft;
    let mut direction_9_ref_0: &reversi::Direction = &mut direction_9;
    let mut direction_10: reversi::Direction = crate::reversi::Direction::LowerRight;
    let mut direction_10_ref_0: &reversi::Direction = &mut direction_10;
    let mut tuple_0: (i8, i8) = crate::reversi::Direction::as_offset(direction_10_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3608() {
//    rusty_monitor::set_test_id(3608);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_0_ref_0: &reversi::Player = &mut player_0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_7: connect_four::Player = crate::connect_four::Player::other(player_6);
    let mut player_7_ref_0: &connect_four::Player = &mut player_7;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 7usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1659() {
//    rusty_monitor::set_test_id(1659);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut player_6: gomoku::Player = crate::gomoku::Player::Player1;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut result_1: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3433() {
//    rusty_monitor::set_test_id(3433);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_9: connect_four::Player = crate::connect_four::Player::other(player_8);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9081() {
//    rusty_monitor::set_test_id(9081);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut player_5: reversi::Player = crate::reversi::Player::other(player_4);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut player_10: reversi::Player = crate::reversi::Player::other(player_9);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player0;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_17: reversi::Player = crate::reversi::Player::Player0;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut player_19: reversi::Player = crate::reversi::Player::other(player_18);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player1;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut player_23: reversi::Player = crate::reversi::Player::other(player_22);
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut player_28: reversi::Player = crate::reversi::Player::other(player_27);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut player_30: reversi::Player = crate::reversi::Player::Player1;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player1;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut player_34: reversi::Player = crate::reversi::Player::other(player_33);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_35: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut player_37: reversi::Player = crate::reversi::Player::Player0;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut player_39: reversi::Player = crate::reversi::Player::Player0;
    let mut player_40: reversi::Player = crate::reversi::Player::other(player_39);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_41: reversi::Player = crate::reversi::Player::Player0;
    let mut player_42: reversi::Player = crate::reversi::Player::other(player_41);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_43: reversi::Player = crate::reversi::Player::Player0;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut player_44: reversi::Player = crate::reversi::Player::Player1;
    let mut player_45: reversi::Player = crate::reversi::Player::other(player_44);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_45);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_46: reversi::Player = crate::reversi::Player::Player1;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_46);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &mut crate::reversi::Reversi = &mut reversi_0;
    crate::reversi::Reversi::check_state(reversi_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_92() {
//    rusty_monitor::set_test_id(92);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut usize_0: usize = 6usize;
    let mut usize_1: usize = 6usize;
    let mut usize_2: usize = 6usize;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_2: gomoku::Player = crate::gomoku::Player::other(player_1);
    let mut player_2_ref_0: &gomoku::Player = &mut player_2;
    let mut usize_3: usize = 6usize;
    let mut usize_4: usize = 3usize;
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Lower;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut player_3_ref_0: &gomoku::Player = &mut player_3;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut direction_1: reversi::Direction = std::clone::Clone::clone(direction_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8963() {
//    rusty_monitor::set_test_id(8963);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Lower;
    let mut usize_0: usize = 4usize;
    let mut usize_1: usize = 5usize;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut player_5: reversi::Player = crate::reversi::Player::other(player_4);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_7);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut player_11: reversi::Player = crate::reversi::Player::other(player_10);
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut player_14: reversi::Player = crate::reversi::Player::other(player_13);
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut player_16: reversi::Player = crate::reversi::Player::other(player_15);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut player_17: reversi::Player = crate::reversi::Player::Player0;
    let mut player_18: reversi::Player = crate::reversi::Player::other(player_17);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut player_21: reversi::Player = crate::reversi::Player::other(player_20);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut player_27: reversi::Player = crate::reversi::Player::other(player_26);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut player_29: reversi::Player = crate::reversi::Player::other(player_28);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut player_33: reversi::Player = crate::reversi::Player::other(player_32);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_35: reversi::Player = crate::reversi::Player::Player0;
    let mut player_36: reversi::Player = crate::reversi::Player::other(player_35);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut player_37: reversi::Player = crate::reversi::Player::Player0;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut player_39: reversi::Player = crate::reversi::Player::other(player_38);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_40: reversi::Player = crate::reversi::Player::Player1;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_41: reversi::Player = crate::reversi::Player::Player1;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_42: reversi::Player = crate::reversi::Player::Player1;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut player_43: reversi::Player = crate::reversi::Player::Player1;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut option_64: std::option::Option<(usize, usize)> = crate::reversi::Reversi::check_occupied_line_in_direction(reversi_0_ref_0, usize_1, usize_0, direction_0, player_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4238() {
//    rusty_monitor::set_test_id(4238);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_4, option_3, option_2];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_7, option_6, option_5];
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut player_6_ref_0: &connect_four::Player = &mut player_6;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 7usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    crate::reversi::Direction::iter();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_201() {
//    rusty_monitor::set_test_id(201);
    let mut usize_0: usize = 7usize;
    let mut usize_1: usize = 4usize;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_1);
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_5);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut player_9: reversi::Player = crate::reversi::Player::other(player_8);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut player_11: reversi::Player = crate::reversi::Player::other(player_10);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut player_14: reversi::Player = crate::reversi::Player::other(player_13);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player1;
    let mut player_19: reversi::Player = crate::reversi::Player::other(player_18);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut player_24: reversi::Player = crate::reversi::Player::other(player_23);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut player_27: reversi::Player = crate::reversi::Player::other(player_26);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut player_29: reversi::Player = crate::reversi::Player::other(player_28);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut player_30: reversi::Player = crate::reversi::Player::Player1;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player1;
    let mut player_32: reversi::Player = crate::reversi::Player::other(player_31);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut player_35: reversi::Player = crate::reversi::Player::other(player_34);
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player0;
    let mut player_37: reversi::Player = crate::reversi::Player::other(player_36);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut option_64: &std::option::Option<reversi::Player> = crate::reversi::Reversi::get(reversi_0_ref_0, usize_1, usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7608() {
//    rusty_monitor::set_test_id(7608);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut gamestate_5: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_5_ref_0: &reversi::GameState = &mut gamestate_5;
    let mut gamestate_6: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_6_ref_0: &reversi::GameState = &mut gamestate_6;
    let mut gamestate_7: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_7_ref_0: &reversi::GameState = &mut gamestate_7;
    let mut gamestate_8: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_8_ref_0: &reversi::GameState = &mut gamestate_8;
    let mut gamestate_9: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_10: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut gamestate_10_ref_0: &reversi::GameState = &mut gamestate_10;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_7_ref_0, gamestate_6_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_5_ref_0, gamestate_4_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_114() {
//    rusty_monitor::set_test_id(114);
    let mut usize_0: usize = 16usize;
    let mut usize_1: usize = 16usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut usize_2: usize = 4usize;
    let mut usize_3: usize = 2usize;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Right;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_4: usize = 3usize;
    let mut usize_5: usize = 63usize;
    let mut tuple_0: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
    let mut reversierror_2: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut player_5: reversi::Player = crate::reversi::Player::other(player_4);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_1_ref_0);
    let mut direction_1: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut player_5_ref_0: &reversi::Player = &mut player_5;
    let mut player_6: reversi::Player = std::clone::Clone::clone(player_5_ref_0);
    let mut reversierror_2_ref_0: &reversi::ReversiError = &mut reversierror_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(reversierror_2_ref_0, reversierror_0_ref_0);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_512() {
//    rusty_monitor::set_test_id(512);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut direction_2: reversi::Direction = crate::reversi::Direction::Upper;
    let mut direction_2_ref_0: &reversi::Direction = &mut direction_2;
    let mut direction_3: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut direction_3_ref_0: &reversi::Direction = &mut direction_3;
    let mut direction_4: reversi::Direction = crate::reversi::Direction::Right;
    let mut direction_4_ref_0: &reversi::Direction = &mut direction_4;
    let mut direction_5: reversi::Direction = crate::reversi::Direction::LowerLeft;
    let mut direction_5_ref_0: &reversi::Direction = &mut direction_5;
    let mut direction_6: reversi::Direction = crate::reversi::Direction::Left;
    let mut direction_6_ref_0: &reversi::Direction = &mut direction_6;
    let mut direction_7: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut direction_7_ref_0: &reversi::Direction = &mut direction_7;
    let mut direction_8: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut direction_8_ref_0: &reversi::Direction = &mut direction_8;
    let mut direction_9: reversi::Direction = crate::reversi::Direction::LowerLeft;
    let mut direction_9_ref_0: &reversi::Direction = &mut direction_9;
    let mut direction_10: reversi::Direction = crate::reversi::Direction::LowerRight;
    let mut direction_10_ref_0: &reversi::Direction = &mut direction_10;
    let mut tuple_0: (i8, i8) = crate::reversi::Direction::as_offset(direction_10_ref_0);
    let mut tuple_1: (i8, i8) = crate::reversi::Direction::as_offset(direction_9_ref_0);
    let mut tuple_2: (i8, i8) = crate::reversi::Direction::as_offset(direction_8_ref_0);
    let mut tuple_3: (i8, i8) = crate::reversi::Direction::as_offset(direction_7_ref_0);
    let mut tuple_4: (i8, i8) = crate::reversi::Direction::as_offset(direction_6_ref_0);
    let mut tuple_5: (i8, i8) = crate::reversi::Direction::as_offset(direction_5_ref_0);
    let mut tuple_6: (i8, i8) = crate::reversi::Direction::as_offset(direction_4_ref_0);
    let mut tuple_7: (i8, i8) = crate::reversi::Direction::as_offset(direction_3_ref_0);
    let mut tuple_8: (i8, i8) = crate::reversi::Direction::as_offset(direction_2_ref_0);
    let mut tuple_9: (i8, i8) = crate::reversi::Direction::as_offset(direction_1_ref_0);
    let mut tuple_10: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4319() {
//    rusty_monitor::set_test_id(4319);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut player_7: reversi::Player = crate::reversi::Player::other(player_6);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut player_8: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
    let mut player_12: reversi::Player = crate::reversi::Player::other(player_11);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_13: reversi::Player = crate::reversi::Player::Player0;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut player_15: reversi::Player = crate::reversi::Player::other(player_14);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_16: reversi::Player = crate::reversi::Player::Player0;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player0;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut player_19: reversi::Player = crate::reversi::Player::other(player_18);
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut player_21: reversi::Player = crate::reversi::Player::other(player_20);
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player0;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut player_26: reversi::Player = crate::reversi::Player::other(player_25);
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut player_28: reversi::Player = crate::reversi::Player::other(player_27);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_30: reversi::Player = crate::reversi::Player::Player1;
    let mut player_31: reversi::Player = crate::reversi::Player::other(player_30);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut player_33: reversi::Player = crate::reversi::Player::other(player_32);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut player_35: reversi::Player = crate::reversi::Player::Player0;
    let mut player_36: reversi::Player = crate::reversi::Player::other(player_35);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut player_37: reversi::Player = crate::reversi::Player::Player0;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut player_38: reversi::Player = crate::reversi::Player::Player1;
    let mut player_39: reversi::Player = crate::reversi::Player::other(player_38);
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut player_40: reversi::Player = crate::reversi::Player::Player0;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut player_41: reversi::Player = crate::reversi::Player::Player0;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_42: reversi::Player = crate::reversi::Player::Player0;
    let mut player_43: reversi::Player = crate::reversi::Player::other(player_42);
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut player_44: reversi::Player = crate::reversi::Player::Player0;
    let mut player_45: reversi::Player = crate::reversi::Player::other(player_44);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_45);
    let mut player_46: reversi::Player = crate::reversi::Player::Player0;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_46);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_47: reversi::Player = crate::reversi::Player::Player1;
    let mut player_48: reversi::Player = crate::reversi::Player::other(player_47);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_48);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_49: reversi::Player = crate::reversi::Player::Player1;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_49);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_50: reversi::Player = crate::reversi::Player::Player0;
    let mut player_51: reversi::Player = crate::reversi::Player::other(player_50);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_51);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_52: reversi::Player = crate::reversi::Player::Player0;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_52);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_53: reversi::Player = crate::reversi::Player::Player0;
    let mut player_54: reversi::Player = crate::reversi::Player::other(player_53);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_54);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut player_55: reversi::Player = crate::reversi::Player::Player0;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::Lower;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut player_56: reversi::Player = crate::reversi::Player::Player1;
    let mut player_57: reversi::Player = crate::reversi::Player::Player0;
    let mut player_58: reversi::Player = crate::reversi::Player::other(player_55);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_58);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut gamestate_5: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_5_ref_0: &reversi::GameState = &mut gamestate_5;
    let mut gamestate_6: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_6_ref_0: &reversi::GameState = &mut gamestate_6;
    let mut gamestate_7: reversi::GameState = crate::reversi::GameState::Win(player_56);
    let mut gamestate_7_ref_0: &reversi::GameState = &mut gamestate_7;
    let mut gamestate_8: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_8_ref_0: &reversi::GameState = &mut gamestate_8;
    let mut gamestate_9: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_9_ref_0: &reversi::GameState = &mut gamestate_9;
    let mut player_59: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_10: reversi::GameState = crate::reversi::GameState::Win(player_59);
    let mut gamestate_10_ref_0: &reversi::GameState = &mut gamestate_10;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_10_ref_0, gamestate_9_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_7_ref_0, gamestate_6_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(gamestate_5_ref_0, gamestate_4_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(direction_1_ref_0, direction_0_ref_0);
    let mut bool_5: bool = crate::reversi::Reversi::can_player_move(reversi_0_ref_0, player_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_677() {
//    rusty_monitor::set_test_id(677);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut usize_0: usize = 2usize;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut usize_1: usize = 86usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_9: connect_four::Player = crate::connect_four::Player::other(player_8);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: reversi::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_271() {
//    rusty_monitor::set_test_id(271);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_1);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut player_9: reversi::Player = crate::reversi::Player::other(player_8);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut player_17: reversi::Player = crate::reversi::Player::other(player_16);
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut player_20: reversi::Player = crate::reversi::Player::other(player_19);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_27: reversi::Player = crate::reversi::Player::Player0;
    let mut player_28: reversi::Player = crate::reversi::Player::other(player_27);
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut player_31: reversi::Player = crate::reversi::Player::other(player_30);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut player_34: reversi::Player = crate::reversi::Player::other(player_33);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_35: reversi::Player = crate::reversi::Player::Player0;
    let mut player_36: reversi::Player = crate::reversi::Player::other(player_35);
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_37: reversi::Player = crate::reversi::Player::Player0;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_39: reversi::Player = crate::reversi::Player::Player1;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_40: reversi::Player = crate::reversi::Player::Player1;
    let mut player_41: reversi::Player = crate::reversi::Player::other(player_40);
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_42: reversi::Player = crate::reversi::Player::Player0;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_43: reversi::Player = crate::reversi::Player::Player1;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_44: reversi::Player = crate::reversi::Player::Player1;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_44);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut gamestate_1: &reversi::GameState = crate::reversi::Reversi::status(reversi_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_312() {
//    rusty_monitor::set_test_id(312);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut player_5: reversi::Player = crate::reversi::Player::other(player_4);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut player_9: reversi::Player = crate::reversi::Player::other(player_8);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut player_19: reversi::Player = crate::reversi::Player::other(player_18);
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player1;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut player_24: reversi::Player = crate::reversi::Player::other(player_23);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut player_26: reversi::Player = crate::reversi::Player::other(player_25);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut player_29: reversi::Player = crate::reversi::Player::other(player_28);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut player_31: reversi::Player = crate::reversi::Player::other(player_30);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player1;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut player_35: reversi::Player = crate::reversi::Player::other(player_34);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_37: reversi::Player = crate::reversi::Player::Player0;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_39: reversi::Player = crate::reversi::Player::Player1;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut player_40: reversi::Player = crate::reversi::Player::Player0;
    let mut player_41: reversi::Player = crate::reversi::Player::other(player_40);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_42: reversi::Player = crate::reversi::Player::Player0;
    let mut player_43: reversi::Player = crate::reversi::Player::other(player_42);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut player_44: reversi::Player = crate::reversi::Player::Player0;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_44);
    let mut player_45: reversi::Player = crate::reversi::Player::Player0;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_45);
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_46: reversi::Player = crate::reversi::Player::Player1;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_46);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut bool_0: bool = crate::reversi::Reversi::can_player_move(reversi_0_ref_0, player_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_643() {
//    rusty_monitor::set_test_id(643);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut usize_0: usize = 5usize;
    let mut usize_1: usize = 8usize;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_7);
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut player_9: reversi::Player = crate::reversi::Player::Player0;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut player_12: reversi::Player = crate::reversi::Player::other(player_11);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut player_14: reversi::Player = crate::reversi::Player::other(player_13);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut player_16: reversi::Player = crate::reversi::Player::other(player_15);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut player_18: reversi::Player = crate::reversi::Player::Player1;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut player_21: reversi::Player = crate::reversi::Player::other(player_20);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_25: reversi::Player = crate::reversi::Player::Player1;
    let mut player_26: reversi::Player = crate::reversi::Player::other(player_25);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_30: reversi::Player = crate::reversi::Player::Player1;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player1;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut player_33: reversi::Player = crate::reversi::Player::other(player_32);
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut player_35: reversi::Player = crate::reversi::Player::other(player_34);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_37: reversi::Player = crate::reversi::Player::Player0;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_38: reversi::Player = crate::reversi::Player::Player1;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut result_0: std::result::Result<(), reversi::ReversiError> = crate::reversi::Reversi::simple_check_position_validity(reversi_0_ref_0, usize_1, usize_0, player_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_779() {
//    rusty_monitor::set_test_id(779);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut usize_0: usize = 0usize;
    let mut usize_1: usize = 6usize;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut player_7: reversi::Player = crate::reversi::Player::other(player_6);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut player_9: reversi::Player = crate::reversi::Player::other(player_8);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player0;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut player_18: reversi::Player = crate::reversi::Player::other(player_17);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player1;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut player_22: reversi::Player = crate::reversi::Player::other(player_21);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut player_24: reversi::Player = crate::reversi::Player::other(player_23);
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut player_26: reversi::Player = crate::reversi::Player::other(player_25);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut player_29: reversi::Player = crate::reversi::Player::Player0;
    let mut player_30: reversi::Player = crate::reversi::Player::other(player_29);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut player_34: reversi::Player = crate::reversi::Player::other(player_33);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut result_0: std::result::Result<(), reversi::ReversiError> = crate::reversi::Reversi::check_position_validity(reversi_0_ref_0, usize_1, usize_0, player_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3804() {
//    rusty_monitor::set_test_id(3804);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut gamestate_5: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_6: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_6_ref_0: &reversi::GameState = &mut gamestate_6;
    let mut gamestate_7: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_7_ref_0: &reversi::GameState = &mut gamestate_7;
    let mut gamestate_8: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_8_ref_0: &reversi::GameState = &mut gamestate_8;
    let mut gamestate_9: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_9_ref_0: &reversi::GameState = &mut gamestate_9;
    let mut gamestate_10: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_10_ref_0: &reversi::GameState = &mut gamestate_10;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_11: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut gamestate_11_ref_0: &reversi::GameState = &mut gamestate_11;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_11_ref_0, gamestate_10_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_9_ref_0, gamestate_8_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(gamestate_7_ref_0, gamestate_6_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_0_ref_0, gamestate_4_ref_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1630() {
//    rusty_monitor::set_test_id(1630);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_1);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_5);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
    let mut player_12: reversi::Player = crate::reversi::Player::other(player_11);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut player_13: reversi::Player = crate::reversi::Player::Player0;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player0;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player1;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player0;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut player_27: reversi::Player = crate::reversi::Player::other(player_26);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut player_29: reversi::Player = crate::reversi::Player::other(player_28);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_30: reversi::Player = crate::reversi::Player::Player1;
    let mut player_31: reversi::Player = crate::reversi::Player::other(player_30);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player1;
    let mut player_34: reversi::Player = crate::reversi::Player::other(player_33);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_35: reversi::Player = crate::reversi::Player::Player0;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut bool_0: bool = std::cmp::PartialEq::eq(reversierror_1_ref_0, reversierror_0_ref_0);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut option_64: std::option::Option<reversi::Player> = crate::reversi::Reversi::winner(reversi_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8126() {
//    rusty_monitor::set_test_id(8126);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Upper;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_4_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_231() {
//    rusty_monitor::set_test_id(231);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Lower;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::Lower;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut direction_2: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut direction_2_ref_0: &reversi::Direction = &mut direction_2;
    let mut direction_3: reversi::Direction = crate::reversi::Direction::LowerLeft;
    let mut direction_3_ref_0: &reversi::Direction = &mut direction_3;
    let mut direction_4: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut direction_4_ref_0: &reversi::Direction = &mut direction_4;
    let mut direction_5: reversi::Direction = crate::reversi::Direction::Upper;
    let mut direction_5_ref_0: &reversi::Direction = &mut direction_5;
    let mut direction_6: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut direction_6_ref_0: &reversi::Direction = &mut direction_6;
    let mut direction_7: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut direction_7_ref_0: &reversi::Direction = &mut direction_7;
    let mut direction_8: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut direction_8_ref_0: &reversi::Direction = &mut direction_8;
    let mut direction_9: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut direction_9_ref_0: &reversi::Direction = &mut direction_9;
    let mut direction_10: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut direction_10_ref_0: &reversi::Direction = &mut direction_10;
    let mut tuple_0: (i8, i8) = crate::reversi::Direction::as_offset(direction_10_ref_0);
    let mut tuple_1: (i8, i8) = crate::reversi::Direction::as_offset(direction_9_ref_0);
    let mut tuple_2: (i8, i8) = crate::reversi::Direction::as_offset(direction_8_ref_0);
    let mut tuple_3: (i8, i8) = crate::reversi::Direction::as_offset(direction_7_ref_0);
    let mut tuple_4: (i8, i8) = crate::reversi::Direction::as_offset(direction_6_ref_0);
    let mut tuple_5: (i8, i8) = crate::reversi::Direction::as_offset(direction_5_ref_0);
    let mut tuple_6: (i8, i8) = crate::reversi::Direction::as_offset(direction_4_ref_0);
    let mut tuple_7: (i8, i8) = crate::reversi::Direction::as_offset(direction_3_ref_0);
    let mut tuple_8: (i8, i8) = crate::reversi::Direction::as_offset(direction_2_ref_0);
    let mut tuple_9: (i8, i8) = crate::reversi::Direction::as_offset(direction_1_ref_0);
    let mut tuple_10: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_282() {
//    rusty_monitor::set_test_id(282);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_1: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_2: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_3: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_4: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_5: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_6: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_7: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_8: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_9: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_10: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_11: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_12: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_13: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_14: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_15: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_16: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_17: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_18: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_19: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_20: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_21: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_22: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_23: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_24: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_25: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_26: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_27: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_28: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut result_29: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_541() {
//    rusty_monitor::set_test_id(541);
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut player_1_ref_0: &gomoku::Player = &mut player_1;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_3: gomoku::Player = crate::gomoku::Player::other(player_2);
    let mut player_3_ref_0: &gomoku::Player = &mut player_3;
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Win;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_139() {
//    rusty_monitor::set_test_id(139);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut usize_0: usize = 16usize;
    let mut usize_1: usize = 6usize;
    let mut usize_2: usize = 7usize;
    let mut usize_3: usize = 8usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut usize_4: usize = 15usize;
    let mut usize_5: usize = 0usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1_ref_0: &reversi::Player = &mut player_1;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut reversierror_2: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_2_ref_0: &reversi::ReversiError = &mut reversierror_2;
    let mut reversierror_3: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_3_ref_0: &reversi::ReversiError = &mut reversierror_3;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_3_ref_0);
    let mut reversierror_4: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut reversierror_4_ref_0: &reversi::ReversiError = &mut reversierror_4;
    let mut bool_0: bool = std::cmp::PartialEq::eq(reversierror_4_ref_0, reversierror_2_ref_0);
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut player_0_ref_0: &gomoku::Player = &mut player_0;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5555() {
//    rusty_monitor::set_test_id(5555);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut player_5: reversi::Player = crate::reversi::Player::other(player_4);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut player_10: reversi::Player = crate::reversi::Player::other(player_9);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Win(player_10);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_56() {
//    rusty_monitor::set_test_id(56);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Right;
    let mut usize_0: usize = 6usize;
    let mut usize_1: usize = 52usize;
    let mut usize_2: usize = 15usize;
    let mut usize_3: usize = 0usize;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut player_7: reversi::Player = crate::reversi::Player::other(player_6);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut player_8: reversi::Player = crate::reversi::Player::Player1;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut player_10: reversi::Player = crate::reversi::Player::other(player_9);
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut player_12: reversi::Player = crate::reversi::Player::other(player_11);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut player_16: reversi::Player = crate::reversi::Player::other(player_15);
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player1;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut player_25: reversi::Player = crate::reversi::Player::Player1;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player0;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut player_31: reversi::Player = crate::reversi::Player::other(player_30);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player1;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_35: reversi::Player = crate::reversi::Player::Player0;
    let mut player_36: reversi::Player = crate::reversi::Player::other(player_35);
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_37: reversi::Player = crate::reversi::Player::Player1;
    let mut player_38: reversi::Player = crate::reversi::Player::other(player_37);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut player_39: reversi::Player = crate::reversi::Player::Player1;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &mut crate::reversi::Reversi = &mut reversi_0;
    crate::reversi::Reversi::flip(reversi_0_ref_0, usize_3, usize_2, usize_1, usize_0, direction_0, player_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_319() {
//    rusty_monitor::set_test_id(319);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut reversierror_2: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_2_ref_0: &reversi::ReversiError = &mut reversierror_2;
    let mut reversierror_3: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut reversierror_3_ref_0: &reversi::ReversiError = &mut reversierror_3;
    let mut reversierror_4: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut reversierror_4_ref_0: &reversi::ReversiError = &mut reversierror_4;
    let mut reversierror_5: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_5_ref_0: &reversi::ReversiError = &mut reversierror_5;
    let mut reversierror_6: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_6_ref_0: &reversi::ReversiError = &mut reversierror_6;
    let mut reversierror_7: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_7_ref_0: &reversi::ReversiError = &mut reversierror_7;
    let mut reversierror_8: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_8_ref_0: &reversi::ReversiError = &mut reversierror_8;
    let mut reversierror_9: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_9_ref_0: &reversi::ReversiError = &mut reversierror_9;
    let mut reversierror_10: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_10_ref_0: &reversi::ReversiError = &mut reversierror_10;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_10_ref_0);
    let mut option_1: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_9_ref_0);
    let mut option_2: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_8_ref_0);
    let mut option_3: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_7_ref_0);
    let mut option_4: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_6_ref_0);
    let mut option_5: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_5_ref_0);
    let mut option_6: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_4_ref_0);
    let mut option_7: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_3_ref_0);
    let mut option_8: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_2_ref_0);
    let mut option_9: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_1_ref_0);
    let mut option_10: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_60() {
//    rusty_monitor::set_test_id(60);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut player_1_ref_0: &reversi::Player = &mut player_1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_0: usize = 19usize;
    let mut usize_1: usize = 74usize;
    let mut usize_2: usize = 1usize;
    let mut usize_3: usize = 7usize;
    let mut usize_4: usize = 8usize;
    let mut usize_5: usize = 15usize;
    let mut usize_6: usize = 8usize;
    let mut usize_7: usize = 30usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_8: usize = 2usize;
    let mut usize_9: usize = 3usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_10: usize = 8usize;
    let mut usize_11: usize = 6usize;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut player_2_ref_0: &reversi::Player = &mut player_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(player_2_ref_0, player_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_513() {
//    rusty_monitor::set_test_id(513);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_5);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut player_10: reversi::Player = crate::reversi::Player::other(player_9);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut player_16: reversi::Player = crate::reversi::Player::other(player_15);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut player_20: reversi::Player = crate::reversi::Player::Player1;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player0;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut player_26: reversi::Player = crate::reversi::Player::other(player_25);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player0;
    let mut player_30: reversi::Player = crate::reversi::Player::other(player_29);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player1;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut player_35: reversi::Player = crate::reversi::Player::Player0;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player0;
    let mut player_37: reversi::Player = crate::reversi::Player::other(player_36);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_39: reversi::Player = crate::reversi::Player::Player1;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut player_40: reversi::Player = crate::reversi::Player::Player1;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut player_41: reversi::Player = crate::reversi::Reversi::get_next_player(reversi_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2835() {
//    rusty_monitor::set_test_id(2835);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_6, option_5, option_4];
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_7: connect_four::Player = crate::connect_four::Player::other(player_6);
    let mut player_7_ref_0: &connect_four::Player = &mut player_7;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 7usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut option_7: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_491() {
//    rusty_monitor::set_test_id(491);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut gamestate_5: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_5_ref_0: &reversi::GameState = &mut gamestate_5;
    let mut gamestate_6: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_6_ref_0: &reversi::GameState = &mut gamestate_6;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_7: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut gamestate_7_ref_0: &reversi::GameState = &mut gamestate_7;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_8: reversi::GameState = crate::reversi::GameState::Win(player_3);
    let mut gamestate_8_ref_0: &reversi::GameState = &mut gamestate_8;
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
#[timeout(30000)]fn rusty_test_9110() {
//    rusty_monitor::set_test_id(9110);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Upper;
    let mut usize_0: usize = 2usize;
    let mut usize_1: usize = 45usize;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_8: reversi::Player = crate::reversi::Player::Player1;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player0;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut player_11: reversi::Player = crate::reversi::Player::other(player_10);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut player_15: reversi::Player = crate::reversi::Player::other(player_14);
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_17: reversi::Player = crate::reversi::Player::Player0;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut player_20: reversi::Player = crate::reversi::Player::other(player_19);
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut player_28: reversi::Player = crate::reversi::Player::other(player_27);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut player_30: reversi::Player = crate::reversi::Player::other(player_29);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut player_35: reversi::Player = crate::reversi::Player::Player0;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut player_37: reversi::Player = crate::reversi::Player::Player0;
    let mut player_38: reversi::Player = crate::reversi::Player::other(player_37);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut option_64: std::option::Option<(usize, usize)> = crate::reversi::Reversi::check_occupied_line_in_direction(reversi_0_ref_0, usize_1, usize_0, direction_0, player_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4710() {
//    rusty_monitor::set_test_id(4710);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::Lower;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut gamestate_5: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_5_ref_0: &reversi::GameState = &mut gamestate_5;
    let mut gamestate_6: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_6_ref_0: &reversi::GameState = &mut gamestate_6;
    let mut gamestate_7: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_7_ref_0: &reversi::GameState = &mut gamestate_7;
    let mut gamestate_8: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_8_ref_0: &reversi::GameState = &mut gamestate_8;
    let mut gamestate_9: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_9_ref_0: &reversi::GameState = &mut gamestate_9;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_10: reversi::GameState = crate::reversi::GameState::Win(player_3);
    let mut gamestate_10_ref_0: &reversi::GameState = &mut gamestate_10;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_8_ref_0, gamestate_7_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(direction_1_ref_0, direction_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_278() {
//    rusty_monitor::set_test_id(278);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_5: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut gamestate_5_ref_0: &reversi::GameState = &mut gamestate_5;
    let mut gamestate_6: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_6_ref_0: &reversi::GameState = &mut gamestate_6;
    let mut gamestate_7: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_7_ref_0: &reversi::GameState = &mut gamestate_7;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_8: reversi::GameState = crate::reversi::GameState::Win(player_3);
    let mut gamestate_8_ref_0: &reversi::GameState = &mut gamestate_8;
    let mut gamestate_9: reversi::GameState = std::clone::Clone::clone(gamestate_8_ref_0);
    let mut gamestate_10: reversi::GameState = std::clone::Clone::clone(gamestate_7_ref_0);
    let mut gamestate_11: reversi::GameState = std::clone::Clone::clone(gamestate_6_ref_0);
    let mut gamestate_12: reversi::GameState = std::clone::Clone::clone(gamestate_5_ref_0);
    let mut gamestate_13: reversi::GameState = std::clone::Clone::clone(gamestate_4_ref_0);
    let mut gamestate_14: reversi::GameState = std::clone::Clone::clone(gamestate_3_ref_0);
    let mut gamestate_15: reversi::GameState = std::clone::Clone::clone(gamestate_2_ref_0);
    let mut gamestate_16: reversi::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut gamestate_17: reversi::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3112() {
//    rusty_monitor::set_test_id(3112);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_9: connect_four::Player = crate::connect_four::Player::other(player_8);
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_0_ref_0);
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3676() {
//    rusty_monitor::set_test_id(3676);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_5);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut player_9: reversi::Player = crate::reversi::Player::other(player_8);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut player_12: reversi::Player = crate::reversi::Player::other(player_11);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut player_14: reversi::Player = crate::reversi::Player::other(player_13);
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut player_17: reversi::Player = crate::reversi::Player::Player0;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut player_19: reversi::Player = crate::reversi::Player::other(player_18);
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player1;
    let mut player_21: reversi::Player = crate::reversi::Player::other(player_20);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut player_23: reversi::Player = crate::reversi::Player::other(player_22);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player0;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut player_31: reversi::Player = crate::reversi::Player::other(player_30);
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player1;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut player_35: reversi::Player = crate::reversi::Player::other(player_34);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player0;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_37: reversi::Player = crate::reversi::Player::Player1;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_39: reversi::Player = crate::reversi::Player::Player0;
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_40: reversi::Player = crate::reversi::Player::Player1;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut player_41: reversi::Player = crate::reversi::Player::Player0;
    let mut player_42: reversi::Player = crate::reversi::Player::other(player_41);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut option_64: std::option::Option<reversi::Player> = crate::reversi::Reversi::winner(reversi_0_ref_0);
    let mut usize_0: usize = 8usize;
    let mut usize_1: usize = 7usize;
    let mut player_43: reversi::Player = crate::reversi::Player::Player0;
    let mut player_44: reversi::Player = crate::reversi::Player::other(player_43);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_45: reversi::Player = crate::reversi::Player::Player0;
    let mut option_65: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_46: reversi::Player = crate::reversi::Player::Player1;
    let mut player_47: reversi::Player = crate::reversi::Player::other(player_46);
    let mut option_66: std::option::Option<reversi::Player> = std::option::Option::Some(player_47);
    let mut player_48: reversi::Player = crate::reversi::Player::Player0;
    let mut player_49: reversi::Player = crate::reversi::Player::other(player_48);
    let mut option_67: std::option::Option<reversi::Player> = std::option::Option::Some(player_49);
    let mut player_50: reversi::Player = crate::reversi::Player::Player1;
    let mut player_51: reversi::Player = crate::reversi::Player::other(player_50);
    let mut option_68: std::option::Option<reversi::Player> = std::option::Option::Some(player_51);
    let mut option_69: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_52: reversi::Player = crate::reversi::Player::Player1;
    let mut player_53: reversi::Player = crate::reversi::Player::other(player_52);
    let mut option_70: std::option::Option<reversi::Player> = std::option::Option::Some(player_53);
    let mut option_71: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_54: reversi::Player = crate::reversi::Player::Player0;
    let mut option_72: std::option::Option<reversi::Player> = std::option::Option::Some(player_54);
    let mut option_array_8: [std::option::Option<reversi::Player>; 8] = [option_72, option_71, option_70, option_69, option_68, option_67, option_66, option_65];
    let mut option_73: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_55: reversi::Player = crate::reversi::Player::Player0;
    let mut option_74: std::option::Option<reversi::Player> = std::option::Option::Some(player_55);
    let mut player_56: reversi::Player = crate::reversi::Player::Player1;
    let mut player_57: reversi::Player = crate::reversi::Player::other(player_56);
    let mut option_75: std::option::Option<reversi::Player> = std::option::Option::Some(player_57);
    let mut player_58: reversi::Player = crate::reversi::Player::Player0;
    let mut option_76: std::option::Option<reversi::Player> = std::option::Option::Some(player_58);
    let mut player_59: reversi::Player = crate::reversi::Player::Player1;
    let mut option_77: std::option::Option<reversi::Player> = std::option::Option::Some(player_59);
    let mut option_78: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_79: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_60: reversi::Player = crate::reversi::Player::Player0;
    let mut option_80: std::option::Option<reversi::Player> = std::option::Option::Some(player_60);
    let mut option_array_9: [std::option::Option<reversi::Player>; 8] = [option_80, option_79, option_78, option_77, option_76, option_75, option_74, option_73];
    let mut option_81: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_61: reversi::Player = crate::reversi::Player::Player0;
    let mut player_62: reversi::Player = crate::reversi::Player::other(player_61);
    let mut option_82: std::option::Option<reversi::Player> = std::option::Option::Some(player_62);
    let mut option_83: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_84: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_85: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_63: reversi::Player = crate::reversi::Player::Player0;
    let mut option_86: std::option::Option<reversi::Player> = std::option::Option::Some(player_63);
    let mut option_87: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_88: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_10: [std::option::Option<reversi::Player>; 8] = [option_88, option_87, option_86, option_85, option_84, option_83, option_82, option_81];
    let mut option_89: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_64: reversi::Player = crate::reversi::Player::Player0;
    let mut option_90: std::option::Option<reversi::Player> = std::option::Option::Some(player_64);
    let mut player_65: reversi::Player = crate::reversi::Player::Player0;
    let mut option_91: std::option::Option<reversi::Player> = std::option::Option::Some(player_65);
    let mut player_66: reversi::Player = crate::reversi::Player::Player1;
    let mut player_67: reversi::Player = crate::reversi::Player::other(player_66);
    let mut option_92: std::option::Option<reversi::Player> = std::option::Option::Some(player_67);
    let mut player_68: reversi::Player = crate::reversi::Player::Player1;
    let mut player_69: reversi::Player = crate::reversi::Player::other(player_68);
    let mut option_93: std::option::Option<reversi::Player> = std::option::Option::Some(player_69);
    let mut option_94: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_70: reversi::Player = crate::reversi::Player::Player0;
    let mut option_95: std::option::Option<reversi::Player> = std::option::Option::Some(player_70);
    let mut option_96: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_11: [std::option::Option<reversi::Player>; 8] = [option_96, option_95, option_94, option_93, option_92, option_91, option_90, option_89];
    let mut option_97: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_98: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_71: reversi::Player = crate::reversi::Player::Player0;
    let mut player_72: reversi::Player = crate::reversi::Player::other(player_71);
    let mut option_99: std::option::Option<reversi::Player> = std::option::Option::Some(player_72);
    let mut option_100: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_101: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_102: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_103: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_104: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_12: [std::option::Option<reversi::Player>; 8] = [option_104, option_103, option_102, option_64, option_100, option_99, option_98, option_97];
    let mut option_105: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_106: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_107: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_73: reversi::Player = crate::reversi::Player::Player0;
    let mut option_108: std::option::Option<reversi::Player> = std::option::Option::Some(player_73);
    let mut player_74: reversi::Player = crate::reversi::Player::Player1;
    let mut option_109: std::option::Option<reversi::Player> = std::option::Option::Some(player_74);
    let mut option_110: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_75: reversi::Player = crate::reversi::Player::Player1;
    let mut option_111: std::option::Option<reversi::Player> = std::option::Option::Some(player_75);
    let mut option_112: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_13: [std::option::Option<reversi::Player>; 8] = [option_112, option_111, option_110, option_109, option_108, option_107, option_106, option_105];
    let mut option_113: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_114: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_76: reversi::Player = crate::reversi::Player::Player0;
    let mut option_115: std::option::Option<reversi::Player> = std::option::Option::Some(player_76);
    let mut player_77: reversi::Player = crate::reversi::Player::Player1;
    let mut player_78: reversi::Player = crate::reversi::Player::other(player_77);
    let mut option_116: std::option::Option<reversi::Player> = std::option::Option::Some(player_78);
    let mut option_117: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_118: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_119: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_79: reversi::Player = crate::reversi::Player::Player1;
    let mut player_80: reversi::Player = crate::reversi::Player::other(player_79);
    let mut option_120: std::option::Option<reversi::Player> = std::option::Option::Some(player_80);
    let mut option_array_14: [std::option::Option<reversi::Player>; 8] = [option_120, option_119, option_118, option_117, option_116, option_115, option_114, option_113];
    let mut option_121: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_81: reversi::Player = crate::reversi::Player::Player0;
    let mut option_122: std::option::Option<reversi::Player> = std::option::Option::Some(player_81);
    let mut player_82: reversi::Player = crate::reversi::Player::Player0;
    let mut option_123: std::option::Option<reversi::Player> = std::option::Option::Some(player_82);
    let mut option_124: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_125: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_83: reversi::Player = crate::reversi::Player::Player1;
    let mut option_126: std::option::Option<reversi::Player> = std::option::Option::Some(player_83);
    let mut option_127: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_84: reversi::Player = crate::reversi::Player::Player0;
    let mut option_128: std::option::Option<reversi::Player> = std::option::Option::Some(player_84);
    let mut option_array_15: [std::option::Option<reversi::Player>; 8] = [option_128, option_127, option_126, option_125, option_124, option_123, option_122, option_121];
    let mut option_array_array_1: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_15, option_array_14, option_array_13, option_array_12, option_array_11, option_array_10, option_array_9, option_array_8];
    let mut reversi_1: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_1, next: player_45, status: gamestate_1};
    let mut reversi_1_ref_0: &mut crate::reversi::Reversi = &mut reversi_1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut result_0: std::result::Result<(), reversi::ReversiError> = crate::reversi::Reversi::place(reversi_1_ref_0, player_44, usize_1, usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9021() {
//    rusty_monitor::set_test_id(9021);
    let mut usize_0: usize = 8usize;
    let mut usize_1: usize = 7usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_5);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_7);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut player_10: reversi::Player = crate::reversi::Player::other(player_9);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut player_14: reversi::Player = crate::reversi::Player::other(player_13);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player0;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut player_19: reversi::Player = crate::reversi::Player::other(player_18);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut player_24: reversi::Player = crate::reversi::Player::other(player_23);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut player_25: reversi::Player = crate::reversi::Player::Player1;
    let mut player_26: reversi::Player = crate::reversi::Player::other(player_25);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_27: reversi::Player = crate::reversi::Player::Player0;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut player_29: reversi::Player = crate::reversi::Player::other(player_28);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut player_31: reversi::Player = crate::reversi::Player::Player1;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut player_35: reversi::Player = crate::reversi::Player::other(player_34);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut player_37: reversi::Player = crate::reversi::Player::other(player_36);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut player_39: reversi::Player = crate::reversi::Player::Player0;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_40: reversi::Player = crate::reversi::Player::Player1;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_41: reversi::Player = crate::reversi::Player::Player0;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut reversi_0_ref_0: &mut crate::reversi::Reversi = &mut reversi_0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut result_0: std::result::Result<(), reversi::ReversiError> = crate::reversi::Reversi::place(reversi_0_ref_0, player_1, usize_1, usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_89() {
//    rusty_monitor::set_test_id(89);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_7);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut player_12: reversi::Player = crate::reversi::Player::other(player_11);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_13: reversi::Player = crate::reversi::Player::Player0;
    let mut player_14: reversi::Player = crate::reversi::Player::other(player_13);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player1;
    let mut player_19: reversi::Player = crate::reversi::Player::other(player_18);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_20: reversi::Player = crate::reversi::Player::Player1;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player1;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut player_35: reversi::Player = crate::reversi::Player::Player1;
    let mut player_36: reversi::Player = crate::reversi::Player::other(player_35);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_36);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_1_ref_0);
    let mut reversi_1: crate::reversi::Reversi = std::clone::Clone::clone(reversi_0_ref_0);
//    panic!("From RustyUnit with love");
}
}