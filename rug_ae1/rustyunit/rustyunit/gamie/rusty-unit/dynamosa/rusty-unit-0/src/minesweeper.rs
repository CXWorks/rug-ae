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
fn rusty_test_7767() {
    rusty_monitor::set_test_id(7767);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut isize_0: isize = 55isize;
    let mut isize_1: isize = -110isize;
    let mut tuple_0: (isize, isize) = (isize_1, isize_0);
    let mut isize_2: isize = 232isize;
    let mut isize_3: isize = -32isize;
    let mut tuple_1: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = -71isize;
    let mut isize_5: isize = 62isize;
    let mut tuple_2: (isize, isize) = (isize_5, isize_4);
    let mut isize_6: isize = 85isize;
    let mut isize_7: isize = -124isize;
    let mut tuple_3: (isize, isize) = (isize_7, isize_6);
    let mut isize_8: isize = -92isize;
    let mut isize_9: isize = 52isize;
    let mut tuple_4: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = 133isize;
    let mut isize_11: isize = 96isize;
    let mut tuple_5: (isize, isize) = (isize_11, isize_10);
    let mut isize_12: isize = 31isize;
    let mut isize_13: isize = 45isize;
    let mut tuple_6: (isize, isize) = (isize_13, isize_12);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_0: std::option::Option<gomoku::Player> = std::option::Option::Some(player_2);
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_1: std::option::Option<gomoku::Player> = std::option::Option::Some(player_3);
    let mut option_2: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_3: std::option::Option<gomoku::Player> = std::option::Option::Some(player_4);
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_4: std::option::Option<gomoku::Player> = std::option::Option::Some(player_5);
    let mut option_5: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_6: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_6: std::option::Option<gomoku::Player> = std::option::Option::Some(player_6);
    let mut player_7: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_7: std::option::Option<gomoku::Player> = std::option::Option::Some(player_7);
    let mut player_8: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_8: std::option::Option<gomoku::Player> = std::option::Option::Some(player_8);
    let mut option_9: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_12: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_13: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_14: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<gomoku::Player>; 15] = [option_14, option_13, option_12, option_11, option_10, option_9, option_8, option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut player_9: gomoku::Player = crate::gomoku::Player::Player0;
    let mut option_15: std::option::Option<gomoku::Player> = std::option::Option::Some(player_9);
    let mut player_10: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_16: std::option::Option<gomoku::Player> = std::option::Option::Some(player_10);
    let mut player_11: gomoku::Player = crate::gomoku::Player::Player1;
    let mut option_17: std::option::Option<gomoku::Player> = std::option::Option::Some(player_11);
    let mut option_18: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut player_12: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_13: gomoku::Player = crate::gomoku::Player::other(player_12);
    let mut option_19: std::option::Option<gomoku::Player> = std::option::Option::Some(player_13);
    let mut option_20: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_21: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_22: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut option_23: std::option::Option<gomoku::Player> = std::option::Option::None;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut gamestate_4: minesweeper::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut player_14: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2592() {
    rusty_monitor::set_test_id(2592);
    let mut bool_0: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4702() {
    rusty_monitor::set_test_id(4702);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_0: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut usize_0: usize = 28usize;
    let mut bool_3: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_3, mine_adjacent: usize_0, is_revealed: bool_2, is_flagged: bool_1};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut usize_1: usize = 35usize;
    let mut usize_2: usize = 65usize;
    let mut usize_3: usize = 31usize;
    let mut usize_4: usize = 40usize;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_4, usize_3, usize_2, usize_1);
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut player_4_ref_0: &reversi::Player = &mut player_4;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(cell_1_ref_0, cell_0_ref_0);
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7621() {
    rusty_monitor::set_test_id(7621);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_0: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut usize_0: usize = 28usize;
    let mut bool_3: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_3, mine_adjacent: usize_0, is_revealed: bool_2, is_flagged: bool_1};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut player_4_ref_0: &reversi::Player = &mut player_4;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(cell_1_ref_0, cell_0_ref_0);
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2670() {
    rusty_monitor::set_test_id(2670);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut gomokuerror_1_ref_0: &gomoku::GomokuError = &mut gomokuerror_1;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_1_ref_0);
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5148() {
    rusty_monitor::set_test_id(5148);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_0: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut usize_0: usize = 28usize;
    let mut bool_3: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_3, mine_adjacent: usize_0, is_revealed: bool_2, is_flagged: bool_1};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut usize_1: usize = 35usize;
    let mut usize_2: usize = 65usize;
    let mut usize_3: usize = 31usize;
    let mut usize_4: usize = 40usize;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_4, usize_3, usize_2, usize_1);
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut player_4_ref_0: &reversi::Player = &mut player_4;
    let mut bool_4: bool = std::cmp::PartialEq::eq(cell_1_ref_0, cell_0_ref_0);
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut adjacentcells_0_ref_0: &mut crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut option_0: std::option::Option<usize> = std::iter::Iterator::next(adjacentcells_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7807() {
    rusty_monitor::set_test_id(7807);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_0: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut usize_0: usize = 28usize;
    let mut bool_3: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_3, mine_adjacent: usize_0, is_revealed: bool_2, is_flagged: bool_1};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut usize_1: usize = 35usize;
    let mut usize_2: usize = 65usize;
    let mut usize_3: usize = 31usize;
    let mut usize_4: usize = 40usize;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_4, usize_3, usize_2, usize_1);
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut player_4_ref_0: &reversi::Player = &mut player_4;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(cell_1_ref_0, cell_0_ref_0);
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6741() {
    rusty_monitor::set_test_id(6741);
    let mut usize_0: usize = 85usize;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_0, mine_adjacent: usize_0, is_revealed: bool_2, is_flagged: bool_1};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_3);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut gomokuerror_1_ref_0: &gomoku::GomokuError = &mut gomokuerror_1;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_0_ref_0);
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut bool_4: bool = std::cmp::PartialEq::ne(cell_1_ref_0, cell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3855() {
    rusty_monitor::set_test_id(3855);
    let mut isize_0: isize = -13isize;
    let mut isize_1: isize = -32isize;
    let mut isize_2: isize = -42isize;
    let mut tuple_0: (isize, isize) = (isize_2, isize_1);
    let mut isize_3: isize = -38isize;
    let mut isize_4: isize = 47isize;
    let mut tuple_1: (isize, isize) = (isize_4, isize_3);
    let mut isize_5: isize = -89isize;
    let mut isize_6: isize = -24isize;
    let mut tuple_2: (isize, isize) = (isize_6, isize_5);
    let mut isize_7: isize = 104isize;
    let mut tuple_3: (isize, isize) = (isize_7, isize_0);
    let mut isize_8: isize = 109isize;
    let mut isize_9: isize = -68isize;
    let mut tuple_4: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = -8isize;
    let mut isize_11: isize = 71isize;
    let mut tuple_5: (isize, isize) = (isize_11, isize_10);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut gamestate_3: minesweeper::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4491() {
    rusty_monitor::set_test_id(4491);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut player_4_ref_0: &reversi::Player = &mut player_4;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7487() {
    rusty_monitor::set_test_id(7487);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_0: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut minesweepererror_3: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut gomokuerror_1_ref_0: &gomoku::GomokuError = &mut gomokuerror_1;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut minesweepererror_3_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_3;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_3_ref_0);
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_1_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2666() {
    rusty_monitor::set_test_id(2666);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 55usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut usize_1: usize = 65usize;
    let mut bool_5: bool = false;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_5, mine_adjacent: usize_1, is_revealed: bool_4, is_flagged: bool_3};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut bool_6: bool = true;
    let mut cell_2: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_6);
    let mut cell_2_ref_0: &crate::minesweeper::Cell = &mut cell_2;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut gomokuerror_1_ref_0: &gomoku::GomokuError = &mut gomokuerror_1;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_1_ref_0);
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut bool_7: bool = std::cmp::PartialEq::ne(cell_2_ref_0, cell_1_ref_0);
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Tie;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_5: gomoku::Player = crate::gomoku::Player::other(player_4);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::Win(player_3);
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(minesweepererror_1_ref_0, minesweepererror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_365() {
    rusty_monitor::set_test_id(365);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 65usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_3);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut gomokuerror_1_ref_0: &gomoku::GomokuError = &mut gomokuerror_1;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_4: bool = std::cmp::PartialEq::eq(cell_0_ref_0, cell_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5707() {
    rusty_monitor::set_test_id(5707);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut bool_0: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_2_ref_0: &connect_four::GameState = &mut gamestate_2;
    let mut gomokuerror_1_ref_0: &gomoku::GomokuError = &mut gomokuerror_1;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_1_ref_0);
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_5: minesweeper::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_122() {
    rusty_monitor::set_test_id(122);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 64usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut usize_1: usize = 85usize;
    let mut usize_2: usize = 55usize;
    let mut usize_3: usize = 48usize;
    let mut usize_4: usize = 55usize;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_4, usize_3, usize_2, usize_1);
    let mut adjacentcells_0_ref_0: &crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut bool_3: bool = false;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_3);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_4: bool = std::cmp::PartialEq::eq(minesweepererror_1_ref_0, minesweepererror_0_ref_0);
    let mut cell_2: crate::minesweeper::Cell = std::clone::Clone::clone(cell_1_ref_0);
    let mut adjacentcells_1: crate::minesweeper::AdjacentCells = std::clone::Clone::clone(adjacentcells_0_ref_0);
    let mut cell_2_ref_0: &crate::minesweeper::Cell = &mut cell_2;
    let mut bool_5: bool = std::cmp::PartialEq::eq(cell_2_ref_0, cell_0_ref_0);
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2663() {
    rusty_monitor::set_test_id(2663);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut isize_0: isize = -32isize;
    let mut isize_1: isize = -42isize;
    let mut tuple_0: (isize, isize) = (isize_1, isize_0);
    let mut isize_2: isize = -89isize;
    let mut isize_3: isize = -24isize;
    let mut tuple_1: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = 99isize;
    let mut isize_5: isize = 104isize;
    let mut tuple_2: (isize, isize) = (isize_5, isize_4);
    let mut isize_6: isize = 109isize;
    let mut isize_7: isize = -68isize;
    let mut tuple_3: (isize, isize) = (isize_7, isize_6);
    let mut isize_8: isize = -94isize;
    let mut isize_9: isize = -80isize;
    let mut tuple_4: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = -8isize;
    let mut isize_11: isize = 71isize;
    let mut tuple_5: (isize, isize) = (isize_11, isize_10);
    let mut isize_12: isize = -92isize;
    let mut isize_13: isize = -88isize;
    let mut tuple_6: (isize, isize) = (isize_13, isize_12);
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut result_1: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1763() {
    rusty_monitor::set_test_id(1763);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 65usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_3);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut gomokuerror_1_ref_0: &gomoku::GomokuError = &mut gomokuerror_1;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_1_ref_0);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut bool_4: bool = std::cmp::PartialEq::ne(cell_1_ref_0, cell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4565() {
    rusty_monitor::set_test_id(4565);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 65usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_3);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_2_ref_0: &connect_four::GameState = &mut gamestate_2;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_1_ref_0);
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut bool_4: bool = std::cmp::PartialEq::ne(cell_1_ref_0, cell_0_ref_0);
    let mut bool_5: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3566() {
    rusty_monitor::set_test_id(3566);
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut vec_1: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_1);
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 96usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut option_0: std::option::Option<reversi::Player> = std::option::Option::Some(player_2);
    let mut option_1: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut option_2: std::option::Option<reversi::Player> = std::option::Option::Some(player_3);
    let mut option_3: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut option_5: std::option::Option<reversi::Player> = std::option::Option::Some(player_4);
    let mut option_6: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_7: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_array_0: [std::option::Option<reversi::Player>; 8] = [option_7, option_6, option_5, option_4, option_3, option_2, option_1, option_0];
    let mut option_8: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_9: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_10: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut option_11: std::option::Option<reversi::Player> = std::option::Option::None;
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_6: reversi::Player = crate::reversi::Player::Player0;
    let mut player_7: reversi::Player = crate::reversi::Player::other(player_1);
    let mut player_8: reversi::Player = crate::reversi::Player::other(player_6);
    let mut player_9: gomoku::Player = crate::gomoku::Player::Player0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut player_7_ref_0: &reversi::Player = &mut player_7;
    let mut player_10: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut bool_3: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_107() {
    rusty_monitor::set_test_id(107);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut usize_0: usize = 3usize;
    let mut usize_1: usize = 67usize;
    let mut usize_2: usize = 36usize;
    let mut usize_3: usize = 95usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_4: usize = 90usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_4, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut usize_5: usize = 72usize;
    let mut bool_5: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_5, mine_adjacent: usize_5, is_revealed: bool_4, is_flagged: bool_3};
    let mut cell_2: crate::minesweeper::Cell = std::clone::Clone::clone(cell_0_ref_0);
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_3, usize_2, usize_1, usize_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3138() {
    rusty_monitor::set_test_id(3138);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut usize_0: usize = 35usize;
    let mut usize_1: usize = 65usize;
    let mut usize_2: usize = 31usize;
    let mut usize_3: usize = 40usize;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_3, usize_2, usize_1, usize_0);
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut player_4_ref_0: &reversi::Player = &mut player_4;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    panic!("From RustyUnit with love");
}
}