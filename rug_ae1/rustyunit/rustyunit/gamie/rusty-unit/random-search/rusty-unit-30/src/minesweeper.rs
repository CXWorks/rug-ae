//! Minesweeper
//!
//! Check struct [`Minesweeper`](https://docs.rs/gamie/*/gamie/minesweeper/struct.Minesweeper.html) for more information
//!
//! # Examples
//!
//! ```rust
//! # fn minesweeper() {
//! use gamie::minesweeper::Minesweeper;
//! use rand::rngs::ThreadRng;
//!
//! let mut game = Minesweeper::new(8, 8, 9, &mut ThreadRng::default()).unwrap();
//!
//! game.toggle_flag(3, 2).unwrap();
//! // ...
//! game.click(7, 7, true).unwrap();
//! // ...
//! # }
//! ```

use crate::std_lib::{iter, Vec, VecDeque};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use snafu::Snafu;

/// Minesweeper
///
/// To avoid unessecary memory allocation, the game board is stored in a single `Vec` rather than a nested one.
///
/// Passing an invalid position to a method will cause panic. Check the target position validity first when dealing with user input
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Minesweeper {
    board: Vec<Cell>,
    height: usize,
    width: usize,
    status: GameState,
}

/// The cell in the board.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Cell {
    pub is_mine: bool,
    pub mine_adjacent: usize,
    pub is_revealed: bool,
    pub is_flagged: bool,
}

impl Cell {
    fn new(is_mine: bool) -> Self {
        Self {
            is_mine,
            mine_adjacent: 0,
            is_revealed: false,
            is_flagged: false,
        }
    }
}

/// Game status
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GameState {
    Win,
    Exploded(Vec<(usize, usize)>),
    InProgress,
}

impl Minesweeper {
    /// Create a new Minesweeper game
    ///
    /// A mutable reference of a random number generator is required for randomizing mine positions
    ///
    /// Return `Err(MinesweeperError::TooManyMines)` if `height * width < mines`
    ///
    /// # Examples
    /// ```rust
    /// # fn minesweeper() {
    /// use gamie::minesweeper::Minesweeper;
    /// use rand::rngs::ThreadRng;
    ///
    /// let mut game = Minesweeper::new(8, 8, 9, &mut ThreadRng::default()).unwrap();
    /// # }
    /// ```
    pub fn new<R: Rng + ?Sized>(
        height: usize,
        width: usize,
        mines: usize,
        rng: &mut R,
    ) -> Result<Self, MinesweeperError> {
        if height * width < mines {
            return Err(MinesweeperError::TooManyMines);
        }

        let board = iter::repeat(Cell::new(true))
            .take(mines)
            .chain(iter::repeat(Cell::new(false)).take(height * width - mines))
            .collect();

        let mut minesweeper = Self {
            board,
            height,
            width,
            status: GameState::InProgress,
        };
        minesweeper.randomize(rng).unwrap();

        Ok(minesweeper)
    }

    /// Randomize the board
    ///
    /// A mutable reference of a random number generator is required for randomizing mine positions
    pub fn randomize<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Result<(), MinesweeperError> {
        if self.is_ended() {
            return Err(MinesweeperError::GameEnded);
        }

        let range = Uniform::from(0..self.height * self.width);

        for idx in 0..self.height * self.width {
            self.board.swap(idx, range.sample(rng));
        }

        self.update_around_mine_count();

        Ok(())
    }

    /// Get a cell reference from the game board
    /// Panic when target position out of bounds
    pub fn get(&self, row: usize, col: usize) -> &Cell {
        if row >= self.height || col >= self.width {
            panic!("Invalid position: ({}, {})", row, col);
        }

        &self.board[row * self.width + col]
    }

    /// Check if the game was end
    pub fn is_ended(&self) -> bool {
        self.status != GameState::InProgress
    }

    /// Get the game status
    pub fn status(&self) -> &GameState {
        &self.status
    }

    /// Click a cell on the game board
    ///
    /// Clicking an already revealed cell will unreveal its adjacent cells if the flagged cell count around it equals to its adjacent mine count
    /// When `auto_flag` is `true`, clicking an already revealed cell will flag its adjacent unflagged-unrevealed cells if the unflagged-revealed cell count around it equals to its adjacent mine count
    ///
    /// The return value indicates if the game board is changed from the click
    ///
    /// Panic when target position out of bounds
    pub fn click(
        &mut self,
        row: usize,
        col: usize,
        auto_flag: bool,
    ) -> Result<bool, MinesweeperError> {
        if row >= self.height || col >= self.width {
            panic!("Invalid position: ({}, {})", row, col);
        }

        if self.is_ended() {
            return Err(MinesweeperError::GameEnded);
        }

        if !self.board[row * self.width + col].is_revealed {
            self.click_unrevealed(row, col)?;
            Ok(true)
        } else {
            Ok(self.click_revealed(row, col, auto_flag)?)
        }
    }

    /// Flag or unflag a cell on the board
    /// Return Err(MinesweeperError::AlreadyRevealed) if the target cell is already revealed
    ///
    /// Panic when target position out of bounds
    pub fn toggle_flag(&mut self, row: usize, col: usize) -> Result<(), MinesweeperError> {
        if row >= self.height || col >= self.width {
            panic!("Invalid position: ({}, {})", row, col);
        }

        if self.is_ended() {
            return Err(MinesweeperError::GameEnded);
        }

        if self.board[row * self.width + col].is_revealed {
            return Err(MinesweeperError::AlreadyRevealed);
        }

        self.board[row * self.width + col].is_flagged =
            !self.board[row * self.width + col].is_flagged;

        self.check_state();

        Ok(())
    }

    fn click_unrevealed(&mut self, row: usize, col: usize) -> Result<(), MinesweeperError> {
        if self.board[row * self.width + col].is_flagged {
            return Err(MinesweeperError::AlreadyFlagged);
        }

        if self.board[row * self.width + col].is_mine {
            self.status = GameState::Exploded(vec![(row, col)]);
            return Ok(());
        }

        self.reveal_from(row * self.width + col);
        self.check_state();

        Ok(())
    }

