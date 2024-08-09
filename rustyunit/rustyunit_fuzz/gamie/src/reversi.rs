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
    ) -> Result<(), ReversiError> {
        self.simple_check_position_validity(row, col, player)?;
        let mut flipped = false;
        for dir in Direction::iter() {
            if let Some((to_row, to_col))
                = self.check_occupied_line_in_direction(row, col, dir, player)
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
        self
            .status = match black_count.cmp(&white_count) {
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
        iter::successors(
                Some((row, col)),
                move |(row, col)| {
                    let (offset_row, offset_col) = dir.as_offset();
                    Some((
                        (*row as i8 + offset_row) as usize,
                        (*col as i8 + offset_col) as usize,
                    ))
                },
            )
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
        assert_eq!(game.place(Player::Player1, 2, 6), Err(ReversiError::WrongPlayer));
        assert_eq!(
            game.place(Player::Player0, 2, 6), Err(ReversiError::InvalidPosition)
        );
    }
}
#[cfg(test)]
mod tests_llm_16_49 {
    use crate::reversi::Direction;
    #[test]
    fn test_as_offset() {
        let _rug_st_tests_llm_16_49_rrrruuuugggg_test_as_offset = 0;
        debug_assert_eq!(Direction::Upper.as_offset(), (- 1, 0));
        debug_assert_eq!(Direction::UpperRight.as_offset(), (- 1, 1));
        debug_assert_eq!(Direction::Right.as_offset(), (0, 1));
        debug_assert_eq!(Direction::LowerRight.as_offset(), (1, 1));
        debug_assert_eq!(Direction::Lower.as_offset(), (1, 0));
        debug_assert_eq!(Direction::LowerLeft.as_offset(), (1, - 1));
        debug_assert_eq!(Direction::Left.as_offset(), (0, - 1));
        debug_assert_eq!(Direction::UpperLeft.as_offset(), (- 1, - 1));
        let _rug_ed_tests_llm_16_49_rrrruuuugggg_test_as_offset = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_52 {
    use super::*;
    use crate::*;
    #[test]
    fn test_other() {
        let _rug_st_tests_llm_16_52_rrrruuuugggg_test_other = 0;
        debug_assert_eq!(Player::Player0.other(), Player::Player1);
        debug_assert_eq!(Player::Player1.other(), Player::Player0);
        let _rug_ed_tests_llm_16_52_rrrruuuugggg_test_other = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_55 {
    use super::*;
    use crate::*;
    use reversi::{Direction, GameState, Player};
    #[test]
    fn test_check_occupied_line_in_direction() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut board = [[None; 8]; 8];
        let reversi = Reversi {
            board,
            next: Player::Player0,
            status: GameState::InProgress,
        };
        let row = rug_fuzz_0;
        let col = rug_fuzz_1;
        let dir = Direction::Upper;
        let player = Player::Player0;
        let result = reversi.check_occupied_line_in_direction(row, col, dir, player);
        debug_assert_eq!(result, None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_56 {
    use super::*;
    use crate::*;
    #[test]
    fn test_check_position_validity_valid_position() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let game = Reversi::new().unwrap();
        debug_assert!(
            game.check_position_validity(rug_fuzz_0, rug_fuzz_1, Player::Player0).is_ok()
        );
             }
});    }
    #[test]
    fn test_check_position_validity_invalid_position() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let game = Reversi::new().unwrap();
        debug_assert!(
            game.check_position_validity(rug_fuzz_0, rug_fuzz_1, Player::Player0)
            .is_err()
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_57 {
    use super::*;
    use crate::*;
    #[test]
    fn test_check_state() {
        let _rug_st_tests_llm_16_57_rrrruuuugggg_test_check_state = 0;
        let mut game = Reversi::new().unwrap();
        game.check_state();
        debug_assert_eq!(game.status(), & GameState::InProgress);
        let _rug_ed_tests_llm_16_57_rrrruuuugggg_test_check_state = 0;
    }
    #[test]
    fn test_check_state_win() {
        let _rug_st_tests_llm_16_57_rrrruuuugggg_test_check_state_win = 0;
        let mut game = Reversi::new().unwrap();
        game
            .board = [
            [
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
            ],
            [
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
            ],
            [
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
            ],
            [
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
            ],
            [
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
            ],
            [
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
            ],
            [
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
            ],
            [
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
            ],
        ];
        game.check_state();
        debug_assert_eq!(game.status(), & GameState::Win(Player::Player0));
        let _rug_ed_tests_llm_16_57_rrrruuuugggg_test_check_state_win = 0;
    }
    #[test]
    fn test_check_state_tie() {
        let _rug_st_tests_llm_16_57_rrrruuuugggg_test_check_state_tie = 0;
        let mut game = Reversi::new().unwrap();
        game
            .board = [
            [
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player0),
                Some(Player::Player0),
            ],
            [
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player0),
                Some(Player::Player0),
            ],
            [
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player1),
                Some(Player::Player1),
            ],
            [
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player1),
                Some(Player::Player1),
            ],
            [
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player1),
                Some(Player::Player1),
            ],
            [
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player1),
                Some(Player::Player1),
            ],
            [
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player0),
                Some(Player::Player0),
            ],
            [
                Some(Player::Player0),
                Some(Player::Player0),
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player1),
                Some(Player::Player0),
                Some(Player::Player0),
            ],
        ];
        game.check_state();
        debug_assert_eq!(game.status(), & GameState::Tie);
        let _rug_ed_tests_llm_16_57_rrrruuuugggg_test_check_state_tie = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_59 {
    use super::*;
    use crate::*;
    use crate::reversi::{Direction, GameState, Player, Reversi};
    #[test]
    fn test_flip() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Reversi::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player1, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player1, rug_fuzz_4, rug_fuzz_5).unwrap();
        game.flip(
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            Direction::Left,
            Player::Player0,
        );
        debug_assert_eq!(& Some(Player::Player0), game.get(3, 2));
        debug_assert_eq!(& Some(Player::Player0), game.get(3, 1));
        debug_assert_eq!(& Some(Player::Player0), game.get(3, 0));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_60 {
    use super::*;
    use crate::*;
    #[test]
    fn test_get_valid_position() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let game = Reversi::new().unwrap();
        let result = game.get(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(result, & Some(Player::Player0));
             }
});    }
    #[test]
    #[should_panic]
    fn test_get_invalid_position() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let game = Reversi::new().unwrap();
        let _result = game.get(rug_fuzz_0, rug_fuzz_1);
             }
});    }
    #[test]
    fn test_get_next_player() {
        let _rug_st_tests_llm_16_60_rrrruuuugggg_test_get_next_player = 0;
        let game = Reversi::new().unwrap();
        let result = game.get_next_player();
        debug_assert_eq!(result, Player::Player0);
        let _rug_ed_tests_llm_16_60_rrrruuuugggg_test_get_next_player = 0;
    }
    #[test]
    fn test_get_status() {
        let _rug_st_tests_llm_16_60_rrrruuuugggg_test_get_status = 0;
        let game = Reversi::new().unwrap();
        let result = game.status();
        debug_assert_eq!(result, & GameState::InProgress);
        let _rug_ed_tests_llm_16_60_rrrruuuugggg_test_get_status = 0;
    }
    #[test]
    fn test_place_valid_position() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Reversi::new().unwrap();
        let result = game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(result, Ok(()));
             }
});    }
    #[test]
    #[should_panic]
    fn test_place_invalid_position() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Reversi::new().unwrap();
        let _result = game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_61 {
    use super::*;
    use crate::*;
    #[test]
    fn test_get_next_player() {
        let _rug_st_tests_llm_16_61_rrrruuuugggg_test_get_next_player = 0;
        let game = Reversi::new().unwrap();
        let next_player = game.get_next_player();
        debug_assert_eq!(next_player, Player::Player0);
        let _rug_ed_tests_llm_16_61_rrrruuuugggg_test_get_next_player = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_63 {
    use crate::reversi::GameState;
    use crate::reversi::Player;
    use crate::reversi::Reversi;
    use crate::reversi::Direction;
    use crate::reversi::ReversiError;
    #[test]
    fn test_is_ended() {
        let _rug_st_tests_llm_16_63_rrrruuuugggg_test_is_ended = 0;
        let reversi = Reversi::new().unwrap();
        debug_assert_eq!(reversi.is_ended(), false);
        let _rug_ed_tests_llm_16_63_rrrruuuugggg_test_is_ended = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_64 {
    use super::*;
    use crate::*;
    use reversi::Direction;
    #[test]
    fn test_iter_positions_in_direction_from() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let reversi = Reversi::new().unwrap();
        let result = reversi
            .iter_positions_in_direction_from(rug_fuzz_0, rug_fuzz_1, Direction::Right)
            .collect::<Vec<_>>();
        debug_assert_eq!(result, vec![(3, 4), (3, 5), (3, 6), (3, 7)]);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_65 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let game = Reversi::new().unwrap();
        debug_assert_eq!(game.board[rug_fuzz_0] [rug_fuzz_1], Some(Player::Player0));
        debug_assert_eq!(game.board[rug_fuzz_2] [rug_fuzz_3], Some(Player::Player0));
        debug_assert_eq!(game.board[rug_fuzz_4] [rug_fuzz_5], Some(Player::Player1));
        debug_assert_eq!(game.board[rug_fuzz_6] [rug_fuzz_7], Some(Player::Player1));
        debug_assert_eq!(game.next, Player::Player0);
        debug_assert_eq!(game.status, GameState::InProgress);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_66 {
    use super::*;
    use crate::*;
    #[test]
    fn test_place_valid_position() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Reversi::new().unwrap();
        let result = game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1);
        debug_assert!(result.is_ok());
        debug_assert_eq!(game.get(rug_fuzz_2, rug_fuzz_3), & Some(Player::Player0));
        debug_assert_eq!(game.get_next_player(), Player::Player1);
             }
});    }
    #[test]
    #[should_panic(expected = "Panic message you expect")]
    fn test_place_invalid_position() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Reversi::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
             }
});    }
    #[test]
    fn test_place_flipped() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Reversi::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        debug_assert_eq!(game.get(rug_fuzz_2, rug_fuzz_3), & Some(Player::Player0));
        debug_assert_eq!(game.get(rug_fuzz_4, rug_fuzz_5), & Some(Player::Player0));
        debug_assert_eq!(game.get(rug_fuzz_6, rug_fuzz_7), & Some(Player::Player0));
        debug_assert_eq!(game.get_next_player(), Player::Player1);
             }
});    }
    #[test]
    fn test_place_not_flipped() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Reversi::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        debug_assert_eq!(game.get(rug_fuzz_2, rug_fuzz_3), & Some(Player::Player0));
        debug_assert_eq!(game.get(rug_fuzz_4, rug_fuzz_5), & Some(Player::Player0));
        debug_assert!(game.place(Player::Player1, rug_fuzz_6, rug_fuzz_7).is_err());
        debug_assert_eq!(game.get_next_player(), Player::Player0);
             }
});    }
    #[test]
    fn test_place_end_game() {
        let _rug_st_tests_llm_16_66_rrrruuuugggg_test_place_end_game = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 3;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 4;
        let rug_fuzz_10 = 4;
        let rug_fuzz_11 = 2;
        let rug_fuzz_12 = 4;
        let rug_fuzz_13 = 4;
        let rug_fuzz_14 = 4;
        let rug_fuzz_15 = 3;
        let rug_fuzz_16 = 3;
        let rug_fuzz_17 = 4;
        let rug_fuzz_18 = 3;
        let rug_fuzz_19 = 5;
        let rug_fuzz_20 = 4;
        let rug_fuzz_21 = 5;
        let rug_fuzz_22 = 2;
        let rug_fuzz_23 = 5;
        let rug_fuzz_24 = 5;
        let rug_fuzz_25 = 4;
        let rug_fuzz_26 = 5;
        let rug_fuzz_27 = 5;
        let rug_fuzz_28 = 3;
        let rug_fuzz_29 = 6;
        let rug_fuzz_30 = 6;
        let rug_fuzz_31 = 6;
        let rug_fuzz_32 = 4;
        let rug_fuzz_33 = 6;
        let rug_fuzz_34 = 6;
        let rug_fuzz_35 = 5;
        let rug_fuzz_36 = 5;
        let rug_fuzz_37 = 6;
        let rug_fuzz_38 = 6;
        let rug_fuzz_39 = 3;
        let rug_fuzz_40 = 3;
        let rug_fuzz_41 = 7;
        let rug_fuzz_42 = 6;
        let rug_fuzz_43 = 7;
        let rug_fuzz_44 = 5;
        let rug_fuzz_45 = 7;
        let rug_fuzz_46 = 7;
        let rug_fuzz_47 = 5;
        let rug_fuzz_48 = 5;
        let rug_fuzz_49 = 2;
        let rug_fuzz_50 = 7;
        let rug_fuzz_51 = 2;
        let rug_fuzz_52 = 6;
        let rug_fuzz_53 = 2;
        let rug_fuzz_54 = 5;
        let rug_fuzz_55 = 1;
        let rug_fuzz_56 = 6;
        let rug_fuzz_57 = 4;
        let rug_fuzz_58 = 7;
        let rug_fuzz_59 = 6;
        let rug_fuzz_60 = 6;
        let rug_fuzz_61 = 7;
        let rug_fuzz_62 = 5;
        let rug_fuzz_63 = 0;
        let rug_fuzz_64 = 7;
        let rug_fuzz_65 = 7;
        let rug_fuzz_66 = 0;
        let rug_fuzz_67 = 0;
        let mut game = Reversi::new().unwrap();
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
        game.place(Player::Player1, rug_fuzz_30, rug_fuzz_31).unwrap();
        game.place(Player::Player0, rug_fuzz_32, rug_fuzz_33).unwrap();
        game.place(Player::Player1, rug_fuzz_34, rug_fuzz_35).unwrap();
        game.place(Player::Player0, rug_fuzz_36, rug_fuzz_37).unwrap();
        game.place(Player::Player1, rug_fuzz_38, rug_fuzz_39).unwrap();
        game.place(Player::Player0, rug_fuzz_40, rug_fuzz_41).unwrap();
        game.place(Player::Player1, rug_fuzz_42, rug_fuzz_43).unwrap();
        game.place(Player::Player0, rug_fuzz_44, rug_fuzz_45).unwrap();
        game.place(Player::Player1, rug_fuzz_46, rug_fuzz_47).unwrap();
        game.place(Player::Player0, rug_fuzz_48, rug_fuzz_49).unwrap();
        game.place(Player::Player1, rug_fuzz_50, rug_fuzz_51).unwrap();
        game.place(Player::Player0, rug_fuzz_52, rug_fuzz_53).unwrap();
        game.place(Player::Player1, rug_fuzz_54, rug_fuzz_55).unwrap();
        game.place(Player::Player0, rug_fuzz_56, rug_fuzz_57).unwrap();
        game.place(Player::Player1, rug_fuzz_58, rug_fuzz_59).unwrap();
        game.place(Player::Player0, rug_fuzz_60, rug_fuzz_61).unwrap();
        game.place(Player::Player1, rug_fuzz_62, rug_fuzz_63).unwrap();
        game.place(Player::Player0, rug_fuzz_64, rug_fuzz_65).unwrap();
        debug_assert_eq!(game.status(), & GameState::Win(Player::Player0));
        debug_assert_eq!(game.winner(), Some(Player::Player0));
        debug_assert!(game.place(Player::Player1, rug_fuzz_66, rug_fuzz_67).is_err());
        debug_assert_eq!(game.get_next_player(), Player::Player0);
        let _rug_ed_tests_llm_16_66_rrrruuuugggg_test_place_end_game = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_67 {
    use super::*;
    use crate::*;
    use crate::reversi::{Player, ReversiError, Reversi};
    #[test]
    fn test_simple_check_position_validity_game_ended() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let game = Reversi {
            board: [[None; 8]; 8],
            next: Player::Player0,
            status: GameState::Win(Player::Player0),
        };
        let result = game
            .simple_check_position_validity(rug_fuzz_0, rug_fuzz_1, Player::Player0);
        debug_assert_eq!(Err(ReversiError::GameEnded), result);
             }
});    }
    #[test]
    fn test_simple_check_position_validity_wrong_player() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let game = Reversi {
            board: [[None; 8]; 8],
            next: Player::Player0,
            status: GameState::InProgress,
        };
        let result = game
            .simple_check_position_validity(rug_fuzz_0, rug_fuzz_1, Player::Player1);
        debug_assert_eq!(Err(ReversiError::WrongPlayer), result);
             }
});    }
    #[test]
    fn test_simple_check_position_validity_occupied_position() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = Reversi::new().unwrap();
        game.board[rug_fuzz_0][rug_fuzz_1] = Some(Player::Player0);
        let result = game
            .simple_check_position_validity(rug_fuzz_2, rug_fuzz_3, Player::Player0);
        debug_assert_eq!(Err(ReversiError::OccupiedPosition), result);
             }
});    }
    #[test]
    fn test_simple_check_position_validity_valid_position() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let game = Reversi::new().unwrap();
        let result = game
            .simple_check_position_validity(rug_fuzz_0, rug_fuzz_1, Player::Player0);
        debug_assert_eq!(Ok(()), result);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_68 {
    use crate::reversi::{Reversi, GameState, Player};
    #[test]
    fn test_status() {
        let _rug_st_tests_llm_16_68_rrrruuuugggg_test_status = 0;
        let reversi = Reversi::new().unwrap();
        let status = reversi.status();
        debug_assert_eq!(* status, GameState::InProgress);
        let _rug_ed_tests_llm_16_68_rrrruuuugggg_test_status = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_69 {
    use crate::reversi::{Reversi, GameState, Player};
    #[test]
    fn test_winner_returns_none_when_game_in_progress() {
        let _rug_st_tests_llm_16_69_rrrruuuugggg_test_winner_returns_none_when_game_in_progress = 0;
        let game = Reversi::new().unwrap();
        debug_assert_eq!(game.winner(), None);
        let _rug_ed_tests_llm_16_69_rrrruuuugggg_test_winner_returns_none_when_game_in_progress = 0;
    }
    #[test]
    fn test_winner_returns_none_when_game_tied() {
        let _rug_st_tests_llm_16_69_rrrruuuugggg_test_winner_returns_none_when_game_tied = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 7;
        let rug_fuzz_3 = 7;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 7;
        let rug_fuzz_6 = 7;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 7;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 6;
        let rug_fuzz_14 = 7;
        let rug_fuzz_15 = 6;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 0;
        let rug_fuzz_18 = 1;
        let rug_fuzz_19 = 7;
        let rug_fuzz_20 = 6;
        let rug_fuzz_21 = 0;
        let rug_fuzz_22 = 6;
        let rug_fuzz_23 = 7;
        let rug_fuzz_24 = 6;
        let rug_fuzz_25 = 1;
        let rug_fuzz_26 = 6;
        let rug_fuzz_27 = 6;
        let rug_fuzz_28 = 1;
        let rug_fuzz_29 = 6;
        let rug_fuzz_30 = 1;
        let rug_fuzz_31 = 1;
        let rug_fuzz_32 = 6;
        let rug_fuzz_33 = 6;
        let rug_fuzz_34 = 1;
        let rug_fuzz_35 = 1;
        let rug_fuzz_36 = 2;
        let rug_fuzz_37 = 0;
        let rug_fuzz_38 = 2;
        let rug_fuzz_39 = 7;
        let rug_fuzz_40 = 5;
        let rug_fuzz_41 = 0;
        let rug_fuzz_42 = 5;
        let rug_fuzz_43 = 7;
        let rug_fuzz_44 = 6;
        let rug_fuzz_45 = 2;
        let rug_fuzz_46 = 6;
        let rug_fuzz_47 = 5;
        let rug_fuzz_48 = 5;
        let rug_fuzz_49 = 2;
        let rug_fuzz_50 = 5;
        let rug_fuzz_51 = 5;
        let rug_fuzz_52 = 2;
        let rug_fuzz_53 = 1;
        let rug_fuzz_54 = 2;
        let rug_fuzz_55 = 1;
        let rug_fuzz_56 = 2;
        let rug_fuzz_57 = 6;
        let rug_fuzz_58 = 2;
        let rug_fuzz_59 = 6;
        let rug_fuzz_60 = 5;
        let rug_fuzz_61 = 1;
        let rug_fuzz_62 = 5;
        let rug_fuzz_63 = 1;
        let rug_fuzz_64 = 5;
        let rug_fuzz_65 = 6;
        let rug_fuzz_66 = 5;
        let rug_fuzz_67 = 6;
        let rug_fuzz_68 = 2;
        let rug_fuzz_69 = 2;
        let rug_fuzz_70 = 2;
        let rug_fuzz_71 = 5;
        let rug_fuzz_72 = 5;
        let rug_fuzz_73 = 2;
        let rug_fuzz_74 = 5;
        let rug_fuzz_75 = 5;
        let rug_fuzz_76 = 3;
        let rug_fuzz_77 = 0;
        let rug_fuzz_78 = 3;
        let rug_fuzz_79 = 7;
        let rug_fuzz_80 = 4;
        let rug_fuzz_81 = 0;
        let rug_fuzz_82 = 4;
        let rug_fuzz_83 = 7;
        let rug_fuzz_84 = 6;
        let rug_fuzz_85 = 3;
        let rug_fuzz_86 = 6;
        let rug_fuzz_87 = 4;
        let rug_fuzz_88 = 5;
        let rug_fuzz_89 = 3;
        let rug_fuzz_90 = 5;
        let rug_fuzz_91 = 4;
        let rug_fuzz_92 = 3;
        let rug_fuzz_93 = 1;
        let rug_fuzz_94 = 3;
        let rug_fuzz_95 = 1;
        let rug_fuzz_96 = 3;
        let rug_fuzz_97 = 6;
        let rug_fuzz_98 = 3;
        let rug_fuzz_99 = 6;
        let rug_fuzz_100 = 4;
        let rug_fuzz_101 = 1;
        let rug_fuzz_102 = 4;
        let rug_fuzz_103 = 1;
        let rug_fuzz_104 = 4;
        let rug_fuzz_105 = 6;
        let rug_fuzz_106 = 4;
        let rug_fuzz_107 = 6;
        let rug_fuzz_108 = 3;
        let rug_fuzz_109 = 2;
        let rug_fuzz_110 = 3;
        let rug_fuzz_111 = 5;
        let rug_fuzz_112 = 4;
        let rug_fuzz_113 = 2;
        let rug_fuzz_114 = 4;
        let rug_fuzz_115 = 5;
        let rug_fuzz_116 = 3;
        let rug_fuzz_117 = 3;
        let rug_fuzz_118 = 3;
        let rug_fuzz_119 = 4;
        let rug_fuzz_120 = 4;
        let rug_fuzz_121 = 3;
        let rug_fuzz_122 = 4;
        let rug_fuzz_123 = 4;
        let rug_fuzz_124 = 2;
        let rug_fuzz_125 = 3;
        let rug_fuzz_126 = 2;
        let rug_fuzz_127 = 4;
        let rug_fuzz_128 = 5;
        let rug_fuzz_129 = 3;
        let rug_fuzz_130 = 5;
        let rug_fuzz_131 = 4;
        let rug_fuzz_132 = 3;
        let rug_fuzz_133 = 5;
        let rug_fuzz_134 = 3;
        let rug_fuzz_135 = 3;
        let rug_fuzz_136 = 3;
        let rug_fuzz_137 = 4;
        let rug_fuzz_138 = 3;
        let rug_fuzz_139 = 4;
        let rug_fuzz_140 = 2;
        let rug_fuzz_141 = 4;
        let rug_fuzz_142 = 2;
        let rug_fuzz_143 = 3;
        let rug_fuzz_144 = 5;
        let rug_fuzz_145 = 3;
        let rug_fuzz_146 = 5;
        let rug_fuzz_147 = 4;
        let rug_fuzz_148 = 4;
        let rug_fuzz_149 = 3;
        let rug_fuzz_150 = 4;
        let rug_fuzz_151 = 4;
        let rug_fuzz_152 = 3;
        let rug_fuzz_153 = 5;
        let rug_fuzz_154 = 3;
        let rug_fuzz_155 = 2;
        let rug_fuzz_156 = 4;
        let rug_fuzz_157 = 5;
        let rug_fuzz_158 = 4;
        let rug_fuzz_159 = 2;
        let rug_fuzz_160 = 2;
        let rug_fuzz_161 = 5;
        let rug_fuzz_162 = 2;
        let rug_fuzz_163 = 2;
        let rug_fuzz_164 = 5;
        let rug_fuzz_165 = 5;
        let rug_fuzz_166 = 5;
        let rug_fuzz_167 = 2;
        let rug_fuzz_168 = 2;
        let rug_fuzz_169 = 5;
        let rug_fuzz_170 = 2;
        let rug_fuzz_171 = 2;
        let rug_fuzz_172 = 5;
        let rug_fuzz_173 = 5;
        let rug_fuzz_174 = 5;
        let rug_fuzz_175 = 2;
        let rug_fuzz_176 = 2;
        let rug_fuzz_177 = 0;
        let rug_fuzz_178 = 2;
        let rug_fuzz_179 = 7;
        let rug_fuzz_180 = 5;
        let rug_fuzz_181 = 0;
        let rug_fuzz_182 = 5;
        let rug_fuzz_183 = 7;
        let rug_fuzz_184 = 0;
        let rug_fuzz_185 = 2;
        let rug_fuzz_186 = 7;
        let rug_fuzz_187 = 2;
        let rug_fuzz_188 = 0;
        let rug_fuzz_189 = 5;
        let rug_fuzz_190 = 7;
        let rug_fuzz_191 = 5;
        let rug_fuzz_192 = 2;
        let rug_fuzz_193 = 0;
        let rug_fuzz_194 = 5;
        let rug_fuzz_195 = 0;
        let rug_fuzz_196 = 2;
        let rug_fuzz_197 = 7;
        let rug_fuzz_198 = 5;
        let rug_fuzz_199 = 7;
        let rug_fuzz_200 = 0;
        let rug_fuzz_201 = 3;
        let rug_fuzz_202 = 7;
        let rug_fuzz_203 = 3;
        let rug_fuzz_204 = 0;
        let rug_fuzz_205 = 4;
        let rug_fuzz_206 = 7;
        let rug_fuzz_207 = 4;
        let rug_fuzz_208 = 3;
        let rug_fuzz_209 = 0;
        let rug_fuzz_210 = 3;
        let rug_fuzz_211 = 7;
        let rug_fuzz_212 = 4;
        let rug_fuzz_213 = 0;
        let rug_fuzz_214 = 4;
        let rug_fuzz_215 = 7;
        let rug_fuzz_216 = 3;
        let rug_fuzz_217 = 7;
        let rug_fuzz_218 = 3;
        let rug_fuzz_219 = 0;
        let rug_fuzz_220 = 4;
        let rug_fuzz_221 = 7;
        let rug_fuzz_222 = 4;
        let rug_fuzz_223 = 0;
        let rug_fuzz_224 = 3;
        let rug_fuzz_225 = 1;
        let rug_fuzz_226 = 3;
        let rug_fuzz_227 = 6;
        let rug_fuzz_228 = 4;
        let rug_fuzz_229 = 1;
        let rug_fuzz_230 = 4;
        let rug_fuzz_231 = 6;
        let rug_fuzz_232 = 1;
        let rug_fuzz_233 = 3;
        let rug_fuzz_234 = 6;
        let rug_fuzz_235 = 3;
        let rug_fuzz_236 = 1;
        let rug_fuzz_237 = 4;
        let rug_fuzz_238 = 6;
        let rug_fuzz_239 = 4;
        let rug_fuzz_240 = 1;
        let rug_fuzz_241 = 3;
        let rug_fuzz_242 = 6;
        let rug_fuzz_243 = 3;
        let rug_fuzz_244 = 1;
        let rug_fuzz_245 = 4;
        let rug_fuzz_246 = 6;
        let rug_fuzz_247 = 4;
        let rug_fuzz_248 = 2;
        let rug_fuzz_249 = 1;
        let rug_fuzz_250 = 5;
        let rug_fuzz_251 = 6;
        let rug_fuzz_252 = 2;
        let rug_fuzz_253 = 6;
        let rug_fuzz_254 = 5;
        let rug_fuzz_255 = 1;
        let rug_fuzz_256 = 1;
        let rug_fuzz_257 = 2;
        let rug_fuzz_258 = 6;
        let rug_fuzz_259 = 5;
        let rug_fuzz_260 = 1;
        let rug_fuzz_261 = 5;
        let rug_fuzz_262 = 6;
        let rug_fuzz_263 = 2;
        let rug_fuzz_264 = 2;
        let rug_fuzz_265 = 1;
        let rug_fuzz_266 = 5;
        let rug_fuzz_267 = 6;
        let rug_fuzz_268 = 6;
        let rug_fuzz_269 = 2;
        let rug_fuzz_270 = 1;
        let rug_fuzz_271 = 5;
        let rug_fuzz_272 = 5;
        let rug_fuzz_273 = 2;
        let rug_fuzz_274 = 2;
        let rug_fuzz_275 = 5;
        let rug_fuzz_276 = 6;
        let rug_fuzz_277 = 5;
        let rug_fuzz_278 = 1;
        let rug_fuzz_279 = 2;
        let rug_fuzz_280 = 1;
        let rug_fuzz_281 = 1;
        let rug_fuzz_282 = 6;
        let rug_fuzz_283 = 6;
        let rug_fuzz_284 = 1;
        let rug_fuzz_285 = 6;
        let rug_fuzz_286 = 6;
        let rug_fuzz_287 = 1;
        let rug_fuzz_288 = 1;
        let rug_fuzz_289 = 1;
        let rug_fuzz_290 = 6;
        let rug_fuzz_291 = 6;
        let rug_fuzz_292 = 6;
        let rug_fuzz_293 = 1;
        let rug_fuzz_294 = 1;
        let rug_fuzz_295 = 6;
        let rug_fuzz_296 = 0;
        let rug_fuzz_297 = 1;
        let rug_fuzz_298 = 7;
        let rug_fuzz_299 = 6;
        let rug_fuzz_300 = 0;
        let rug_fuzz_301 = 6;
        let rug_fuzz_302 = 7;
        let rug_fuzz_303 = 1;
        let rug_fuzz_304 = 1;
        let rug_fuzz_305 = 0;
        let rug_fuzz_306 = 6;
        let rug_fuzz_307 = 7;
        let rug_fuzz_308 = 1;
        let rug_fuzz_309 = 7;
        let rug_fuzz_310 = 6;
        let rug_fuzz_311 = 0;
        let rug_fuzz_312 = 1;
        let rug_fuzz_313 = 0;
        let rug_fuzz_314 = 6;
        let rug_fuzz_315 = 7;
        let rug_fuzz_316 = 6;
        let rug_fuzz_317 = 0;
        let rug_fuzz_318 = 1;
        let rug_fuzz_319 = 7;
        let mut game = Reversi::new().unwrap();
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
        game.place(Player::Player1, rug_fuzz_30, rug_fuzz_31).unwrap();
        game.place(Player::Player0, rug_fuzz_32, rug_fuzz_33).unwrap();
        game.place(Player::Player1, rug_fuzz_34, rug_fuzz_35).unwrap();
        game.place(Player::Player0, rug_fuzz_36, rug_fuzz_37).unwrap();
        game.place(Player::Player1, rug_fuzz_38, rug_fuzz_39).unwrap();
        game.place(Player::Player0, rug_fuzz_40, rug_fuzz_41).unwrap();
        game.place(Player::Player1, rug_fuzz_42, rug_fuzz_43).unwrap();
        game.place(Player::Player0, rug_fuzz_44, rug_fuzz_45).unwrap();
        game.place(Player::Player1, rug_fuzz_46, rug_fuzz_47).unwrap();
        game.place(Player::Player0, rug_fuzz_48, rug_fuzz_49).unwrap();
        game.place(Player::Player1, rug_fuzz_50, rug_fuzz_51).unwrap();
        game.place(Player::Player0, rug_fuzz_52, rug_fuzz_53).unwrap();
        game.place(Player::Player1, rug_fuzz_54, rug_fuzz_55).unwrap();
        game.place(Player::Player0, rug_fuzz_56, rug_fuzz_57).unwrap();
        game.place(Player::Player1, rug_fuzz_58, rug_fuzz_59).unwrap();
        game.place(Player::Player0, rug_fuzz_60, rug_fuzz_61).unwrap();
        game.place(Player::Player1, rug_fuzz_62, rug_fuzz_63).unwrap();
        game.place(Player::Player0, rug_fuzz_64, rug_fuzz_65).unwrap();
        game.place(Player::Player1, rug_fuzz_66, rug_fuzz_67).unwrap();
        game.place(Player::Player0, rug_fuzz_68, rug_fuzz_69).unwrap();
        game.place(Player::Player1, rug_fuzz_70, rug_fuzz_71).unwrap();
        game.place(Player::Player0, rug_fuzz_72, rug_fuzz_73).unwrap();
        game.place(Player::Player1, rug_fuzz_74, rug_fuzz_75).unwrap();
        game.place(Player::Player0, rug_fuzz_76, rug_fuzz_77).unwrap();
        game.place(Player::Player1, rug_fuzz_78, rug_fuzz_79).unwrap();
        game.place(Player::Player0, rug_fuzz_80, rug_fuzz_81).unwrap();
        game.place(Player::Player1, rug_fuzz_82, rug_fuzz_83).unwrap();
        game.place(Player::Player0, rug_fuzz_84, rug_fuzz_85).unwrap();
        game.place(Player::Player1, rug_fuzz_86, rug_fuzz_87).unwrap();
        game.place(Player::Player0, rug_fuzz_88, rug_fuzz_89).unwrap();
        game.place(Player::Player1, rug_fuzz_90, rug_fuzz_91).unwrap();
        game.place(Player::Player0, rug_fuzz_92, rug_fuzz_93).unwrap();
        game.place(Player::Player1, rug_fuzz_94, rug_fuzz_95).unwrap();
        game.place(Player::Player0, rug_fuzz_96, rug_fuzz_97).unwrap();
        game.place(Player::Player1, rug_fuzz_98, rug_fuzz_99).unwrap();
        game.place(Player::Player0, rug_fuzz_100, rug_fuzz_101).unwrap();
        game.place(Player::Player1, rug_fuzz_102, rug_fuzz_103).unwrap();
        game.place(Player::Player0, rug_fuzz_104, rug_fuzz_105).unwrap();
        game.place(Player::Player1, rug_fuzz_106, rug_fuzz_107).unwrap();
        game.place(Player::Player0, rug_fuzz_108, rug_fuzz_109).unwrap();
        game.place(Player::Player1, rug_fuzz_110, rug_fuzz_111).unwrap();
        game.place(Player::Player0, rug_fuzz_112, rug_fuzz_113).unwrap();
        game.place(Player::Player1, rug_fuzz_114, rug_fuzz_115).unwrap();
        game.place(Player::Player0, rug_fuzz_116, rug_fuzz_117).unwrap();
        game.place(Player::Player1, rug_fuzz_118, rug_fuzz_119).unwrap();
        game.place(Player::Player0, rug_fuzz_120, rug_fuzz_121).unwrap();
        game.place(Player::Player1, rug_fuzz_122, rug_fuzz_123).unwrap();
        game.place(Player::Player0, rug_fuzz_124, rug_fuzz_125).unwrap();
        game.place(Player::Player1, rug_fuzz_126, rug_fuzz_127).unwrap();
        game.place(Player::Player0, rug_fuzz_128, rug_fuzz_129).unwrap();
        game.place(Player::Player1, rug_fuzz_130, rug_fuzz_131).unwrap();
        game.place(Player::Player0, rug_fuzz_132, rug_fuzz_133).unwrap();
        game.place(Player::Player1, rug_fuzz_134, rug_fuzz_135).unwrap();
        game.place(Player::Player0, rug_fuzz_136, rug_fuzz_137).unwrap();
        game.place(Player::Player1, rug_fuzz_138, rug_fuzz_139).unwrap();
        game.place(Player::Player0, rug_fuzz_140, rug_fuzz_141).unwrap();
        game.place(Player::Player1, rug_fuzz_142, rug_fuzz_143).unwrap();
        game.place(Player::Player0, rug_fuzz_144, rug_fuzz_145).unwrap();
        game.place(Player::Player1, rug_fuzz_146, rug_fuzz_147).unwrap();
        game.place(Player::Player0, rug_fuzz_148, rug_fuzz_149).unwrap();
        game.place(Player::Player1, rug_fuzz_150, rug_fuzz_151).unwrap();
        game.place(Player::Player0, rug_fuzz_152, rug_fuzz_153).unwrap();
        game.place(Player::Player1, rug_fuzz_154, rug_fuzz_155).unwrap();
        game.place(Player::Player0, rug_fuzz_156, rug_fuzz_157).unwrap();
        game.place(Player::Player1, rug_fuzz_158, rug_fuzz_159).unwrap();
        game.place(Player::Player0, rug_fuzz_160, rug_fuzz_161).unwrap();
        game.place(Player::Player1, rug_fuzz_162, rug_fuzz_163).unwrap();
        game.place(Player::Player0, rug_fuzz_164, rug_fuzz_165).unwrap();
        game.place(Player::Player1, rug_fuzz_166, rug_fuzz_167).unwrap();
        game.place(Player::Player0, rug_fuzz_168, rug_fuzz_169).unwrap();
        game.place(Player::Player1, rug_fuzz_170, rug_fuzz_171).unwrap();
        game.place(Player::Player0, rug_fuzz_172, rug_fuzz_173).unwrap();
        game.place(Player::Player1, rug_fuzz_174, rug_fuzz_175).unwrap();
        game.place(Player::Player0, rug_fuzz_176, rug_fuzz_177).unwrap();
        game.place(Player::Player1, rug_fuzz_178, rug_fuzz_179).unwrap();
        game.place(Player::Player0, rug_fuzz_180, rug_fuzz_181).unwrap();
        game.place(Player::Player1, rug_fuzz_182, rug_fuzz_183).unwrap();
        game.place(Player::Player0, rug_fuzz_184, rug_fuzz_185).unwrap();
        game.place(Player::Player1, rug_fuzz_186, rug_fuzz_187).unwrap();
        game.place(Player::Player0, rug_fuzz_188, rug_fuzz_189).unwrap();
        game.place(Player::Player1, rug_fuzz_190, rug_fuzz_191).unwrap();
        game.place(Player::Player0, rug_fuzz_192, rug_fuzz_193).unwrap();
        game.place(Player::Player1, rug_fuzz_194, rug_fuzz_195).unwrap();
        game.place(Player::Player0, rug_fuzz_196, rug_fuzz_197).unwrap();
        game.place(Player::Player1, rug_fuzz_198, rug_fuzz_199).unwrap();
        game.place(Player::Player0, rug_fuzz_200, rug_fuzz_201).unwrap();
        game.place(Player::Player1, rug_fuzz_202, rug_fuzz_203).unwrap();
        game.place(Player::Player0, rug_fuzz_204, rug_fuzz_205).unwrap();
        game.place(Player::Player1, rug_fuzz_206, rug_fuzz_207).unwrap();
        game.place(Player::Player0, rug_fuzz_208, rug_fuzz_209).unwrap();
        game.place(Player::Player1, rug_fuzz_210, rug_fuzz_211).unwrap();
        game.place(Player::Player0, rug_fuzz_212, rug_fuzz_213).unwrap();
        game.place(Player::Player1, rug_fuzz_214, rug_fuzz_215).unwrap();
        game.place(Player::Player0, rug_fuzz_216, rug_fuzz_217).unwrap();
        game.place(Player::Player1, rug_fuzz_218, rug_fuzz_219).unwrap();
        game.place(Player::Player0, rug_fuzz_220, rug_fuzz_221).unwrap();
        game.place(Player::Player1, rug_fuzz_222, rug_fuzz_223).unwrap();
        game.place(Player::Player0, rug_fuzz_224, rug_fuzz_225).unwrap();
        game.place(Player::Player1, rug_fuzz_226, rug_fuzz_227).unwrap();
        game.place(Player::Player0, rug_fuzz_228, rug_fuzz_229).unwrap();
        game.place(Player::Player1, rug_fuzz_230, rug_fuzz_231).unwrap();
        game.place(Player::Player0, rug_fuzz_232, rug_fuzz_233).unwrap();
        game.place(Player::Player1, rug_fuzz_234, rug_fuzz_235).unwrap();
        game.place(Player::Player0, rug_fuzz_236, rug_fuzz_237).unwrap();
        game.place(Player::Player1, rug_fuzz_238, rug_fuzz_239).unwrap();
        game.place(Player::Player0, rug_fuzz_240, rug_fuzz_241).unwrap();
        game.place(Player::Player1, rug_fuzz_242, rug_fuzz_243).unwrap();
        game.place(Player::Player0, rug_fuzz_244, rug_fuzz_245).unwrap();
        game.place(Player::Player1, rug_fuzz_246, rug_fuzz_247).unwrap();
        game.place(Player::Player0, rug_fuzz_248, rug_fuzz_249).unwrap();
        game.place(Player::Player1, rug_fuzz_250, rug_fuzz_251).unwrap();
        game.place(Player::Player0, rug_fuzz_252, rug_fuzz_253).unwrap();
        game.place(Player::Player1, rug_fuzz_254, rug_fuzz_255).unwrap();
        game.place(Player::Player0, rug_fuzz_256, rug_fuzz_257).unwrap();
        game.place(Player::Player1, rug_fuzz_258, rug_fuzz_259).unwrap();
        game.place(Player::Player0, rug_fuzz_260, rug_fuzz_261).unwrap();
        game.place(Player::Player1, rug_fuzz_262, rug_fuzz_263).unwrap();
        game.place(Player::Player0, rug_fuzz_264, rug_fuzz_265).unwrap();
        game.place(Player::Player1, rug_fuzz_266, rug_fuzz_267).unwrap();
        game.place(Player::Player0, rug_fuzz_268, rug_fuzz_269).unwrap();
        game.place(Player::Player1, rug_fuzz_270, rug_fuzz_271).unwrap();
        game.place(Player::Player0, rug_fuzz_272, rug_fuzz_273).unwrap();
        game.place(Player::Player1, rug_fuzz_274, rug_fuzz_275).unwrap();
        game.place(Player::Player0, rug_fuzz_276, rug_fuzz_277).unwrap();
        game.place(Player::Player1, rug_fuzz_278, rug_fuzz_279).unwrap();
        game.place(Player::Player0, rug_fuzz_280, rug_fuzz_281).unwrap();
        game.place(Player::Player1, rug_fuzz_282, rug_fuzz_283).unwrap();
        game.place(Player::Player0, rug_fuzz_284, rug_fuzz_285).unwrap();
        game.place(Player::Player1, rug_fuzz_286, rug_fuzz_287).unwrap();
        game.place(Player::Player0, rug_fuzz_288, rug_fuzz_289).unwrap();
        game.place(Player::Player1, rug_fuzz_290, rug_fuzz_291).unwrap();
        game.place(Player::Player0, rug_fuzz_292, rug_fuzz_293).unwrap();
        game.place(Player::Player1, rug_fuzz_294, rug_fuzz_295).unwrap();
        game.place(Player::Player0, rug_fuzz_296, rug_fuzz_297).unwrap();
        game.place(Player::Player1, rug_fuzz_298, rug_fuzz_299).unwrap();
        game.place(Player::Player0, rug_fuzz_300, rug_fuzz_301).unwrap();
        game.place(Player::Player1, rug_fuzz_302, rug_fuzz_303).unwrap();
        game.place(Player::Player0, rug_fuzz_304, rug_fuzz_305).unwrap();
        game.place(Player::Player1, rug_fuzz_306, rug_fuzz_307).unwrap();
        game.place(Player::Player0, rug_fuzz_308, rug_fuzz_309).unwrap();
        game.place(Player::Player1, rug_fuzz_310, rug_fuzz_311).unwrap();
        game.place(Player::Player0, rug_fuzz_312, rug_fuzz_313).unwrap();
        game.place(Player::Player1, rug_fuzz_314, rug_fuzz_315).unwrap();
        game.place(Player::Player0, rug_fuzz_316, rug_fuzz_317).unwrap();
        game.place(Player::Player1, rug_fuzz_318, rug_fuzz_319).unwrap();
        debug_assert_eq!(game.winner(), None);
        let _rug_ed_tests_llm_16_69_rrrruuuugggg_test_winner_returns_none_when_game_tied = 0;
    }
    #[test]
    fn test_winner_returns_winner_player_when_game_won() {
        let _rug_st_tests_llm_16_69_rrrruuuugggg_test_winner_returns_winner_player_when_game_won = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 7;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 6;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 5;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 3;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 4;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 0;
        let rug_fuzz_18 = 1;
        let rug_fuzz_19 = 7;
        let rug_fuzz_20 = 1;
        let rug_fuzz_21 = 1;
        let rug_fuzz_22 = 1;
        let rug_fuzz_23 = 6;
        let rug_fuzz_24 = 1;
        let rug_fuzz_25 = 2;
        let rug_fuzz_26 = 1;
        let rug_fuzz_27 = 5;
        let rug_fuzz_28 = 1;
        let rug_fuzz_29 = 3;
        let rug_fuzz_30 = 1;
        let rug_fuzz_31 = 4;
        let rug_fuzz_32 = 2;
        let rug_fuzz_33 = 0;
        let rug_fuzz_34 = 2;
        let rug_fuzz_35 = 7;
        let rug_fuzz_36 = 2;
        let rug_fuzz_37 = 1;
        let rug_fuzz_38 = 2;
        let rug_fuzz_39 = 6;
        let rug_fuzz_40 = 2;
        let rug_fuzz_41 = 2;
        let rug_fuzz_42 = 2;
        let rug_fuzz_43 = 5;
        let rug_fuzz_44 = 2;
        let rug_fuzz_45 = 3;
        let rug_fuzz_46 = 2;
        let rug_fuzz_47 = 4;
        let rug_fuzz_48 = 3;
        let rug_fuzz_49 = 0;
        let rug_fuzz_50 = 3;
        let rug_fuzz_51 = 7;
        let rug_fuzz_52 = 3;
        let rug_fuzz_53 = 1;
        let rug_fuzz_54 = 3;
        let rug_fuzz_55 = 6;
        let rug_fuzz_56 = 3;
        let rug_fuzz_57 = 2;
        let rug_fuzz_58 = 3;
        let rug_fuzz_59 = 5;
        let rug_fuzz_60 = 3;
        let rug_fuzz_61 = 3;
        let rug_fuzz_62 = 3;
        let rug_fuzz_63 = 4;
        let rug_fuzz_64 = 4;
        let rug_fuzz_65 = 0;
        let rug_fuzz_66 = 4;
        let rug_fuzz_67 = 7;
        let rug_fuzz_68 = 4;
        let rug_fuzz_69 = 1;
        let rug_fuzz_70 = 4;
        let rug_fuzz_71 = 6;
        let rug_fuzz_72 = 4;
        let rug_fuzz_73 = 2;
        let rug_fuzz_74 = 4;
        let rug_fuzz_75 = 5;
        let rug_fuzz_76 = 4;
        let rug_fuzz_77 = 3;
        let rug_fuzz_78 = 4;
        let rug_fuzz_79 = 4;
        let rug_fuzz_80 = 5;
        let rug_fuzz_81 = 0;
        let rug_fuzz_82 = 5;
        let rug_fuzz_83 = 7;
        let rug_fuzz_84 = 5;
        let rug_fuzz_85 = 1;
        let rug_fuzz_86 = 5;
        let rug_fuzz_87 = 6;
        let rug_fuzz_88 = 5;
        let rug_fuzz_89 = 2;
        let rug_fuzz_90 = 5;
        let rug_fuzz_91 = 5;
        let rug_fuzz_92 = 5;
        let rug_fuzz_93 = 3;
        let rug_fuzz_94 = 5;
        let rug_fuzz_95 = 4;
        let rug_fuzz_96 = 6;
        let rug_fuzz_97 = 0;
        let rug_fuzz_98 = 6;
        let rug_fuzz_99 = 7;
        let rug_fuzz_100 = 6;
        let rug_fuzz_101 = 1;
        let rug_fuzz_102 = 6;
        let rug_fuzz_103 = 6;
        let rug_fuzz_104 = 6;
        let rug_fuzz_105 = 2;
        let rug_fuzz_106 = 6;
        let rug_fuzz_107 = 5;
        let rug_fuzz_108 = 6;
        let rug_fuzz_109 = 3;
        let rug_fuzz_110 = 6;
        let rug_fuzz_111 = 4;
        let rug_fuzz_112 = 7;
        let rug_fuzz_113 = 0;
        let rug_fuzz_114 = 7;
        let rug_fuzz_115 = 7;
        let rug_fuzz_116 = 7;
        let rug_fuzz_117 = 1;
        let rug_fuzz_118 = 7;
        let rug_fuzz_119 = 6;
        let rug_fuzz_120 = 7;
        let rug_fuzz_121 = 2;
        let rug_fuzz_122 = 7;
        let rug_fuzz_123 = 5;
        let rug_fuzz_124 = 7;
        let rug_fuzz_125 = 3;
        let rug_fuzz_126 = 7;
        let rug_fuzz_127 = 4;
        let rug_fuzz_128 = 7;
        let rug_fuzz_129 = 4;
        let mut game = Reversi::new().unwrap();
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
        game.place(Player::Player1, rug_fuzz_30, rug_fuzz_31).unwrap();
        game.place(Player::Player0, rug_fuzz_32, rug_fuzz_33).unwrap();
        game.place(Player::Player1, rug_fuzz_34, rug_fuzz_35).unwrap();
        game.place(Player::Player0, rug_fuzz_36, rug_fuzz_37).unwrap();
        game.place(Player::Player1, rug_fuzz_38, rug_fuzz_39).unwrap();
        game.place(Player::Player0, rug_fuzz_40, rug_fuzz_41).unwrap();
        game.place(Player::Player1, rug_fuzz_42, rug_fuzz_43).unwrap();
        game.place(Player::Player0, rug_fuzz_44, rug_fuzz_45).unwrap();
        game.place(Player::Player1, rug_fuzz_46, rug_fuzz_47).unwrap();
        game.place(Player::Player0, rug_fuzz_48, rug_fuzz_49).unwrap();
        game.place(Player::Player1, rug_fuzz_50, rug_fuzz_51).unwrap();
        game.place(Player::Player0, rug_fuzz_52, rug_fuzz_53).unwrap();
        game.place(Player::Player1, rug_fuzz_54, rug_fuzz_55).unwrap();
        game.place(Player::Player0, rug_fuzz_56, rug_fuzz_57).unwrap();
        game.place(Player::Player1, rug_fuzz_58, rug_fuzz_59).unwrap();
        game.place(Player::Player0, rug_fuzz_60, rug_fuzz_61).unwrap();
        game.place(Player::Player1, rug_fuzz_62, rug_fuzz_63).unwrap();
        game.place(Player::Player0, rug_fuzz_64, rug_fuzz_65).unwrap();
        game.place(Player::Player1, rug_fuzz_66, rug_fuzz_67).unwrap();
        game.place(Player::Player0, rug_fuzz_68, rug_fuzz_69).unwrap();
        game.place(Player::Player1, rug_fuzz_70, rug_fuzz_71).unwrap();
        game.place(Player::Player0, rug_fuzz_72, rug_fuzz_73).unwrap();
        game.place(Player::Player1, rug_fuzz_74, rug_fuzz_75).unwrap();
        game.place(Player::Player0, rug_fuzz_76, rug_fuzz_77).unwrap();
        game.place(Player::Player1, rug_fuzz_78, rug_fuzz_79).unwrap();
        game.place(Player::Player0, rug_fuzz_80, rug_fuzz_81).unwrap();
        game.place(Player::Player1, rug_fuzz_82, rug_fuzz_83).unwrap();
        game.place(Player::Player0, rug_fuzz_84, rug_fuzz_85).unwrap();
        game.place(Player::Player1, rug_fuzz_86, rug_fuzz_87).unwrap();
        game.place(Player::Player0, rug_fuzz_88, rug_fuzz_89).unwrap();
        game.place(Player::Player1, rug_fuzz_90, rug_fuzz_91).unwrap();
        game.place(Player::Player0, rug_fuzz_92, rug_fuzz_93).unwrap();
        game.place(Player::Player1, rug_fuzz_94, rug_fuzz_95).unwrap();
        game.place(Player::Player0, rug_fuzz_96, rug_fuzz_97).unwrap();
        game.place(Player::Player1, rug_fuzz_98, rug_fuzz_99).unwrap();
        game.place(Player::Player0, rug_fuzz_100, rug_fuzz_101).unwrap();
        game.place(Player::Player1, rug_fuzz_102, rug_fuzz_103).unwrap();
        game.place(Player::Player0, rug_fuzz_104, rug_fuzz_105).unwrap();
        game.place(Player::Player1, rug_fuzz_106, rug_fuzz_107).unwrap();
        game.place(Player::Player0, rug_fuzz_108, rug_fuzz_109).unwrap();
        game.place(Player::Player1, rug_fuzz_110, rug_fuzz_111).unwrap();
        game.place(Player::Player0, rug_fuzz_112, rug_fuzz_113).unwrap();
        game.place(Player::Player1, rug_fuzz_114, rug_fuzz_115).unwrap();
        game.place(Player::Player0, rug_fuzz_116, rug_fuzz_117).unwrap();
        game.place(Player::Player1, rug_fuzz_118, rug_fuzz_119).unwrap();
        game.place(Player::Player0, rug_fuzz_120, rug_fuzz_121).unwrap();
        game.place(Player::Player1, rug_fuzz_122, rug_fuzz_123).unwrap();
        game.place(Player::Player0, rug_fuzz_124, rug_fuzz_125).unwrap();
        game.place(Player::Player1, rug_fuzz_126, rug_fuzz_127).unwrap();
        game.place(Player::Player0, rug_fuzz_128, rug_fuzz_129).unwrap();
        debug_assert_eq!(game.winner(), Some(Player::Player0));
        let _rug_ed_tests_llm_16_69_rrrruuuugggg_test_winner_returns_winner_player_when_game_won = 0;
    }
}
#[cfg(test)]
mod tests_rug_4 {
    use super::*;
    use crate::reversi::{Reversi, Player};
    #[test]
    fn test_can_player_move() {
        let _rug_st_tests_rug_4_rrrruuuugggg_test_can_player_move = 0;
        let mut p0: Reversi = Reversi::new().unwrap();
        let mut p1: Player = Player::Player0;
        p0.can_player_move(p1);
        let _rug_ed_tests_rug_4_rrrruuuugggg_test_can_player_move = 0;
    }
}
#[cfg(test)]
mod tests_rug_5 {
    use super::*;
    use crate::reversi::Direction;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_5_rrrruuuugggg_test_rug = 0;
        Direction::iter();
        let _rug_ed_tests_rug_5_rrrruuuugggg_test_rug = 0;
    }
}
