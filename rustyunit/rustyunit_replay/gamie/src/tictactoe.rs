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
    ) -> Result<(), TicTacToeError> {
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
            if self.board[row][0].is_some() && self.board[row][0] == self.board[row][1]
                && self.board[row][1] == self.board[row][2]
            {
                self.status = GameState::Win(self.board[row][0].unwrap());
                return;
            }
        }
        for col in 0..3 {
            if self.board[0][col].is_some() && self.board[0][col] == self.board[1][col]
                && self.board[1][col] == self.board[2][col]
            {
                self.status = GameState::Win(self.board[0][col].unwrap());
                return;
            }
        }
        if self.board[0][0].is_some() && self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
        {
            self.status = GameState::Win(self.board[0][0].unwrap());
            return;
        }
        if self.board[0][0].is_some() && self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
        {
            self.status = GameState::Win(self.board[0][2].unwrap());
            return;
        }
        self
            .status = if self.board.iter().flatten().all(|p| p.is_some()) {
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
        assert_eq!(game.place(Player::Player0, 0, 0), Err(TicTacToeError::WrongPlayer));
        assert_eq!(game.place(Player::Player1, 1, 0), Ok(()));
        assert_eq!(game.get_next_player(), Player::Player0,);
        assert!(! game.is_ended());
        assert_eq!(
            game.place(Player::Player0, 1, 1), Err(TicTacToeError::OccupiedPosition)
        );
        assert_eq!(game.place(Player::Player0, 2, 2), Ok(()));
        assert_eq!(game.status(), & GameState::InProgress);
        assert_eq!(game.place(Player::Player1, 2, 0), Ok(()));
        assert_eq!(game.place(Player::Player0, 0, 0), Ok(()));
        assert!(game.is_ended());
        assert_eq!(game.winner(), Some(Player::Player0));
        assert_eq!(game.place(Player::Player0, 0, 2), Err(TicTacToeError::GameEnded));
        assert_eq!(game.winner(), Some(Player::Player0));
    }
}
#[cfg(test)]
mod tests_llm_16_72 {
    use super::*;
    use crate::*;
    #[test]
    fn test_check_state_win_row() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player0, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player0, rug_fuzz_4, rug_fuzz_5).unwrap();
        game.check_state();
        debug_assert_eq!(game.status(), & GameState::Win(Player::Player0));
             }
}
}
}    }
    #[test]
    fn test_check_state_win_column() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        game.place(Player::Player1, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player1, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player1, rug_fuzz_4, rug_fuzz_5).unwrap();
        game.check_state();
        debug_assert_eq!(game.status(), & GameState::Win(Player::Player1));
             }
}
}
}    }
    #[test]
    fn test_check_state_win_diagonal1() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player0, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player0, rug_fuzz_4, rug_fuzz_5).unwrap();
        game.check_state();
        debug_assert_eq!(game.status(), & GameState::Win(Player::Player0));
             }
}
}
}    }
    #[test]
    fn test_check_state_win_diagonal2() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        game.place(Player::Player1, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player1, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player1, rug_fuzz_4, rug_fuzz_5).unwrap();
        game.check_state();
        debug_assert_eq!(game.status(), & GameState::Win(Player::Player1));
             }
}
}
}    }
    #[test]
    fn test_check_state_tie() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player1, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player0, rug_fuzz_4, rug_fuzz_5).unwrap();
        game.place(Player::Player1, rug_fuzz_6, rug_fuzz_7).unwrap();
        game.place(Player::Player0, rug_fuzz_8, rug_fuzz_9).unwrap();
        game.place(Player::Player1, rug_fuzz_10, rug_fuzz_11).unwrap();
        game.place(Player::Player1, rug_fuzz_12, rug_fuzz_13).unwrap();
        game.place(Player::Player0, rug_fuzz_14, rug_fuzz_15).unwrap();
        game.place(Player::Player0, rug_fuzz_16, rug_fuzz_17).unwrap();
        game.check_state();
        debug_assert_eq!(game.status(), & GameState::Tie);
             }
}
}
}    }
    #[test]
    fn test_check_state_in_progress() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.check_state();
        debug_assert_eq!(game.status(), & GameState::InProgress);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_73 {
    use super::*;
    use crate::*;
    use std::panic::catch_unwind;
    #[test]
    fn test_get_valid_position() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let game = TicTacToe::new().unwrap();
        let result = game.get(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(result, & None);
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn test_get_invalid_position() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let game = TicTacToe::new().unwrap();
        let _result = game.get(rug_fuzz_0, rug_fuzz_1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_74 {
    use super::*;
    use crate::*;
    #[test]
    fn test_get_next_player() {
        let _rug_st_tests_llm_16_74_rrrruuuugggg_test_get_next_player = 0;
        let game = TicTacToe::new().unwrap();
        let next_player = game.get_next_player();
        debug_assert_eq!(next_player, Player::Player0);
        let _rug_ed_tests_llm_16_74_rrrruuuugggg_test_get_next_player = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_75 {
    use super::*;
    use crate::*;
    use tictactoe::{GameState, Player, TicTacToe};
    #[test]
    fn test_is_ended_in_progress() {
        let _rug_st_tests_llm_16_75_rrrruuuugggg_test_is_ended_in_progress = 0;
        let game = TicTacToe::new().unwrap();
        debug_assert_eq!(game.is_ended(), false);
        let _rug_ed_tests_llm_16_75_rrrruuuugggg_test_is_ended_in_progress = 0;
    }
    #[test]
    fn test_is_ended_win() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player1, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player0, rug_fuzz_4, rug_fuzz_5).unwrap();
        game.place(Player::Player1, rug_fuzz_6, rug_fuzz_7).unwrap();
        game.place(Player::Player0, rug_fuzz_8, rug_fuzz_9).unwrap();
        debug_assert_eq!(game.is_ended(), true);
             }
}
}
}    }
    #[test]
    fn test_is_ended_tie() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player1, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player0, rug_fuzz_4, rug_fuzz_5).unwrap();
        game.place(Player::Player1, rug_fuzz_6, rug_fuzz_7).unwrap();
        game.place(Player::Player0, rug_fuzz_8, rug_fuzz_9).unwrap();
        game.place(Player::Player1, rug_fuzz_10, rug_fuzz_11).unwrap();
        game.place(Player::Player0, rug_fuzz_12, rug_fuzz_13).unwrap();
        game.place(Player::Player1, rug_fuzz_14, rug_fuzz_15).unwrap();
        game.place(Player::Player0, rug_fuzz_16, rug_fuzz_17).unwrap();
        debug_assert_eq!(game.is_ended(), true);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_76 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new() {
        let _rug_st_tests_llm_16_76_rrrruuuugggg_test_new = 0;
        let game = TicTacToe::new().unwrap();
        for row in game.board.iter() {
            for cell in row.iter() {
                debug_assert_eq!(* cell, None);
            }
        }
        debug_assert_eq!(game.next, Player::Player0);
        debug_assert_eq!(game.status, GameState::InProgress);
        let _rug_ed_tests_llm_16_76_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_77 {
    use crate::tictactoe::*;
    #[test]
    fn test_place_valid_move() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        let player = Player::Player0;
        let row = rug_fuzz_0;
        let col = rug_fuzz_1;
        let result = game.place(player, row, col);
        debug_assert_eq!(result, Ok(()));
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn test_place_invalid_move() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        let player = Player::Player0;
        let row = rug_fuzz_0;
        let col = rug_fuzz_1;
        let _ = game.place(player, row, col);
        let result = game.place(player, row, col);
        debug_assert_eq!(result, Ok(()));
             }
}
}
}    }
    #[test]
    fn test_place_wrong_player() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        let player0 = Player::Player0;
        let player1 = Player::Player1;
        let row = rug_fuzz_0;
        let col = rug_fuzz_1;
        let _ = game.place(player0, row, col);
        let result = game.place(player0, row, col);
        debug_assert_eq!(result, Err(TicTacToeError::WrongPlayer));
        let result = game.place(player1, row, col);
        debug_assert_eq!(result, Ok(()));
             }
}
}
}    }
    #[test]
    fn test_place_occupied_position() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        let player0 = Player::Player0;
        let player1 = Player::Player1;
        let row = rug_fuzz_0;
        let col = rug_fuzz_1;
        let _ = game.place(player0, row, col);
        let result = game.place(player1, row, col);
        debug_assert_eq!(result, Err(TicTacToeError::OccupiedPosition));
             }
}
}
}    }
    #[test]
    fn test_place_game_ended() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        let player0 = Player::Player0;
        let player1 = Player::Player1;
        let row1 = rug_fuzz_0;
        let col1 = rug_fuzz_1;
        let row2 = rug_fuzz_2;
        let col2 = rug_fuzz_3;
        let row3 = rug_fuzz_4;
        let col3 = rug_fuzz_5;
        let row4 = rug_fuzz_6;
        let col4 = rug_fuzz_7;
        let row5 = rug_fuzz_8;
        let col5 = rug_fuzz_9;
        let row6 = rug_fuzz_10;
        let col6 = rug_fuzz_11;
        let row7 = rug_fuzz_12;
        let col7 = rug_fuzz_13;
        let row8 = rug_fuzz_14;
        let col8 = rug_fuzz_15;
        let row9 = rug_fuzz_16;
        let col9 = rug_fuzz_17;
        let _ = game.place(player0, row1, col1);
        let _ = game.place(player1, row2, col2);
        let _ = game.place(player0, row3, col3);
        let _ = game.place(player1, row4, col4);
        let _ = game.place(player0, row5, col5);
        let _ = game.place(player1, row6, col6);
        let _ = game.place(player0, row7, col7);
        let _ = game.place(player1, row8, col8);
        let result = game.place(player0, row9, col9);
        debug_assert_eq!(result, Err(TicTacToeError::GameEnded));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_78 {
    use super::*;
    use crate::*;
    #[test]
    fn test_status() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        debug_assert_eq!(* game.status(), GameState::InProgress);
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player1, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player0, rug_fuzz_4, rug_fuzz_5).unwrap();
        debug_assert_eq!(* game.status(), GameState::InProgress);
        game.place(Player::Player1, rug_fuzz_6, rug_fuzz_7).unwrap();
        game.place(Player::Player0, rug_fuzz_8, rug_fuzz_9).unwrap();
        debug_assert_eq!(* game.status(), GameState::Win(Player::Player0));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_79 {
    use super::*;
    use crate::*;
    #[test]
    fn test_winner_none() {
        let _rug_st_tests_llm_16_79_rrrruuuugggg_test_winner_none = 0;
        let game = TicTacToe::new().unwrap();
        let result = game.winner();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_79_rrrruuuugggg_test_winner_none = 0;
    }
    #[test]
    fn test_winner_player() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player0, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player0, rug_fuzz_4, rug_fuzz_5).unwrap();
        let result = game.winner();
        debug_assert_eq!(result, Some(Player::Player0));
             }
}
}
}    }
    #[test]
    fn test_winner_tie() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut game = TicTacToe::new().unwrap();
        game.place(Player::Player0, rug_fuzz_0, rug_fuzz_1).unwrap();
        game.place(Player::Player1, rug_fuzz_2, rug_fuzz_3).unwrap();
        game.place(Player::Player0, rug_fuzz_4, rug_fuzz_5).unwrap();
        game.place(Player::Player1, rug_fuzz_6, rug_fuzz_7).unwrap();
        game.place(Player::Player0, rug_fuzz_8, rug_fuzz_9).unwrap();
        game.place(Player::Player1, rug_fuzz_10, rug_fuzz_11).unwrap();
        game.place(Player::Player1, rug_fuzz_12, rug_fuzz_13).unwrap();
        game.place(Player::Player0, rug_fuzz_14, rug_fuzz_15).unwrap();
        game.place(Player::Player1, rug_fuzz_16, rug_fuzz_17).unwrap();
        let result = game.winner();
        debug_assert_eq!(result, None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_6 {
    use super::*;
    use crate::tictactoe::Player;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_6_rrrruuuugggg_test_rug = 0;
        let mut p0: Player = Player::Player0;
        <Player>::other(p0);
        let _rug_ed_tests_rug_6_rrrruuuugggg_test_rug = 0;
    }
}