    fn click_revealed(
        &mut self,
        row: usize,
        col: usize,
        auto_flag: bool,
    ) -> Result<bool, MinesweeperError> {
        let mut is_changed = false;

        if self.board[row * self.width + col].mine_adjacent > 0 {
            let mut adjacent_all = 0;
            let mut adjacent_revealed = 0;
            let mut adjacent_flagged = 0;

            self.get_adjacent_cells(row, col)
                .map(|idx| self.board[idx])
                .for_each(|cell| {
                    adjacent_all += 1;

                    if cell.is_revealed {
                        adjacent_revealed += 1;
                    } else if cell.is_flagged {
                        adjacent_flagged += 1;
                    }
                });

            let adjacent_unrevealed = adjacent_all - adjacent_revealed - adjacent_flagged;

            if adjacent_unrevealed > 0 {
                if adjacent_flagged == self.board[row * self.width + col].mine_adjacent {
                    let mut exploded = None;

                    self.get_adjacent_cells(row, col).for_each(|idx| {
                        if !self.board[idx].is_flagged && !self.board[idx].is_revealed {
                            if self.board[idx].is_mine {
                                self.board[idx].is_revealed = true;

                                match exploded {
                                    None => exploded = Some(vec![(row, col)]),
                                    Some(ref mut exploded) => {
                                        exploded.push((row, col));
                                    }
                                }
                            } else {
                                self.reveal_from(idx);
                                is_changed = true;
                            }
                        }
                    });

                    if let Some(exploded) = exploded {
                        self.status = GameState::Exploded(exploded);
                        return Ok(true);
                    }
                }

                if auto_flag
                    && adjacent_unrevealed + adjacent_flagged
                        == self.board[row * self.width + col].mine_adjacent
                {
                    self.get_adjacent_cells(row, col).for_each(|idx| {
                        if !self.board[idx].is_flagged && !self.board[idx].is_revealed {
                            self.board[idx].is_flagged = true;
                            is_changed = true;
                        }
                    });
                }
            }

            self.check_state();
        }

        Ok(is_changed)
    }

    fn reveal_from(&mut self, idx: usize) {
        if self.board[idx].mine_adjacent != 0 {
            self.board[idx].is_revealed = true;
        } else {
            let mut cell_idxs_to_reveal = VecDeque::new();
            cell_idxs_to_reveal.push_back(idx);

            while let Some(cell_idx) = cell_idxs_to_reveal.pop_front() {
                self.board[cell_idx].is_revealed = true;

                for neighbor_idx in
                    self.get_adjacent_cells(cell_idx / self.width, cell_idx % self.width)
                {
                    if !self.board[neighbor_idx].is_flagged && !self.board[neighbor_idx].is_revealed
                    {
                        if self.board[neighbor_idx].mine_adjacent == 0 {
                            cell_idxs_to_reveal.push_back(neighbor_idx);
                        } else {
                            self.board[neighbor_idx].is_revealed = true;
                        }
                    }
                }
            }
        }
    }

    fn check_state(&mut self) {
        self.status = if self
            .board
            .iter()
            .filter(|cell| !cell.is_mine)
            .all(|cell| cell.is_revealed)
        {
            GameState::Win
        } else {
            GameState::InProgress
        };
    }

    fn update_around_mine_count(&mut self) {
        for idx in 0..self.height * self.width {
            let count = self
                .get_adjacent_cells(idx / self.width, idx % self.width)
                .filter(|idx| self.board[*idx].is_mine)
                .count();

            self.board[idx].mine_adjacent = count;
        }
    }

    fn get_adjacent_cells(&self, row: usize, col: usize) -> AdjacentCells {
        AdjacentCells::new(row, col, self.height, self.width)
    }
}

#[derive(Clone)]
struct AdjacentCells {
    around: [(isize, isize); 8],
    board_height: isize,
    board_width: isize,
    offset: usize,
}

impl Iterator for AdjacentCells {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.around[self.offset..]
            .iter()
            .enumerate()
            .filter(|(_, (row, col))| {
                *row >= 0 && *col >= 0 && *row < self.board_height && *col < self.board_width
            })
            .next()
            .map(|(idx, (row, col))| {
                self.offset += idx + 1;
                (row * self.board_width + col) as usize
            })
    }
}

impl AdjacentCells {
    fn new(row: usize, col: usize, board_height: usize, board_width: usize) -> Self {
        let (row, col, board_height, board_width) = (
            row as isize,
            col as isize,
            board_height as isize,
            board_width as isize,
        );

        AdjacentCells {
            around: [
                (row - 1, col - 1),
                (row - 1, col),
                (row - 1, col + 1),
                (row, col - 1),
                (row, col + 1),
                (row + 1, col - 1),
                (row + 1, col),
                (row + 1, col + 1),
            ],
            board_height,
            board_width,
            offset: 0,
        }
    }
}

