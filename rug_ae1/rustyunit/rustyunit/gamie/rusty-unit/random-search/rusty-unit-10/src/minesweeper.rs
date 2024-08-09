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
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use snafu::ErrorCompat;
	use std::iter::Iterator;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1698() {
    rusty_monitor::set_test_id(1698);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut player_1_ref_0: &connect_four::Player = &mut player_1;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_2_ref_0: &connect_four::Player = &mut player_2;
    let mut usize_0: usize = 77usize;
    let mut usize_1: usize = 55usize;
    let mut usize_2: usize = 44usize;
    let mut usize_3: usize = 32usize;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_0: std::option::Option<connect_four::Player> = std::option::Option::Some(player_3);
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut option_1: std::option::Option<connect_four::Player> = std::option::Option::Some(player_4);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Win(player_5);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut option_2: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_3: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut option_4: std::option::Option<connect_four::Player> = std::option::Option::None;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2344() {
    rusty_monitor::set_test_id(2344);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut usize_0: usize = 78usize;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_1: usize = 44usize;
    let mut usize_2: usize = 30usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_3: usize = 25usize;
    let mut usize_4: usize = 16usize;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_5: usize = 20usize;
    let mut usize_6: usize = 48usize;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_7: usize = 35usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_7, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut usize_8: usize = 99usize;
    let mut bool_5: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_5, mine_adjacent: usize_8, is_revealed: bool_4, is_flagged: bool_3};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut usize_9: usize = 77usize;
    let mut bool_8: bool = false;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut reversierror_2: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut reversierror_2_ref_0: &reversi::ReversiError = &mut reversierror_2;
    let mut reversierror_3: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_3_ref_0: &reversi::ReversiError = &mut reversierror_3;
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut result_1: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut gomoku_0: crate::gomoku::Gomoku = std::result::Result::unwrap(result_0);
    let mut cell_2: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_8, mine_adjacent: usize_9, is_revealed: bool_7, is_flagged: bool_6};
    let mut gomoku_0_ref_0: &crate::gomoku::Gomoku = &mut gomoku_0;
    let mut bool_9: bool = crate::gomoku::Gomoku::is_ended(gomoku_0_ref_0);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut cell_2_ref_0: &crate::minesweeper::Cell = &mut cell_2;
    let mut bool_10: bool = std::cmp::PartialEq::eq(cell_2_ref_0, cell_1_ref_0);
    let mut cell_3: crate::minesweeper::Cell = std::clone::Clone::clone(cell_0_ref_0);
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::other(player_4);
    let mut player_6: gomoku::Player = crate::gomoku::Player::other(player_1);
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut gamestate_6: minesweeper::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4627() {
    rusty_monitor::set_test_id(4627);
    let mut bool_0: bool = false;
    let mut usize_0: usize = 14usize;
    let mut usize_1: usize = 69usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_2: usize = 99usize;
    let mut usize_3: usize = 95usize;
    let mut usize_4: usize = 6usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut bool_1: bool = false;
    let mut usize_5: usize = 11usize;
    let mut usize_6: usize = 51usize;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2: reversi::Player = crate::reversi::Player::other(player_1);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut usize_7: usize = 75usize;
    let mut usize_8: usize = 13usize;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_3);
    let mut usize_9: usize = 92usize;
    let mut usize_10: usize = 16usize;
    let mut usize_11: usize = 41usize;
    let mut usize_12: usize = 48usize;
    let mut bool_2: bool = false;
    let mut usize_13: usize = 85usize;
    let mut usize_14: usize = 28usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_15: usize = 64usize;
    let mut usize_16: usize = 35usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_0, height: usize_16, width: usize_15, status: gamestate_1};
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut result_1: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut result_2: std::result::Result<bool, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::click(minesweeper_0_ref_0, usize_14, usize_13, bool_2);
    let mut result_3: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut gomoku_0: crate::gomoku::Gomoku = std::result::Result::unwrap(result_3);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_12, usize_11, usize_10, usize_9);
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_0_ref_0);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_1);
    let mut player_7: reversi::Player = crate::reversi::Player::Player0;
    let mut result_4: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4934() {
    rusty_monitor::set_test_id(4934);
    let mut usize_0: usize = 28usize;
    let mut usize_1: usize = 50usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 26usize;
    let mut usize_3: usize = 36usize;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut usize_4: usize = 56usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_4, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = false;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_3);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_5: usize = 6usize;
    let mut usize_6: usize = 96usize;
    let mut usize_7: usize = 97usize;
    let mut usize_8: usize = 40usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_4: bool = std::cmp::PartialEq::eq(minesweepererror_1_ref_0, minesweepererror_0_ref_0);
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut bool_5: bool = std::cmp::PartialEq::ne(cell_1_ref_0, cell_0_ref_0);
    let mut gamestate_2: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_864() {
    rusty_monitor::set_test_id(864);
    let mut usize_0: usize = 64usize;
    let mut usize_1: usize = 1usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut usize_2: usize = 98usize;
    let mut usize_3: usize = 94usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_0: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut usize_4: usize = 99usize;
    let mut usize_5: usize = 71usize;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut tictactoeerror_2: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_2_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_2;
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut bool_1: bool = std::cmp::PartialEq::eq(minesweepererror_2_ref_0, minesweepererror_1_ref_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_0_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_0_ref_0);
    let mut minesweepererror_3: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut gamestate_0: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3504() {
    rusty_monitor::set_test_id(3504);
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut usize_0: usize = 83usize;
    let mut usize_1: usize = 23usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 36usize;
    let mut usize_3: usize = 18usize;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_3: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    let mut player_5: reversi::Player = crate::reversi::Player::other(player_4);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_4: usize = 73usize;
    let mut bool_2: bool = true;
    let mut usize_5: usize = 83usize;
    let mut usize_6: usize = 17usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_7: usize = 44usize;
    let mut usize_8: usize = 60usize;
    let mut usize_9: usize = 84usize;
    let mut usize_10: usize = 75usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_11: usize = 44usize;
    let mut usize_12: usize = 91usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_0, height: usize_12, width: usize_11, status: gamestate_2};
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut result_0: std::result::Result<(), minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::click_unrevealed(minesweeper_0_ref_0, usize_10, usize_9);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut player_6: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_4, is_revealed: bool_1, is_flagged: bool_0};
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut player_7: reversi::Player = crate::reversi::Player::other(player_5);
    let mut player_8: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut player_9: gomoku::Player = crate::gomoku::Player::other(player_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4387() {
    rusty_monitor::set_test_id(4387);
    let mut usize_0: usize = 53usize;
    let mut isize_0: isize = 117isize;
    let mut isize_1: isize = -183isize;
    let mut isize_2: isize = -176isize;
    let mut isize_3: isize = -61isize;
    let mut tuple_0: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = 15isize;
    let mut isize_5: isize = 123isize;
    let mut tuple_1: (isize, isize) = (isize_5, isize_4);
    let mut isize_6: isize = 97isize;
    let mut isize_7: isize = -74isize;
    let mut tuple_2: (isize, isize) = (isize_7, isize_6);
    let mut isize_8: isize = -105isize;
    let mut isize_9: isize = 149isize;
    let mut tuple_3: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = 51isize;
    let mut isize_11: isize = 117isize;
    let mut tuple_4: (isize, isize) = (isize_11, isize_10);
    let mut isize_12: isize = 139isize;
    let mut isize_13: isize = -118isize;
    let mut tuple_5: (isize, isize) = (isize_13, isize_12);
    let mut isize_14: isize = 65isize;
    let mut isize_15: isize = -38isize;
    let mut tuple_6: (isize, isize) = (isize_15, isize_14);
    let mut isize_16: isize = -110isize;
    let mut isize_17: isize = 170isize;
    let mut tuple_7: (isize, isize) = (isize_17, isize_16);
    let mut tuple_array_0: [(isize, isize); 8] = [tuple_7, tuple_6, tuple_5, tuple_4, tuple_3, tuple_2, tuple_1, tuple_0];
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells {around: tuple_array_0, board_height: isize_1, board_width: isize_0, offset: usize_0};
    let mut adjacentcells_0_ref_0: &crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut usize_1: usize = 17usize;
    let mut usize_2: usize = 16usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_3: usize = 29usize;
    let mut usize_4: usize = 45usize;
    let mut usize_5: usize = 47usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_6: usize = 11usize;
    let mut usize_7: usize = 35usize;
    let mut usize_8: usize = 69usize;
    let mut usize_9: usize = 2usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_10: usize = 83usize;
    let mut usize_11: usize = 80usize;
    let mut usize_12: usize = 79usize;
    let mut usize_13: usize = 40usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_14: usize = 58usize;
    let mut usize_15: usize = 56usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_0, height: usize_15, width: usize_14, status: gamestate_3};
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut result_0: std::result::Result<(), minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::toggle_flag(minesweeper_0_ref_0, usize_9, usize_8);
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_4: minesweeper::GameState = std::clone::Clone::clone(gamestate_2_ref_0);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut adjacentcells_1: crate::minesweeper::AdjacentCells = std::clone::Clone::clone(adjacentcells_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3903() {
    rusty_monitor::set_test_id(3903);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut usize_0: usize = 6usize;
    let mut usize_1: usize = 60usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_2: usize = 44usize;
    let mut usize_3: usize = 30usize;
    let mut usize_4: usize = 31usize;
    let mut usize_5: usize = 89usize;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_6: usize = 7usize;
    let mut usize_7: usize = 27usize;
    let mut bool_0: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_0_ref_0);
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_3: minesweeper::GameState = std::clone::Clone::clone(gamestate_1_ref_0);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Win(player_1);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut bool_1: bool = std::cmp::PartialEq::eq(minesweepererror_1_ref_0, minesweepererror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4019() {
    rusty_monitor::set_test_id(4019);
    let mut usize_0: usize = 18usize;
    let mut usize_1: usize = 99usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_2: usize = 49usize;
    let mut usize_3: usize = 29usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_4: usize = 45usize;
    let mut usize_5: usize = 45usize;
    let mut usize_6: usize = 3usize;
    let mut usize_7: usize = 1usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_8: usize = 41usize;
    let mut usize_9: usize = 29usize;
    let mut usize_10: usize = 7usize;
    let mut usize_11: usize = 94usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut bool_0: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_1: bool = true;
    let mut usize_12: usize = 74usize;
    let mut usize_13: usize = 88usize;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_14: usize = 9usize;
    let mut usize_15: usize = 75usize;
    let mut usize_16: usize = 48usize;
    let mut usize_17: usize = 87usize;
    let mut usize_18: usize = 93usize;
    let mut usize_19: usize = 8usize;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_19, usize_18, usize_17, usize_16);
    let mut adjacentcells_0_ref_0: &crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut usize_20: usize = 71usize;
    let mut usize_21: usize = 97usize;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_22: usize = 77usize;
    let mut usize_23: usize = 18usize;
    let mut usize_24: usize = 95usize;
    let mut usize_25: usize = 6usize;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut usize_26: usize = 37usize;
    let mut usize_27: usize = 17usize;
    let mut vec_1: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_1, height: usize_27, width: usize_26, status: gamestate_5};
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut result_1: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut bool_2: bool = std::cmp::PartialEq::eq(minesweepererror_2_ref_0, minesweepererror_1_ref_0);
    let mut minesweepererror_3: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweeper_0_ref_0: &crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut cell_1: &crate::minesweeper::Cell = crate::minesweeper::Minesweeper::get(minesweeper_0_ref_0, usize_25, usize_24);
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut adjacentcells_1: crate::minesweeper::AdjacentCells = std::clone::Clone::clone(adjacentcells_0_ref_0);
    let mut minesweepererror_3_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_3;
    let mut cell_2: crate::minesweeper::Cell = std::clone::Clone::clone(cell_0_ref_0);
    let mut gamestate_6: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut minesweepererror_4: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_5: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut result_2: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut minesweepererror_4_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_4;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_4_ref_0);
    let mut gamestate_7: connect_four::GameState = crate::connect_four::GameState::InProgress;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2491() {
    rusty_monitor::set_test_id(2491);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut bool_0: bool = true;
    let mut usize_0: usize = 42usize;
    let mut usize_1: usize = 12usize;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 38usize;
    let mut usize_3: usize = 69usize;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_4: usize = 96usize;
    let mut usize_5: usize = 97usize;
    let mut bool_1: bool = false;
    let mut usize_6: usize = 18usize;
    let mut usize_7: usize = 75usize;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_8: usize = 81usize;
    let mut usize_9: usize = 97usize;
    let mut bool_2: bool = false;
    let mut usize_10: usize = 31usize;
    let mut usize_11: usize = 77usize;
    let mut gamestate_6: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_12: usize = 49usize;
    let mut usize_13: usize = 36usize;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_1);
    let mut gamestate_7: minesweeper::GameState = std::clone::Clone::clone(gamestate_2_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_814() {
    rusty_monitor::set_test_id(814);
    let mut usize_0: usize = 29usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_1: usize = 4usize;
    let mut usize_2: usize = 8usize;
    let mut usize_3: usize = 11usize;
    let mut usize_4: usize = 10usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut usize_5: usize = 12usize;
    let mut isize_0: isize = 62isize;
    let mut isize_1: isize = -36isize;
    let mut isize_2: isize = -39isize;
    let mut isize_3: isize = -88isize;
    let mut tuple_0: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = 133isize;
    let mut isize_5: isize = -75isize;
    let mut tuple_1: (isize, isize) = (isize_5, isize_4);
    let mut isize_6: isize = 40isize;
    let mut isize_7: isize = 168isize;
    let mut tuple_2: (isize, isize) = (isize_7, isize_6);
    let mut isize_8: isize = -76isize;
    let mut isize_9: isize = 157isize;
    let mut tuple_3: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = 26isize;
    let mut isize_11: isize = -106isize;
    let mut tuple_4: (isize, isize) = (isize_11, isize_10);
    let mut isize_12: isize = 104isize;
    let mut isize_13: isize = -269isize;
    let mut tuple_5: (isize, isize) = (isize_13, isize_12);
    let mut isize_14: isize = 31isize;
    let mut isize_15: isize = 72isize;
    let mut tuple_6: (isize, isize) = (isize_15, isize_14);
    let mut isize_16: isize = -51isize;
    let mut isize_17: isize = 80isize;
    let mut tuple_7: (isize, isize) = (isize_17, isize_16);
    let mut tuple_array_0: [(isize, isize); 8] = [tuple_7, tuple_6, tuple_5, tuple_4, tuple_3, tuple_2, tuple_1, tuple_0];
    let mut usize_6: usize = 29usize;
    let mut usize_7: usize = 16usize;
    let mut usize_8: usize = 23usize;
    let mut usize_9: usize = 32usize;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_9, usize_8, usize_7, usize_6);
    let mut adjacentcells_0_ref_0: &crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut usize_10: usize = 34usize;
    let mut usize_11: usize = 29usize;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_12: usize = 68usize;
    let mut usize_13: usize = 95usize;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_5_ref_0: &minesweeper::GameState = &mut gamestate_5;
    let mut gamestate_6: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_14: usize = 95usize;
    let mut usize_15: usize = 65usize;
    let mut gamestate_6_ref_0: &minesweeper::GameState = &mut gamestate_6;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_6_ref_0, gamestate_5_ref_0);
    let mut adjacentcells_1: crate::minesweeper::AdjacentCells = std::clone::Clone::clone(adjacentcells_0_ref_0);
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut player_1: connect_four::Player = crate::connect_four::Player::Player0;
    let mut adjacentcells_2: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells {around: tuple_array_0, board_height: isize_1, board_width: isize_0, offset: usize_5};
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut bool_1: bool = std::cmp::PartialEq::ne(gamestate_4_ref_0, gamestate_3_ref_0);
    let mut gomoku_0: crate::gomoku::Gomoku = std::result::Result::unwrap(result_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut gomoku_0_ref_0: &mut crate::gomoku::Gomoku = &mut gomoku_0;
    let mut result_1: std::result::Result<(), gomoku::GomokuError> = crate::gomoku::Gomoku::place(gomoku_0_ref_0, player_0, usize_4, usize_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_206() {
    rusty_monitor::set_test_id(206);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_1_ref_0: &tictactoe::GameState = &mut gamestate_1;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_1: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_2: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_3);
    let mut option_array_0: [std::option::Option<tictactoe::Player>; 3] = [option_2, option_1, option_0];
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_3: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_4);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut option_4: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_5);
    let mut option_5: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_1: [std::option::Option<tictactoe::Player>; 3] = [option_5, option_4, option_3];
    let mut option_6: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_7: std::option::Option<tictactoe::Player> = std::option::Option::Some(player_6);
    let mut option_8: std::option::Option<tictactoe::Player> = std::option::Option::None;
    let mut option_array_2: [std::option::Option<tictactoe::Player>; 3] = [option_8, option_7, option_6];
    let mut option_array_array_0: [[std::option::Option<tictactoe::Player>; 3]; 3] = [option_array_2, option_array_1, option_array_0];
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut bool_0: bool = std::cmp::PartialEq::eq(gamestate_4_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3610() {
    rusty_monitor::set_test_id(3610);
    let mut bool_0: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut usize_0: usize = 59usize;
    let mut bool_3: bool = true;
    let mut usize_1: usize = 23usize;
    let mut usize_2: usize = 12usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut usize_3: usize = 8usize;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_4: usize = 69usize;
    let mut usize_5: usize = 77usize;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut bool_4: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    let mut gamestate_4: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_3, mine_adjacent: usize_0, is_revealed: bool_2, is_flagged: bool_1};
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3459() {
    rusty_monitor::set_test_id(3459);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut player_1_ref_0: &reversi::Player = &mut player_1;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut player_3_ref_0: &reversi::Player = &mut player_3;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_3: reversi::GameState = crate::reversi::GameState::Win(player_5);
    let mut gamestate_3_ref_0: &reversi::GameState = &mut gamestate_3;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::Win(player_4);
    let mut gamestate_6: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(minesweepererror_1_ref_0, minesweepererror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1260() {
    rusty_monitor::set_test_id(1260);
    let mut usize_0: usize = 83usize;
    let mut usize_1: usize = 59usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut usize_2: usize = 14usize;
    let mut usize_3: usize = 64usize;
    let mut usize_4: usize = 55usize;
    let mut usize_5: usize = 94usize;
    let mut usize_6: usize = 86usize;
    let mut usize_7: usize = 31usize;
    let mut usize_8: usize = 16usize;
    let mut usize_9: usize = 31usize;
    let mut usize_10: usize = 29usize;
    let mut usize_11: usize = 48usize;
    let mut usize_12: usize = 20usize;
    let mut usize_13: usize = 58usize;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_13, usize_12, usize_11, usize_10);
    let mut adjacentcells_0_ref_0: &mut crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut usize_14: usize = 43usize;
    let mut usize_15: usize = 66usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_16: usize = 65usize;
    let mut usize_17: usize = 87usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_0, height: usize_17, width: usize_16, status: gamestate_0};
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut minesweeper_0_ref_0: &crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut cell_0: &crate::minesweeper::Cell = crate::minesweeper::Minesweeper::get(minesweeper_0_ref_0, usize_15, usize_14);
    let mut option_0: std::option::Option<usize> = std::iter::Iterator::next(adjacentcells_0_ref_0);
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_823() {
    rusty_monitor::set_test_id(823);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_0: usize = 24usize;
    let mut usize_1: usize = 6usize;
    let mut bool_0: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_1: bool = false;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_1);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 57usize;
    let mut usize_3: usize = 26usize;
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut usize_4: usize = 75usize;
    let mut usize_5: usize = 43usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut usize_6: usize = 75usize;
    let mut usize_7: usize = 34usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_8: usize = 24usize;
    let mut usize_9: usize = 6usize;
    let mut usize_10: usize = 67usize;
    let mut usize_11: usize = 50usize;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_1_ref_0);
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut reversi_0: crate::reversi::Reversi = std::result::Result::unwrap(result_0);
    let mut reversi_0_ref_0: &crate::reversi::Reversi = &mut reversi_0;
    let mut option_0: &std::option::Option<reversi::Player> = crate::reversi::Reversi::get(reversi_0_ref_0, usize_5, usize_4);
    let mut player_1_ref_0: &gomoku::Player = &mut player_1;
    let mut result_1: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut result_2: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut gamestate_5: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut bool_2: bool = std::cmp::PartialEq::ne(cell_1_ref_0, cell_0_ref_0);
    let mut player_4: reversi::Player = crate::reversi::Player::Player0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1103() {
    rusty_monitor::set_test_id(1103);
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut usize_0: usize = 55usize;
    let mut usize_1: usize = 24usize;
    let mut usize_2: usize = 43usize;
    let mut usize_3: usize = 53usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_4: usize = 28usize;
    let mut usize_5: usize = 70usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_6: usize = 60usize;
    let mut usize_7: usize = 94usize;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut usize_8: usize = 78usize;
    let mut usize_9: usize = 5usize;
    let mut usize_10: usize = 7usize;
    let mut usize_11: usize = 31usize;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_11, usize_10, usize_9, usize_8);
    let mut adjacentcells_0_ref_0: &mut crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut usize_12: usize = 62usize;
    let mut usize_13: usize = 9usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_14: usize = 38usize;
    let mut usize_15: usize = 0usize;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut option_0: std::option::Option<usize> = std::iter::Iterator::next(adjacentcells_0_ref_0);
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut tictactoeerror_1_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_1;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4747() {
    rusty_monitor::set_test_id(4747);
    let mut bool_0: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut usize_0: usize = 3usize;
    let mut bool_3: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_3, mine_adjacent: usize_0, is_revealed: bool_2, is_flagged: bool_1};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut usize_1: usize = 55usize;
    let mut usize_2: usize = 90usize;
    let mut usize_3: usize = 28usize;
    let mut usize_4: usize = 44usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut usize_5: usize = 61usize;
    let mut usize_6: usize = 42usize;
    let mut usize_7: usize = 66usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_1_ref_0: &gomoku::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = std::clone::Clone::clone(gamestate_0_ref_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut player_3: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut option_1: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_2_ref_0);
    let mut minesweepererror_3: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut cell_2: crate::minesweeper::Cell = std::clone::Clone::clone(cell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_624() {
    rusty_monitor::set_test_id(624);
    let mut usize_0: usize = 46usize;
    let mut isize_0: isize = 219isize;
    let mut isize_1: isize = -102isize;
    let mut isize_2: isize = -76isize;
    let mut isize_3: isize = -87isize;
    let mut tuple_0: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = -43isize;
    let mut isize_5: isize = -90isize;
    let mut tuple_1: (isize, isize) = (isize_5, isize_4);
    let mut isize_6: isize = 176isize;
    let mut isize_7: isize = -24isize;
    let mut tuple_2: (isize, isize) = (isize_7, isize_6);
    let mut isize_8: isize = 115isize;
    let mut isize_9: isize = 124isize;
    let mut tuple_3: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = -236isize;
    let mut isize_11: isize = 120isize;
    let mut tuple_4: (isize, isize) = (isize_11, isize_10);
    let mut isize_12: isize = 149isize;
    let mut isize_13: isize = -32isize;
    let mut tuple_5: (isize, isize) = (isize_13, isize_12);
    let mut isize_14: isize = -1isize;
    let mut isize_15: isize = 44isize;
    let mut tuple_6: (isize, isize) = (isize_15, isize_14);
    let mut isize_16: isize = 29isize;
    let mut isize_17: isize = 174isize;
    let mut tuple_7: (isize, isize) = (isize_17, isize_16);
    let mut tuple_array_0: [(isize, isize); 8] = [tuple_7, tuple_6, tuple_5, tuple_4, tuple_3, tuple_2, tuple_1, tuple_0];
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells {around: tuple_array_0, board_height: isize_1, board_width: isize_0, offset: usize_0};
    let mut adjacentcells_0_ref_0: &crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut adjacentcells_1: crate::minesweeper::AdjacentCells = std::clone::Clone::clone(adjacentcells_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4730() {
    rusty_monitor::set_test_id(4730);
    let mut bool_0: bool = false;
    let mut usize_0: usize = 67usize;
    let mut usize_1: usize = 69usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 28usize;
    let mut usize_3: usize = 72usize;
    let mut usize_4: usize = 54usize;
    let mut usize_5: usize = 49usize;
    let mut usize_6: usize = 5usize;
    let mut usize_7: usize = 42usize;
    let mut bool_1: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_1);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut usize_8: usize = 54usize;
    let mut bool_4: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_4, mine_adjacent: usize_8, is_revealed: bool_3, is_flagged: bool_2};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut bool_5: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut bool_6: bool = std::cmp::PartialEq::eq(cell_1_ref_0, cell_0_ref_0);
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_7, usize_6, usize_5, usize_4);
    let mut result_0: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_54() {
    rusty_monitor::set_test_id(54);
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut usize_0: usize = 8usize;
    let mut usize_1: usize = 86usize;
    let mut usize_2: usize = 82usize;
    let mut usize_3: usize = 15usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_4: usize = 26usize;
    let mut usize_5: usize = 63usize;
    let mut usize_6: usize = 27usize;
    let mut usize_7: usize = 52usize;
    let mut usize_8: usize = 86usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_9: usize = 97usize;
    let mut usize_10: usize = 23usize;
    let mut bool_0: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut cell_1: crate::minesweeper::Cell = std::clone::Clone::clone(cell_0_ref_0);
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_1_ref_0);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut option_1: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_1_ref_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_3, usize_2, usize_1, usize_0);
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_339() {
    rusty_monitor::set_test_id(339);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_2: gomoku::Player = crate::gomoku::Player::other(player_1);
    let mut usize_0: usize = 22usize;
    let mut usize_1: usize = 21usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 91usize;
    let mut usize_3: usize = 77usize;
    let mut bool_0: bool = true;
    let mut usize_4: usize = 48usize;
    let mut usize_5: usize = 31usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_6: usize = 28usize;
    let mut usize_7: usize = 52usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_8: usize = 55usize;
    let mut usize_9: usize = 69usize;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut usize_10: usize = 27usize;
    let mut usize_11: usize = 92usize;
    let mut vec_1: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_1, height: usize_11, width: usize_10, status: gamestate_3};
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut result_0: std::result::Result<bool, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::click_revealed(minesweeper_0_ref_0, usize_5, usize_4, bool_0);
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut result_1: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut gamestate_6: gomoku::GameState = crate::gomoku::GameState::Win(player_2);
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4083() {
    rusty_monitor::set_test_id(4083);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut player_1_ref_0: &reversi::Player = &mut player_1;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut player_2_ref_0: &reversi::Player = &mut player_2;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_0: usize = 63usize;
    let mut usize_1: usize = 24usize;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player0;
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut gamestate_1: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut gamestate_1_ref_0: &connect_four::GameState = &mut gamestate_1;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_476() {
    rusty_monitor::set_test_id(476);
    let mut usize_0: usize = 37usize;
    let mut usize_1: usize = 46usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 90usize;
    let mut usize_3: usize = 61usize;
    let mut usize_4: usize = 90usize;
    let mut usize_5: usize = 85usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_6: usize = 32usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_6, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_3);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut bool_4: bool = false;
    let mut cell_2: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_4);
    let mut cell_2_ref_0: &crate::minesweeper::Cell = &mut cell_2;
    let mut bool_5: bool = true;
    let mut cell_3: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_5);
    let mut cell_3_ref_0: &crate::minesweeper::Cell = &mut cell_3;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut cell_4: crate::minesweeper::Cell = std::clone::Clone::clone(cell_3_ref_0);
    let mut bool_6: bool = std::cmp::PartialEq::ne(cell_2_ref_0, cell_1_ref_0);
    let mut gamestate_1: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut cell_4_ref_0: &crate::minesweeper::Cell = &mut cell_4;
    let mut cell_5: crate::minesweeper::Cell = std::clone::Clone::clone(cell_4_ref_0);
    let mut player_2: gomoku::Player = crate::gomoku::Player::other(player_1);
    let mut cell_5_ref_0: &crate::minesweeper::Cell = &mut cell_5;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_5_ref_0);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3885() {
    rusty_monitor::set_test_id(3885);
    let mut usize_0: usize = 85usize;
    let mut isize_0: isize = 154isize;
    let mut isize_1: isize = 4isize;
    let mut isize_2: isize = -91isize;
    let mut isize_3: isize = -243isize;
    let mut tuple_0: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = 119isize;
    let mut isize_5: isize = 80isize;
    let mut tuple_1: (isize, isize) = (isize_5, isize_4);
    let mut isize_6: isize = 165isize;
    let mut isize_7: isize = -126isize;
    let mut tuple_2: (isize, isize) = (isize_7, isize_6);
    let mut isize_8: isize = 40isize;
    let mut isize_9: isize = -11isize;
    let mut tuple_3: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = 179isize;
    let mut isize_11: isize = -47isize;
    let mut tuple_4: (isize, isize) = (isize_11, isize_10);
    let mut isize_12: isize = -68isize;
    let mut isize_13: isize = 94isize;
    let mut tuple_5: (isize, isize) = (isize_13, isize_12);
    let mut isize_14: isize = -65isize;
    let mut isize_15: isize = -133isize;
    let mut tuple_6: (isize, isize) = (isize_15, isize_14);
    let mut isize_16: isize = 262isize;
    let mut isize_17: isize = 141isize;
    let mut tuple_7: (isize, isize) = (isize_17, isize_16);
    let mut tuple_array_0: [(isize, isize); 8] = [tuple_7, tuple_6, tuple_5, tuple_4, tuple_3, tuple_2, tuple_1, tuple_0];
    let mut usize_1: usize = 15usize;
    let mut usize_2: usize = 54usize;
    let mut usize_3: usize = 41usize;
    let mut bool_0: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_4: usize = 32usize;
    let mut usize_5: usize = 66usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_6: usize = 47usize;
    let mut usize_7: usize = 33usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_8: usize = 31usize;
    let mut usize_9: usize = 3usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_0, height: usize_9, width: usize_8, status: gamestate_2};
    let mut tuple_8: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_0_ref_0);
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    crate::minesweeper::Minesweeper::check_state(minesweeper_0_ref_0);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells {around: tuple_array_0, board_height: isize_1, board_width: isize_0, offset: usize_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4264() {
    rusty_monitor::set_test_id(4264);
    let mut gamestate_0: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_0_ref_0: &tictactoe::GameState = &mut gamestate_0;
    let mut usize_0: usize = 65usize;
    let mut usize_1: usize = 36usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut usize_2: usize = 40usize;
    let mut usize_3: usize = 98usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_4: usize = 89usize;
    let mut usize_5: usize = 33usize;
    let mut usize_6: usize = 76usize;
    let mut isize_0: isize = 15isize;
    let mut isize_1: isize = 89isize;
    let mut isize_2: isize = -17isize;
    let mut isize_3: isize = 79isize;
    let mut tuple_0: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = -129isize;
    let mut isize_5: isize = 10isize;
    let mut tuple_1: (isize, isize) = (isize_5, isize_4);
    let mut isize_6: isize = -64isize;
    let mut isize_7: isize = -3isize;
    let mut tuple_2: (isize, isize) = (isize_7, isize_6);
    let mut isize_8: isize = 103isize;
    let mut isize_9: isize = 79isize;
    let mut tuple_3: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = -72isize;
    let mut isize_11: isize = -23isize;
    let mut tuple_4: (isize, isize) = (isize_11, isize_10);
    let mut isize_12: isize = -17isize;
    let mut isize_13: isize = -142isize;
    let mut tuple_5: (isize, isize) = (isize_13, isize_12);
    let mut isize_14: isize = 13isize;
    let mut isize_15: isize = -92isize;
    let mut tuple_6: (isize, isize) = (isize_15, isize_14);
    let mut isize_16: isize = -127isize;
    let mut isize_17: isize = -101isize;
    let mut tuple_7: (isize, isize) = (isize_17, isize_16);
    let mut tuple_array_0: [(isize, isize); 8] = [tuple_7, tuple_6, tuple_5, tuple_4, tuple_3, tuple_2, tuple_1, tuple_0];
    let mut bool_0: bool = true;
    let mut usize_7: usize = 45usize;
    let mut usize_8: usize = 38usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_9: usize = 78usize;
    let mut usize_10: usize = 89usize;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_11: usize = 70usize;
    let mut usize_12: usize = 44usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_0, height: usize_12, width: usize_11, status: gamestate_3};
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells {around: tuple_array_0, board_height: isize_1, board_width: isize_0, offset: usize_6};
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut minesweeper_0_ref_0: &crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut gamestate_5: &minesweeper::GameState = crate::minesweeper::Minesweeper::status(minesweeper_0_ref_0);
    let mut gamestate_4_ref_0: &tictactoe::GameState = &mut gamestate_4;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_854() {
    rusty_monitor::set_test_id(854);
    let mut usize_0: usize = 1usize;
    let mut usize_1: usize = 13usize;
    let mut usize_2: usize = 46usize;
    let mut usize_3: usize = 62usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut player_2_ref_0: &reversi::Player = &mut player_2;
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut player_3_ref_0: &reversi::Player = &mut player_3;
    let mut player_4: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_0: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_0_ref_0: &connect_four::GameState = &mut gamestate_0;
    let mut usize_4: usize = 75usize;
    let mut usize_5: usize = 65usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_6: usize = 2usize;
    let mut usize_7: usize = 83usize;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::Win(player_1);
    let mut player_6: connect_four::Player = crate::connect_four::Player::Player1;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_1_ref_0);
    let mut result_1: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut bool_0: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 19usize;
    let mut usize_1: usize = 36usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 11usize;
    let mut usize_3: usize = 80usize;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut usize_4: usize = 27usize;
    let mut bool_4: bool = false;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player1;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut player_2_ref_0: &gomoku::Player = &mut player_2;
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut bool_5: bool = std::cmp::PartialEq::eq(minesweepererror_2_ref_0, minesweepererror_1_ref_0);
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_4, mine_adjacent: usize_4, is_revealed: bool_3, is_flagged: bool_2};
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut bool_6: bool = std::cmp::PartialEq::eq(cell_1_ref_0, cell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3634() {
    rusty_monitor::set_test_id(3634);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut usize_0: usize = 79usize;
    let mut usize_1: usize = 20usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 54usize;
    let mut usize_3: usize = 36usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut bool_0: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_4: usize = 7usize;
    let mut usize_5: usize = 45usize;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_6: usize = 1usize;
    let mut usize_7: usize = 12usize;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_5_ref_0: &minesweeper::GameState = &mut gamestate_5;
    let mut gamestate_6: minesweeper::GameState = std::clone::Clone::clone(gamestate_5_ref_0);
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_4_ref_0);
    let mut cell_1: crate::minesweeper::Cell = std::clone::Clone::clone(cell_0_ref_0);
    let mut gamestate_7: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player0;
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}
}