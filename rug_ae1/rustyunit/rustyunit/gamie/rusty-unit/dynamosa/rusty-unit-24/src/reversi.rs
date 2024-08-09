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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5400() {
    rusty_monitor::set_test_id(5400);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_0_ref_0: &reversi::Player = &mut player_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 44usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut usize_1: usize = 12usize;
    let mut bool_5: bool = false;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_5, mine_adjacent: usize_1, is_revealed: bool_4, is_flagged: bool_3};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut gomokuerror_1_ref_0: &gomoku::GomokuError = &mut gomokuerror_1;
    let mut gomokuerror_2: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_2_ref_0: &gomoku::GomokuError = &mut gomokuerror_2;
    let mut gomokuerror_3: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_3_ref_0: &gomoku::GomokuError = &mut gomokuerror_3;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut player_3_ref_0: &reversi::Player = &mut player_3;
    let mut bool_6: bool = std::cmp::PartialEq::eq(player_3_ref_0, player_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2303() {
    rusty_monitor::set_test_id(2303);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_1);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_11: connect_four::Player = crate::connect_four::Player::other(player_10);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8453() {
    rusty_monitor::set_test_id(8453);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_1);
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_5);
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_8: reversi::Player = crate::reversi::Player::Player1;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player1;
    let mut player_19: reversi::Player = crate::reversi::Player::other(player_18);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut player_20: reversi::Player = crate::reversi::Player::Player1;
    let mut player_21: reversi::Player = crate::reversi::Player::other(player_20);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut player_23: reversi::Player = crate::reversi::Player::other(player_22);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player0;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut player_29: reversi::Player = crate::reversi::Player::other(player_28);
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut player_33: reversi::Player = crate::reversi::Player::other(player_32);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut player_35: reversi::Player = crate::reversi::Player::other(player_34);
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut player_37: reversi::Player = crate::reversi::Player::other(player_36);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut player_38: reversi::Player = crate::reversi::Player::Player1;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_39: reversi::Player = crate::reversi::Player::Player0;
    let mut player_40: reversi::Player = crate::reversi::Player::other(player_39);
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut player_41: reversi::Player = crate::reversi::Player::Player1;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_42: reversi::Player = crate::reversi::Player::Player0;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_43: reversi::Player = crate::reversi::Player::Player0;
    let mut player_44: reversi::Player = crate::reversi::Player::other(player_43);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_44);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_45: reversi::Player = crate::reversi::Player::Player1;
    let mut player_46: reversi::Player = crate::reversi::Player::other(player_45);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_46);
    let mut player_47: reversi::Player = crate::reversi::Player::Player0;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_47);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_48: reversi::Player = crate::reversi::Player::Player1;
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_48);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_49: reversi::Player = crate::reversi::Player::Player1;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_49);
    let mut player_50: reversi::Player = crate::reversi::Player::Player1;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_50);
    let mut player_51: reversi::Player = crate::reversi::Player::Player0;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_51);
    let mut player_52: reversi::Player = crate::reversi::Player::Player1;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_52);
    let mut player_53: reversi::Player = crate::reversi::Player::Player1;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_53);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut reversi_0_ref_0: &mut crate::reversi::Reversi = &mut reversi_0;
    crate::reversi::Reversi::check_state(reversi_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2115() {
    rusty_monitor::set_test_id(2115);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_1);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut usize_0: usize = 37usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut usize_1: usize = 38usize;
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_7);
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_11: connect_four::Player = crate::connect_four::Player::other(player_10);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_12);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut usize_2: usize = 43usize;
    let mut player_13: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_14: connect_four::Player = crate::connect_four::Player::other(player_13);
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::Some(player_14);
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::Some(player_15);
    let mut player_16: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::Some(player_16);
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut player_17: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_17_ref_0: &connect_four::Player = &mut player_17;
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut player_19: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_20: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_18_ref_0: &reversi::Player = &mut player_18;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_18_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2619() {
    rusty_monitor::set_test_id(2619);
    let mut usize_0: usize = 21usize;
    let mut usize_1: usize = 67usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_7);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut player_11: reversi::Player = crate::reversi::Player::other(player_10);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut player_15: reversi::Player = crate::reversi::Player::other(player_14);
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut player_20: reversi::Player = crate::reversi::Player::other(player_19);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player0;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player0;
    let mut player_30: reversi::Player = crate::reversi::Player::other(player_29);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut player_31: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_32: connect_four::Player = crate::connect_four::Player::other(player_31);
    let mut player_33: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_34: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_0_ref_0);
    let mut player_35: connect_four::Player = crate::connect_four::Player::Player1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut player_36: connect_four::Player = crate::connect_four::Player::other(player_35);
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::Win(player_34);
    let mut gamestate_3_ref_0: &gomoku::GameState = &mut gamestate_3;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::LowerLeft;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_37: connect_four::Player = crate::connect_four::Player::other(player_32);
    let mut option_64: &std::option::Option<reversi::Player> = crate::reversi::Reversi::get(reversi_0_ref_0, usize_1, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_745() {
    rusty_monitor::set_test_id(745);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut player_5: reversi::Player = crate::reversi::Player::other(player_4);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player0;
    let mut player_10: reversi::Player = crate::reversi::Player::other(player_9);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut player_12: reversi::Player = crate::reversi::Player::other(player_11);
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut player_19: reversi::Player = crate::reversi::Player::other(player_18);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut player_23: reversi::Player = crate::reversi::Player::other(player_22);
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut player_27: reversi::Player = crate::reversi::Player::other(player_26);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_29: reversi::Player = crate::reversi::Player::Player0;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_30: reversi::Player = crate::reversi::Player::Player1;
    let mut player_31: reversi::Player = crate::reversi::Player::other(player_30);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player1;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut player_35: reversi::Player = crate::reversi::Player::other(player_34);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_37: reversi::Player = crate::reversi::Player::Player0;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut player_38: reversi::Player = crate::reversi::Player::Player1;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut player_39: reversi::Player = crate::reversi::Player::Player0;
    let mut bool_0: bool = crate::reversi::Reversi::can_player_move(reversi_0_ref_0, player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2151() {
    rusty_monitor::set_test_id(2151);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Right;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_7);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_9: reversi::Player = crate::reversi::Player::Player0;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_13: reversi::Player = crate::reversi::Player::Player0;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut player_15: reversi::Player = crate::reversi::Player::other(player_14);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_16: reversi::Player = crate::reversi::Player::Player0;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut player_17: reversi::Player = crate::reversi::Player::Player0;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut player_18: reversi::Player = crate::reversi::Player::Player1;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut player_25: reversi::Player = crate::reversi::Player::Player1;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_27: reversi::Player = crate::reversi::Player::Player0;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut player_29: reversi::Player = crate::reversi::Player::Player0;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut player_30: reversi::Player = crate::reversi::Player::Player1;
    let mut player_31: reversi::Player = crate::reversi::Player::other(player_30);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut player_34: reversi::Player = crate::reversi::Player::other(player_33);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut player_35: reversi::Player = crate::reversi::Player::Player1;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_36: reversi::Player = crate::reversi::Player::Player0;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_37: reversi::Player = crate::reversi::Player::Player1;
    let mut player_38: reversi::Player = crate::reversi::Player::other(player_37);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut player_39: reversi::Player = crate::reversi::Player::Player0;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_40: reversi::Player = crate::reversi::Player::Player0;
    let mut player_41: reversi::Player = crate::reversi::Player::other(player_40);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_42: reversi::Player = crate::reversi::Player::Player0;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut player_43: reversi::Player = crate::reversi::Player::Player1;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_44: reversi::Player = crate::reversi::Player::Player0;
    let mut player_45: reversi::Player = crate::reversi::Player::other(player_44);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_45);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_46: reversi::Player = crate::reversi::Player::Player0;
    let mut player_47: reversi::Player = crate::reversi::Player::other(player_46);
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_47);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::Lower;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut usize_0: usize = 28usize;
    let mut usize_1: usize = 72usize;
    let mut player_48: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut player_49: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_50: gomoku::Player = crate::gomoku::Player::other(player_49);
    let mut player_51: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_64: std::option::Option<gomoku::Player> = std::option::Option::Some(player_51);
    let mut option_65: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_66: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_52: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_67: std::option::Option<gomoku::Player> = std::option::Option::Some(player_52);
    let mut player_53: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_54: gomoku::Player = crate::gomoku::Player::other(player_53);
    let mut option_68: std::option::Option<gomoku::Player> = std::option::Option::Some(player_54);
    let mut option_69: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_70: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_55: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_71: std::option::Option<gomoku::Player> = std::option::Option::Some(player_55);
    let mut player_56: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_72: std::option::Option<gomoku::Player> = std::option::Option::Some(player_56);
    let mut player_57: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_58: gomoku::Player = crate::gomoku::Player::other(player_57);
    let mut option_73: std::option::Option<gomoku::Player> = std::option::Option::Some(player_58);
    let mut option_74: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_59: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_60: gomoku::Player = crate::gomoku::Player::other(player_59);
    let mut option_75: std::option::Option<gomoku::Player> = std::option::Option::Some(player_60);
    let mut player_61: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_62: gomoku::Player = crate::gomoku::Player::other(player_61);
    let mut option_76: std::option::Option<gomoku::Player> = std::option::Option::Some(player_62);
    let mut option_77: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_78: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_array_8: [std::option::Option<gomoku::Player>; 15] = [option_78, option_77, option_76, option_75, option_74, option_73, option_72, option_71, option_70, option_69, option_68, option_67, option_66, option_65, option_64];
    let mut player_63: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_79: std::option::Option<gomoku::Player> = std::option::Option::Some(player_63);
    let mut player_64: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_80: std::option::Option<gomoku::Player> = std::option::Option::Some(player_64);
    let mut option_81: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_65: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_82: std::option::Option<gomoku::Player> = std::option::Option::Some(player_65);
    let mut player_66: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_67: gomoku::Player = crate::gomoku::Player::other(player_66);
    let mut option_83: std::option::Option<gomoku::Player> = std::option::Option::Some(player_67);
    let mut option_84: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_85: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_68: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_69: gomoku::Player = crate::gomoku::Player::other(player_68);
    let mut option_86: std::option::Option<gomoku::Player> = std::option::Option::Some(player_69);
    let mut player_70: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_87: std::option::Option<gomoku::Player> = std::option::Option::Some(player_70);
    let mut option_88: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_71: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_89: std::option::Option<gomoku::Player> = std::option::Option::Some(player_71);
    let mut player_72: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_73: gomoku::Player = crate::gomoku::Player::other(player_72);
    let mut option_90: std::option::Option<gomoku::Player> = std::option::Option::Some(player_73);
    let mut option_91: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_92: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_74: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_93: std::option::Option<gomoku::Player> = std::option::Option::Some(player_74);
    let mut option_array_9: [std::option::Option<gomoku::Player>; 15] = [option_93, option_92, option_91, option_90, option_89, option_88, option_87, option_86, option_85, option_84, option_83, option_82, option_81, option_80, option_79];
    let mut player_75: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_76: gomoku::Player = crate::gomoku::Player::other(player_75);
    let mut option_94: std::option::Option<gomoku::Player> = std::option::Option::Some(player_76);
    let mut player_77: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_95: std::option::Option<gomoku::Player> = std::option::Option::Some(player_77);
    let mut option_96: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_78: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_79: gomoku::Player = crate::gomoku::Player::other(player_78);
    let mut option_97: std::option::Option<gomoku::Player> = std::option::Option::Some(player_79);
    let mut player_80: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_98: std::option::Option<gomoku::Player> = std::option::Option::Some(player_80);
    let mut option_99: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_81: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_100: std::option::Option<gomoku::Player> = std::option::Option::Some(player_81);
    let mut option_101: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_102: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_103: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_104: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_105: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_82: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_106: std::option::Option<gomoku::Player> = std::option::Option::Some(player_82);
    let mut option_107: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_83: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_108: std::option::Option<gomoku::Player> = std::option::Option::Some(player_83);
    let mut option_array_10: [std::option::Option<gomoku::Player>; 15] = [option_108, option_107, option_106, option_105, option_104, option_103, option_102, option_101, option_100, option_99, option_98, option_97, option_96, option_95, option_94];
    let mut player_84: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_85: gomoku::Player = crate::gomoku::Player::other(player_84);
    let mut option_109: std::option::Option<gomoku::Player> = std::option::Option::Some(player_85);
    let mut option_110: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_86: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_87: gomoku::Player = crate::gomoku::Player::other(player_86);
    let mut option_111: std::option::Option<gomoku::Player> = std::option::Option::Some(player_87);
    let mut option_112: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_113: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_114: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_88: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_89: gomoku::Player = crate::gomoku::Player::other(player_88);
    let mut option_115: std::option::Option<gomoku::Player> = std::option::Option::Some(player_89);
    let mut player_90: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_116: std::option::Option<gomoku::Player> = std::option::Option::Some(player_90);
    let mut player_91: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_117: std::option::Option<gomoku::Player> = std::option::Option::Some(player_91);
    let mut option_118: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_92: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_93: gomoku::Player = crate::gomoku::Player::other(player_92);
    let mut option_119: std::option::Option<gomoku::Player> = std::option::Option::Some(player_93);
    let mut player_94: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_120: std::option::Option<gomoku::Player> = std::option::Option::Some(player_94);
    let mut option_121: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_122: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_123: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_array_11: [std::option::Option<gomoku::Player>; 15] = [option_123, option_122, option_121, option_120, option_119, option_118, option_117, option_116, option_115, option_114, option_113, option_112, option_111, option_110, option_109];
    let mut player_95: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_124: std::option::Option<gomoku::Player> = std::option::Option::Some(player_95);
    let mut player_96: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_97: gomoku::Player = crate::gomoku::Player::other(player_96);
    let mut option_125: std::option::Option<gomoku::Player> = std::option::Option::Some(player_97);
    let mut player_98: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_126: std::option::Option<gomoku::Player> = std::option::Option::Some(player_98);
    let mut option_127: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_128: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_129: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_99: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_100: gomoku::Player = crate::gomoku::Player::other(player_99);
    let mut option_130: std::option::Option<gomoku::Player> = std::option::Option::Some(player_100);
    let mut option_131: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_101: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_132: std::option::Option<gomoku::Player> = std::option::Option::Some(player_101);
    let mut player_102: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_133: std::option::Option<gomoku::Player> = std::option::Option::Some(player_102);
    let mut player_103: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_104: gomoku::Player = crate::gomoku::Player::other(player_103);
    let mut option_134: std::option::Option<gomoku::Player> = std::option::Option::Some(player_104);
    let mut player_105: gomoku::Player = crate::gomoku::Player::Player0;
    let mut tuple_0: (i8, i8) = crate::reversi::Direction::as_offset(direction_1_ref_0);
    let mut player_106: reversi::Player = crate::reversi::Player::Player1;
    let mut option_135: std::option::Option<(usize, usize)> = crate::reversi::Reversi::check_occupied_line_in_direction(reversi_0_ref_0, usize_0, usize_1, direction_0, player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7273() {
    rusty_monitor::set_test_id(7273);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 44usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut usize_1: usize = 12usize;
    let mut bool_5: bool = false;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_5, mine_adjacent: usize_1, is_revealed: bool_4, is_flagged: bool_3};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut gomokuerror_1_ref_0: &gomoku::GomokuError = &mut gomokuerror_1;
    let mut gomokuerror_2: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_2_ref_0: &gomoku::GomokuError = &mut gomokuerror_2;
    let mut gomokuerror_3: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_3_ref_0: &gomoku::GomokuError = &mut gomokuerror_3;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut player_2_ref_0: &reversi::Player = &mut player_2;
    let mut player_3: reversi::Player = std::clone::Clone::clone(player_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2635() {
    rusty_monitor::set_test_id(2635);
    let mut usize_0: usize = 53usize;
    let mut usize_1: usize = 11usize;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_2);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_6);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_7);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut u64_0: u64 = 29u64;
    let mut u64_1: u64 = 8u64;
    let mut steprng_0: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_0;
    let mut usize_2: usize = 88usize;
    let mut usize_3: usize = 43usize;
    let mut usize_4: usize = 28usize;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut result_0: std::result::Result<crate::minesweeper::Minesweeper, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::new(usize_4, usize_3, usize_2, steprng_0_ref_0);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: reversi::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_22() {
    rusty_monitor::set_test_id(22);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut player_5: reversi::Player = crate::reversi::Player::other(player_4);
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_8: reversi::Player = crate::reversi::Player::Player1;
    let mut player_9: reversi::Player = crate::reversi::Player::other(player_8);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut player_15: reversi::Player = crate::reversi::Player::other(player_14);
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player0;
    let mut player_17: reversi::Player = crate::reversi::Player::other(player_16);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut player_18: reversi::Player = crate::reversi::Player::Player1;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut player_21: reversi::Player = crate::reversi::Player::other(player_20);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut player_24: reversi::Player = crate::reversi::Player::other(player_23);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut player_25: reversi::Player = crate::reversi::Player::Player1;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player0;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player1;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut player_33: reversi::Player = crate::reversi::Player::other(player_32);
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_35: reversi::Player = crate::reversi::Player::Player1;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut player_37: reversi::Player = crate::reversi::Player::other(player_36);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut player_39: reversi::Player = crate::reversi::Player::other(player_38);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_40: reversi::Player = crate::reversi::Player::Player1;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_41: reversi::Player = crate::reversi::Player::Player1;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_42: reversi::Player = crate::reversi::Player::Player0;
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut player_43: reversi::Player = crate::reversi::Player::Player0;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut player_44: reversi::Player = crate::reversi::Player::Player0;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_44);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_45: reversi::Player = crate::reversi::Player::Player1;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_45);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_46: reversi::Player = crate::reversi::Player::Player1;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_46);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut gamestate_1: &reversi::GameState = crate::reversi::Reversi::status(reversi_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3892() {
    rusty_monitor::set_test_id(3892);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut usize_0: usize = 46usize;
    let mut usize_1: usize = 89usize;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_5);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_8: reversi::Player = crate::reversi::Player::Player1;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_9: reversi::Player = crate::reversi::Player::Player0;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut player_11: reversi::Player = crate::reversi::Player::other(player_10);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player1;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut player_20: reversi::Player = crate::reversi::Player::other(player_19);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut player_24: reversi::Player = crate::reversi::Player::other(player_23);
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_0);
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut player_28: reversi::Player = crate::reversi::Player::other(player_27);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut player_29: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player1;
    let mut player_32: reversi::Player = crate::reversi::Player::other(player_31);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player1;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_35: reversi::Player = crate::reversi::Player::Player0;
    let mut player_36: reversi::Player = crate::reversi::Player::other(player_35);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut player_37: reversi::Player = crate::reversi::Player::Player1;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut player_39: reversi::Player = crate::reversi::Player::Player1;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_2, status: gamestate_1};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut player_40: reversi::Player = crate::reversi::Player::Player0;
    let mut player_41: reversi::Player = crate::reversi::Player::other(player_40);
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Win(player_41);
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut gamestate_3: reversi::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Lower;
    let mut result_0: std::result::Result<(), reversi::ReversiError> = crate::reversi::Reversi::simple_check_position_validity(reversi_0_ref_0, usize_1, usize_0, player_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1580() {
    rusty_monitor::set_test_id(1580);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_1);
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_5);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_7);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
    let mut player_12: reversi::Player = crate::reversi::Player::other(player_11);
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut player_13: reversi::Player = crate::reversi::Player::Player0;
    let mut player_14: reversi::Player = crate::reversi::Player::other(player_13);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut player_18: reversi::Player = crate::reversi::Player::other(player_17);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut player_24: reversi::Player = crate::reversi::Player::other(player_23);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut player_26: reversi::Player = crate::reversi::Player::other(player_25);
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut player_33: reversi::Player = crate::reversi::Player::other(player_32);
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut player_35: reversi::Player = crate::reversi::Player::other(player_34);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut player_37: reversi::Player = crate::reversi::Player::other(player_36);
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut player_38: reversi::Player = crate::reversi::Player::Player1;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut player_39: reversi::Player = crate::reversi::Player::Player0;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut player_40: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_41: reversi::Player = crate::reversi::Player::Player0;
    let mut option_64: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_65: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_66: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_67: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_68: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_42: reversi::Player = crate::reversi::Player::Player1;
    let mut option_69: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut option_70: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_43: reversi::Player = crate::reversi::Player::Player1;
    let mut player_44: reversi::Player = crate::reversi::Player::other(player_43);
    let mut option_71: std::option::Option<reversi::Player> = std::option::Option::Some(player_44);
    let mut option_array_8: [std::option::Option<reversi::Player>; 8] = [option_71, option_70, option_69, option_68, option_67, option_66, option_65, option_64];
    let mut player_45: reversi::Player = crate::reversi::Player::Player1;
    let mut player_46: reversi::Player = crate::reversi::Player::other(player_45);
    let mut option_72: std::option::Option<reversi::Player> = std::option::Option::Some(player_46);
    let mut player_47: reversi::Player = crate::reversi::Player::Player0;
    let mut player_48: reversi::Player = crate::reversi::Player::other(player_47);
    let mut option_73: std::option::Option<reversi::Player> = std::option::Option::Some(player_48);
    let mut option_74: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_49: reversi::Player = crate::reversi::Player::Player1;
    let mut option_75: std::option::Option<reversi::Player> = std::option::Option::Some(player_49);
    let mut player_50: reversi::Player = crate::reversi::Player::Player0;
    let mut player_51: reversi::Player = crate::reversi::Player::other(player_50);
    let mut option_76: std::option::Option<reversi::Player> = std::option::Option::Some(player_51);
    let mut option_77: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_78: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_79: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_9: [std::option::Option<reversi::Player>; 8] = [option_79, option_78, option_77, option_76, option_75, option_74, option_73, option_72];
    let mut option_80: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_81: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_82: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_83: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_52: reversi::Player = crate::reversi::Player::Player1;
    let mut option_84: std::option::Option<reversi::Player> = std::option::Option::Some(player_52);
    let mut option_85: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_53: reversi::Player = crate::reversi::Player::Player1;
    let mut player_54: reversi::Player = crate::reversi::Player::other(player_53);
    let mut option_86: std::option::Option<reversi::Player> = std::option::Option::Some(player_54);
    let mut player_55: reversi::Player = crate::reversi::Player::Player1;
    let mut option_87: std::option::Option<reversi::Player> = std::option::Option::Some(player_55);
    let mut option_array_10: [std::option::Option<reversi::Player>; 8] = [option_87, option_86, option_85, option_84, option_83, option_82, option_81, option_80];
    let mut player_56: reversi::Player = crate::reversi::Player::Player1;
    let mut player_57: reversi::Player = crate::reversi::Player::other(player_56);
    let mut option_88: std::option::Option<reversi::Player> = std::option::Option::Some(player_57);
    let mut option_89: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_58: reversi::Player = crate::reversi::Player::Player0;
    let mut option_90: std::option::Option<reversi::Player> = std::option::Option::Some(player_58);
    let mut player_59: reversi::Player = crate::reversi::Player::Player0;
    let mut option_91: std::option::Option<reversi::Player> = std::option::Option::Some(player_59);
    let mut option_92: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_93: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_94: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_95: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_11: [std::option::Option<reversi::Player>; 8] = [option_95, option_94, option_93, option_92, option_91, option_90, option_89, option_88];
    let mut option_96: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_60: reversi::Player = crate::reversi::Player::Player1;
    let mut option_97: std::option::Option<reversi::Player> = std::option::Option::Some(player_60);
    let mut option_98: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_61: reversi::Player = crate::reversi::Player::Player0;
    let mut option_99: std::option::Option<reversi::Player> = std::option::Option::Some(player_61);
    let mut player_62: reversi::Player = crate::reversi::Player::Player1;
    let mut player_63: reversi::Player = crate::reversi::Player::other(player_62);
    let mut option_100: std::option::Option<reversi::Player> = std::option::Option::Some(player_63);
    let mut option_101: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_102: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_103: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_12: [std::option::Option<reversi::Player>; 8] = [option_103, option_102, option_101, option_100, option_99, option_98, option_97, option_96];
    let mut player_64: reversi::Player = crate::reversi::Player::Player1;
    let mut player_65: reversi::Player = crate::reversi::Player::other(player_64);
    let mut option_104: std::option::Option<reversi::Player> = std::option::Option::Some(player_65);
    let mut player_66: reversi::Player = crate::reversi::Player::Player1;
    let mut option_105: std::option::Option<reversi::Player> = std::option::Option::Some(player_66);
    let mut player_67: reversi::Player = crate::reversi::Player::Player0;
    let mut option_106: std::option::Option<reversi::Player> = std::option::Option::Some(player_67);
    let mut option_107: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_108: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_109: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_110: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_68: reversi::Player = crate::reversi::Player::Player1;
    let mut option_111: std::option::Option<reversi::Player> = std::option::Option::Some(player_68);
    let mut option_array_13: [std::option::Option<reversi::Player>; 8] = [option_111, option_110, option_109, option_108, option_107, option_106, option_105, option_104];
    let mut player_69: reversi::Player = crate::reversi::Player::Player1;
    let mut option_112: std::option::Option<reversi::Player> = std::option::Option::Some(player_69);
    let mut option_113: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_114: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_70: reversi::Player = crate::reversi::Player::Player0;
    let mut option_115: std::option::Option<reversi::Player> = std::option::Option::Some(player_70);
    let mut option_116: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_117: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_71: reversi::Player = crate::reversi::Player::Player1;
    let mut option_118: std::option::Option<reversi::Player> = std::option::Option::Some(player_71);
    let mut option_119: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_14: [std::option::Option<reversi::Player>; 8] = [option_119, option_118, option_117, option_116, option_115, option_114, option_113, option_112];
    let mut player_72: reversi::Player = crate::reversi::Player::Player1;
    let mut option_120: std::option::Option<reversi::Player> = std::option::Option::Some(player_72);
    let mut option_121: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_122: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_123: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_124: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_73: reversi::Player = crate::reversi::Player::Player1;
    let mut option_125: std::option::Option<reversi::Player> = std::option::Option::Some(player_73);
    let mut player_74: reversi::Player = crate::reversi::Player::Player0;
    let mut option_126: std::option::Option<reversi::Player> = std::option::Option::Some(player_74);
    let mut player_75: reversi::Player = crate::reversi::Player::Player0;
    let mut option_127: std::option::Option<reversi::Player> = std::option::Option::Some(player_75);
    let mut option_array_15: [std::option::Option<reversi::Player>; 8] = [option_127, option_126, option_125, option_124, option_123, option_122, option_121, option_120];
    let mut option_array_array_1: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_15, option_array_14, option_array_13, option_array_12, option_array_11, option_array_10, option_array_9, option_array_8];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut player_76: reversi::Player = crate::reversi::Player::Player1;
    let mut player_76_ref_0: &reversi::Player = &mut player_76;
    let mut player_77: reversi::Player = crate::reversi::Player::Player1;
    let mut player_77_ref_0: &reversi::Player = &mut player_77;
    let mut bool_0: bool = std::cmp::PartialEq::eq(player_77_ref_0, player_76_ref_0);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Upper;
    let mut bool_1: bool = crate::reversi::Reversi::can_player_move(reversi_0_ref_0, player_40);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7392() {
    rusty_monitor::set_test_id(7392);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 25usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    let mut gamestate_5: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut gamestate_5_ref_0: &gomoku::GameState = &mut gamestate_5;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_6: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut gamestate_6_ref_0: &reversi::GameState = &mut gamestate_6;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_6_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1017() {
    rusty_monitor::set_test_id(1017);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Left;
    let mut usize_0: usize = 93usize;
    let mut usize_1: usize = 75usize;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut player_5: reversi::Player = crate::reversi::Player::other(player_4);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut player_9: reversi::Player = crate::reversi::Player::Player0;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut player_11: reversi::Player = crate::reversi::Player::other(player_10);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut player_16: reversi::Player = crate::reversi::Player::other(player_15);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_17: reversi::Player = crate::reversi::Player::Player0;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_25: reversi::Player = crate::reversi::Player::Player1;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut player_30: reversi::Player = crate::reversi::Player::other(player_29);
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::Lower;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut option_64: std::option::Option<(usize, usize)> = crate::reversi::Reversi::check_occupied_line_in_direction(reversi_0_ref_0, usize_1, usize_0, direction_0, player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3292() {
    rusty_monitor::set_test_id(3292);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut player_5: reversi::Player = crate::reversi::Player::other(player_4);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut player_11: reversi::Player = crate::reversi::Player::other(player_10);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut player_15: reversi::Player = crate::reversi::Player::other(player_14);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player0;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut player_17: reversi::Player = crate::reversi::Player::Player0;
    let mut player_18: reversi::Player = crate::reversi::Player::other(player_17);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut player_20: reversi::Player = crate::reversi::Player::other(player_19);
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut player_27: reversi::Player = crate::reversi::Player::other(player_26);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_30: reversi::Player = crate::reversi::Player::Player1;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut player_35: reversi::Player = crate::reversi::Player::other(player_34);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player0;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_37: reversi::Player = crate::reversi::Player::Player0;
    let mut player_38: reversi::Player = crate::reversi::Player::other(player_37);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_39: reversi::Player = crate::reversi::Player::Player1;
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut player_40: reversi::Player = crate::reversi::Player::Player0;
    let mut player_41: reversi::Player = crate::reversi::Player::other(player_40);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_42: reversi::Player = crate::reversi::Player::Player1;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_43: reversi::Player = crate::reversi::Player::Player1;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut option_64: std::option::Option<reversi::Player> = crate::reversi::Reversi::winner(reversi_0_ref_0);
    let mut player_44: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_45: reversi::Player = crate::reversi::Player::Player0;
    let mut option_65: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_66: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_67: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_68: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_69: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_46: reversi::Player = crate::reversi::Player::Player1;
    let mut option_70: std::option::Option<reversi::Player> = std::option::Option::Some(player_46);
    let mut option_71: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_47: reversi::Player = crate::reversi::Player::Player1;
    let mut player_48: reversi::Player = crate::reversi::Player::other(player_47);
    let mut option_72: std::option::Option<reversi::Player> = std::option::Option::Some(player_48);
    let mut option_array_8: [std::option::Option<reversi::Player>; 8] = [option_72, option_71, option_70, option_69, option_68, option_67, option_66, option_65];
    let mut player_49: reversi::Player = crate::reversi::Player::Player1;
    let mut player_50: reversi::Player = crate::reversi::Player::other(player_49);
    let mut option_73: std::option::Option<reversi::Player> = std::option::Option::Some(player_50);
    let mut player_51: reversi::Player = crate::reversi::Player::Player0;
    let mut player_52: reversi::Player = crate::reversi::Player::other(player_51);
    let mut option_74: std::option::Option<reversi::Player> = std::option::Option::Some(player_52);
    let mut option_75: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_53: reversi::Player = crate::reversi::Player::Player1;
    let mut option_76: std::option::Option<reversi::Player> = std::option::Option::Some(player_53);
    let mut player_54: reversi::Player = crate::reversi::Player::Player0;
    let mut player_55: reversi::Player = crate::reversi::Player::other(player_54);
    let mut option_77: std::option::Option<reversi::Player> = std::option::Option::Some(player_55);
    let mut option_78: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_79: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_80: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_9: [std::option::Option<reversi::Player>; 8] = [option_80, option_79, option_78, option_77, option_76, option_75, option_74, option_73];
    let mut option_81: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_82: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_83: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_84: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_56: reversi::Player = crate::reversi::Player::Player1;
    let mut option_85: std::option::Option<reversi::Player> = std::option::Option::Some(player_56);
    let mut option_86: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_57: reversi::Player = crate::reversi::Player::Player1;
    let mut player_58: reversi::Player = crate::reversi::Player::other(player_57);
    let mut option_87: std::option::Option<reversi::Player> = std::option::Option::Some(player_58);
    let mut player_59: reversi::Player = crate::reversi::Player::Player1;
    let mut option_88: std::option::Option<reversi::Player> = std::option::Option::Some(player_59);
    let mut option_array_10: [std::option::Option<reversi::Player>; 8] = [option_88, option_87, option_86, option_85, option_84, option_83, option_82, option_81];
    let mut player_60: reversi::Player = crate::reversi::Player::Player1;
    let mut player_61: reversi::Player = crate::reversi::Player::other(player_60);
    let mut option_89: std::option::Option<reversi::Player> = std::option::Option::Some(player_61);
    let mut option_90: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_62: reversi::Player = crate::reversi::Player::Player0;
    let mut option_91: std::option::Option<reversi::Player> = std::option::Option::Some(player_62);
    let mut player_63: reversi::Player = crate::reversi::Player::Player0;
    let mut option_92: std::option::Option<reversi::Player> = std::option::Option::Some(player_63);
    let mut option_93: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_94: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_95: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_96: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_11: [std::option::Option<reversi::Player>; 8] = [option_96, option_95, option_94, option_93, option_92, option_91, option_90, option_89];
    let mut option_97: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_64: reversi::Player = crate::reversi::Player::Player1;
    let mut option_98: std::option::Option<reversi::Player> = std::option::Option::Some(player_64);
    let mut option_99: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_65: reversi::Player = crate::reversi::Player::Player0;
    let mut option_100: std::option::Option<reversi::Player> = std::option::Option::Some(player_65);
    let mut player_66: reversi::Player = crate::reversi::Player::Player1;
    let mut player_67: reversi::Player = crate::reversi::Player::other(player_66);
    let mut option_101: std::option::Option<reversi::Player> = std::option::Option::Some(player_67);
    let mut option_102: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_103: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_104: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_12: [std::option::Option<reversi::Player>; 8] = [option_104, option_103, option_102, option_101, option_100, option_99, option_98, option_97];
    let mut player_68: reversi::Player = crate::reversi::Player::Player1;
    let mut player_69: reversi::Player = crate::reversi::Player::other(player_68);
    let mut option_105: std::option::Option<reversi::Player> = std::option::Option::Some(player_69);
    let mut player_70: reversi::Player = crate::reversi::Player::Player1;
    let mut option_106: std::option::Option<reversi::Player> = std::option::Option::Some(player_70);
    let mut player_71: reversi::Player = crate::reversi::Player::Player0;
    let mut option_107: std::option::Option<reversi::Player> = std::option::Option::Some(player_71);
    let mut option_108: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_109: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_110: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_111: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_72: reversi::Player = crate::reversi::Player::Player1;
    let mut option_112: std::option::Option<reversi::Player> = std::option::Option::Some(player_72);
    let mut option_array_13: [std::option::Option<reversi::Player>; 8] = [option_112, option_111, option_110, option_109, option_64, option_107, option_106, option_108];
    let mut player_73: reversi::Player = crate::reversi::Player::Player1;
    let mut option_113: std::option::Option<reversi::Player> = std::option::Option::Some(player_73);
    let mut option_114: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_115: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_74: reversi::Player = crate::reversi::Player::Player0;
    let mut option_116: std::option::Option<reversi::Player> = std::option::Option::Some(player_74);
    let mut option_117: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_118: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_75: reversi::Player = crate::reversi::Player::Player1;
    let mut option_119: std::option::Option<reversi::Player> = std::option::Option::Some(player_75);
    let mut option_120: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_14: [std::option::Option<reversi::Player>; 8] = [option_120, option_119, option_118, option_117, option_116, option_115, option_114, option_113];
    let mut player_76: reversi::Player = crate::reversi::Player::Player1;
    let mut option_121: std::option::Option<reversi::Player> = std::option::Option::Some(player_76);
    let mut option_122: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_123: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_124: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_125: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_77: reversi::Player = crate::reversi::Player::Player1;
    let mut option_126: std::option::Option<reversi::Player> = std::option::Option::Some(player_77);
    let mut player_78: reversi::Player = crate::reversi::Player::Player0;
    let mut option_127: std::option::Option<reversi::Player> = std::option::Option::Some(player_78);
    let mut player_79: reversi::Player = crate::reversi::Player::Player0;
    let mut option_128: std::option::Option<reversi::Player> = std::option::Option::Some(player_79);
    let mut option_array_15: [std::option::Option<reversi::Player>; 8] = [option_128, option_127, option_126, option_125, option_124, option_123, option_122, option_121];
    let mut option_array_array_1: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_15, option_array_14, option_array_13, option_array_12, option_array_11, option_array_10, option_array_9, option_array_8];
    let mut reversi_1: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_1, next: player_45, status: gamestate_1};
    let mut reversi_1_ref_0: &crate::reversi::Reversi = &mut reversi_1;
    let mut player_80: reversi::Player = crate::reversi::Player::Player1;
    let mut player_80_ref_0: &reversi::Player = &mut player_80;
    let mut player_81: reversi::Player = crate::reversi::Player::Player1;
    let mut player_81_ref_0: &reversi::Player = &mut player_81;
    let mut bool_0: bool = std::cmp::PartialEq::eq(player_81_ref_0, player_80_ref_0);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Upper;
    let mut bool_1: bool = crate::reversi::Reversi::can_player_move(reversi_1_ref_0, player_44);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8281() {
    rusty_monitor::set_test_id(8281);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut usize_0: usize = 2usize;
    let mut usize_1: usize = 96usize;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_7);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut player_17: reversi::Player = crate::reversi::Player::other(player_16);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut player_23: reversi::Player = crate::reversi::Player::other(player_22);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut player_28: reversi::Player = crate::reversi::Player::other(player_27);
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player0;
    let mut player_30: reversi::Player = crate::reversi::Player::other(player_29);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_31: reversi::Player = crate::reversi::Player::Player1;
    let mut player_32: reversi::Player = crate::reversi::Player::other(player_31);
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut player_35: reversi::Player = crate::reversi::Player::other(player_34);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut player_37: reversi::Player = crate::reversi::Player::Player0;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_38: reversi::Player = crate::reversi::Player::Player1;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut player_39: reversi::Player = crate::reversi::Player::Player1;
    let mut player_40: reversi::Player = crate::reversi::Player::other(player_39);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut player_41: reversi::Player = crate::reversi::Player::Player0;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut player_42: reversi::Player = crate::reversi::Player::Player1;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_43: reversi::Player = crate::reversi::Player::Player0;
    let mut player_44: reversi::Player = crate::reversi::Player::other(player_43);
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_44);
    let mut player_45: reversi::Player = crate::reversi::Player::Player1;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_45);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_46: reversi::Player = crate::reversi::Player::Player1;
    let mut player_47: reversi::Player = crate::reversi::Player::other(player_46);
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_47);
    let mut player_48: reversi::Player = crate::reversi::Player::Player0;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_48);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_49: reversi::Player = crate::reversi::Player::Player1;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_49);
    let mut player_50: reversi::Player = crate::reversi::Player::Player0;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_50);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_51: reversi::Player = crate::reversi::Player::Player0;
    let mut player_52: reversi::Player = crate::reversi::Player::other(player_51);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_52);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::Upper;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut tuple_0: (i8, i8) = crate::reversi::Direction::as_offset(direction_1_ref_0);
    let mut player_53: reversi::Player = crate::reversi::Player::Player1;
    let mut option_64: std::option::Option<(usize, usize)> = crate::reversi::Reversi::check_occupied_line_in_direction(reversi_0_ref_0, usize_1, usize_0, direction_0, player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5508() {
    rusty_monitor::set_test_id(5508);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut usize_0: usize = 2usize;
    let mut usize_1: usize = 96usize;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut player_9: reversi::Player = crate::reversi::Player::other(player_8);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut player_14: reversi::Player = crate::reversi::Player::other(player_13);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut player_18: reversi::Player = crate::reversi::Player::other(player_17);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut player_20: reversi::Player = crate::reversi::Player::Player1;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut player_24: reversi::Player = crate::reversi::Player::other(player_23);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut player_25: reversi::Player = crate::reversi::Player::Player1;
    let mut player_26: reversi::Player = crate::reversi::Player::other(player_0);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut player_29: reversi::Player = crate::reversi::Player::other(player_28);
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut player_31: reversi::Player = crate::reversi::Player::other(player_30);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut player_33: reversi::Player = crate::reversi::Player::other(player_32);
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_35: reversi::Player = crate::reversi::Player::Player1;
    let mut player_36: reversi::Player = crate::reversi::Player::other(player_35);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut player_37: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_39: reversi::Player = crate::reversi::Player::Player1;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut player_40: reversi::Player = crate::reversi::Player::Player1;
    let mut player_41: reversi::Player = crate::reversi::Player::other(player_40);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut player_42: reversi::Player = crate::reversi::Player::Player0;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut player_43: reversi::Player = crate::reversi::Player::Player1;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_44: reversi::Player = crate::reversi::Player::Player0;
    let mut player_45: reversi::Player = crate::reversi::Player::other(player_44);
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_45);
    let mut player_46: reversi::Player = crate::reversi::Player::Player1;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_46);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_47: reversi::Player = crate::reversi::Player::Player1;
    let mut player_48: reversi::Player = crate::reversi::Player::other(player_47);
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_49: reversi::Player = crate::reversi::Player::Player0;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_49);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_50: reversi::Player = crate::reversi::Player::Player1;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_50);
    let mut player_51: reversi::Player = crate::reversi::Player::Player0;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_51);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_52: reversi::Player = crate::reversi::Player::Player0;
    let mut player_53: reversi::Player = crate::reversi::Player::other(player_52);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_53);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_3, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut direction_2: reversi::Direction = crate::reversi::Direction::Upper;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut tuple_0: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
    let mut player_54: reversi::Player = crate::reversi::Player::Player1;
    let mut option_64: std::option::Option<(usize, usize)> = crate::reversi::Reversi::check_occupied_line_in_direction(reversi_0_ref_0, usize_1, usize_0, direction_1, player_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1957() {
    rusty_monitor::set_test_id(1957);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut bool_0: bool = true;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut usize_0: usize = 25usize;
    let mut bool_4: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_4, mine_adjacent: usize_0, is_revealed: bool_3, is_flagged: bool_2};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_2);
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut gamestate_4: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut gamestate_4_ref_0: &gomoku::GameState = &mut gamestate_4;
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_5: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut gamestate_5_ref_0: &reversi::GameState = &mut gamestate_5;
    let mut bool_5: bool = std::cmp::PartialEq::ne(gamestate_5_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5994() {
    rusty_monitor::set_test_id(5994);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_7);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut player_17: reversi::Player = crate::reversi::Player::other(player_16);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_30, option_29, option_28, option_27, option_26, option_25, option_24, option_23];
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut player_23: reversi::Player = crate::reversi::Player::other(player_22);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut player_28: reversi::Player = crate::reversi::Player::other(player_27);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player0;
    let mut player_30: reversi::Player = crate::reversi::Player::other(player_29);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_38, option_37, option_36, option_35, option_34, option_33, option_32, option_31];
    let mut player_31: reversi::Player = crate::reversi::Player::Player1;
    let mut player_32: reversi::Player = crate::reversi::Player::other(player_31);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_0);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::other(player_2);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut player_35: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_46, option_45, option_44, option_43, option_42, option_41, option_40, option_39];
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut player_37: reversi::Player = crate::reversi::Player::Player1;
    let mut player_38: reversi::Player = crate::reversi::Player::other(player_37);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut player_39: reversi::Player = crate::reversi::Player::Player0;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut player_40: reversi::Player = crate::reversi::Player::Player1;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_41: reversi::Player = crate::reversi::Player::Player0;
    let mut player_42: reversi::Player = crate::reversi::Player::other(player_41);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut player_43: reversi::Player = crate::reversi::Player::Player1;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_54, option_53, option_52, option_51, option_50, option_49, option_48, option_47];
    let mut player_44: reversi::Player = crate::reversi::Player::Player1;
    let mut player_45: reversi::Player = crate::reversi::Player::other(player_44);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_45);
    let mut player_46: reversi::Player = crate::reversi::Player::Player0;
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_46);
    let mut player_47: reversi::Player = crate::reversi::Player::Player1;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_47);
    let mut player_48: reversi::Player = crate::reversi::Player::Player0;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_48);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::Upper;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut tuple_0: (i8, i8) = crate::reversi::Direction::as_offset(direction_1_ref_0);
    let mut player_49: reversi::Player = crate::reversi::Player::Player1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1067() {
    rusty_monitor::set_test_id(1067);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut player_5: reversi::Player = crate::reversi::Player::other(player_4);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_14, option_13, option_12, option_11, option_10, option_9, option_8, option_7];
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_0);
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut player_9: reversi::Player = crate::reversi::Player::other(player_8);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut player_11: reversi::Player = crate::reversi::Player::other(player_10);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_22, option_21, option_20, option_19, option_18, option_17, option_16, option_15];
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut player_17: reversi::Player = crate::reversi::Player::Player0;
    let mut player_18: reversi::Player = crate::reversi::Player::other(player_17);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_30, option_29, option_28, option_27, option_26, option_25, option_24, option_23];
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player1;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut player_22: reversi::Player = crate::reversi::Player::other(player_21);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player0;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_25: reversi::Player = crate::reversi::Player::Player1;
    let mut player_26: reversi::Player = crate::reversi::Player::other(player_25);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player0;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_45, option_44, option_43, option_42, option_41, option_40, option_39, option_38];
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut player_30: reversi::Player = crate::reversi::Player::other(player_29);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player1;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_53, option_52, option_51, option_50, option_49, option_48, option_47, option_46];
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut player_34: reversi::Player = crate::reversi::Player::other(player_33);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut player_35: reversi::Player = crate::reversi::Player::Player1;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_36: reversi::Player = crate::reversi::Player::Player0;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut player_37: reversi::Player = crate::reversi::Player::Player1;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_61, option_60, option_59, option_58, option_57, option_56, option_55, option_54];
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut player_39: reversi::Player = crate::reversi::Player::other(player_38);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_39);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: reversi::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Lower;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3609() {
    rusty_monitor::set_test_id(3609);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_14, option_13, option_12, option_11, option_10, option_9, option_8, option_7];
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_7);
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut player_10: reversi::Player = crate::reversi::Player::other(player_9);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut player_12: reversi::Player = crate::reversi::Player::other(player_11);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_22, option_21, option_20, option_19, option_18, option_17, option_16, option_15];
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_30, option_29, option_28, option_27, option_26, option_25, option_24, option_23];
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut player_17: reversi::Player = crate::reversi::Player::other(player_16);
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player1;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_52, option_51, option_50, option_49, option_48, option_47, option_46, option_45];
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut player_26: reversi::Player = crate::reversi::Player::other(player_25);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_60, option_59, option_58, option_57, option_56, option_55, option_54, option_53];
    let mut player_27: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_28: connect_four::Player = crate::connect_four::Player::other(player_27);
    let mut player_29: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_30: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::LowerRight;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut tuple_0: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_0_ref_0);
    let mut player_31: connect_four::Player = crate::connect_four::Player::Player1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut player_32: connect_four::Player = crate::connect_four::Player::other(player_31);
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::Win(player_30);
    let mut gamestate_3_ref_0: &gomoku::GameState = &mut gamestate_3;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::LowerLeft;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_33: connect_four::Player = crate::connect_four::Player::other(player_28);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6477() {
    rusty_monitor::set_test_id(6477);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_3);
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    let mut gamestate_5: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut gamestate_5_ref_0: &gomoku::GameState = &mut gamestate_5;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_6: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut gamestate_6_ref_0: &reversi::GameState = &mut gamestate_6;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_6_ref_0, gamestate_0_ref_0);
    let mut gamestate_7: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_119() {
    rusty_monitor::set_test_id(119);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_5);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut player_8: reversi::Player = crate::reversi::Player::Player1;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player0;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut player_20: reversi::Player = crate::reversi::Player::other(player_19);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut player_23: reversi::Player = crate::reversi::Player::other(player_22);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut player_27: reversi::Player = crate::reversi::Player::other(player_26);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut player_29: reversi::Player = crate::reversi::Player::other(player_28);
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut player_31: reversi::Player = crate::reversi::Player::other(player_30);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut player_33: reversi::Player = crate::reversi::Player::other(player_32);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut player_35: reversi::Player = crate::reversi::Player::other(player_34);
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player0;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut player_37: reversi::Player = crate::reversi::Player::Player0;
    let mut player_38: reversi::Player = crate::reversi::Player::other(player_37);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_39: reversi::Player = crate::reversi::Player::Player0;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_40: reversi::Player = crate::reversi::Player::Player0;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut player_41: reversi::Player = crate::reversi::Player::Player0;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_42: reversi::Player = crate::reversi::Player::Player1;
    let mut player_43: reversi::Player = crate::reversi::Player::other(player_42);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut player_44: reversi::Player = crate::reversi::Player::Player0;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut reversi_1: crate::reversi::Reversi = std::clone::Clone::clone(reversi_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8403() {
    rusty_monitor::set_test_id(8403);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_0);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_8, option_7, option_6, option_5, option_4, option_3, option_2, option_1];
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut player_7: reversi::Player = crate::reversi::Player::other(player_6);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_16, option_15, option_14, option_13, option_12, option_11, option_10, option_9];
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player0;
    let mut player_10: reversi::Player = crate::reversi::Player::other(player_9);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut player_13: reversi::Player = crate::reversi::Player::Player0;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut player_15: reversi::Player = crate::reversi::Player::other(player_14);
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_24, option_23, option_22, option_21, option_20, option_19, option_18, option_17];
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut player_17: reversi::Player = crate::reversi::Player::other(player_16);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut player_18: reversi::Player = crate::reversi::Player::Player1;
    let mut player_19: reversi::Player = crate::reversi::Player::other(player_18);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_32, option_31, option_30, option_29, option_28, option_27, option_26, option_25];
    let mut player_23: reversi::Player = crate::reversi::Player::Player0;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut player_27: reversi::Player = crate::reversi::Player::other(player_26);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut player_29: reversi::Player = crate::reversi::Player::other(player_28);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut player_31: reversi::Player = crate::reversi::Player::other(player_30);
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_40, option_39, option_38, option_37, option_36, option_35, option_34, option_33];
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut player_34: reversi::Player = crate::reversi::Player::other(player_33);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_35: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_48, option_47, option_46, option_45, option_44, option_43, option_42, option_41];
    let mut player_37: reversi::Player = crate::reversi::Player::Player1;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_39: reversi::Player = crate::reversi::Player::Player0;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut player_40: reversi::Player = crate::reversi::Player::Player0;
    let mut player_41: reversi::Player = crate::reversi::Player::other(player_40);
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_56, option_55, option_54, option_53, option_52, option_51, option_50, option_49];
    let mut player_42: reversi::Player = crate::reversi::Player::Player1;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut player_43: reversi::Player = crate::reversi::Player::Player0;
    let mut player_44: reversi::Player = crate::reversi::Player::other(player_43);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_44);
    let mut player_45: reversi::Player = crate::reversi::Player::Player1;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_45);
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_46: reversi::Player = crate::reversi::Player::Player1;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_46);
    let mut player_47: reversi::Player = crate::reversi::Player::Player0;
    let mut player_48: reversi::Player = crate::reversi::Player::other(player_47);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_48);
    let mut player_49: reversi::Player = crate::reversi::Player::Player1;
    let mut player_50: reversi::Player = crate::reversi::Player::other(player_49);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_50);
    let mut option_64: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_64, option_63, option_62, option_61, option_60, option_59, option_58, option_57];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut option_65: std::option::Option<reversi::Player> = crate::reversi::Reversi::winner(reversi_0_ref_0);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_51: reversi::Player = crate::reversi::Player::Player0;
    let mut option_66: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_52: reversi::Player = crate::reversi::Player::Player1;
    let mut player_53: reversi::Player = crate::reversi::Player::other(player_52);
    let mut option_67: std::option::Option<reversi::Player> = std::option::Option::Some(player_53);
    let mut option_68: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_54: reversi::Player = crate::reversi::Player::Player1;
    let mut player_55: reversi::Player = crate::reversi::Player::other(player_54);
    let mut option_69: std::option::Option<reversi::Player> = std::option::Option::Some(player_55);
    let mut player_56: reversi::Player = crate::reversi::Player::Player0;
    let mut option_70: std::option::Option<reversi::Player> = std::option::Option::Some(player_56);
    let mut option_71: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_72: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_57: reversi::Player = crate::reversi::Player::Player1;
    let mut option_73: std::option::Option<reversi::Player> = std::option::Option::Some(player_57);
    let mut option_array_8: [std::option::Option<reversi::Player>; 8] = [option_73, option_72, option_71, option_70, option_69, option_68, option_67, option_66];
    let mut player_58: reversi::Player = crate::reversi::Player::Player1;
    let mut option_74: std::option::Option<reversi::Player> = std::option::Option::Some(player_58);
    let mut option_75: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_76: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_59: reversi::Player = crate::reversi::Player::Player1;
    let mut option_77: std::option::Option<reversi::Player> = std::option::Option::Some(player_59);
    let mut player_60: reversi::Player = crate::reversi::Player::Player1;
    let mut player_61: reversi::Player = crate::reversi::Player::other(player_60);
    let mut option_78: std::option::Option<reversi::Player> = std::option::Option::Some(player_61);
    let mut player_62: reversi::Player = crate::reversi::Player::Player1;
    let mut player_63: reversi::Player = crate::reversi::Player::other(player_62);
    let mut option_79: std::option::Option<reversi::Player> = std::option::Option::Some(player_63);
    let mut player_64: reversi::Player = crate::reversi::Player::Player1;
    let mut player_65: reversi::Player = crate::reversi::Player::other(player_64);
    let mut option_80: std::option::Option<reversi::Player> = std::option::Option::Some(player_65);
    let mut player_66: reversi::Player = crate::reversi::Player::Player1;
    let mut player_67: reversi::Player = crate::reversi::Player::other(player_66);
    let mut option_81: std::option::Option<reversi::Player> = std::option::Option::Some(player_67);
    let mut option_array_9: [std::option::Option<reversi::Player>; 8] = [option_81, option_80, option_79, option_78, option_77, option_76, option_75, option_74];
    let mut option_82: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_68: reversi::Player = crate::reversi::Player::Player1;
    let mut option_83: std::option::Option<reversi::Player> = std::option::Option::Some(player_68);
    let mut option_84: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_69: reversi::Player = crate::reversi::Player::Player1;
    let mut option_85: std::option::Option<reversi::Player> = std::option::Option::Some(player_69);
    let mut player_70: reversi::Player = crate::reversi::Player::Player0;
    let mut player_71: reversi::Player = crate::reversi::Player::other(player_70);
    let mut option_86: std::option::Option<reversi::Player> = std::option::Option::Some(player_71);
    let mut option_87: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_88: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_89: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_10: [std::option::Option<reversi::Player>; 8] = [option_89, option_88, option_87, option_86, option_85, option_84, option_83, option_82];
    let mut player_72: reversi::Player = crate::reversi::Player::Player0;
    let mut player_73: reversi::Player = crate::reversi::Player::other(player_72);
    let mut option_90: std::option::Option<reversi::Player> = std::option::Option::Some(player_73);
    let mut player_74: reversi::Player = crate::reversi::Player::Player1;
    let mut option_91: std::option::Option<reversi::Player> = std::option::Option::Some(player_74);
    let mut option_92: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_75: reversi::Player = crate::reversi::Player::Player1;
    let mut player_76: reversi::Player = crate::reversi::Player::other(player_75);
    let mut option_93: std::option::Option<reversi::Player> = std::option::Option::Some(player_76);
    let mut player_77: reversi::Player = crate::reversi::Player::Player1;
    let mut option_94: std::option::Option<reversi::Player> = std::option::Option::Some(player_77);
    let mut player_78: reversi::Player = crate::reversi::Player::Player1;
    let mut option_95: std::option::Option<reversi::Player> = std::option::Option::Some(player_78);
    let mut player_79: reversi::Player = crate::reversi::Player::Player1;
    let mut player_80: reversi::Player = crate::reversi::Player::other(player_79);
    let mut option_96: std::option::Option<reversi::Player> = std::option::Option::Some(player_80);
    let mut player_81: reversi::Player = crate::reversi::Player::Player0;
    let mut option_97: std::option::Option<reversi::Player> = std::option::Option::Some(player_81);
    let mut option_array_11: [std::option::Option<reversi::Player>; 8] = [option_97, option_96, option_95, option_94, option_93, option_92, option_91, option_90];
    let mut player_82: reversi::Player = crate::reversi::Player::Player1;
    let mut option_98: std::option::Option<reversi::Player> = std::option::Option::Some(player_82);
    let mut option_99: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_83: reversi::Player = crate::reversi::Player::Player1;
    let mut player_84: reversi::Player = crate::reversi::Player::other(player_83);
    let mut option_100: std::option::Option<reversi::Player> = std::option::Option::Some(player_84);
    let mut option_101: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_85: reversi::Player = crate::reversi::Player::Player1;
    let mut option_102: std::option::Option<reversi::Player> = std::option::Option::Some(player_85);
    let mut player_86: reversi::Player = crate::reversi::Player::Player1;
    let mut option_103: std::option::Option<reversi::Player> = std::option::Option::Some(player_86);
    let mut option_104: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_105: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_12: [std::option::Option<reversi::Player>; 8] = [option_105, option_104, option_103, option_102, option_101, option_100, option_99, option_98];
    let mut option_106: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_107: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_108: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_109: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_110: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_111: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_112: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_87: reversi::Player = crate::reversi::Player::Player0;
    let mut option_113: std::option::Option<reversi::Player> = std::option::Option::Some(player_87);
    let mut option_array_13: [std::option::Option<reversi::Player>; 8] = [option_113, option_112, option_111, option_110, option_109, option_108, option_107, option_106];
    let mut player_88: reversi::Player = crate::reversi::Player::Player1;
    let mut option_114: std::option::Option<reversi::Player> = std::option::Option::Some(player_88);
    let mut player_89: reversi::Player = crate::reversi::Player::Player1;
    let mut option_115: std::option::Option<reversi::Player> = std::option::Option::Some(player_89);
    let mut option_116: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_90: reversi::Player = crate::reversi::Player::Player0;
    let mut player_91: reversi::Player = crate::reversi::Player::other(player_90);
    let mut option_117: std::option::Option<reversi::Player> = std::option::Option::Some(player_91);
    let mut player_92: reversi::Player = crate::reversi::Player::Player0;
    let mut option_118: std::option::Option<reversi::Player> = std::option::Option::Some(player_92);
    let mut option_119: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_93: reversi::Player = crate::reversi::Player::Player0;
    let mut player_94: reversi::Player = crate::reversi::Player::other(player_93);
    let mut option_120: std::option::Option<reversi::Player> = std::option::Option::Some(player_94);
    let mut player_95: reversi::Player = crate::reversi::Player::Player1;
    let mut option_121: std::option::Option<reversi::Player> = std::option::Option::Some(player_95);
    let mut option_array_14: [std::option::Option<reversi::Player>; 8] = [option_121, option_120, option_119, option_118, option_117, option_116, option_115, option_114];
    let mut option_122: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_123: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_124: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_96: reversi::Player = crate::reversi::Player::Player0;
    let mut player_97: reversi::Player = crate::reversi::Player::other(player_96);
    let mut option_125: std::option::Option<reversi::Player> = std::option::Option::Some(player_97);
    let mut option_126: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_127: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_98: reversi::Player = crate::reversi::Player::Player1;
    let mut player_99: reversi::Player = crate::reversi::Player::other(player_98);
    let mut option_128: std::option::Option<reversi::Player> = std::option::Option::Some(player_99);
    let mut option_129: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_15: [std::option::Option<reversi::Player>; 8] = [option_129, option_128, option_127, option_126, option_125, option_124, option_123, option_122];
    let mut option_array_array_1: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_15, option_array_14, option_array_13, option_array_12, option_array_11, option_array_10, option_array_9, option_array_8];
    let mut reversi_1: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_1, next: player_51, status: gamestate_1};
    let mut reversi_1_ref_0: &crate::reversi::Reversi = &mut reversi_1;
    let mut option_130: std::option::Option<reversi::Player> = crate::reversi::Reversi::winner(reversi_1_ref_0);
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_100: reversi::Player = crate::reversi::Player::Player0;
    let mut player_101: reversi::Player = crate::reversi::Player::other(player_100);
    let mut option_131: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_102: reversi::Player = crate::reversi::Player::Player1;
    let mut player_103: reversi::Player = crate::reversi::Player::other(player_102);
    let mut option_132: std::option::Option<reversi::Player> = std::option::Option::Some(player_103);
    let mut option_133: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_104: reversi::Player = crate::reversi::Player::Player1;
    let mut option_134: std::option::Option<reversi::Player> = std::option::Option::Some(player_104);
    let mut player_105: reversi::Player = crate::reversi::Player::Player0;
    let mut option_135: std::option::Option<reversi::Player> = std::option::Option::Some(player_105);
    let mut player_106: reversi::Player = crate::reversi::Player::Player1;
    let mut option_136: std::option::Option<reversi::Player> = std::option::Option::Some(player_106);
    let mut option_137: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_138: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_16: [std::option::Option<reversi::Player>; 8] = [option_138, option_137, option_136, option_135, option_134, option_133, option_132, option_131];
    let mut player_107: reversi::Player = crate::reversi::Player::Player0;
    let mut option_139: std::option::Option<reversi::Player> = std::option::Option::Some(player_107);
    let mut option_140: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_108: reversi::Player = crate::reversi::Player::Player1;
    let mut option_141: std::option::Option<reversi::Player> = std::option::Option::Some(player_108);
    let mut option_142: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_109: reversi::Player = crate::reversi::Player::Player0;
    let mut option_143: std::option::Option<reversi::Player> = std::option::Option::Some(player_109);
    let mut option_144: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_110: reversi::Player = crate::reversi::Player::Player1;
    let mut option_145: std::option::Option<reversi::Player> = std::option::Option::Some(player_110);
    let mut player_111: reversi::Player = crate::reversi::Player::Player0;
    let mut option_146: std::option::Option<reversi::Player> = std::option::Option::Some(player_111);
    let mut option_array_17: [std::option::Option<reversi::Player>; 8] = [option_146, option_145, option_144, option_143, option_142, option_141, option_140, option_139];
    let mut option_147: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_148: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_112: reversi::Player = crate::reversi::Player::Player0;
    let mut option_149: std::option::Option<reversi::Player> = std::option::Option::Some(player_112);
    let mut player_113: reversi::Player = crate::reversi::Player::Player1;
    let mut option_150: std::option::Option<reversi::Player> = std::option::Option::Some(player_113);
    let mut player_114: reversi::Player = crate::reversi::Player::Player0;
    let mut option_151: std::option::Option<reversi::Player> = std::option::Option::Some(player_114);
    let mut option_152: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_115: reversi::Player = crate::reversi::Player::Player1;
    let mut player_116: reversi::Player = crate::reversi::Player::other(player_115);
    let mut option_153: std::option::Option<reversi::Player> = std::option::Option::Some(player_116);
    let mut option_154: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_18: [std::option::Option<reversi::Player>; 8] = [option_154, option_153, option_152, option_151, option_150, option_149, option_148, option_147];
    let mut option_155: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_117: reversi::Player = crate::reversi::Player::Player0;
    let mut option_156: std::option::Option<reversi::Player> = std::option::Option::Some(player_117);
    let mut player_118: reversi::Player = crate::reversi::Player::Player0;
    let mut player_119: reversi::Player = crate::reversi::Player::other(player_118);
    let mut option_157: std::option::Option<reversi::Player> = std::option::Option::Some(player_119);
    let mut option_158: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_159: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_120: reversi::Player = crate::reversi::Player::Player0;
    let mut player_121: reversi::Player = crate::reversi::Player::other(player_120);
    let mut option_160: std::option::Option<reversi::Player> = std::option::Option::Some(player_121);
    let mut player_122: reversi::Player = crate::reversi::Player::Player0;
    let mut option_161: std::option::Option<reversi::Player> = std::option::Option::Some(player_122);
    let mut option_162: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_19: [std::option::Option<reversi::Player>; 8] = [option_162, option_161, option_160, option_159, option_158, option_157, option_156, option_155];
    let mut option_163: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_164: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_123: reversi::Player = crate::reversi::Player::Player0;
    let mut option_165: std::option::Option<reversi::Player> = std::option::Option::Some(player_123);
    let mut player_124: reversi::Player = crate::reversi::Player::Player0;
    let mut option_166: std::option::Option<reversi::Player> = std::option::Option::Some(player_124);
    let mut option_167: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_168: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_169: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_125: reversi::Player = crate::reversi::Player::Player1;
    let mut option_170: std::option::Option<reversi::Player> = std::option::Option::Some(player_125);
    let mut option_array_20: [std::option::Option<reversi::Player>; 8] = [option_170, option_169, option_168, option_167, option_166, option_165, option_164, option_163];
    let mut player_126: reversi::Player = crate::reversi::Player::Player0;
    let mut option_171: std::option::Option<reversi::Player> = std::option::Option::Some(player_126);
    let mut player_127: reversi::Player = crate::reversi::Player::Player1;
    let mut option_172: std::option::Option<reversi::Player> = std::option::Option::Some(player_127);
    let mut player_128: reversi::Player = crate::reversi::Player::Player1;
    let mut option_173: std::option::Option<reversi::Player> = std::option::Option::Some(player_128);
    let mut player_129: reversi::Player = crate::reversi::Player::Player0;
    let mut player_130: reversi::Player = crate::reversi::Player::other(player_129);
    let mut option_174: std::option::Option<reversi::Player> = std::option::Option::Some(player_130);
    let mut player_131: reversi::Player = crate::reversi::Player::Player1;
    let mut option_175: std::option::Option<reversi::Player> = std::option::Option::Some(player_131);
    let mut player_132: reversi::Player = crate::reversi::Player::Player0;
    let mut option_176: std::option::Option<reversi::Player> = std::option::Option::Some(player_132);
    let mut option_177: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_133: reversi::Player = crate::reversi::Player::Player0;
    let mut option_178: std::option::Option<reversi::Player> = std::option::Option::Some(player_133);
    let mut option_array_21: [std::option::Option<reversi::Player>; 8] = [option_178, option_177, option_176, option_175, option_174, option_173, option_172, option_171];
    let mut option_179: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_180: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_134: reversi::Player = crate::reversi::Player::Player1;
    let mut option_181: std::option::Option<reversi::Player> = std::option::Option::Some(player_134);
    let mut option_182: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_135: reversi::Player = crate::reversi::Player::Player1;
    let mut option_183: std::option::Option<reversi::Player> = std::option::Option::Some(player_135);
    let mut option_184: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_136: reversi::Player = crate::reversi::Player::Player0;
    let mut option_185: std::option::Option<reversi::Player> = std::option::Option::Some(player_136);
    let mut option_186: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_22: [std::option::Option<reversi::Player>; 8] = [option_186, option_185, option_184, option_183, option_182, option_181, option_180, option_179];
    let mut option_187: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_137: reversi::Player = crate::reversi::Player::Player0;
    let mut option_188: std::option::Option<reversi::Player> = std::option::Option::Some(player_137);
    let mut player_138: reversi::Player = crate::reversi::Player::Player0;
    let mut option_189: std::option::Option<reversi::Player> = std::option::Option::Some(player_138);
    let mut option_190: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_139: reversi::Player = crate::reversi::Player::Player0;
    let mut player_140: reversi::Player = crate::reversi::Player::other(player_139);
    let mut option_191: std::option::Option<reversi::Player> = std::option::Option::Some(player_140);
    let mut player_141: reversi::Player = crate::reversi::Player::Player0;
    let mut option_192: std::option::Option<reversi::Player> = std::option::Option::Some(player_141);
    let mut player_142: reversi::Player = crate::reversi::Player::Player0;
    let mut option_193: std::option::Option<reversi::Player> = std::option::Option::Some(player_142);
    let mut player_143: reversi::Player = crate::reversi::Player::Player0;
    let mut option_194: std::option::Option<reversi::Player> = std::option::Option::Some(player_143);
    let mut option_array_23: [std::option::Option<reversi::Player>; 8] = [option_194, option_193, option_192, option_191, option_190, option_189, option_188, option_187];
    let mut option_array_array_2: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_23, option_array_22, option_array_21, option_array_20, option_array_19, option_array_18, option_array_17, option_array_16];
    let mut reversi_2: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_2, next: player_101, status: gamestate_2};
    let mut reversi_2_ref_0: &crate::reversi::Reversi = &mut reversi_2;
    let mut option_195: std::option::Option<reversi::Player> = crate::reversi::Reversi::winner(reversi_2_ref_0);
    let mut option_196: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_144: reversi::Player = crate::reversi::Player::Player0;
    let mut option_197: std::option::Option<reversi::Player> = std::option::Option::Some(player_144);
    let mut option_198: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_145: reversi::Player = crate::reversi::Player::Player1;
    let mut player_146: reversi::Player = crate::reversi::Player::other(player_145);
    let mut option_199: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_200: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_201: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_147: reversi::Player = crate::reversi::Player::Player1;
    let mut option_202: std::option::Option<reversi::Player> = std::option::Option::Some(player_147);
    let mut option_203: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_148: reversi::Player = crate::reversi::Player::Player1;
    let mut option_204: std::option::Option<reversi::Player> = std::option::Option::Some(player_148);
    let mut player_149: reversi::Player = crate::reversi::Player::Player0;
    let mut player_150: reversi::Player = crate::reversi::Player::other(player_149);
    let mut option_205: std::option::Option<reversi::Player> = std::option::Option::Some(player_150);
    let mut option_206: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_24: [std::option::Option<reversi::Player>; 8] = [option_206, option_205, option_204, option_203, option_202, option_201, option_200, option_199];
    let mut option_207: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_151: reversi::Player = crate::reversi::Player::Player1;
    let mut player_152: reversi::Player = crate::reversi::Player::other(player_151);
    let mut option_208: std::option::Option<reversi::Player> = std::option::Option::Some(player_152);
    let mut player_153: reversi::Player = crate::reversi::Player::Player1;
    let mut option_209: std::option::Option<reversi::Player> = std::option::Option::Some(player_153);
    let mut option_210: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_154: reversi::Player = crate::reversi::Player::Player1;
    let mut option_211: std::option::Option<reversi::Player> = std::option::Option::Some(player_154);
    let mut option_212: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_213: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_214: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_25: [std::option::Option<reversi::Player>; 8] = [option_214, option_213, option_212, option_211, option_210, option_209, option_208, option_207];
    let mut option_215: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_216: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_217: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_155: reversi::Player = crate::reversi::Player::Player0;
    let mut player_156: reversi::Player = crate::reversi::Player::other(player_155);
    let mut option_218: std::option::Option<reversi::Player> = std::option::Option::Some(player_156);
    let mut option_219: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_220: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_157: reversi::Player = crate::reversi::Player::Player0;
    let mut player_158: reversi::Player = crate::reversi::Player::other(player_157);
    let mut option_221: std::option::Option<reversi::Player> = std::option::Option::Some(player_158);
    let mut option_222: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_26: [std::option::Option<reversi::Player>; 8] = [option_222, option_221, option_220, option_219, option_218, option_217, option_216, option_215];
    let mut option_223: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_159: reversi::Player = crate::reversi::Player::Player1;
    let mut player_160: reversi::Player = crate::reversi::Player::other(player_159);
    let mut option_224: std::option::Option<reversi::Player> = std::option::Option::Some(player_160);
    let mut option_225: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_226: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_161: reversi::Player = crate::reversi::Player::Player0;
    let mut player_162: reversi::Player = crate::reversi::Player::other(player_161);
    let mut option_227: std::option::Option<reversi::Player> = std::option::Option::Some(player_162);
    let mut option_228: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_229: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_163: reversi::Player = crate::reversi::Player::Player0;
    let mut option_230: std::option::Option<reversi::Player> = std::option::Option::Some(player_163);
    let mut option_array_27: [std::option::Option<reversi::Player>; 8] = [option_230, option_229, option_228, option_227, option_226, option_225, option_224, option_223];
    let mut player_164: reversi::Player = crate::reversi::Player::Player1;
    let mut option_231: std::option::Option<reversi::Player> = std::option::Option::Some(player_164);
    let mut player_165: reversi::Player = crate::reversi::Player::Player1;
    let mut option_232: std::option::Option<reversi::Player> = std::option::Option::Some(player_165);
    let mut option_233: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_166: reversi::Player = crate::reversi::Player::Player0;
    let mut player_167: reversi::Player = crate::reversi::Player::other(player_166);
    let mut option_234: std::option::Option<reversi::Player> = std::option::Option::Some(player_167);
    let mut player_168: reversi::Player = crate::reversi::Player::Player1;
    let mut player_169: reversi::Player = crate::reversi::Player::other(player_168);
    let mut option_235: std::option::Option<reversi::Player> = std::option::Option::Some(player_169);
    let mut option_236: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_237: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_238: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_28: [std::option::Option<reversi::Player>; 8] = [option_238, option_237, option_236, option_235, option_234, option_233, option_232, option_231];
    let mut option_239: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_170: reversi::Player = crate::reversi::Player::Player0;
    let mut option_240: std::option::Option<reversi::Player> = std::option::Option::Some(player_170);
    let mut player_171: reversi::Player = crate::reversi::Player::Player1;
    let mut option_241: std::option::Option<reversi::Player> = std::option::Option::Some(player_171);
    let mut player_172: reversi::Player = crate::reversi::Player::Player1;
    let mut player_173: reversi::Player = crate::reversi::Player::other(player_172);
    let mut option_242: std::option::Option<reversi::Player> = std::option::Option::Some(player_173);
    let mut option_243: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_174: reversi::Player = crate::reversi::Player::Player0;
    let mut option_244: std::option::Option<reversi::Player> = std::option::Option::Some(player_174);
    let mut player_175: reversi::Player = crate::reversi::Player::Player0;
    let mut option_245: std::option::Option<reversi::Player> = std::option::Option::Some(player_175);
    let mut player_176: reversi::Player = crate::reversi::Player::Player0;
    let mut option_246: std::option::Option<reversi::Player> = std::option::Option::Some(player_176);
    let mut option_array_29: [std::option::Option<reversi::Player>; 8] = [option_246, option_245, option_244, option_243, option_242, option_241, option_240, option_239];
    let mut option_247: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_177: reversi::Player = crate::reversi::Player::Player0;
    let mut option_248: std::option::Option<reversi::Player> = std::option::Option::Some(player_177);
    let mut player_178: reversi::Player = crate::reversi::Player::Player0;
    let mut option_249: std::option::Option<reversi::Player> = std::option::Option::Some(player_178);
    let mut player_179: reversi::Player = crate::reversi::Player::Player0;
    let mut player_180: reversi::Player = crate::reversi::Player::other(player_179);
    let mut option_250: std::option::Option<reversi::Player> = std::option::Option::Some(player_180);
    let mut player_181: reversi::Player = crate::reversi::Player::Player0;
    let mut option_251: std::option::Option<reversi::Player> = std::option::Option::Some(player_181);
    let mut option_252: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_253: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_182: reversi::Player = crate::reversi::Player::Player1;
    let mut option_254: std::option::Option<reversi::Player> = std::option::Option::Some(player_182);
    let mut option_array_30: [std::option::Option<reversi::Player>; 8] = [option_254, option_253, option_252, option_251, option_250, option_249, option_248, option_247];
    let mut option_255: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_183: reversi::Player = crate::reversi::Player::Player1;
    let mut option_256: std::option::Option<reversi::Player> = std::option::Option::Some(player_183);
    let mut option_257: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_258: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_259: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_184: reversi::Player = crate::reversi::Player::Player0;
    let mut player_185: reversi::Player = crate::reversi::Player::other(player_184);
    let mut option_260: std::option::Option<reversi::Player> = std::option::Option::Some(player_185);
    let mut player_186: reversi::Player = crate::reversi::Player::Player1;
    let mut player_187: reversi::Player = crate::reversi::Player::other(player_186);
    let mut option_261: std::option::Option<reversi::Player> = std::option::Option::Some(player_187);
    let mut option_262: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_31: [std::option::Option<reversi::Player>; 8] = [option_262, option_261, option_260, option_259, option_258, option_257, option_256, option_255];
    let mut option_array_array_3: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_31, option_array_30, option_array_29, option_array_28, option_array_27, option_array_26, option_array_25, option_array_24];
    let mut reversi_3: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_3, next: player_146, status: gamestate_3};
    let mut reversi_3_ref_0: &crate::reversi::Reversi = &mut reversi_3;
    let mut option_263: std::option::Option<reversi::Player> = crate::reversi::Reversi::winner(reversi_3_ref_0);
    let mut option_array_32: [std::option::Option<reversi::Player>; 8] = [option_263, option_198, option_197, option_196, option_195, option_130, option_65, option_0];
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_188: reversi::Player = crate::reversi::Player::Player1;
    let mut player_189: reversi::Player = crate::reversi::Player::Player1;
    let mut option_264: std::option::Option<reversi::Player> = std::option::Option::Some(player_189);
    let mut player_190: reversi::Player = crate::reversi::Player::Player1;
    let mut option_265: std::option::Option<reversi::Player> = std::option::Option::Some(player_190);
    let mut player_191: reversi::Player = crate::reversi::Player::Player0;
    let mut option_266: std::option::Option<reversi::Player> = std::option::Option::Some(player_191);
    let mut player_192: reversi::Player = crate::reversi::Player::Player1;
    let mut option_267: std::option::Option<reversi::Player> = std::option::Option::Some(player_192);
    let mut player_193: reversi::Player = crate::reversi::Player::Player0;
    let mut player_194: reversi::Player = crate::reversi::Player::other(player_193);
    let mut option_268: std::option::Option<reversi::Player> = std::option::Option::Some(player_194);
    let mut option_269: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_270: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_195: reversi::Player = crate::reversi::Player::Player0;
    let mut option_271: std::option::Option<reversi::Player> = std::option::Option::Some(player_195);
    let mut option_array_33: [std::option::Option<reversi::Player>; 8] = [option_271, option_270, option_269, option_268, option_267, option_266, option_265, option_264];
    let mut player_196: reversi::Player = crate::reversi::Player::Player1;
    let mut option_272: std::option::Option<reversi::Player> = std::option::Option::Some(player_196);
    let mut option_273: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_197: reversi::Player = crate::reversi::Player::Player1;
    let mut option_274: std::option::Option<reversi::Player> = std::option::Option::Some(player_197);
    let mut option_275: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_198: reversi::Player = crate::reversi::Player::Player1;
    let mut option_276: std::option::Option<reversi::Player> = std::option::Option::Some(player_198);
    let mut option_277: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_278: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_279: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_34: [std::option::Option<reversi::Player>; 8] = [option_279, option_278, option_277, option_276, option_275, option_274, option_273, option_272];
    let mut player_199: reversi::Player = crate::reversi::Player::Player1;
    let mut option_280: std::option::Option<reversi::Player> = std::option::Option::Some(player_199);
    let mut player_200: reversi::Player = crate::reversi::Player::Player0;
    let mut option_281: std::option::Option<reversi::Player> = std::option::Option::Some(player_200);
    let mut player_201: reversi::Player = crate::reversi::Player::Player1;
    let mut option_282: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_283: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_202: reversi::Player = crate::reversi::Player::Player1;
    let mut option_284: std::option::Option<reversi::Player> = std::option::Option::Some(player_202);
    let mut player_203: reversi::Player = crate::reversi::Player::Player0;
    let mut option_285: std::option::Option<reversi::Player> = std::option::Option::Some(player_203);
    let mut player_204: reversi::Player = crate::reversi::Player::Player1;
    let mut option_286: std::option::Option<reversi::Player> = std::option::Option::Some(player_204);
    let mut player_205: reversi::Player = crate::reversi::Player::Player1;
    let mut option_287: std::option::Option<reversi::Player> = std::option::Option::Some(player_205);
    let mut option_array_35: [std::option::Option<reversi::Player>; 8] = [option_287, option_286, option_285, option_284, option_283, option_282, option_281, option_280];
    let mut option_288: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_206: reversi::Player = crate::reversi::Player::Player1;
    let mut player_207: reversi::Player = crate::reversi::Player::other(player_206);
    let mut option_289: std::option::Option<reversi::Player> = std::option::Option::Some(player_207);
    let mut player_208: reversi::Player = crate::reversi::Player::Player1;
    let mut player_209: reversi::Player = crate::reversi::Player::other(player_208);
    let mut option_290: std::option::Option<reversi::Player> = std::option::Option::Some(player_209);
    let mut player_210: reversi::Player = crate::reversi::Player::Player1;
    let mut player_211: reversi::Player = crate::reversi::Player::other(player_210);
    let mut option_291: std::option::Option<reversi::Player> = std::option::Option::Some(player_211);
    let mut option_292: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_212: reversi::Player = crate::reversi::Player::Player1;
    let mut player_213: reversi::Player = crate::reversi::Player::other(player_212);
    let mut option_293: std::option::Option<reversi::Player> = std::option::Option::Some(player_213);
    let mut player_214: reversi::Player = crate::reversi::Player::Player0;
    let mut option_294: std::option::Option<reversi::Player> = std::option::Option::Some(player_214);
    let mut player_215: reversi::Player = crate::reversi::Player::Player0;
    let mut option_295: std::option::Option<reversi::Player> = std::option::Option::Some(player_215);
    let mut option_array_36: [std::option::Option<reversi::Player>; 8] = [option_295, option_294, option_293, option_292, option_291, option_290, option_289, option_288];
    let mut player_216: reversi::Player = crate::reversi::Player::Player1;
    let mut player_217: reversi::Player = crate::reversi::Player::other(player_216);
    let mut option_296: std::option::Option<reversi::Player> = std::option::Option::Some(player_217);
    let mut option_297: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_298: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_218: reversi::Player = crate::reversi::Player::Player0;
    let mut option_299: std::option::Option<reversi::Player> = std::option::Option::Some(player_218);
    let mut player_219: reversi::Player = crate::reversi::Player::Player0;
    let mut option_300: std::option::Option<reversi::Player> = std::option::Option::Some(player_219);
    let mut option_301: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_302: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_220: reversi::Player = crate::reversi::Player::Player0;
    let mut player_221: reversi::Player = crate::reversi::Player::other(player_220);
    let mut option_303: std::option::Option<reversi::Player> = std::option::Option::Some(player_221);
    let mut option_array_37: [std::option::Option<reversi::Player>; 8] = [option_303, option_302, option_301, option_300, option_299, option_298, option_297, option_296];
    let mut player_222: reversi::Player = crate::reversi::Player::Player1;
    let mut player_223: reversi::Player = crate::reversi::Player::other(player_222);
    let mut option_304: std::option::Option<reversi::Player> = std::option::Option::Some(player_223);
    let mut option_305: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_224: reversi::Player = crate::reversi::Player::Player1;
    let mut player_225: reversi::Player = crate::reversi::Player::other(player_224);
    let mut option_306: std::option::Option<reversi::Player> = std::option::Option::Some(player_225);
    let mut player_226: reversi::Player = crate::reversi::Player::Player1;
    let mut option_307: std::option::Option<reversi::Player> = std::option::Option::Some(player_226);
    let mut option_308: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_227: reversi::Player = crate::reversi::Player::Player0;
    let mut player_228: reversi::Player = crate::reversi::Player::other(player_227);
    let mut option_309: std::option::Option<reversi::Player> = std::option::Option::Some(player_228);
    let mut player_229: reversi::Player = crate::reversi::Player::Player1;
    let mut option_310: std::option::Option<reversi::Player> = std::option::Option::Some(player_229);
    let mut option_311: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_38: [std::option::Option<reversi::Player>; 8] = [option_311, option_310, option_309, option_308, option_307, option_306, option_305, option_304];
    let mut player_230: reversi::Player = crate::reversi::Player::Player0;
    let mut option_312: std::option::Option<reversi::Player> = std::option::Option::Some(player_230);
    let mut option_313: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_314: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_231: reversi::Player = crate::reversi::Player::Player0;
    let mut player_232: reversi::Player = crate::reversi::Player::other(player_231);
    let mut option_315: std::option::Option<reversi::Player> = std::option::Option::Some(player_232);
    let mut option_316: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_233: reversi::Player = crate::reversi::Player::Player1;
    let mut player_234: reversi::Player = crate::reversi::Player::other(player_233);
    let mut option_317: std::option::Option<reversi::Player> = std::option::Option::Some(player_234);
    let mut player_235: reversi::Player = crate::reversi::Player::Player0;
    let mut option_318: std::option::Option<reversi::Player> = std::option::Option::Some(player_235);
    let mut option_319: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_39: [std::option::Option<reversi::Player>; 8] = [option_319, option_318, option_317, option_316, option_315, option_314, option_313, option_312];
    let mut player_236: reversi::Player = crate::reversi::Player::Player1;
    let mut option_320: std::option::Option<reversi::Player> = std::option::Option::Some(player_236);
    let mut option_321: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_237: reversi::Player = crate::reversi::Player::Player1;
    let mut option_322: std::option::Option<reversi::Player> = std::option::Option::Some(player_237);
    let mut player_238: reversi::Player = crate::reversi::Player::Player1;
    let mut option_323: std::option::Option<reversi::Player> = std::option::Option::Some(player_238);
    let mut player_239: reversi::Player = crate::reversi::Player::Player0;
    let mut option_324: std::option::Option<reversi::Player> = std::option::Option::Some(player_239);
    let mut player_240: reversi::Player = crate::reversi::Player::Player1;
    let mut option_325: std::option::Option<reversi::Player> = std::option::Option::Some(player_240);
    let mut player_241: reversi::Player = crate::reversi::Player::Player1;
    let mut option_326: std::option::Option<reversi::Player> = std::option::Option::Some(player_241);
    let mut option_327: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_40: [std::option::Option<reversi::Player>; 8] = [option_327, option_326, option_325, option_324, option_323, option_322, option_321, option_320];
    let mut option_array_array_4: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_40, option_array_39, option_array_38, option_array_32, option_array_36, option_array_35, option_array_34, option_array_33];
    let mut reversi_4: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_4, next: player_188, status: gamestate_4};
    let mut reversi_4_ref_0: &mut crate::reversi::Reversi = &mut reversi_4;
    crate::reversi::Reversi::check_state(reversi_4_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8003() {
    rusty_monitor::set_test_id(8003);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Win(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_8: connect_four::Player = crate::connect_four::Player::other(player_7);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_array_0: [std::option::Option<connect_four::Player>; 6] = [option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::Some(player_9);
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::Some(player_10);
    let mut player_11: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_9: std::option::Option<connect_four::Player> = std::option::Option::Some(player_11);
    let mut player_12: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_13: connect_four::Player = crate::connect_four::Player::other(player_12);
    let mut option_10: std::option::Option<connect_four::Player> = std::option::Option::Some(player_13);
    let mut player_14: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_11: std::option::Option<connect_four::Player> = std::option::Option::Some(player_14);
    let mut option_array_1: [std::option::Option<connect_four::Player>; 6] = [option_11, option_10, option_9, option_8, option_7, option_6];
    let mut player_15: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_16: connect_four::Player = crate::connect_four::Player::other(player_15);
    let mut option_12: std::option::Option<connect_four::Player> = std::option::Option::Some(player_16);
    let mut option_13: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_17: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_16: std::option::Option<connect_four::Player> = std::option::Option::Some(player_17);
    let mut player_18: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_17: std::option::Option<connect_four::Player> = std::option::Option::Some(player_18);
    let mut option_array_2: [std::option::Option<connect_four::Player>; 6] = [option_17, option_16, option_15, option_14, option_13, option_12];
    let mut player_19: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_19_ref_0: &connect_four::Player = &mut player_19;
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_2_ref_0: &connect_four::GameState = &mut gamestate_2;
    let mut player_21: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_22: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_0_ref_0);
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::InProgress;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7408() {
    rusty_monitor::set_test_id(7408);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_1);
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_7);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut player_10: reversi::Player = crate::reversi::Player::other(player_9);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut player_15: reversi::Player = crate::reversi::Player::other(player_14);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player0;
    let mut player_18: reversi::Player = crate::reversi::Player::other(player_17);
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut player_23: reversi::Player = crate::reversi::Player::other(player_22);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_29, option_28, option_27, option_26, option_25, option_24, option_23, option_22];
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut player_27: reversi::Player = crate::reversi::Player::other(player_26);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut player_30: reversi::Player = crate::reversi::Player::other(player_29);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_37, option_36, option_35, option_34, option_33, option_32, option_31, option_30];
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut player_33: reversi::Player = crate::reversi::Player::other(player_32);
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut player_35: reversi::Player = crate::reversi::Player::Player1;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_36: reversi::Player = crate::reversi::Player::Player0;
    let mut player_37: reversi::Player = crate::reversi::Player::other(player_36);
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut player_38: reversi::Player = crate::reversi::Player::Player1;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut player_39: reversi::Player = crate::reversi::Player::Player1;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_40: reversi::Player = crate::reversi::Player::Player1;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_52, option_51, option_50, option_49, option_48, option_47, option_46, option_45];
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_41: reversi::Player = crate::reversi::Player::Player1;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut player_42: reversi::Player = crate::reversi::Player::Player0;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_43: reversi::Player = crate::reversi::Player::Player0;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut player_44: reversi::Player = crate::reversi::Player::Player1;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_44);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_60, option_59, option_58, option_57, option_56, option_55, option_54, option_53];
    let mut player_45: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_45);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut option_61: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_0_ref_0);
    let mut backtrace_0: &snafu::Backtrace = std::option::Option::unwrap(option_61);
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_1_ref_0);
    panic!("From RustyUnit with love");
}
}