/// Errors that can occur.
#[derive(Debug, Eq, PartialEq, Snafu)]
pub enum MinesweeperError {
    #[snafu(display("Too many mines"))]
    TooManyMines,
    #[snafu(display("Clicked an already flagged cell"))]
    AlreadyFlagged,
    #[snafu(display("Clicked an already revealed cell"))]
    AlreadyRevealed,
    #[snafu(display("The game was already end"))]
    GameEnded,
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use snafu::ErrorCompat;
	use std::iter::Iterator;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2737() {
    rusty_monitor::set_test_id(2737);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut bool_0: bool = true;
    let mut usize_0: usize = 94usize;
    let mut usize_1: usize = 22usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut usize_2: usize = 22usize;
    let mut usize_3: usize = 42usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_4: usize = 29usize;
    let mut usize_5: usize = 65usize;
    let mut usize_6: usize = 28usize;
    let mut usize_7: usize = 72usize;
    let mut usize_8: usize = 9usize;
    let mut usize_9: usize = 65usize;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_9, usize_8, usize_7, usize_6);
    let mut adjacentcells_0_ref_0: &mut crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut usize_10: usize = 22usize;
    let mut usize_11: usize = 97usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_12: usize = 65usize;
    let mut usize_13: usize = 66usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_14: usize = 1usize;
    let mut usize_15: usize = 77usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_0, height: usize_15, width: usize_14, status: gamestate_2};
    let mut option_0: std::option::Option<usize> = std::iter::Iterator::next(adjacentcells_0_ref_0);
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_1_ref_0);
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut result_0: std::result::Result<bool, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::click_revealed(minesweeper_0_ref_0, usize_1, usize_0, bool_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_196() {
    rusty_monitor::set_test_id(196);
    let mut usize_0: usize = 47usize;
    let mut usize_1: usize = 86usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 26usize;
    let mut usize_3: usize = 99usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_4: usize = 85usize;
    let mut usize_5: usize = 29usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut usize_6: usize = 83usize;
    let mut usize_7: usize = 85usize;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_8: usize = 79usize;
    let mut usize_9: usize = 22usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut usize_10: usize = 41usize;
    let mut usize_11: usize = 70usize;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2_ref_0: &tictactoe::Player = &mut player_2;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::Win(player_3);
    let mut gamestate_5_ref_0: &tictactoe::GameState = &mut gamestate_5;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut usize_12: usize = 52usize;
    let mut usize_13: usize = 85usize;
    let mut usize_14: usize = 12usize;
    let mut usize_15: usize = 95usize;
    let mut usize_16: usize = 19usize;
    let mut usize_17: usize = 99usize;
    let mut usize_18: usize = 9usize;
    let mut usize_19: usize = 31usize;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_20: usize = 77usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_20, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut reversi_0: crate::reversi::Reversi = std::result::Result::unwrap(result_0);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut player_6_ref_0: &tictactoe::Player = &mut player_6;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut player_7: reversi::Player = crate::reversi::Reversi::get_next_player(reversi_0_ref_0);
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_4);
    let mut gamestate_6: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_6_ref_0: &minesweeper::GameState = &mut gamestate_6;
    let mut gamestate_7: minesweeper::GameState = std::clone::Clone::clone(gamestate_6_ref_0);
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut bool_3: bool = std::cmp::PartialEq::ne(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut gamestate_7_ref_0: &minesweeper::GameState = &mut gamestate_7;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_7_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_672() {
    rusty_monitor::set_test_id(672);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut usize_0: usize = 94usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::Some(player_0);
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Win(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut usize_1: usize = 92usize;
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::Some(player_6);
    let mut option_5: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_7: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_8: connect_four::Player = crate::connect_four::Player::other(player_7);
    let mut option_6: std::option::Option<connect_four::Player> = std::option::Option::Some(player_8);
    let mut option_7: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_9: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Win(player_9);
    let mut player_10: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_8: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut player_11: gomoku::Player = crate::gomoku::Player::Player1;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(minesweepererror_1_ref_0, minesweepererror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_452() {
    rusty_monitor::set_test_id(452);
    let mut usize_0: usize = 61usize;
    let mut usize_1: usize = 34usize;
    let mut usize_2: usize = 50usize;
    let mut usize_3: usize = 4usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut usize_4: usize = 88usize;
    let mut usize_5: usize = 2usize;
    let mut usize_6: usize = 61usize;
    let mut usize_7: usize = 32usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_8: usize = 73usize;
    let mut usize_9: usize = 42usize;
    let mut usize_10: usize = 80usize;
    let mut usize_11: usize = 14usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut result_1: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut player_2: gomoku::Player = crate::gomoku::Player::other(player_1);
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_4_ref_0, gamestate_0_ref_0);
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_7, usize_6, usize_5, usize_4);
    let mut gamestate_5: gomoku::GameState = crate::gomoku::GameState::InProgress;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1788() {
    rusty_monitor::set_test_id(1788);
    let mut usize_0: usize = 6usize;
    let mut usize_1: usize = 75usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 29usize;
    let mut usize_3: usize = 32usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_4: usize = 87usize;
    let mut usize_5: usize = 75usize;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_6: usize = 85usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_6, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = false;
    let mut usize_7: usize = 64usize;
    let mut usize_8: usize = 90usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_9: usize = 52usize;
    let mut usize_10: usize = 34usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut usize_11: usize = 55usize;
    let mut bool_6: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_6, mine_adjacent: usize_11, is_revealed: bool_5, is_flagged: bool_4};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut bool_7: bool = false;
    let mut cell_2: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_7);
    let mut cell_2_ref_0: &crate::minesweeper::Cell = &mut cell_2;
    let mut bool_8: bool = true;
    let mut usize_12: usize = 45usize;
    let mut usize_13: usize = 62usize;
    let mut usize_14: usize = 47usize;
    let mut usize_15: usize = 93usize;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut usize_16: usize = 34usize;
    let mut usize_17: usize = 87usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut usize_18: usize = 92usize;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_19: usize = 18usize;
    let mut usize_20: usize = 51usize;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut bool_9: bool = std::cmp::PartialEq::ne(gamestate_4_ref_0, gamestate_3_ref_0);
    let mut reversi_0: crate::reversi::Reversi = std::result::Result::unwrap(result_0);
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut result_1: std::result::Result<(), reversi::ReversiError> = crate::reversi::Reversi::check_position_validity(reversi_0_ref_0, usize_17, usize_16, player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut minesweepererror_3: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut minesweepererror_3_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_3;
    let mut gamestate_6: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut cell_3: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_8);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut gamestate_7: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_5: reversi::Player = crate::reversi::Player::Player0;
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut gomokuerror_2: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut bool_10: bool = std::cmp::PartialEq::eq(cell_2_ref_0, cell_1_ref_0);
    let mut player_6: reversi::Player = crate::reversi::Player::other(player_0);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_8: reversi::GameState = crate::reversi::GameState::Tie;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_32() {
    rusty_monitor::set_test_id(32);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut usize_0: usize = 96usize;
    let mut usize_1: usize = 12usize;
    let mut usize_2: usize = 60usize;
    let mut usize_3: usize = 95usize;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_2: gomoku::Player = crate::gomoku::Player::other(player_1);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut usize_4: usize = 47usize;
    let mut usize_5: usize = 66usize;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut usize_6: usize = 2usize;
    let mut usize_7: usize = 73usize;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5_ref_0: &tictactoe::Player = &mut player_5;
    let mut usize_8: usize = 62usize;
    let mut usize_9: usize = 4usize;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_6);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_9: tictactoe::Player = crate::tictactoe::Player::other(player_8);
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_9);
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_11: tictactoe::Player = crate::tictactoe::Player::other(player_10);
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_11);
    let mut player_12: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_12);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_13);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_14);
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut usize_10: usize = 68usize;
    let mut usize_11: usize = 83usize;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_12: usize = 13usize;
    let mut usize_13: usize = 98usize;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut reversi_0: crate::reversi::Reversi = std::result::Result::unwrap(result_0);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut gamestate_4: &reversi::GameState = crate::reversi::Reversi::status(reversi_0_ref_0);
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut result_1: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut reversi_1: crate::reversi::Reversi = std::result::Result::unwrap(result_1);
    let mut reversi_1_ref_0: &mut crate::reversi::Reversi = &mut reversi_1;
    let mut result_2: std::result::Result<(), reversi::ReversiError> = crate::reversi::Reversi::place(reversi_1_ref_0, player_4, usize_5, usize_4);
    let mut player_15: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut gamestate_5_ref_0: &tictactoe::GameState = &mut gamestate_5;
    let mut gamestate_6: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut player_16: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_17: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut gamestate_7: minesweeper::GameState = std::clone::Clone::clone(gamestate_3_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2161() {
    rusty_monitor::set_test_id(2161);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 69usize;
    let mut bool_2: bool = true;
    let mut usize_1: usize = 15usize;
    let mut usize_2: usize = 65usize;
    let mut usize_3: usize = 80usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_3: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_3);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut usize_4: usize = 66usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_5: usize = 84usize;
    let mut usize_6: usize = 45usize;
    let mut bool_4: bool = true;
    let mut usize_7: usize = 23usize;
    let mut usize_8: usize = 96usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_9: usize = 28usize;
    let mut usize_10: usize = 11usize;
    let mut bool_5: bool = false;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut usize_11: usize = 98usize;
    let mut usize_12: usize = 78usize;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_13: usize = 68usize;
    let mut usize_14: usize = 69usize;
    let mut bool_6: bool = true;
    let mut usize_15: usize = 35usize;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut usize_16: usize = 84usize;
    let mut bool_9: bool = false;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_9, mine_adjacent: usize_16, is_revealed: bool_8, is_flagged: bool_7};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_17: usize = 56usize;
    let mut usize_18: usize = 39usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut bool_10: bool = true;
    let mut usize_19: usize = 40usize;
    let mut usize_20: usize = 86usize;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_21: usize = 49usize;
    let mut usize_22: usize = 23usize;
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_0, height: usize_18, width: usize_17, status: gamestate_4};
    let mut cell_2: crate::minesweeper::Cell = std::clone::Clone::clone(cell_1_ref_0);
    let mut gamestate_5_ref_0: &minesweeper::GameState = &mut gamestate_5;
    let mut gamestate_6: minesweeper::GameState = std::clone::Clone::clone(gamestate_5_ref_0);
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    crate::minesweeper::Minesweeper::reveal_from(minesweeper_0_ref_0, usize_15);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut cell_3: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_6);
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut gamestate_7: minesweeper::GameState = std::clone::Clone::clone(gamestate_3_ref_0);
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut cell_4: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_5);
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_8: minesweeper::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_0_ref_0);
    let mut cell_5: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2173() {
    rusty_monitor::set_test_id(2173);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut usize_0: usize = 1usize;
    let mut usize_1: usize = 25usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 49usize;
    let mut usize_3: usize = 99usize;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut usize_4: usize = 96usize;
    let mut usize_5: usize = 85usize;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut usize_6: usize = 35usize;
    let mut usize_7: usize = 1usize;
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_8: usize = 80usize;
    let mut usize_9: usize = 50usize;
    let mut usize_10: usize = 31usize;
    let mut usize_11: usize = 13usize;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_12: usize = 8usize;
    let mut usize_13: usize = 40usize;
    let mut usize_14: usize = 31usize;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(minesweepererror_1_ref_0, minesweepererror_0_ref_0);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut player_6: gomoku::Player = crate::gomoku::Player::other(player_1);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut gamestate_5: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut bool_1: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1948() {
    rusty_monitor::set_test_id(1948);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 48usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut usize_1: usize = 73usize;
    let mut usize_2: usize = 97usize;
    let mut usize_3: usize = 9usize;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut usize_4: usize = 2usize;
    let mut bool_5: bool = false;
    let mut usize_5: usize = 89usize;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_2: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_2);
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_5, mine_adjacent: usize_4, is_revealed: bool_4, is_flagged: bool_3};
    let mut player_5_ref_0: &connect_four::Player = &mut player_5;
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut bool_6: bool = std::cmp::PartialEq::ne(cell_1_ref_0, cell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1815() {
    rusty_monitor::set_test_id(1815);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 18usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut usize_1: usize = 51usize;
    let mut usize_2: usize = 75usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut usize_3: usize = 41usize;
    let mut usize_4: usize = 59usize;
    let mut usize_5: usize = 73usize;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_6: usize = 35usize;
    let mut usize_7: usize = 10usize;
    let mut bool_3: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_3);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_1_ref_0);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut gamestate_5: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut cell_2: crate::minesweeper::Cell = std::clone::Clone::clone(cell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1167() {
    rusty_monitor::set_test_id(1167);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 37usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut usize_1: usize = 97usize;
    let mut bool_5: bool = true;
    let mut gamestate_0: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut gamestate_0_ref_0: &gomoku::GameState = &mut gamestate_0;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::Win(player_1);
    let mut gamestate_1_ref_0: &gomoku::GameState = &mut gamestate_1;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_2_ref_0: &gomoku::GameState = &mut gamestate_2;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_3: gomoku::Player = crate::gomoku::Player::other(player_2);
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::Win(player_3);
    let mut gamestate_3_ref_0: &gomoku::GameState = &mut gamestate_3;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_4_ref_0: &connect_four::Player = &mut player_4;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_5, mine_adjacent: usize_1, is_revealed: bool_4, is_flagged: bool_3};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut bool_6: bool = std::cmp::PartialEq::eq(cell_1_ref_0, cell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_91() {
    rusty_monitor::set_test_id(91);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2_ref_0: &reversi::Player = &mut player_2;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut usize_0: usize = 58usize;
    let mut usize_1: usize = 31usize;
    let mut usize_2: usize = 73usize;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_3: usize = 14usize;
    let mut bool_2: bool = false;
    let mut usize_4: usize = 83usize;
    let mut usize_5: usize = 65usize;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut player_5_ref_0: &connect_four::Player = &mut player_5;
    let mut u64_0: u64 = 60u64;
    let mut u64_1: u64 = 55u64;
    let mut steprng_0: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_0;
    let mut usize_6: usize = 73usize;
    let mut usize_7: usize = 6usize;
    let mut usize_8: usize = 57usize;
    let mut result_0: std::result::Result<crate::minesweeper::Minesweeper, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::new(usize_8, usize_7, usize_6, steprng_0_ref_0);
    let mut minesweeper_0: crate::minesweeper::Minesweeper = std::result::Result::unwrap(result_0);
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_3, is_revealed: bool_1, is_flagged: bool_0};
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Win(player_3);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    crate::minesweeper::Minesweeper::update_around_mine_count(minesweeper_0_ref_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Tie;
    let mut player_6_ref_0: &tictactoe::Player = &mut player_6;
    let mut player_7: connect_four::Player = crate::connect_four::Player::other(player_1);
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut result_1: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2614() {
    rusty_monitor::set_test_id(2614);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut bool_0: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut usize_0: usize = 3usize;
    let mut usize_1: usize = 6usize;
    let mut usize_2: usize = 54usize;
    let mut usize_3: usize = 74usize;
    let mut usize_4: usize = 74usize;
    let mut usize_5: usize = 39usize;
    let mut usize_6: usize = 20usize;
    let mut usize_7: usize = 28usize;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_7, usize_6, usize_5, usize_4);
    let mut adjacentcells_0_ref_0: &mut crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut usize_8: usize = 72usize;
    let mut usize_9: usize = 27usize;
    let mut usize_10: usize = 68usize;
    let mut usize_11: usize = 19usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut usize_12: usize = 59usize;
    let mut bool_3: bool = false;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_3, mine_adjacent: usize_12, is_revealed: bool_2, is_flagged: bool_1};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut bool_4: bool = true;
    let mut usize_13: usize = 73usize;
    let mut usize_14: usize = 44usize;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_15: usize = 14usize;
    let mut usize_16: usize = 43usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut gamestate_6: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_0, height: usize_16, width: usize_15, status: gamestate_4};
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut result_0: std::result::Result<bool, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::click(minesweeper_0_ref_0, usize_14, usize_13, bool_4);
    let mut gamestate_6_ref_0: &tictactoe::GameState = &mut gamestate_6;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_1_ref_0);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::other(player_5);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_5_ref_0: &tictactoe::GameState = &mut gamestate_5;
    let mut player_9: reversi::Player = crate::reversi::Player::Player0;
    let mut player_7_ref_0: &tictactoe::Player = &mut player_7;
    let mut option_1: std::option::Option<usize> = std::iter::Iterator::next(adjacentcells_0_ref_0);
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut adjacentcells_1: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_3, usize_2, usize_1, usize_0);
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut cell_2: crate::minesweeper::Cell = std::clone::Clone::clone(cell_0_ref_0);
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut player_10: gomoku::Player = crate::gomoku::Player::Player1;
    let mut bool_5: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1770() {
    rusty_monitor::set_test_id(1770);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut bool_0: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut usize_0: usize = 67usize;
    let mut usize_1: usize = 22usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_2: usize = 33usize;
    let mut usize_3: usize = 32usize;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_4_ref_0, gamestate_3_ref_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_0_ref_0);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut bool_2: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_5: minesweeper::GameState = std::clone::Clone::clone(gamestate_2_ref_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_0_ref_0);
    let mut gamestate_6: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_7: reversi::GameState = crate::reversi::GameState::Win(player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2876() {
    rusty_monitor::set_test_id(2876);
    let mut usize_0: usize = 17usize;
    let mut isize_0: isize = -55isize;
    let mut isize_1: isize = 32isize;
    let mut isize_2: isize = -118isize;
    let mut isize_3: isize = -172isize;
    let mut tuple_0: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = 184isize;
    let mut isize_5: isize = -51isize;
    let mut tuple_1: (isize, isize) = (isize_5, isize_4);
    let mut isize_6: isize = 235isize;
    let mut isize_7: isize = 148isize;
    let mut tuple_2: (isize, isize) = (isize_7, isize_6);
    let mut isize_8: isize = 51isize;
    let mut isize_9: isize = 47isize;
    let mut tuple_3: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = -28isize;
    let mut isize_11: isize = 105isize;
    let mut tuple_4: (isize, isize) = (isize_11, isize_10);
    let mut isize_12: isize = -28isize;
    let mut isize_13: isize = -71isize;
    let mut tuple_5: (isize, isize) = (isize_13, isize_12);
    let mut isize_14: isize = 46isize;
    let mut isize_15: isize = -94isize;
    let mut tuple_6: (isize, isize) = (isize_15, isize_14);
    let mut isize_16: isize = -219isize;
    let mut isize_17: isize = 5isize;
    let mut tuple_7: (isize, isize) = (isize_17, isize_16);
    let mut tuple_array_0: [(isize, isize); 8] = [tuple_7, tuple_6, tuple_5, tuple_4, tuple_3, tuple_2, tuple_1, tuple_0];
    let mut usize_1: usize = 72usize;
    let mut usize_2: usize = 80usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut usize_3: usize = 45usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_4: usize = 68usize;
    let mut usize_5: usize = 78usize;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_6: usize = 70usize;
    let mut bool_2: bool = false;
    let mut usize_7: usize = 45usize;
    let mut isize_18: isize = -13isize;
    let mut isize_19: isize = 76isize;
    let mut isize_20: isize = 91isize;
    let mut isize_21: isize = -213isize;
    let mut tuple_8: (isize, isize) = (isize_21, isize_20);
    let mut isize_22: isize = 61isize;
    let mut isize_23: isize = -83isize;
    let mut tuple_9: (isize, isize) = (isize_23, isize_22);
    let mut isize_24: isize = -183isize;
    let mut isize_25: isize = -27isize;
    let mut tuple_10: (isize, isize) = (isize_25, isize_24);
    let mut isize_26: isize = 86isize;
    let mut isize_27: isize = -51isize;
    let mut tuple_11: (isize, isize) = (isize_27, isize_26);
    let mut isize_28: isize = -112isize;
    let mut isize_29: isize = -73isize;
    let mut tuple_12: (isize, isize) = (isize_29, isize_28);
    let mut isize_30: isize = 116isize;
    let mut isize_31: isize = -41isize;
    let mut tuple_13: (isize, isize) = (isize_31, isize_30);
    let mut isize_32: isize = -129isize;
    let mut isize_33: isize = 37isize;
    let mut tuple_14: (isize, isize) = (isize_33, isize_32);
    let mut isize_34: isize = 29isize;
    let mut isize_35: isize = -46isize;
    let mut tuple_15: (isize, isize) = (isize_35, isize_34);
    let mut tuple_array_1: [(isize, isize); 8] = [tuple_15, tuple_14, tuple_13, tuple_12, tuple_11, tuple_10, tuple_9, tuple_8];
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_8: usize = 25usize;
    let mut usize_9: usize = 50usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_0, height: usize_9, width: usize_8, status: gamestate_2};
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells {around: tuple_array_1, board_height: isize_19, board_width: isize_18, offset: usize_7};
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    crate::minesweeper::Minesweeper::check_state(minesweeper_0_ref_0);
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_6, is_revealed: bool_1, is_flagged: bool_0};
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut adjacentcells_0_ref_0: &mut crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut option_0: std::option::Option<usize> = std::iter::Iterator::next(adjacentcells_0_ref_0);
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
    let mut adjacentcells_1: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells {around: tuple_array_0, board_height: isize_1, board_width: isize_0, offset: usize_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2014() {
    rusty_monitor::set_test_id(2014);
    let mut usize_0: usize = 6usize;
    let mut usize_1: usize = 81usize;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_2: usize = 43usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_2, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_3: usize = 54usize;
    let mut usize_4: usize = 2usize;
    let mut usize_5: usize = 3usize;
    let mut isize_0: isize = -119isize;
    let mut isize_1: isize = 60isize;
    let mut isize_2: isize = 8isize;
    let mut isize_3: isize = -30isize;
    let mut tuple_0: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = 110isize;
    let mut isize_5: isize = 16isize;
    let mut tuple_1: (isize, isize) = (isize_5, isize_4);
    let mut isize_6: isize = 80isize;
    let mut isize_7: isize = -85isize;
    let mut tuple_2: (isize, isize) = (isize_7, isize_6);
    let mut isize_8: isize = -35isize;
    let mut isize_9: isize = 4isize;
    let mut tuple_3: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = -24isize;
    let mut isize_11: isize = -39isize;
    let mut tuple_4: (isize, isize) = (isize_11, isize_10);
    let mut isize_12: isize = 192isize;
    let mut isize_13: isize = 106isize;
    let mut tuple_5: (isize, isize) = (isize_13, isize_12);
    let mut isize_14: isize = -74isize;
    let mut isize_15: isize = -60isize;
    let mut tuple_6: (isize, isize) = (isize_15, isize_14);
    let mut isize_16: isize = -27isize;
    let mut isize_17: isize = 99isize;
    let mut tuple_7: (isize, isize) = (isize_17, isize_16);
    let mut tuple_array_0: [(isize, isize); 8] = [tuple_7, tuple_6, tuple_5, tuple_4, tuple_3, tuple_2, tuple_1, tuple_0];
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_6: usize = 30usize;
    let mut usize_7: usize = 24usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut player_1_ref_0: &connect_four::Player = &mut player_1;
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_0, height: usize_7, width: usize_6, status: gamestate_1};
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells {around: tuple_array_0, board_height: isize_1, board_width: isize_0, offset: usize_5};
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut cell_1: crate::minesweeper::Cell = std::clone::Clone::clone(cell_0_ref_0);
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut result_0: std::result::Result<(), minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::click_unrevealed(minesweeper_0_ref_0, usize_1, usize_0);
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_136() {
    rusty_monitor::set_test_id(136);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_0: usize = 51usize;
    let mut usize_1: usize = 24usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_2: usize = 96usize;
    let mut usize_3: usize = 93usize;
    let mut usize_4: usize = 29usize;
    let mut usize_5: usize = 13usize;
    let mut usize_6: usize = 79usize;
    let mut usize_7: usize = 72usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut usize_8: usize = 84usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_9: usize = 70usize;
    let mut usize_10: usize = 22usize;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut gamestate_5: minesweeper::GameState = std::clone::Clone::clone(gamestate_4_ref_0);
    let mut gamestate_5_ref_0: &minesweeper::GameState = &mut gamestate_5;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_5_ref_0, gamestate_2_ref_0);
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_7, usize_6, usize_5, usize_4);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_1_ref_0);
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut player_0_ref_0: &connect_four::Player = &mut player_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2704() {
    rusty_monitor::set_test_id(2704);
    let mut bool_0: bool = true;
    let mut usize_0: usize = 14usize;
    let mut usize_1: usize = 77usize;
    let mut usize_2: usize = 52usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_3: usize = 49usize;
    let mut usize_4: usize = 76usize;
    let mut usize_5: usize = 85usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_6: usize = 81usize;
    let mut usize_7: usize = 36usize;
    let mut usize_8: usize = 97usize;
    let mut usize_9: usize = 38usize;
    let mut usize_10: usize = 71usize;
    let mut usize_11: usize = 3usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut u64_0: u64 = 91u64;
    let mut u64_1: u64 = 28u64;
    let mut steprng_0: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_0;
    let mut usize_12: usize = 38usize;
    let mut usize_13: usize = 0usize;
    let mut usize_14: usize = 78usize;
    let mut result_0: std::result::Result<crate::minesweeper::Minesweeper, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::new(usize_14, usize_13, usize_12, steprng_0_ref_0);
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_1: bool = std::cmp::PartialEq::eq(minesweepererror_1_ref_0, minesweepererror_0_ref_0);
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_11, usize_10, usize_9, usize_8);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut minesweeper_0: crate::minesweeper::Minesweeper = std::result::Result::unwrap(result_0);
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut result_1: std::result::Result<bool, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::click_revealed(minesweeper_0_ref_0, usize_1, usize_0, bool_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2652() {
    rusty_monitor::set_test_id(2652);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut usize_0: usize = 15usize;
    let mut bool_3: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_3, mine_adjacent: usize_0, is_revealed: bool_2, is_flagged: bool_1};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut usize_1: usize = 27usize;
    let mut bool_6: bool = false;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut usize_2: usize = 62usize;
    let mut usize_3: usize = 51usize;
    let mut usize_4: usize = 79usize;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::Win(player_0);
    let mut gamestate_1_ref_0: &gomoku::GameState = &mut gamestate_1;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_2_ref_0: &gomoku::GameState = &mut gamestate_2;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_6, mine_adjacent: usize_1, is_revealed: bool_5, is_flagged: bool_4};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut bool_7: bool = std::cmp::PartialEq::eq(cell_1_ref_0, cell_0_ref_0);
    let mut cell_2: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_886() {
    rusty_monitor::set_test_id(886);
    let mut usize_0: usize = 78usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_1: usize = 56usize;
    let mut usize_2: usize = 86usize;
    let mut usize_3: usize = 34usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_4: usize = 50usize;
    let mut usize_5: usize = 79usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut usize_6: usize = 61usize;
    let mut usize_7: usize = 14usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_8: usize = 39usize;
    let mut usize_9: usize = 28usize;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut usize_10: usize = 72usize;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_11: usize = 51usize;
    let mut usize_12: usize = 77usize;
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_4_ref_0, gamestate_3_ref_0);
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_6: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_5_ref_0: &connect_four::GameState = &mut gamestate_5;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut gomokuerror_2: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gamestate_7: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut gomokuerror_3: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_8: minesweeper::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut gamestate_9: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2007() {
    rusty_monitor::set_test_id(2007);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_0_ref_0: &tictactoe::Player = &mut player_0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1_ref_0: &tictactoe::Player = &mut player_1;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut player_4_ref_0: &tictactoe::Player = &mut player_4;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_5_ref_0: &tictactoe::Player = &mut player_5;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_6);
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut player_7: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_7_ref_0: &tictactoe::Player = &mut player_7;
    let mut player_8: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::Win(player_2);
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2622() {
    rusty_monitor::set_test_id(2622);
    let mut usize_0: usize = 69usize;
    let mut usize_1: usize = 77usize;
    let mut usize_2: usize = 54usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut usize_3: usize = 58usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_4: usize = 16usize;
    let mut usize_5: usize = 38usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut u64_0: u64 = 23u64;
    let mut u64_1: u64 = 41u64;
    let mut steprng_0: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_0;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_6: usize = 7usize;
    let mut usize_7: usize = 36usize;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_143() {
    rusty_monitor::set_test_id(143);
    let mut usize_0: usize = 0usize;
    let mut isize_0: isize = 82isize;
    let mut isize_1: isize = 2isize;
    let mut isize_2: isize = -30isize;
    let mut isize_3: isize = 214isize;
    let mut tuple_0: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = -15isize;
    let mut isize_5: isize = 58isize;
    let mut tuple_1: (isize, isize) = (isize_5, isize_4);
    let mut isize_6: isize = 227isize;
    let mut isize_7: isize = 59isize;
    let mut tuple_2: (isize, isize) = (isize_7, isize_6);
    let mut isize_8: isize = -224isize;
    let mut isize_9: isize = -59isize;
    let mut tuple_3: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = 45isize;
    let mut isize_11: isize = -126isize;
    let mut tuple_4: (isize, isize) = (isize_11, isize_10);
    let mut isize_12: isize = 194isize;
    let mut isize_13: isize = 55isize;
    let mut tuple_5: (isize, isize) = (isize_13, isize_12);
    let mut isize_14: isize = -228isize;
    let mut isize_15: isize = 17isize;
    let mut tuple_6: (isize, isize) = (isize_15, isize_14);
    let mut isize_16: isize = 44isize;
    let mut isize_17: isize = -32isize;
    let mut tuple_7: (isize, isize) = (isize_17, isize_16);
    let mut tuple_array_0: [(isize, isize); 8] = [tuple_7, tuple_6, tuple_5, tuple_4, tuple_3, tuple_2, tuple_1, tuple_0];
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells {around: tuple_array_0, board_height: isize_1, board_width: isize_0, offset: usize_0};
    let mut adjacentcells_0_ref_0: &crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut adjacentcells_1: crate::minesweeper::AdjacentCells = std::clone::Clone::clone(adjacentcells_0_ref_0);
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4157() {
    rusty_monitor::set_test_id(4157);
    let mut usize_0: usize = 14usize;
    let mut usize_1: usize = 23usize;
    let mut usize_2: usize = 95usize;
    let mut usize_3: usize = 98usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut usize_4: usize = 79usize;
    let mut usize_5: usize = 23usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut usize_6: usize = 19usize;
    let mut usize_7: usize = 33usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_8: usize = 62usize;
    let mut usize_9: usize = 85usize;
    let mut usize_10: usize = 62usize;
    let mut usize_11: usize = 20usize;
    let mut usize_12: usize = 83usize;
    let mut usize_13: usize = 20usize;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_13, usize_12, usize_11, usize_10);
    let mut adjacentcells_0_ref_0: &mut crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut usize_14: usize = 49usize;
    let mut usize_15: usize = 55usize;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_3: minesweeper::GameState = std::clone::Clone::clone(gamestate_2_ref_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut player_1_ref_0: &tictactoe::Player = &mut player_1;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut option_0: std::option::Option<usize> = std::iter::Iterator::next(adjacentcells_0_ref_0);
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player0;
    let mut adjacentcells_1: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_3, usize_2, usize_1, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3781() {
    rusty_monitor::set_test_id(3781);
    let mut usize_0: usize = 6usize;
    let mut isize_0: isize = -8isize;
    let mut isize_1: isize = -22isize;
    let mut isize_2: isize = 0isize;
    let mut isize_3: isize = 46isize;
    let mut tuple_0: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = -44isize;
    let mut isize_5: isize = 112isize;
    let mut tuple_1: (isize, isize) = (isize_5, isize_4);
    let mut isize_6: isize = -148isize;
    let mut isize_7: isize = -10isize;
    let mut tuple_2: (isize, isize) = (isize_7, isize_6);
    let mut isize_8: isize = 56isize;
    let mut isize_9: isize = 108isize;
    let mut tuple_3: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = 35isize;
    let mut isize_11: isize = -130isize;
    let mut tuple_4: (isize, isize) = (isize_11, isize_10);
    let mut isize_12: isize = -32isize;
    let mut isize_13: isize = -13isize;
    let mut tuple_5: (isize, isize) = (isize_13, isize_12);
    let mut isize_14: isize = 172isize;
    let mut isize_15: isize = -15isize;
    let mut tuple_6: (isize, isize) = (isize_15, isize_14);
    let mut isize_16: isize = 34isize;
    let mut isize_17: isize = -164isize;
    let mut tuple_7: (isize, isize) = (isize_17, isize_16);
    let mut tuple_array_0: [(isize, isize); 8] = [tuple_7, tuple_6, tuple_5, tuple_4, tuple_3, tuple_2, tuple_1, tuple_0];
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells {around: tuple_array_0, board_height: isize_1, board_width: isize_0, offset: usize_0};
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut adjacentcells_0_ref_0: &mut crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut option_0: std::option::Option<usize> = std::iter::Iterator::next(adjacentcells_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2842() {
    rusty_monitor::set_test_id(2842);
    let mut usize_0: usize = 73usize;
    let mut usize_1: usize = 15usize;
    let mut usize_2: usize = 87usize;
    let mut usize_3: usize = 46usize;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_3, usize_2, usize_1, usize_0);
    let mut adjacentcells_0_ref_0: &crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut usize_4: usize = 94usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_4, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_3: gomoku::Player = crate::gomoku::Player::other(player_2);
    let mut player_4: reversi::Player = crate::reversi::Player::Player1;
    let mut usize_5: usize = 45usize;
    let mut usize_6: usize = 77usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_7: usize = 56usize;
    let mut usize_8: usize = 86usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_0, height: usize_8, width: usize_7, status: gamestate_0};
    let mut minesweeper_0_ref_0: &crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut adjacentcells_1: crate::minesweeper::AdjacentCells = crate::minesweeper::Minesweeper::get_adjacent_cells(minesweeper_0_ref_0, usize_6, usize_5);
    let mut player_5: reversi::Player = crate::reversi::Player::other(player_4);
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::Win(player_3);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_0_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(minesweepererror_1_ref_0, minesweepererror_0_ref_0);
    let mut player_6: gomoku::Player = crate::gomoku::Player::other(player_1);
    let mut player_7: gomoku::Player = crate::gomoku::Player::Player0;
    let mut adjacentcells_1_ref_0: &crate::minesweeper::AdjacentCells = &mut adjacentcells_1;
    let mut adjacentcells_2: crate::minesweeper::AdjacentCells = std::clone::Clone::clone(adjacentcells_1_ref_0);
    let mut adjacentcells_3: crate::minesweeper::AdjacentCells = std::clone::Clone::clone(adjacentcells_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4166() {
    rusty_monitor::set_test_id(4166);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_0: usize = 99usize;
    let mut usize_1: usize = 20usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_2: usize = 4usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_2, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut usize_3: usize = 39usize;
    let mut bool_5: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_5, mine_adjacent: usize_3, is_revealed: bool_4, is_flagged: bool_3};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut bool_6: bool = true;
    let mut usize_4: usize = 1usize;
    let mut usize_5: usize = 69usize;
    let mut usize_6: usize = 78usize;
    let mut usize_7: usize = 51usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut backtrace_0: &snafu::Backtrace = std::option::Option::unwrap(option_0);
    let mut connectfour_0: crate::connect_four::ConnectFour = std::result::Result::unwrap(result_0);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut cell_2: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_6);
    let mut cell_2_ref_0: &crate::minesweeper::Cell = &mut cell_2;
    let mut bool_7: bool = std::cmp::PartialEq::eq(cell_2_ref_0, cell_0_ref_0);
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3616() {
    rusty_monitor::set_test_id(3616);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut usize_0: usize = 41usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_1: usize = 95usize;
    let mut usize_2: usize = 39usize;
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player1;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_3: usize = 49usize;
    let mut bool_2: bool = true;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut gamestate_1_ref_0: &gomoku::GameState = &mut gamestate_1;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_2_ref_0: &gomoku::GameState = &mut gamestate_2;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_4: gomoku::GameState = crate::gomoku::GameState::Win(player_2);
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_3, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut cell_1: crate::minesweeper::Cell = std::clone::Clone::clone(cell_0_ref_0);
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_6: connect_four::GameState = crate::connect_four::GameState::Win(player_1);
    let mut gamestate_5_ref_0: &tictactoe::GameState = &mut gamestate_5;
    let mut gamestate_7: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_6_ref_0: &connect_four::GameState = &mut gamestate_6;
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut cell_2: crate::minesweeper::Cell = std::clone::Clone::clone(cell_1_ref_0);
    let mut gamestate_8: reversi::GameState = crate::reversi::GameState::Tie;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4557() {
    rusty_monitor::set_test_id(4557);
    let mut bool_0: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_1: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_1);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_0: usize = 2usize;
    let mut usize_1: usize = 69usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 2usize;
    let mut usize_3: usize = 86usize;
    let mut usize_4: usize = 61usize;
    let mut usize_5: usize = 12usize;
    let mut usize_6: usize = 62usize;
    let mut bool_2: bool = false;
    let mut usize_7: usize = 78usize;
    let mut usize_8: usize = 15usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_9: usize = 48usize;
    let mut usize_10: usize = 47usize;
    let mut usize_11: usize = 77usize;
    let mut usize_12: usize = 35usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_3_ref_0);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut cell_2: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_2);
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut cell_3: crate::minesweeper::Cell = std::clone::Clone::clone(cell_1_ref_0);
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut cell_2_ref_0: &crate::minesweeper::Cell = &mut cell_2;
    let mut bool_3: bool = std::cmp::PartialEq::ne(cell_2_ref_0, cell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_254() {
    rusty_monitor::set_test_id(254);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut usize_0: usize = 56usize;
    let mut usize_1: usize = 12usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 38usize;
    let mut usize_3: usize = 90usize;
    let mut usize_4: usize = 23usize;
    let mut usize_5: usize = 5usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_6: usize = 31usize;
    let mut usize_7: usize = 90usize;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut bool_0: bool = false;
    let mut usize_8: usize = 56usize;
    let mut usize_9: usize = 77usize;
    let mut usize_10: usize = 36usize;
    let mut usize_11: usize = 20usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_12: usize = 73usize;
    let mut usize_13: usize = 89usize;
    let mut usize_14: usize = 2usize;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_15: usize = 57usize;
    let mut usize_16: usize = 25usize;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_5_ref_0: &minesweeper::GameState = &mut gamestate_5;
    let mut usize_17: usize = 38usize;
    let mut usize_18: usize = 61usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_6: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_19: usize = 23usize;
    let mut usize_20: usize = 90usize;
    let mut gamestate_7: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_21: usize = 67usize;
    let mut usize_22: usize = 70usize;
    let mut bool_1: bool = false;
    let mut usize_23: usize = 63usize;
    let mut usize_24: usize = 70usize;
    let mut gamestate_8: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_25: usize = 29usize;
    let mut usize_26: usize = 86usize;
    let mut gamestate_9: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_27: usize = 49usize;
    let mut usize_28: usize = 50usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_2);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_10: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut usize_29: usize = 3usize;
    let mut usize_30: usize = 95usize;
    let mut vec_1: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_1, height: usize_30, width: usize_29, status: gamestate_10};
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_0_ref_0);
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_11: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut bool_3: bool = std::cmp::PartialEq::ne(gamestate_5_ref_0, gamestate_4_ref_0);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut result_0: std::result::Result<bool, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::click(minesweeper_0_ref_0, usize_9, usize_8, bool_0);
    let mut minesweepererror_3: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut gamestate_12: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut bool_4: bool = std::cmp::PartialEq::eq(minesweepererror_1_ref_0, minesweepererror_0_ref_0);
    panic!("From RustyUnit with love");
}
}