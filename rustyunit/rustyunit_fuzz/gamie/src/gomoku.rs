//! Gomoku game
//!
//! Check struct [`Gomoku`](https://docs.rs/gamie/*/gamie/gomoku/struct.Gomoku.html) for more information
//!
//! # Examples
//!
//! ```rust
//! # fn gomoku() {
//! use gamie::gomoku::{Gomoku, Player as GomokuPlayer};
//!
//! let mut game = Gomoku::new().unwrap();
//! game.place(GomokuPlayer::Player0, 7, 8).unwrap();
//! game.place(GomokuPlayer::Player1, 8, 7).unwrap();
//! // ...
//! # }
//! ```
use crate::std_lib::{iter, Box, Infallible};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use snafu::Snafu;
/// Gomoku
///
/// Passing an invalid position to a method will cause panic. Check the target position validity first when dealing with user input
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Gomoku {
    board: [[Option<Player>; 15]; 15],
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
impl Gomoku {
    /// Create a new Gomoku game.
    pub fn new() -> Result<Self, Infallible> {
        Ok(Self {
            board: [[None; 15]; 15],
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
        if let GameState::Win(player) = self.status { Some(player) } else { None }
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
    pub fn place(
        &mut self,
        player: Player,
        row: usize,
        col: usize,
    ) -> Result<(), GomokuError> {
        if self.is_ended() {
            return Err(GomokuError::GameEnded);
        }
        if player != self.next {
            return Err(GomokuError::WrongPlayer);
        }
        if self.board[row][col].is_some() {
            return Err(GomokuError::OccupiedPosition);
        }
        self.board[row][col] = Some(player);
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
                    if count == 5 && cell.is_some() {
                        self.status = GameState::Win(cell.unwrap());
                        return;
                    }
                }
            }
        }
        if self.board.iter().flatten().all(|cell| cell.is_some()) {
            self.status = GameState::Tie;
        }
    }
    fn get_connectable() -> impl Iterator<
        Item = Box<dyn Iterator<Item = (usize, usize)>>,
    > {
        let horizontal = (0usize..15)
            .map(move |row| {
                Box::new((0usize..15).map(move |col| (row, col)))
                    as Box<dyn Iterator<Item = (usize, usize)>>
            });
        let vertical = (0usize..15)
            .map(move |col| {
                Box::new((0usize..15).map(move |row| (row, col)))
                    as Box<dyn Iterator<Item = (usize, usize)>>
            });
        let horizontal_upper_left_to_lower_right = (0usize..15)
            .map(move |col| {
                Box::new(
                    iter::successors(
                            Some((0usize, col)),
                            |(row, col)| Some((row + 1, col + 1)),
                        )
                        .take(15 - col),
                ) as Box<dyn Iterator<Item = (usize, usize)>>
            });
        let vertical_upper_left_to_lower_right = (0usize..15)
            .map(move |row| {
                Box::new(
                    iter::successors(
                            Some((row, 0usize)),
                            |(row, col)| Some((row + 1, col + 1)),
                        )
                        .take(15 - row),
                ) as Box<dyn Iterator<Item = (usize, usize)>>
            });
        let horizontal_upper_right_to_lower_left = (0usize..15)
            .map(move |col| {
                Box::new(
                    iter::successors(
                            Some((0usize, col)),
                            |(row, col)| {
                                col.checked_sub(1).map(|new_col| (row + 1, new_col))
                            },
                        )
                        .take(1 + col),
                ) as Box<dyn Iterator<Item = (usize, usize)>>
            });
        let vertical_upper_right_to_lower_left = (0usize..15)
            .map(move |row| {
                Box::new(
                    iter::successors(
                            Some((row, 14usize)),
                            |(row, col)| Some((row + 1, col - 1)),
                        )
                        .take(15 - row),
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
/// Errors that can occur when placing a piece on the board
#[derive(Debug, Eq, PartialEq, Snafu)]
pub enum GomokuError {
    #[snafu(display("Wrong player"))]
    WrongPlayer,
    #[snafu(display("Occupied position"))]
    OccupiedPosition,
    #[snafu(display("The game was already end"))]
    GameEnded,
}
#[cfg(test)]
mod tests_llm_16_21 {
    use super::*;
    use crate::*;
    use gomoku::{Gomoku, GameState, Player};
    #[test]
    fn test_check_state_win_horizontal() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Gomoku::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player0, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player0, rug_fuzz_4, rug_fuzz_5).unwrap();
        game.place(Player::Player0, rug_fuzz_6, rug_fuzz_7).unwrap();
        game.place(Player::Player0, rug_fuzz_8, rug_fuzz_9).unwrap();
        debug_assert_eq!(game.status, GameState::Win(Player::Player0));
             }
});    }
    #[test]
    fn test_check_state_win_vertical() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Gomoku::new().unwrap();
        game.place(Player::Player1, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player1, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player1, rug_fuzz_4, rug_fuzz_5).unwrap();
        game.place(Player::Player1, rug_fuzz_6, rug_fuzz_7).unwrap();
        game.place(Player::Player1, rug_fuzz_8, rug_fuzz_9).unwrap();
        debug_assert_eq!(game.status, GameState::Win(Player::Player1));
             }
});    }
    #[test]
    fn test_check_state_win_diagonal_up_right() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Gomoku::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player0, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player0, rug_fuzz_4, rug_fuzz_5).unwrap();
        game.place(Player::Player0, rug_fuzz_6, rug_fuzz_7).unwrap();
        game.place(Player::Player0, rug_fuzz_8, rug_fuzz_9).unwrap();
        debug_assert_eq!(game.status, GameState::Win(Player::Player0));
             }
});    }
    #[test]
    fn test_check_state_win_diagonal_down_right() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Gomoku::new().unwrap();
        game.place(Player::Player1, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player1, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player1, rug_fuzz_4, rug_fuzz_5).unwrap();
        game.place(Player::Player1, rug_fuzz_6, rug_fuzz_7).unwrap();
        game.place(Player::Player1, rug_fuzz_8, rug_fuzz_9).unwrap();
        debug_assert_eq!(game.status, GameState::Win(Player::Player1));
             }
});    }
    #[test]
    fn test_check_state_tie() {
        let _rug_st_tests_llm_16_21_rrrruuuugggg_test_check_state_tie = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 4;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 1;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 2;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 3;
        let rug_fuzz_18 = 1;
        let rug_fuzz_19 = 4;
        let rug_fuzz_20 = 2;
        let rug_fuzz_21 = 0;
        let rug_fuzz_22 = 2;
        let rug_fuzz_23 = 1;
        let rug_fuzz_24 = 2;
        let rug_fuzz_25 = 2;
        let rug_fuzz_26 = 2;
        let rug_fuzz_27 = 3;
        let rug_fuzz_28 = 2;
        let rug_fuzz_29 = 4;
        let rug_fuzz_30 = 3;
        let rug_fuzz_31 = 0;
        let rug_fuzz_32 = 3;
        let rug_fuzz_33 = 1;
        let rug_fuzz_34 = 3;
        let rug_fuzz_35 = 2;
        let rug_fuzz_36 = 3;
        let rug_fuzz_37 = 3;
        let rug_fuzz_38 = 3;
        let rug_fuzz_39 = 4;
        let rug_fuzz_40 = 4;
        let rug_fuzz_41 = 0;
        let rug_fuzz_42 = 4;
        let rug_fuzz_43 = 1;
        let rug_fuzz_44 = 4;
        let rug_fuzz_45 = 2;
        let rug_fuzz_46 = 4;
        let rug_fuzz_47 = 3;
        let rug_fuzz_48 = 4;
        let rug_fuzz_49 = 4;
        let mut game = Gomoku::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player1, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player0, rug_fuzz_4, rug_fuzz_5).unwrap();
        game.place(Player::Player1, rug_fuzz_6, rug_fuzz_7).unwrap();
        game.place(Player::Player0, rug_fuzz_8, rug_fuzz_9).unwrap();
        game.place(Player::Player1, rug_fuzz_10, rug_fuzz_11).unwrap();
        game.place(Player::Player0, rug_fuzz_12, rug_fuzz_13).unwrap();
        game.place(Player::Player1, rug_fuzz_14, rug_fuzz_15).unwrap();
        game.place(Player::Player0, rug_fuzz_16, rug_fuzz_17).unwrap();
        game.place(Player::Player1, rug_fuzz_18, rug_fuzz_19).unwrap();
        game.place(Player::Player0, rug_fuzz_20, rug_fuzz_21).unwrap();
        game.place(Player::Player1, rug_fuzz_22, rug_fuzz_23).unwrap();
        game.place(Player::Player0, rug_fuzz_24, rug_fuzz_25).unwrap();
        game.place(Player::Player1, rug_fuzz_26, rug_fuzz_27).unwrap();
        game.place(Player::Player0, rug_fuzz_28, rug_fuzz_29).unwrap();
        game.place(Player::Player0, rug_fuzz_30, rug_fuzz_31).unwrap();
        game.place(Player::Player1, rug_fuzz_32, rug_fuzz_33).unwrap();
        game.place(Player::Player0, rug_fuzz_34, rug_fuzz_35).unwrap();
        game.place(Player::Player1, rug_fuzz_36, rug_fuzz_37).unwrap();
        game.place(Player::Player0, rug_fuzz_38, rug_fuzz_39).unwrap();
        game.place(Player::Player1, rug_fuzz_40, rug_fuzz_41).unwrap();
        game.place(Player::Player0, rug_fuzz_42, rug_fuzz_43).unwrap();
        game.place(Player::Player1, rug_fuzz_44, rug_fuzz_45).unwrap();
        game.place(Player::Player0, rug_fuzz_46, rug_fuzz_47).unwrap();
        game.place(Player::Player1, rug_fuzz_48, rug_fuzz_49).unwrap();
        debug_assert_eq!(game.status, GameState::Tie);
        let _rug_ed_tests_llm_16_21_rrrruuuugggg_test_check_state_tie = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_22 {
    use super::*;
    use crate::*;
    use std::convert::Infallible;
    #[test]
    fn test_get_valid_position() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Gomoku::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        let cell = game.get(rug_fuzz_2, rug_fuzz_3);
        debug_assert_eq!(cell, & Some(Player::Player0));
             }
});    }
    #[test]
    #[should_panic(expected = "index out of bounds: the len is 15 but the index is 15")]
    fn test_get_invalid_position() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let game = Gomoku::new().unwrap();
        let cell = game.get(rug_fuzz_0, rug_fuzz_1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_24_llm_16_23 {
    use super::*;
    use crate::*;
    use crate::gomoku::Gomoku;
    #[test]
    fn test_get_connectable() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let connectable = Gomoku::get_connectable();
        let expected_len = rug_fuzz_0;
        debug_assert_eq!(connectable.count(), expected_len);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_25 {
    use super::*;
    use crate::*;
    use gomoku::{Gomoku, Player};
    #[test]
    fn test_get_next_player() {
        let _rug_st_tests_llm_16_25_rrrruuuugggg_test_get_next_player = 0;
        let game = Gomoku::new().unwrap();
        debug_assert_eq!(game.get_next_player(), Player::Player0);
        let _rug_ed_tests_llm_16_25_rrrruuuugggg_test_get_next_player = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_26 {
    use super::*;
    use crate::*;
    use gomoku::Gomoku;
    use gomoku::GameState;
    #[test]
    fn test_is_ended_true() {
        let _rug_st_tests_llm_16_26_rrrruuuugggg_test_is_ended_true = 0;
        let gomoku = Gomoku {
            board: [
                [
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                ],
                [
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                ],
                [
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                ],
                [
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                ],
                [
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                ],
                [
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                ],
                [
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                ],
                [
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                ],
                [
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                ],
                [
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                ],
                [
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                ],
                [
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                ],
                [
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                ],
                [
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                ],
                [
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                    Some(Player::Player1),
                    Some(Player::Player0),
                ],
            ],
            next: Player::Player1,
            status: GameState::Win(Player::Player1),
        };
        debug_assert_eq!(gomoku.is_ended(), true);
        let _rug_ed_tests_llm_16_26_rrrruuuugggg_test_is_ended_true = 0;
    }
    #[test]
    fn test_is_ended_false() {
        let _rug_st_tests_llm_16_26_rrrruuuugggg_test_is_ended_false = 0;
        let gomoku = Gomoku::new().unwrap();
        debug_assert_eq!(gomoku.is_ended(), false);
        let _rug_ed_tests_llm_16_26_rrrruuuugggg_test_is_ended_false = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_27 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new() {
        let _rug_st_tests_llm_16_27_rrrruuuugggg_test_new = 0;
        let result = Gomoku::new();
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_27_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_28 {
    use crate::gomoku::{Gomoku, Player, GameState, GomokuError};
    #[test]
    fn test_place_valid_position() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Gomoku::new().unwrap();
        let result = game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(result, Ok(()));
        debug_assert_eq!(game.get(rug_fuzz_2, rug_fuzz_3), & Some(Player::Player0));
        debug_assert_eq!(game.get_next_player(), Player::Player1);
        debug_assert_eq!(game.is_ended(), false);
        debug_assert_eq!(game.winner(), None);
        debug_assert_eq!(game.status(), & GameState::InProgress);
             }
});    }
    #[test]
    fn test_place_invalid_position() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Gomoku::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        let result = game.place(Player::Player1, rug_fuzz_2, rug_fuzz_3);
        debug_assert_eq!(result, Err(GomokuError::OccupiedPosition));
        debug_assert_eq!(game.get(rug_fuzz_4, rug_fuzz_5), & Some(Player::Player0));
        debug_assert_eq!(game.get_next_player(), Player::Player1);
        debug_assert_eq!(game.is_ended(), false);
        debug_assert_eq!(game.winner(), None);
        debug_assert_eq!(game.status(), & GameState::InProgress);
             }
});    }
    #[test]
    fn test_place_wrong_player() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Gomoku::new().unwrap();
        let result = game.place(Player::Player1, rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(result, Err(GomokuError::WrongPlayer));
        debug_assert_eq!(game.get(rug_fuzz_2, rug_fuzz_3), & None);
        debug_assert_eq!(game.get_next_player(), Player::Player0);
        debug_assert_eq!(game.is_ended(), false);
        debug_assert_eq!(game.winner(), None);
        debug_assert_eq!(game.status(), & GameState::InProgress);
             }
});    }
    #[test]
    fn test_place_game_ended() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Gomoku::new().unwrap();
        for row in rug_fuzz_0..rug_fuzz_1 {
            for col in rug_fuzz_2..rug_fuzz_3 {
                game.place(Player::Player0, row, col).unwrap();
                game.place(Player::Player1, row, col).unwrap();
            }
        }
        let result = game.place(Player::Player0, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(result, Err(GomokuError::GameEnded));
        debug_assert_eq!(game.get(rug_fuzz_6, rug_fuzz_7), & None);
        debug_assert_eq!(game.get_next_player(), Player::Player0);
        debug_assert_eq!(game.is_ended(), true);
        debug_assert_eq!(game.winner(), None);
        debug_assert_eq!(game.status(), & GameState::Tie);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_30 {
    use super::*;
    use crate::*;
    #[test]
    fn test_status() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Gomoku::new().unwrap();
        debug_assert_eq!(* game.status(), GameState::InProgress);
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        debug_assert_eq!(* game.status(), GameState::InProgress);
        game.place(Player::Player1, rug_fuzz_2, rug_fuzz_3).unwrap();
        debug_assert_eq!(* game.status(), GameState::InProgress);
        game.place(Player::Player0, rug_fuzz_4, rug_fuzz_5).unwrap();
        debug_assert_eq!(* game.status(), GameState::InProgress);
        game.place(Player::Player1, rug_fuzz_6, rug_fuzz_7).unwrap();
        debug_assert_eq!(* game.status(), GameState::InProgress);
        game.place(Player::Player0, rug_fuzz_8, rug_fuzz_9).unwrap();
        debug_assert_eq!(* game.status(), GameState::InProgress);
        game.place(Player::Player1, rug_fuzz_10, rug_fuzz_11).unwrap();
        debug_assert_eq!(* game.status(), GameState::InProgress);
        game.place(Player::Player0, rug_fuzz_12, rug_fuzz_13).unwrap();
        debug_assert_eq!(* game.status(), GameState::InProgress);
        game.place(Player::Player1, rug_fuzz_14, rug_fuzz_15).unwrap();
        debug_assert_eq!(* game.status(), GameState::InProgress);
        game.place(Player::Player0, rug_fuzz_16, rug_fuzz_17).unwrap();
        debug_assert_eq!(* game.status(), GameState::Win(Player::Player0));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_31 {
    use super::*;
    use crate::*;
    use gomoku::{GameState, Gomoku, Player};
    #[test]
    fn test_winner_returns_none_when_game_is_not_ended() {
        let _rug_st_tests_llm_16_31_rrrruuuugggg_test_winner_returns_none_when_game_is_not_ended = 0;
        let gomoku = Gomoku::new().unwrap();
        let result = gomoku.winner();
        let expected = None;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_31_rrrruuuugggg_test_winner_returns_none_when_game_is_not_ended = 0;
    }
    #[test]
    fn test_winner_returns_none_when_game_is_tied() {
        let _rug_st_tests_llm_16_31_rrrruuuugggg_test_winner_returns_none_when_game_is_tied = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 2;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 2;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 2;
        let rug_fuzz_17 = 2;
        let rug_fuzz_18 = 0;
        let rug_fuzz_19 = 3;
        let rug_fuzz_20 = 1;
        let rug_fuzz_21 = 3;
        let rug_fuzz_22 = 0;
        let rug_fuzz_23 = 4;
        let rug_fuzz_24 = 1;
        let rug_fuzz_25 = 4;
        let rug_fuzz_26 = 2;
        let rug_fuzz_27 = 3;
        let rug_fuzz_28 = 2;
        let rug_fuzz_29 = 4;
        let mut gomoku = Gomoku::new().unwrap();
        gomoku.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        gomoku.place(Player::Player1, rug_fuzz_2, rug_fuzz_3).unwrap();
        gomoku.place(Player::Player0, rug_fuzz_4, rug_fuzz_5).unwrap();
        gomoku.place(Player::Player1, rug_fuzz_6, rug_fuzz_7).unwrap();
        gomoku.place(Player::Player0, rug_fuzz_8, rug_fuzz_9).unwrap();
        gomoku.place(Player::Player1, rug_fuzz_10, rug_fuzz_11).unwrap();
        gomoku.place(Player::Player1, rug_fuzz_12, rug_fuzz_13).unwrap();
        gomoku.place(Player::Player0, rug_fuzz_14, rug_fuzz_15).unwrap();
        gomoku.place(Player::Player1, rug_fuzz_16, rug_fuzz_17).unwrap();
        gomoku.place(Player::Player0, rug_fuzz_18, rug_fuzz_19).unwrap();
        gomoku.place(Player::Player1, rug_fuzz_20, rug_fuzz_21).unwrap();
        gomoku.place(Player::Player0, rug_fuzz_22, rug_fuzz_23).unwrap();
        gomoku.place(Player::Player1, rug_fuzz_24, rug_fuzz_25).unwrap();
        gomoku.place(Player::Player0, rug_fuzz_26, rug_fuzz_27).unwrap();
        gomoku.place(Player::Player1, rug_fuzz_28, rug_fuzz_29).unwrap();
        let result = gomoku.winner();
        let expected = None;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_31_rrrruuuugggg_test_winner_returns_none_when_game_is_tied = 0;
    }
    #[test]
    fn test_winner_returns_player_when_player_wins() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut gomoku = Gomoku::new().unwrap();
        gomoku.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        gomoku.place(Player::Player1, rug_fuzz_2, rug_fuzz_3).unwrap();
        gomoku.place(Player::Player0, rug_fuzz_4, rug_fuzz_5).unwrap();
        gomoku.place(Player::Player1, rug_fuzz_6, rug_fuzz_7).unwrap();
        gomoku.place(Player::Player0, rug_fuzz_8, rug_fuzz_9).unwrap();
        gomoku.place(Player::Player1, rug_fuzz_10, rug_fuzz_11).unwrap();
        gomoku.place(Player::Player0, rug_fuzz_12, rug_fuzz_13).unwrap();
        gomoku.place(Player::Player1, rug_fuzz_14, rug_fuzz_15).unwrap();
        gomoku.place(Player::Player0, rug_fuzz_16, rug_fuzz_17).unwrap();
        let result = gomoku.winner();
        let expected = Some(Player::Player0);
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_32 {
    use super::*;
    use crate::*;
    #[test]
    fn test_other_player0() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_other_player0 = 0;
        debug_assert_eq!(Player::Player0.other(), Player::Player1);
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_other_player0 = 0;
    }
    #[test]
    fn test_other_player1() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_other_player1 = 0;
        debug_assert_eq!(Player::Player1.other(), Player::Player0);
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_other_player1 = 0;
    }
}
