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
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use snafu::ErrorCompat;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6165() {
//    rusty_monitor::set_test_id(6165);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 3usize;
    let mut bool_2: bool = true;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_0_ref_0);
    let mut gamestate_2: reversi::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_0_ref_0);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut tuple_1: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8808() {
//    rusty_monitor::set_test_id(8808);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::LowerRight;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::LowerRight;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 3usize;
    let mut bool_2: bool = true;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_0_ref_0);
    let mut gamestate_2: reversi::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut direction_2: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_0_ref_0);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut direction_2_ref_0: &reversi::Direction = &mut direction_2;
    let mut tuple_1: (i8, i8) = crate::reversi::Direction::as_offset(direction_2_ref_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(direction_1_ref_0, direction_0_ref_0);
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_133() {
//    rusty_monitor::set_test_id(133);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut player_8: reversi::Player = crate::reversi::Player::Player1;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player0;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_13: reversi::Player = crate::reversi::Player::Player0;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut player_16: reversi::Player = crate::reversi::Player::other(player_15);
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut player_18: reversi::Player = crate::reversi::Player::other(player_17);
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut player_21: reversi::Player = crate::reversi::Player::other(player_20);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut player_27: reversi::Player = crate::reversi::Player::other(player_26);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut player_30: reversi::Player = crate::reversi::Player::other(player_29);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player1;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut player_35: reversi::Player = crate::reversi::Player::Player0;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut player_37: reversi::Player = crate::reversi::Player::Player1;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut player_39: reversi::Player = crate::reversi::Player::other(player_38);
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_40: reversi::Player = crate::reversi::Player::Player0;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_41: reversi::Player = crate::reversi::Player::Player1;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_42: reversi::Player = crate::reversi::Player::Player0;
    let mut player_43: reversi::Player = crate::reversi::Player::other(player_42);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut player_44: reversi::Player = crate::reversi::Player::Player1;
    let mut player_45: reversi::Player = crate::reversi::Player::other(player_44);
    let mut player_45_ref_0: &reversi::Player = &mut player_45;
    let mut player_46: reversi::Player = std::clone::Clone::clone(player_45_ref_0);
    let mut reversi_1: crate::reversi::Reversi = std::clone::Clone::clone(reversi_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7173() {
//    rusty_monitor::set_test_id(7173);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut gamestate_2: reversi::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_0_ref_0);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut tuple_0: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6704() {
//    rusty_monitor::set_test_id(6704);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_1);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut player_7: reversi::Player = crate::reversi::Player::other(player_6);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_8: reversi::Player = crate::reversi::Player::Player1;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut player_16: reversi::Player = crate::reversi::Player::other(player_15);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player1;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_20: reversi::Player = crate::reversi::Player::Player1;
    let mut player_21: reversi::Player = crate::reversi::Player::other(player_20);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut player_23: reversi::Player = crate::reversi::Player::other(player_22);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut player_27: reversi::Player = crate::reversi::Player::other(player_26);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut player_33: reversi::Player = crate::reversi::Player::Player1;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut player_35: reversi::Player = crate::reversi::Player::other(player_34);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_36: reversi::Player = crate::reversi::Player::Player0;
    let mut player_37: reversi::Player = crate::reversi::Player::other(player_36);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut reversi_0_ref_0: &mut crate::reversi::Reversi = &mut reversi_0;
    crate::reversi::Direction::iter();
    crate::reversi::Reversi::check_state(reversi_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7149() {
//    rusty_monitor::set_test_id(7149);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::LowerRight;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_0: std::option::Option<gomoku::Player> = std::option::Option::Some(player_2);
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_1: std::option::Option<gomoku::Player> = std::option::Option::Some(player_3);
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut reversierror_2: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_0_ref_0);
    let mut gamestate_3: reversi::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut direction_1: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut tuple_1: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_222() {
//    rusty_monitor::set_test_id(222);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut usize_0: usize = 2usize;
    let mut usize_1: usize = 0usize;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut player_5: reversi::Player = crate::reversi::Player::other(player_4);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut player_8: reversi::Player = crate::reversi::Player::Player1;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player0;
    let mut player_10: reversi::Player = crate::reversi::Player::other(player_9);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut player_11: reversi::Player = crate::reversi::Player::Player0;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut player_15: reversi::Player = crate::reversi::Player::other(player_14);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player0;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut player_20: reversi::Player = crate::reversi::Player::other(player_19);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player0;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_30: reversi::Player = crate::reversi::Player::Player1;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player1;
    let mut player_32: reversi::Player = crate::reversi::Player::other(player_31);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut result_0: std::result::Result<(), reversi::ReversiError> = crate::reversi::Reversi::check_position_validity(reversi_0_ref_0, usize_1, usize_0, player_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4105() {
//    rusty_monitor::set_test_id(4105);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Lower;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut reversierror_2: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_2_ref_0: &reversi::ReversiError = &mut reversierror_2;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut player_1_ref_0: &reversi::Player = &mut player_1;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut vec_1: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_2_ref_0);
    let mut direction_1: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_0_ref_0, gamestate_1_ref_0);
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut tuple_1: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
    let mut direction_2: reversi::Direction = crate::reversi::Direction::UpperRight;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_144() {
//    rusty_monitor::set_test_id(144);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 0usize;
    let mut bool_2: bool = true;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Right;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut player_2_ref_0: &tictactoe::Player = &mut player_2;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_3);
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4_ref_0: &tictactoe::Player = &mut player_4;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_4_ref_0, gamestate_0_ref_0);
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_0_ref_0);
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_212() {
//    rusty_monitor::set_test_id(212);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut player_5: reversi::Player = crate::reversi::Player::other(player_4);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut player_7: reversi::Player = crate::reversi::Player::other(player_6);
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut player_9: reversi::Player = crate::reversi::Player::other(player_8);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut player_11: reversi::Player = crate::reversi::Player::other(player_10);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut player_16: reversi::Player = crate::reversi::Player::other(player_15);
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut player_18: reversi::Player = crate::reversi::Player::Player1;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut player_20: reversi::Player = crate::reversi::Player::other(player_19);
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut player_23: reversi::Player = crate::reversi::Player::other(player_22);
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut player_27: reversi::Player = crate::reversi::Player::other(player_26);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut player_30: reversi::Player = crate::reversi::Player::other(player_29);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut player_33: reversi::Player = crate::reversi::Player::other(player_32);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_35: reversi::Player = crate::reversi::Player::Player1;
    let mut player_36: reversi::Player = crate::reversi::Player::other(player_35);
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_37: reversi::Player = crate::reversi::Player::Player1;
    let mut player_38: reversi::Player = crate::reversi::Player::other(player_37);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_39: reversi::Player = crate::reversi::Player::Player0;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_40: reversi::Player = crate::reversi::Player::Player1;
    let mut player_41: reversi::Player = crate::reversi::Player::other(player_40);
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut player_42: reversi::Player = crate::reversi::Player::Player1;
    let mut player_43: reversi::Player = crate::reversi::Player::other(player_42);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_44: reversi::Player = crate::reversi::Player::Player1;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_44);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_45: reversi::Player = crate::reversi::Player::Player1;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_45);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut bool_0: bool = crate::reversi::Reversi::can_player_move(reversi_0_ref_0, player_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9131() {
//    rusty_monitor::set_test_id(9131);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_5: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_4_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4392() {
//    rusty_monitor::set_test_id(4392);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut vec_1: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_1);
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_1_ref_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut bool_0: bool = std::cmp::PartialEq::eq(reversierror_1_ref_0, reversierror_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7977() {
//    rusty_monitor::set_test_id(7977);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut minesweepererror_3: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_5: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut minesweepererror_3_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_3;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2645() {
//    rusty_monitor::set_test_id(2645);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Right;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_0: std::option::Option<gomoku::Player> = std::option::Option::Some(player_2);
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_1: std::option::Option<gomoku::Player> = std::option::Option::Some(player_3);
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_2: std::option::Option<gomoku::Player> = std::option::Option::Some(player_4);
    let mut option_3: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_6: gomoku::Player = crate::gomoku::Player::other(player_5);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut reversierror_2: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_0_ref_0);
    let mut gamestate_3: reversi::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut direction_1: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut tuple_1: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9045() {
//    rusty_monitor::set_test_id(9045);
    let mut usize_0: usize = 15usize;
    let mut usize_1: usize = 0usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_5);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player0;
    let mut player_10: reversi::Player = crate::reversi::Player::other(player_9);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut player_12: reversi::Player = crate::reversi::Player::other(player_11);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut player_13: reversi::Player = crate::reversi::Player::Player0;
    let mut player_14: reversi::Player = crate::reversi::Player::other(player_13);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player0;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_17: reversi::Player = crate::reversi::Player::Player0;
    let mut player_18: reversi::Player = crate::reversi::Player::other(player_17);
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut player_23: reversi::Player = crate::reversi::Player::other(player_22);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_25: reversi::Player = crate::reversi::Player::Player1;
    let mut player_26: reversi::Player = crate::reversi::Player::other(player_25);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_27: reversi::Player = crate::reversi::Player::Player0;
    let mut player_28: reversi::Player = crate::reversi::Player::other(player_27);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut player_30: reversi::Player = crate::reversi::Player::other(player_29);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_35: reversi::Player = crate::reversi::Player::Player1;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut player_37: reversi::Player = crate::reversi::Player::Player1;
    let mut player_38: reversi::Player = crate::reversi::Player::other(player_37);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut player_39: reversi::Player = crate::reversi::Player::Player1;
    let mut player_40: reversi::Player = crate::reversi::Player::other(player_39);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut player_41: reversi::Player = crate::reversi::Player::Player0;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_42: reversi::Player = crate::reversi::Player::Player1;
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_43: reversi::Player = crate::reversi::Player::Player0;
    let mut player_44: reversi::Player = crate::reversi::Player::other(player_43);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_44);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_45: reversi::Player = crate::reversi::Player::Player0;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_45);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &mut crate::reversi::Reversi = &mut reversi_0;
    let mut result_0: std::result::Result<(), reversi::ReversiError> = crate::reversi::Reversi::place(reversi_0_ref_0, player_0, usize_1, usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8100() {
//    rusty_monitor::set_test_id(8100);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_0_ref_0);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut tuple_1: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3089() {
//    rusty_monitor::set_test_id(3089);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Left;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::Lower;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 3usize;
    let mut bool_2: bool = true;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_0_ref_0);
    let mut gamestate_2: reversi::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut direction_2: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_0_ref_0);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut direction_2_ref_0: &reversi::Direction = &mut direction_2;
    let mut tuple_1: (i8, i8) = crate::reversi::Direction::as_offset(direction_2_ref_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(direction_1_ref_0, direction_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_65() {
//    rusty_monitor::set_test_id(65);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_1);
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_5);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_7);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut player_12: reversi::Player = crate::reversi::Player::other(player_11);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_13: reversi::Player = crate::reversi::Player::Player0;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut player_17: reversi::Player = crate::reversi::Player::other(player_16);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut player_18: reversi::Player = crate::reversi::Player::Player1;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut player_20: reversi::Player = crate::reversi::Player::Player1;
    let mut player_21: reversi::Player = crate::reversi::Player::other(player_20);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut player_23: reversi::Player = crate::reversi::Player::Player0;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player0;
    let mut player_30: reversi::Player = crate::reversi::Player::other(player_29);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut player_33: reversi::Player = crate::reversi::Player::other(player_32);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut player_35: reversi::Player = crate::reversi::Player::Player1;
    let mut player_36: reversi::Player = crate::reversi::Player::other(player_35);
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_37: reversi::Player = crate::reversi::Player::Player1;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut player_39: reversi::Player = crate::reversi::Player::other(player_38);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_40: reversi::Player = crate::reversi::Player::Player1;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut player_41: reversi::Player = crate::reversi::Player::Player0;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut player_42: reversi::Player = crate::reversi::Player::Player0;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut option_64: std::option::Option<reversi::Player> = crate::reversi::Reversi::winner(reversi_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_84() {
//    rusty_monitor::set_test_id(84);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::LowerRight;
    let mut usize_0: usize = 4usize;
    let mut usize_1: usize = 50usize;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut player_9: reversi::Player = crate::reversi::Player::Player0;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_16: reversi::Player = crate::reversi::Player::Player0;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut player_20: reversi::Player = crate::reversi::Player::Player0;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut player_28: reversi::Player = crate::reversi::Player::other(player_27);
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut player_29: reversi::Player = crate::reversi::Player::Player0;
    let mut player_30: reversi::Player = crate::reversi::Player::other(player_29);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut player_31: reversi::Player = crate::reversi::Player::Player1;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut player_34: reversi::Player = crate::reversi::Player::other(player_33);
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_35: reversi::Player = crate::reversi::Player::Player1;
    let mut player_36: reversi::Player = crate::reversi::Player::other(player_35);
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_3, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut usize_2: usize = 7usize;
    let mut usize_3: usize = 28usize;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::LowerRight;
    let mut player_37: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_64: std::option::Option<(usize, usize)> = crate::reversi::Reversi::check_occupied_line_in_direction(reversi_0_ref_0, usize_1, usize_0, direction_0, player_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2599() {
//    rusty_monitor::set_test_id(2599);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 3usize;
    let mut bool_2: bool = true;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_0_ref_0);
    let mut gamestate_2: reversi::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_0_ref_0);
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut tuple_1: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
    crate::reversi::Direction::iter();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_230() {
//    rusty_monitor::set_test_id(230);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_0_ref_0: &reversi::Player = &mut player_0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1_ref_0: &reversi::Player = &mut player_1;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2_ref_0: &reversi::Player = &mut player_2;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut player_3_ref_0: &reversi::Player = &mut player_3;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut player_4_ref_0: &reversi::Player = &mut player_4;
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut player_5_ref_0: &reversi::Player = &mut player_5;
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut player_6_ref_0: &reversi::Player = &mut player_6;
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_7);
    let mut player_8_ref_0: &reversi::Player = &mut player_8;
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut player_9_ref_0: &reversi::Player = &mut player_9;
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut player_10_ref_0: &reversi::Player = &mut player_10;
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut player_11_ref_0: &reversi::Player = &mut player_11;
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut player_12_ref_0: &reversi::Player = &mut player_12;
    let mut bool_0: bool = std::cmp::PartialEq::eq(player_12_ref_0, player_11_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(player_10_ref_0, player_9_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(player_8_ref_0, player_6_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(player_5_ref_0, player_4_ref_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(player_3_ref_0, player_2_ref_0);
    let mut bool_5: bool = std::cmp::PartialEq::eq(player_1_ref_0, player_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2067() {
//    rusty_monitor::set_test_id(2067);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut reversierror_2: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_2_ref_0: &reversi::ReversiError = &mut reversierror_2;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut player_2_ref_0: &reversi::Player = &mut player_2;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut vec_1: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_1);
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_2_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut bool_1: bool = std::cmp::PartialEq::eq(reversierror_2_ref_0, reversierror_1_ref_0);
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(reversierror_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_423() {
//    rusty_monitor::set_test_id(423);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_1);
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player1;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut player_16: reversi::Player = crate::reversi::Player::other(player_15);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player0;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut player_20: reversi::Player = crate::reversi::Player::other(player_19);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut player_23: reversi::Player = crate::reversi::Player::other(player_22);
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut player_25: reversi::Player = crate::reversi::Player::Player1;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_35: reversi::Player = crate::reversi::Player::Player0;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut player_37: reversi::Player = crate::reversi::Player::other(player_36);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_39: reversi::Player = crate::reversi::Player::Player1;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_40: reversi::Player = crate::reversi::Player::Player1;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut player_41: reversi::Player = crate::reversi::Reversi::get_next_player(reversi_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_415() {
//    rusty_monitor::set_test_id(415);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut usize_0: usize = 0usize;
    let mut usize_1: usize = 7usize;
    let mut usize_2: usize = 8usize;
    let mut usize_3: usize = 7usize;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut player_7: reversi::Player = crate::reversi::Player::other(player_6);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player0;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut player_11: reversi::Player = crate::reversi::Player::other(player_10);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_13: reversi::Player = crate::reversi::Player::Player0;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player0;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player0;
    let mut player_18: reversi::Player = crate::reversi::Player::other(player_17);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut player_20: reversi::Player = crate::reversi::Player::other(player_19);
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut player_23: reversi::Player = crate::reversi::Player::Player0;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_25: reversi::Player = crate::reversi::Player::Player1;
    let mut player_26: reversi::Player = crate::reversi::Player::other(player_25);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_27: reversi::Player = crate::reversi::Player::Player0;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut player_29: reversi::Player = crate::reversi::Player::other(player_28);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_30: reversi::Player = crate::reversi::Player::Player1;
    let mut player_31: reversi::Player = crate::reversi::Player::other(player_30);
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut player_33: reversi::Player = crate::reversi::Player::other(player_32);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut player_35: reversi::Player = crate::reversi::Player::other(player_34);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_36: reversi::Player = crate::reversi::Player::Player0;
    let mut player_37: reversi::Player = crate::reversi::Player::other(player_36);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_39: reversi::Player = crate::reversi::Player::Player0;
    let mut player_40: reversi::Player = crate::reversi::Player::other(player_39);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut player_41: reversi::Player = crate::reversi::Player::Player0;
    let mut player_42: reversi::Player = crate::reversi::Player::other(player_41);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_43: reversi::Player = crate::reversi::Player::Player1;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_44: reversi::Player = crate::reversi::Player::Player0;
    let mut player_45: reversi::Player = crate::reversi::Player::other(player_44);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_45);
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_46: reversi::Player = crate::reversi::Player::Player1;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_46);
    let mut player_47: reversi::Player = crate::reversi::Player::Player0;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_47);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut reversi_0_ref_0: &mut crate::reversi::Reversi = &mut reversi_0;
    crate::reversi::Reversi::flip(reversi_0_ref_0, usize_3, usize_2, usize_1, usize_0, direction_0, player_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6406() {
//    rusty_monitor::set_test_id(6406);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Upper;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_0);
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut reversierror_2: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_0_ref_0);
    let mut gamestate_4: reversi::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut direction_1: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut tuple_1: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5941() {
//    rusty_monitor::set_test_id(5941);
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::LowerLeft;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_2);
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Win(player_5);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut gamestate_3: reversi::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut direction_1: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut tuple_0: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3911() {
//    rusty_monitor::set_test_id(3911);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut player_2_ref_0: &reversi::Player = &mut player_2;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_2_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut bool_0: bool = std::cmp::PartialEq::eq(reversierror_1_ref_0, reversierror_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_418() {
//    rusty_monitor::set_test_id(418);
    let mut usize_0: usize = 16usize;
    let mut usize_1: usize = 6usize;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut player_7: reversi::Player = crate::reversi::Player::other(player_6);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut player_8: reversi::Player = crate::reversi::Player::Player1;
    let mut player_9: reversi::Player = crate::reversi::Player::other(player_8);
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut player_11: reversi::Player = crate::reversi::Player::other(player_10);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut player_16: reversi::Player = crate::reversi::Player::other(player_15);
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut player_18: reversi::Player = crate::reversi::Player::Player1;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut player_20: reversi::Player = crate::reversi::Player::other(player_19);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player0;
    let mut player_23: reversi::Player = crate::reversi::Player::other(player_22);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut player_25: reversi::Player = crate::reversi::Player::other(player_24);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_26: reversi::Player = crate::reversi::Player::Player0;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut player_28: reversi::Player = crate::reversi::Player::Player0;
    let mut player_29: reversi::Player = crate::reversi::Player::other(player_28);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut player_30: reversi::Player = crate::reversi::Player::Player1;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut player_33: reversi::Player = crate::reversi::Player::other(player_32);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player0;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut player_35: reversi::Player = crate::reversi::Player::Player0;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_37: reversi::Player = crate::reversi::Player::Player1;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_38: reversi::Player = crate::reversi::Player::Player1;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_39: reversi::Player = crate::reversi::Player::Player1;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_40: reversi::Player = crate::reversi::Player::Player1;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_41: reversi::Player = crate::reversi::Player::Player0;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_42: reversi::Player = crate::reversi::Player::Player0;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut option_64: &std::option::Option<reversi::Player> = crate::reversi::Reversi::get(reversi_0_ref_0, usize_1, usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7951() {
//    rusty_monitor::set_test_id(7951);
    let mut usize_0: usize = 1usize;
    let mut usize_1: usize = 3usize;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_1);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_7);
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player0;
    let mut player_10: reversi::Player = crate::reversi::Player::other(player_9);
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut player_13: reversi::Player = crate::reversi::Player::other(player_12);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut player_15: reversi::Player = crate::reversi::Player::other(player_14);
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player0;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_17: reversi::Player = crate::reversi::Player::Player0;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_18: reversi::Player = crate::reversi::Player::Player0;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut player_20: reversi::Player = crate::reversi::Player::Player1;
    let mut player_21: reversi::Player = crate::reversi::Player::other(player_20);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut player_25: reversi::Player = crate::reversi::Player::Player1;
    let mut player_26: reversi::Player = crate::reversi::Player::other(player_25);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_27: reversi::Player = crate::reversi::Player::Player0;
    let mut player_28: reversi::Player = crate::reversi::Player::other(player_27);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut player_32: reversi::Player = crate::reversi::Player::other(player_31);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut player_34: reversi::Player = crate::reversi::Player::other(player_33);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut player_35: reversi::Player = crate::reversi::Player::Player1;
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_36: reversi::Player = crate::reversi::Player::Player0;
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut player_37: reversi::Player = crate::reversi::Player::Player1;
    let mut player_38: reversi::Player = crate::reversi::Player::other(player_37);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut player_39: reversi::Player = crate::reversi::Player::Player1;
    let mut player_40: reversi::Player = crate::reversi::Player::other(player_39);
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut player_41: reversi::Player = crate::reversi::Player::Player1;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut player_42: reversi::Player = crate::reversi::Player::Player0;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut player_43: reversi::Player = crate::reversi::Player::Player1;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut player_44: reversi::Player = crate::reversi::Player::Player1;
    let mut player_45: reversi::Player = crate::reversi::Player::other(player_44);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_45);
    let mut player_46: reversi::Player = crate::reversi::Player::Player1;
    let mut player_47: reversi::Player = crate::reversi::Player::other(player_46);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_47);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_0, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Right;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut player_48: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Win(player_48);
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut player_49: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_50: connect_four::Player = crate::connect_four::Player::other(player_49);
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut player_51: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Win(player_51);
    let mut gamestate_4_ref_0: &connect_four::GameState = &mut gamestate_4;
    let mut player_52: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::Win(player_52);
    let mut gamestate_5_ref_0: &connect_four::GameState = &mut gamestate_5;
    let mut gamestate_6: connect_four::GameState = crate::connect_four::GameState::Win(player_50);
    let mut gamestate_6_ref_0: &connect_four::GameState = &mut gamestate_6;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut reversierror_2: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_0_ref_0);
    let mut gamestate_7: reversi::GameState = std::clone::Clone::clone(gamestate_2_ref_0);
    let mut direction_1: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_7_ref_0: &reversi::GameState = &mut gamestate_7;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut tuple_1: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut option_64: &std::option::Option<reversi::Player> = crate::reversi::Reversi::get(reversi_0_ref_0, usize_1, usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4958() {
//    rusty_monitor::set_test_id(4958);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut player_2_ref_0: &reversi::Player = &mut player_2;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut vec_1: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_1);
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_2_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut bool_1: bool = std::cmp::PartialEq::eq(reversierror_1_ref_0, reversierror_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6826() {
//    rusty_monitor::set_test_id(6826);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_0_ref_0);
    let mut gamestate_1: reversi::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut tuple_1: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7412() {
//    rusty_monitor::set_test_id(7412);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut player_1_ref_0: &reversi::Player = &mut player_1;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut vec_1: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_1);
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(player_1_ref_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut bool_1: bool = std::cmp::PartialEq::eq(reversierror_1_ref_0, reversierror_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_616() {
//    rusty_monitor::set_test_id(616);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_5);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_7);
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::Some(player_8);
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_9: reversi::Player = crate::reversi::Player::Player0;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut player_11: reversi::Player = crate::reversi::Player::other(player_10);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_12: reversi::Player = crate::reversi::Player::Player1;
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut player_14: reversi::Player = crate::reversi::Player::Player0;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_15: reversi::Player = crate::reversi::Player::Player0;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut player_17: reversi::Player = crate::reversi::Player::other(player_16);
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_17);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_18: reversi::Player = crate::reversi::Player::Player1;
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut player_20: reversi::Player = crate::reversi::Player::other(player_19);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut player_23: reversi::Player = crate::reversi::Player::other(player_22);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_25: reversi::Player = crate::reversi::Player::Player1;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::Some(player_27);
    let mut player_28: reversi::Player = crate::reversi::Player::Player1;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_30: reversi::Player = crate::reversi::Player::Player1;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut player_31: reversi::Player = crate::reversi::Player::Player1;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player1;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut player_35: reversi::Player = crate::reversi::Player::Player0;
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut player_37: reversi::Player = crate::reversi::Player::other(player_36);
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_38: reversi::Player = crate::reversi::Player::Player0;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut player_39: reversi::Player = crate::reversi::Player::Player1;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_39);
    let mut player_40: reversi::Player = crate::reversi::Player::Player0;
    let mut player_41: reversi::Player = crate::reversi::Player::other(player_40);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut player_42: reversi::Player = crate::reversi::Player::Player0;
    let mut player_43: reversi::Player = crate::reversi::Player::other(player_42);
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::Some(player_43);
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_44: reversi::Player = crate::reversi::Player::Player0;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_44);
    let mut player_45: reversi::Player = crate::reversi::Player::Player1;
    let mut player_46: reversi::Player = crate::reversi::Player::other(player_45);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_46);
    let mut player_47: reversi::Player = crate::reversi::Player::Player0;
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_47);
    let mut player_48: reversi::Player = crate::reversi::Player::Player1;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_48);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_49: reversi::Player = crate::reversi::Player::Player0;
    let mut player_50: reversi::Player = crate::reversi::Player::other(player_49);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::Some(player_50);
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut option_64: std::option::Option<reversi::Player> = crate::reversi::Reversi::winner(reversi_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8252() {
//    rusty_monitor::set_test_id(8252);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Upper;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut direction_1: reversi::Direction = crate::reversi::Direction::UpperLeft;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_1);
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut reversierror_2: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_0_ref_0);
    let mut gamestate_3: reversi::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut tuple_1: (i8, i8) = crate::reversi::Direction::as_offset(direction_1_ref_0);
    let mut direction_2: reversi::Direction = std::clone::Clone::clone(direction_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1795() {
//    rusty_monitor::set_test_id(1795);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut vec_1: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut vec_2: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut gamestate_6: reversi::GameState = crate::reversi::GameState::Win(player_2);
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut gamestate_7: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_1);
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_3_ref_0, gamestate_1_ref_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5912() {
//    rusty_monitor::set_test_id(5912);
    let mut direction_0: reversi::Direction = crate::reversi::Direction::Left;
    let mut direction_0_ref_0: &reversi::Direction = &mut direction_0;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_0: std::option::Option<gomoku::Player> = std::option::Option::Some(player_3);
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_1: std::option::Option<gomoku::Player> = std::option::Option::Some(player_4);
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_2: std::option::Option<gomoku::Player> = std::option::Option::Some(player_5);
    let mut option_3: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut reversierror_2: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(reversierror_0_ref_0);
    let mut gamestate_2: reversi::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
    let mut direction_1: reversi::Direction = crate::reversi::Direction::UpperRight;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut direction_1_ref_0: &reversi::Direction = &mut direction_1;
    let mut tuple_1: (i8, i8) = crate::reversi::Direction::as_offset(direction_0_ref_0);
    let mut player_6: gomoku::Player = crate::gomoku::Player::Player0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9153() {
//    rusty_monitor::set_test_id(9153);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_6: reversi::Player = crate::reversi::Player::Player1;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_8: reversi::Player = crate::reversi::Player::Player0;
    let mut player_9: reversi::Player = crate::reversi::Player::other(player_8);
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut player_10: reversi::Player = crate::reversi::Player::Player0;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::Some(player_11);
    let mut player_12: reversi::Player = crate::reversi::Player::Player0;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut player_13: reversi::Player = crate::reversi::Player::Player0;
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut player_15: reversi::Player = crate::reversi::Player::other(player_14);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut player_18: reversi::Player = crate::reversi::Player::other(player_17);
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player0;
    let mut player_20: reversi::Player = crate::reversi::Player::other(player_19);
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_21: reversi::Player = crate::reversi::Player::Player1;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player0;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_24: reversi::Player = crate::reversi::Player::Player0;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut player_26: reversi::Player = crate::reversi::Player::other(player_25);
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut player_27: reversi::Player = crate::reversi::Player::Player1;
    let mut player_28: reversi::Player = crate::reversi::Player::other(player_27);
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_29: reversi::Player = crate::reversi::Player::Player1;
    let mut player_30: reversi::Player = crate::reversi::Player::other(player_29);
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_31: reversi::Player = crate::reversi::Player::Player1;
    let mut player_32: reversi::Player = crate::reversi::Player::other(player_31);
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::Some(player_33);
    let mut player_34: reversi::Player = crate::reversi::Player::Player1;
    let mut player_35: reversi::Player = crate::reversi::Player::other(player_34);
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_37: reversi::Player = crate::reversi::Player::Player0;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_38: reversi::Player = crate::reversi::Player::Player1;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::Some(player_38);
    let mut player_39: reversi::Player = crate::reversi::Player::Player0;
    let mut player_40: reversi::Player = crate::reversi::Player::other(player_39);
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_40);
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_41: reversi::Player = crate::reversi::Player::Player1;
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::Some(player_41);
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_42: reversi::Player = crate::reversi::Player::Player0;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::Some(player_42);
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_43: reversi::Player = crate::reversi::Player::Player0;
    let mut player_44: reversi::Player = crate::reversi::Player::other(player_43);
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::Some(player_44);
    let mut player_45: reversi::Player = crate::reversi::Player::Player0;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_45);
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_2, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut player_46: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_47: gomoku::Player = crate::gomoku::Player::other(player_46);
    let mut player_48: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_49: gomoku::Player = crate::gomoku::Player::other(player_48);
    let mut bool_0: bool = crate::reversi::Reversi::can_player_move(reversi_0_ref_0, player_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_254() {
//    rusty_monitor::set_test_id(254);
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::Some(player_5);
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut player_7: reversi::Player = crate::reversi::Player::other(player_6);
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_7);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_8: reversi::Player = crate::reversi::Player::Player1;
    let mut player_9: reversi::Player = crate::reversi::Player::other(player_8);
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::Some(player_9);
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_10: reversi::Player = crate::reversi::Player::Player1;
    let mut option_12: std::option::Option<reversi::Player> = std::option::Option::Some(player_10);
    let mut option_13: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_15: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<reversi::Player>; 8] = [option_15, option_14, option_13, option_12, option_11, option_10, option_9, option_8];
    let mut player_11: reversi::Player = crate::reversi::Player::Player1;
    let mut player_12: reversi::Player = crate::reversi::Player::other(player_11);
    let mut option_16: std::option::Option<reversi::Player> = std::option::Option::Some(player_12);
    let mut option_17: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_18: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_13: reversi::Player = crate::reversi::Player::Player1;
    let mut option_19: std::option::Option<reversi::Player> = std::option::Option::Some(player_13);
    let mut option_20: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_14: reversi::Player = crate::reversi::Player::Player1;
    let mut option_21: std::option::Option<reversi::Player> = std::option::Option::Some(player_14);
    let mut option_22: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_23: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<reversi::Player>; 8] = [option_23, option_22, option_21, option_20, option_19, option_18, option_17, option_16];
    let mut player_15: reversi::Player = crate::reversi::Player::Player1;
    let mut option_24: std::option::Option<reversi::Player> = std::option::Option::Some(player_15);
    let mut option_25: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_26: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_16: reversi::Player = crate::reversi::Player::Player1;
    let mut option_27: std::option::Option<reversi::Player> = std::option::Option::Some(player_16);
    let mut player_17: reversi::Player = crate::reversi::Player::Player1;
    let mut player_18: reversi::Player = crate::reversi::Player::other(player_17);
    let mut option_28: std::option::Option<reversi::Player> = std::option::Option::Some(player_18);
    let mut player_19: reversi::Player = crate::reversi::Player::Player1;
    let mut option_29: std::option::Option<reversi::Player> = std::option::Option::Some(player_19);
    let mut player_20: reversi::Player = crate::reversi::Player::Player1;
    let mut option_30: std::option::Option<reversi::Player> = std::option::Option::Some(player_20);
    let mut player_21: reversi::Player = crate::reversi::Player::Player0;
    let mut option_31: std::option::Option<reversi::Player> = std::option::Option::Some(player_21);
    let mut option_array_3: [std::option::Option<reversi::Player>; 8] = [option_31, option_30, option_29, option_28, option_27, option_26, option_25, option_24];
    let mut player_22: reversi::Player = crate::reversi::Player::Player1;
    let mut option_32: std::option::Option<reversi::Player> = std::option::Option::Some(player_22);
    let mut option_33: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_23: reversi::Player = crate::reversi::Player::Player1;
    let mut option_34: std::option::Option<reversi::Player> = std::option::Option::Some(player_23);
    let mut player_24: reversi::Player = crate::reversi::Player::Player1;
    let mut option_35: std::option::Option<reversi::Player> = std::option::Option::Some(player_24);
    let mut option_36: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_25: reversi::Player = crate::reversi::Player::Player0;
    let mut option_37: std::option::Option<reversi::Player> = std::option::Option::Some(player_25);
    let mut option_38: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_39: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_4: [std::option::Option<reversi::Player>; 8] = [option_39, option_38, option_37, option_36, option_35, option_34, option_33, option_32];
    let mut player_26: reversi::Player = crate::reversi::Player::Player1;
    let mut option_40: std::option::Option<reversi::Player> = std::option::Option::Some(player_26);
    let mut option_41: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_27: reversi::Player = crate::reversi::Player::Player0;
    let mut player_28: reversi::Player = crate::reversi::Player::other(player_27);
    let mut option_42: std::option::Option<reversi::Player> = std::option::Option::Some(player_28);
    let mut player_29: reversi::Player = crate::reversi::Player::Player0;
    let mut option_43: std::option::Option<reversi::Player> = std::option::Option::Some(player_29);
    let mut option_44: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_45: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_30: reversi::Player = crate::reversi::Player::Player0;
    let mut option_46: std::option::Option<reversi::Player> = std::option::Option::Some(player_30);
    let mut player_31: reversi::Player = crate::reversi::Player::Player0;
    let mut option_47: std::option::Option<reversi::Player> = std::option::Option::Some(player_31);
    let mut option_array_5: [std::option::Option<reversi::Player>; 8] = [option_47, option_46, option_45, option_44, option_43, option_42, option_41, option_40];
    let mut option_48: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_49: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_50: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_51: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_52: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_32: reversi::Player = crate::reversi::Player::Player0;
    let mut option_53: std::option::Option<reversi::Player> = std::option::Option::Some(player_32);
    let mut player_33: reversi::Player = crate::reversi::Player::Player0;
    let mut player_34: reversi::Player = crate::reversi::Player::other(player_33);
    let mut option_54: std::option::Option<reversi::Player> = std::option::Option::Some(player_34);
    let mut option_55: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_6: [std::option::Option<reversi::Player>; 8] = [option_55, option_54, option_53, option_52, option_51, option_50, option_49, option_48];
    let mut option_56: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_57: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_58: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_35: reversi::Player = crate::reversi::Player::Player1;
    let mut option_59: std::option::Option<reversi::Player> = std::option::Option::Some(player_35);
    let mut option_60: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_36: reversi::Player = crate::reversi::Player::Player1;
    let mut option_61: std::option::Option<reversi::Player> = std::option::Option::Some(player_36);
    let mut player_37: reversi::Player = crate::reversi::Player::Player0;
    let mut option_62: std::option::Option<reversi::Player> = std::option::Option::Some(player_37);
    let mut option_63: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_7: [std::option::Option<reversi::Player>; 8] = [option_63, option_62, option_61, option_60, option_59, option_58, option_57, option_56];
    let mut option_array_array_0: [[std::option::Option<reversi::Player>; 8]; 8] = [option_array_7, option_array_6, option_array_5, option_array_4, option_array_3, option_array_2, option_array_1, option_array_0];
    let mut reversi_0: crate::reversi::Reversi = crate::reversi::Reversi {board: option_array_array_0, next: player_1, status: gamestate_0};
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut gamestate_1: &reversi::GameState = crate::reversi::Reversi::status(reversi_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_534() {
//    rusty_monitor::set_test_id(534);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut reversierror_2: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut reversierror_2_ref_0: &reversi::ReversiError = &mut reversierror_2;
    let mut reversierror_3: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_3_ref_0: &reversi::ReversiError = &mut reversierror_3;
    let mut reversierror_4: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_4_ref_0: &reversi::ReversiError = &mut reversierror_4;
    let mut reversierror_5: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_5_ref_0: &reversi::ReversiError = &mut reversierror_5;
    let mut reversierror_6: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_6_ref_0: &reversi::ReversiError = &mut reversierror_6;
    let mut reversierror_7: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut reversierror_7_ref_0: &reversi::ReversiError = &mut reversierror_7;
    let mut reversierror_8: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_8_ref_0: &reversi::ReversiError = &mut reversierror_8;
    let mut reversierror_9: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_9_ref_0: &reversi::ReversiError = &mut reversierror_9;
    let mut reversierror_10: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
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
}