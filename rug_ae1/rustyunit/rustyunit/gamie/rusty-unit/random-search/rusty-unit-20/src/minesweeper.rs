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
fn rusty_test_1728() {
    rusty_monitor::set_test_id(1728);
    let mut usize_0: usize = 53usize;
    let mut usize_1: usize = 21usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut usize_2: usize = 55usize;
    let mut usize_3: usize = 55usize;
    let mut usize_4: usize = 32usize;
    let mut usize_5: usize = 41usize;
    let mut bool_0: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player1;
    let mut usize_6: usize = 81usize;
    let mut usize_7: usize = 71usize;
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_6: gomoku::Player = crate::gomoku::Player::other(player_5);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut usize_8: usize = 31usize;
    let mut usize_9: usize = 39usize;
    let mut player_7: gomoku::Player = crate::gomoku::Player::Player0;
    let mut usize_10: usize = 24usize;
    let mut usize_11: usize = 4usize;
    let mut usize_12: usize = 88usize;
    let mut usize_13: usize = 10usize;
    let mut player_8: reversi::Player = crate::reversi::Player::Player1;
    let mut player_9: reversi::Player = crate::reversi::Player::other(player_8);
    let mut player_10: reversi::Player = crate::reversi::Player::other(player_9);
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_13, usize_12, usize_11, usize_10);
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_0_ref_0);
    let mut result_1: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut gomoku_0: crate::gomoku::Gomoku = std::result::Result::unwrap(result_0);
    let mut player_11: gomoku::Player = crate::gomoku::Player::other(player_4);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut bool_1: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
    let mut player_12: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut player_13: tictactoe::Player = crate::tictactoe::Player::other(player_2);
    let mut cell_1: crate::minesweeper::Cell = std::clone::Clone::clone(cell_0_ref_0);
    let mut adjacentcells_1: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_5, usize_4, usize_3, usize_2);
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut gomoku_0_ref_0: &mut crate::gomoku::Gomoku = &mut gomoku_0;
    let mut result_2: std::result::Result<(), gomoku::GomokuError> = crate::gomoku::Gomoku::place(gomoku_0_ref_0, player_0, usize_1, usize_0);
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::InProgress;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1007() {
    rusty_monitor::set_test_id(1007);
    let mut usize_0: usize = 29usize;
    let mut usize_1: usize = 60usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 90usize;
    let mut usize_3: usize = 10usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_4: usize = 70usize;
    let mut usize_5: usize = 38usize;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut usize_6: usize = 71usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_6, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_7: usize = 55usize;
    let mut usize_8: usize = 97usize;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut usize_9: usize = 25usize;
    let mut usize_10: usize = 12usize;
    let mut usize_11: usize = 63usize;
    let mut usize_12: usize = 24usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_13: usize = 35usize;
    let mut usize_14: usize = 17usize;
    let mut bool_3: bool = true;
    let mut usize_15: usize = 25usize;
    let mut usize_16: usize = 51usize;
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_17: usize = 14usize;
    let mut usize_18: usize = 16usize;
    let mut gamestate_6: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_19: usize = 76usize;
    let mut usize_20: usize = 27usize;
    let mut usize_21: usize = 81usize;
    let mut usize_22: usize = 26usize;
    let mut gamestate_7: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_23: usize = 77usize;
    let mut usize_24: usize = 89usize;
    let mut player_1: reversi::Player = crate::reversi::Player::Player1;
    let mut usize_25: usize = 4usize;
    let mut usize_26: usize = 84usize;
    let mut usize_27: usize = 63usize;
    let mut usize_28: usize = 84usize;
    let mut bool_4: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_4);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut usize_29: usize = 88usize;
    let mut usize_30: usize = 60usize;
    let mut gamestate_8: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_8_ref_0: &minesweeper::GameState = &mut gamestate_8;
    let mut gamestate_9: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_31: usize = 16usize;
    let mut usize_32: usize = 47usize;
    let mut gamestate_9_ref_0: &minesweeper::GameState = &mut gamestate_9;
    let mut bool_5: bool = std::cmp::PartialEq::ne(gamestate_9_ref_0, gamestate_8_ref_0);
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut cell_2: crate::minesweeper::Cell = std::clone::Clone::clone(cell_1_ref_0);
    let mut gomoku_0: crate::gomoku::Gomoku = std::result::Result::unwrap(result_0);
    let mut gamestate_10: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut gomoku_0_ref_0: &crate::gomoku::Gomoku = &mut gomoku_0;
    let mut gamestate_11: &gomoku::GameState = crate::gomoku::Gomoku::status(gomoku_0_ref_0);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_12: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_2);
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut gamestate_13: minesweeper::GameState = std::clone::Clone::clone(gamestate_3_ref_0);
    let mut cell_2_ref_0: &crate::minesweeper::Cell = &mut cell_2;
    let mut bool_6: bool = std::cmp::PartialEq::eq(cell_2_ref_0, cell_0_ref_0);
    let mut player_1_ref_0: &reversi::Player = &mut player_1;
    let mut result_1: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut player_5: gomoku::Player = crate::gomoku::Player::Player0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3672() {
    rusty_monitor::set_test_id(3672);
    let mut usize_0: usize = 67usize;
    let mut usize_1: usize = 64usize;
    let mut usize_2: usize = 33usize;
    let mut usize_3: usize = 2usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_4: usize = 98usize;
    let mut usize_5: usize = 65usize;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_6: usize = 51usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_6, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = false;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_3);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut usize_7: usize = 72usize;
    let mut usize_8: usize = 66usize;
    let mut usize_9: usize = 28usize;
    let mut usize_10: usize = 66usize;
    let mut u64_0: u64 = 33u64;
    let mut u64_1: u64 = 6u64;
    let mut steprng_0: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_0;
    let mut usize_11: usize = 92usize;
    let mut usize_12: usize = 16usize;
    let mut usize_13: usize = 60usize;
    let mut result_0: std::result::Result<crate::minesweeper::Minesweeper, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::new(usize_13, usize_12, usize_11, steprng_0_ref_0);
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_10, usize_9, usize_8, usize_7);
    let mut adjacentcells_0_ref_0: &crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut adjacentcells_1: crate::minesweeper::AdjacentCells = std::clone::Clone::clone(adjacentcells_0_ref_0);
    let mut minesweeper_0: crate::minesweeper::Minesweeper = std::result::Result::unwrap(result_0);
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut bool_4: bool = std::cmp::PartialEq::eq(cell_1_ref_0, cell_0_ref_0);
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1383() {
    rusty_monitor::set_test_id(1383);
    let mut usize_0: usize = 47usize;
    let mut usize_1: usize = 40usize;
    let mut usize_2: usize = 9usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut usize_3: usize = 89usize;
    let mut usize_4: usize = 89usize;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_2_ref_0: &gomoku::Player = &mut player_2;
    let mut gamestate_0: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_0_ref_0: &gomoku::GameState = &mut gamestate_0;
    let mut bool_0: bool = false;
    let mut usize_5: usize = 89usize;
    let mut usize_6: usize = 13usize;
    let mut usize_7: usize = 14usize;
    let mut usize_8: usize = 49usize;
    let mut usize_9: usize = 20usize;
    let mut usize_10: usize = 60usize;
    let mut usize_11: usize = 7usize;
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_4: gomoku::Player = crate::gomoku::Player::other(player_3);
    let mut player_4_ref_0: &gomoku::Player = &mut player_4;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_1: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut player_6: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1048() {
    rusty_monitor::set_test_id(1048);
    let mut usize_0: usize = 10usize;
    let mut usize_1: usize = 87usize;
    let mut usize_2: usize = 84usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_3: usize = 43usize;
    let mut usize_4: usize = 7usize;
    let mut usize_5: usize = 73usize;
    let mut usize_6: usize = 97usize;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_7: usize = 37usize;
    let mut usize_8: usize = 22usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_5: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player0;
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut gamestate_6: minesweeper::GameState = std::clone::Clone::clone(gamestate_4_ref_0);
    let mut gamestate_7: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_8: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_9: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut player_1_ref_0: &gomoku::Player = &mut player_1;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3750() {
    rusty_monitor::set_test_id(3750);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_0: usize = 13usize;
    let mut usize_1: usize = 37usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut usize_2: usize = 89usize;
    let mut usize_3: usize = 64usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_4: usize = 48usize;
    let mut usize_5: usize = 92usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_6: usize = 49usize;
    let mut usize_7: usize = 40usize;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_5_ref_0: &minesweeper::GameState = &mut gamestate_5;
    let mut bool_0: bool = std::cmp::PartialEq::ne(gamestate_5_ref_0, gamestate_4_ref_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut gamestate_6: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_3_ref_0, gamestate_2_ref_0);
    let mut player_3: gomoku::Player = crate::gomoku::Player::other(player_1);
    let mut tictactoeerror_1: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2479() {
    rusty_monitor::set_test_id(2479);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 30usize;
    let mut bool_2: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut usize_1: usize = 18usize;
    let mut bool_5: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_5, mine_adjacent: usize_1, is_revealed: bool_4, is_flagged: bool_3};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut usize_2: usize = 70usize;
    let mut isize_0: isize = -234isize;
    let mut isize_1: isize = 75isize;
    let mut isize_2: isize = 26isize;
    let mut isize_3: isize = -58isize;
    let mut tuple_0: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = 2isize;
    let mut isize_5: isize = -120isize;
    let mut tuple_1: (isize, isize) = (isize_5, isize_4);
    let mut isize_6: isize = 27isize;
    let mut isize_7: isize = 11isize;
    let mut tuple_2: (isize, isize) = (isize_7, isize_6);
    let mut isize_8: isize = 87isize;
    let mut isize_9: isize = -52isize;
    let mut tuple_3: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = -141isize;
    let mut isize_11: isize = -33isize;
    let mut tuple_4: (isize, isize) = (isize_11, isize_10);
    let mut isize_12: isize = -17isize;
    let mut isize_13: isize = 3isize;
    let mut tuple_5: (isize, isize) = (isize_13, isize_12);
    let mut isize_14: isize = -34isize;
    let mut isize_15: isize = -95isize;
    let mut tuple_6: (isize, isize) = (isize_15, isize_14);
    let mut isize_16: isize = -19isize;
    let mut isize_17: isize = 5isize;
    let mut tuple_7: (isize, isize) = (isize_17, isize_16);
    let mut tuple_array_0: [(isize, isize); 8] = [tuple_7, tuple_6, tuple_5, tuple_4, tuple_3, tuple_2, tuple_1, tuple_0];
    let mut usize_3: usize = 67usize;
    let mut usize_4: usize = 88usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_0: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_0_ref_0: &reversi::GameState = &mut gamestate_0;
    let mut usize_5: usize = 34usize;
    let mut usize_6: usize = 55usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_7: usize = 77usize;
    let mut usize_8: usize = 2usize;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut usize_9: usize = 93usize;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut usize_10: usize = 3usize;
    let mut bool_11: bool = true;
    let mut cell_2: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_11, mine_adjacent: usize_10, is_revealed: bool_10, is_flagged: bool_9};
    let mut cell_2_ref_0: &crate::minesweeper::Cell = &mut cell_2;
    let mut tuple_8: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_2_ref_0);
    let mut cell_3: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_8, mine_adjacent: usize_9, is_revealed: bool_7, is_flagged: bool_6};
    let mut gamestate_2: reversi::GameState = crate::reversi::GameState::Win(player_1);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut result_1: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2_ref_0: &reversi::GameState = &mut gamestate_2;
    let mut gomoku_0: crate::gomoku::Gomoku = std::result::Result::unwrap(result_1);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut tuple_9: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_0_ref_0);
    let mut gomoku_0_ref_0: &crate::gomoku::Gomoku = &mut gomoku_0;
    let mut player_2: gomoku::Player = crate::gomoku::Gomoku::get_next_player(gomoku_0_ref_0);
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells {around: tuple_array_0, board_height: isize_1, board_width: isize_0, offset: usize_2};
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::Win(player_0);
    let mut bool_12: bool = std::cmp::PartialEq::ne(cell_1_ref_0, cell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2900() {
    rusty_monitor::set_test_id(2900);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_0: usize = 31usize;
    let mut usize_1: usize = 30usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_2: usize = 79usize;
    let mut usize_3: usize = 94usize;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_4: usize = 84usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_4, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = false;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_3);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut bool_4: bool = std::cmp::PartialEq::ne(cell_1_ref_0, cell_0_ref_0);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut reversierror_0_ref_0: &reversi::ReversiError = &mut reversierror_0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut connectfourerror_1: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::GameEnded;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4730() {
    rusty_monitor::set_test_id(4730);
    let mut usize_0: usize = 29usize;
    let mut isize_0: isize = 28isize;
    let mut isize_1: isize = -247isize;
    let mut isize_2: isize = -82isize;
    let mut isize_3: isize = 68isize;
    let mut tuple_0: (isize, isize) = (isize_3, isize_2);
    let mut isize_4: isize = 31isize;
    let mut isize_5: isize = 51isize;
    let mut tuple_1: (isize, isize) = (isize_5, isize_4);
    let mut isize_6: isize = 27isize;
    let mut isize_7: isize = 41isize;
    let mut tuple_2: (isize, isize) = (isize_7, isize_6);
    let mut isize_8: isize = -86isize;
    let mut isize_9: isize = 18isize;
    let mut tuple_3: (isize, isize) = (isize_9, isize_8);
    let mut isize_10: isize = 158isize;
    let mut isize_11: isize = 4isize;
    let mut tuple_4: (isize, isize) = (isize_11, isize_10);
    let mut isize_12: isize = -35isize;
    let mut isize_13: isize = -31isize;
    let mut tuple_5: (isize, isize) = (isize_13, isize_12);
    let mut isize_14: isize = -37isize;
    let mut isize_15: isize = -57isize;
    let mut tuple_6: (isize, isize) = (isize_15, isize_14);
    let mut isize_16: isize = -16isize;
    let mut isize_17: isize = 92isize;
    let mut tuple_7: (isize, isize) = (isize_17, isize_16);
    let mut tuple_array_0: [(isize, isize); 8] = [tuple_7, tuple_6, tuple_5, tuple_4, tuple_3, tuple_2, tuple_1, tuple_0];
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells {around: tuple_array_0, board_height: isize_1, board_width: isize_0, offset: usize_0};
    let mut adjacentcells_0_ref_0: &mut crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut option_0: std::option::Option<usize> = std::iter::Iterator::next(adjacentcells_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4130() {
    rusty_monitor::set_test_id(4130);
    let mut usize_0: usize = 35usize;
    let mut usize_1: usize = 96usize;
    let mut usize_2: usize = 44usize;
    let mut usize_3: usize = 31usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_4: usize = 78usize;
    let mut usize_5: usize = 54usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_6: usize = 42usize;
    let mut usize_7: usize = 20usize;
    let mut usize_8: usize = 3usize;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_1: connect_four::Player = crate::connect_four::Player::other(player_0);
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(minesweepererror_2_ref_0, minesweepererror_1_ref_0);
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_1_ref_0);
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::InProgress;
    let mut gomoku_0: crate::gomoku::Gomoku = std::result::Result::unwrap(result_0);
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gomoku_0_ref_0: &crate::gomoku::Gomoku = &mut gomoku_0;
    let mut bool_2: bool = crate::gomoku::Gomoku::is_ended(gomoku_0_ref_0);
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::ColumnFilled;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_3, usize_2, usize_1, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_517() {
    rusty_monitor::set_test_id(517);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_0: usize = 97usize;
    let mut usize_1: usize = 47usize;
    let mut usize_2: usize = 23usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_3: usize = 97usize;
    let mut usize_4: usize = 61usize;
    let mut usize_5: usize = 39usize;
    let mut usize_6: usize = 65usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut player_3: connect_four::Player = crate::connect_four::Player::Player0;
    let mut player_4: connect_four::Player = crate::connect_four::Player::other(player_3);
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_7: usize = 26usize;
    let mut usize_8: usize = 77usize;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(minesweepererror_2_ref_0, minesweepererror_1_ref_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut player_5: connect_four::Player = crate::connect_four::Player::other(player_4);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_0_ref_0);
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_2);
    let mut gamestate_4: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_5: reversi::GameState = crate::reversi::GameState::Tie;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1464() {
    rusty_monitor::set_test_id(1464);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_1: reversi::GameState = crate::reversi::GameState::Win(player_0);
    let mut gamestate_1_ref_0: &reversi::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_0: usize = 98usize;
    let mut usize_1: usize = 78usize;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut usize_2: usize = 81usize;
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_3: reversi::Player = crate::reversi::Player::Player1;
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player1;
    let mut player_6: connect_four::Player = crate::connect_four::Player::other(player_5);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_5: connect_four::GameState = crate::connect_four::GameState::Win(player_6);
    let mut player_7: reversi::Player = crate::reversi::Player::other(player_4);
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut gamestate_6: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut player_7_ref_0: &reversi::Player = &mut player_7;
    let mut gamestate_5_ref_0: &connect_four::GameState = &mut gamestate_5;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut gamestate_7: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut player_8: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_4_ref_0: &reversi::GameState = &mut gamestate_4;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut player_8_ref_0: &connect_four::Player = &mut player_8;
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4065() {
    rusty_monitor::set_test_id(4065);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_0: usize = 1usize;
    let mut usize_1: usize = 57usize;
    let mut usize_2: usize = 36usize;
    let mut usize_3: usize = 2usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player1;
    let mut bool_0: bool = false;
    let mut usize_4: usize = 10usize;
    let mut usize_5: usize = 27usize;
    let mut usize_6: usize = 50usize;
    let mut usize_7: usize = 73usize;
    let mut usize_8: usize = 14usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_9: usize = 79usize;
    let mut usize_10: usize = 25usize;
    let mut usize_11: usize = 47usize;
    let mut usize_12: usize = 45usize;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut usize_13: usize = 91usize;
    let mut usize_14: usize = 58usize;
    let mut usize_15: usize = 98usize;
    let mut usize_16: usize = 48usize;
    let mut usize_17: usize = 9usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_18: usize = 99usize;
    let mut usize_19: usize = 10usize;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_20: usize = 94usize;
    let mut usize_21: usize = 24usize;
    let mut usize_22: usize = 28usize;
    let mut usize_23: usize = 67usize;
    let mut usize_24: usize = 85usize;
    let mut usize_25: usize = 76usize;
    let mut usize_26: usize = 40usize;
    let mut usize_27: usize = 57usize;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_28: usize = 88usize;
    let mut usize_29: usize = 83usize;
    let mut player_2: reversi::Player = crate::reversi::Player::Player0;
    let mut player_3: reversi::Player = crate::reversi::Player::other(player_2);
    let mut gamestate_5: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_30: usize = 88usize;
    let mut usize_31: usize = 25usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_0, height: usize_31, width: usize_30, status: gamestate_5};
    let mut player_4: reversi::Player = crate::reversi::Player::other(player_3);
    let mut gamestate_6: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut gamestate_7: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut result_1: std::result::Result<(), minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::toggle_flag(minesweeper_0_ref_0, usize_25, usize_24);
    let mut gamestate_8: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut gamestate_9: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut result_2: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::GameEnded;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut gamestate_6_ref_0: &connect_four::GameState = &mut gamestate_6;
    let mut player_5: connect_four::Player = crate::connect_four::Player::Player0;
    let mut gamestate_10: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_11: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut gomokuerror_0_ref_0: &gomoku::GomokuError = &mut gomokuerror_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_216() {
    rusty_monitor::set_test_id(216);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut usize_0: usize = 17usize;
    let mut usize_1: usize = 61usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 23usize;
    let mut usize_3: usize = 16usize;
    let mut bool_0: bool = false;
    let mut usize_4: usize = 68usize;
    let mut usize_5: usize = 41usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut player_0_ref_0: &tictactoe::Player = &mut player_0;
    let mut usize_6: usize = 14usize;
    let mut usize_7: usize = 42usize;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut usize_8: usize = 89usize;
    let mut bool_3: bool = false;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_2_ref_0: &tictactoe::GameState = &mut gamestate_2;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_3, mine_adjacent: usize_8, is_revealed: bool_2, is_flagged: bool_1};
    let mut result_0: std::result::Result<crate::gomoku::Gomoku, std::convert::Infallible> = crate::gomoku::Gomoku::new();
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_0_ref_0);
    let mut player_2: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_4: connect_four::GameState = crate::connect_four::GameState::Tie;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1644() {
    rusty_monitor::set_test_id(1644);
    let mut usize_0: usize = 5usize;
    let mut usize_1: usize = 59usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut usize_2: usize = 5usize;
    let mut usize_3: usize = 91usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_4: usize = 10usize;
    let mut usize_5: usize = 95usize;
    let mut bool_0: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_1: bool = false;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_1);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_6: usize = 14usize;
    let mut usize_7: usize = 49usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_1);
    let mut gamestate_3_ref_0: &tictactoe::GameState = &mut gamestate_3;
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_0, height: usize_7, width: usize_6, status: gamestate_2};
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::ne(cell_1_ref_0, cell_0_ref_0);
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut bool_3: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut result_0: std::result::Result<(), minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::click_unrevealed(minesweeper_0_ref_0, usize_1, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3342() {
    rusty_monitor::set_test_id(3342);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_0: usize = 87usize;
    let mut usize_1: usize = 75usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut gamestate_4: minesweeper::GameState = std::clone::Clone::clone(gamestate_3_ref_0);
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut bool_0: bool = std::cmp::PartialEq::eq(minesweepererror_1_ref_0, minesweepererror_0_ref_0);
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut gamestate_5: minesweeper::GameState = std::clone::Clone::clone(gamestate_4_ref_0);
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_2_ref_0);
    let mut minesweepererror_3: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut connectfourerror_0: connect_four::ConnectFourError = crate::connect_four::ConnectFourError::WrongPlayer;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut reversierror_1_ref_0: &reversi::ReversiError = &mut reversierror_1;
    let mut minesweepererror_4: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut connectfourerror_0_ref_0: &connect_four::ConnectFourError = &mut connectfourerror_0;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_1_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3418() {
    rusty_monitor::set_test_id(3418);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 18usize;
    let mut bool_2: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_0, is_revealed: bool_1, is_flagged: bool_0};
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut usize_1: usize = 63usize;
    let mut bool_5: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_5, mine_adjacent: usize_1, is_revealed: bool_4, is_flagged: bool_3};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut usize_2: usize = 24usize;
    let mut usize_3: usize = 87usize;
    let mut player_0: reversi::Player = crate::reversi::Player::Player0;
    let mut player_1: reversi::Player = crate::reversi::Player::other(player_0);
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player1;
    let mut usize_4: usize = 21usize;
    let mut usize_5: usize = 4usize;
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut usize_6: usize = 27usize;
    let mut usize_7: usize = 2usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_8: usize = 8usize;
    let mut usize_9: usize = 86usize;
    let mut u64_0: u64 = 1u64;
    let mut u64_1: u64 = 94u64;
    let mut steprng_0: rand::rngs::mock::StepRng = rand::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut rand::rngs::mock::StepRng = &mut steprng_0;
    let mut usize_10: usize = 22usize;
    let mut usize_11: usize = 87usize;
    let mut usize_12: usize = 2usize;
    let mut player_4: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_4);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut reversierror_1: reversi::ReversiError = crate::reversi::ReversiError::WrongPlayer;
    let mut result_0: std::result::Result<crate::minesweeper::Minesweeper, minesweeper::MinesweeperError> = crate::minesweeper::Minesweeper::new(usize_12, usize_11, usize_10, steprng_0_ref_0);
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut bool_6: bool = std::cmp::PartialEq::eq(gamestate_2_ref_0, gamestate_0_ref_0);
    let mut gamestate_5: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut player_5: gomoku::Player = crate::gomoku::Player::other(player_2);
    let mut result_1: std::result::Result<crate::connect_four::ConnectFour, std::convert::Infallible> = crate::connect_four::ConnectFour::new();
    let mut bool_7: bool = std::cmp::PartialEq::eq(cell_1_ref_0, cell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3023() {
    rusty_monitor::set_test_id(3023);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_0: usize = 81usize;
    let mut usize_1: usize = 45usize;
    let mut usize_2: usize = 60usize;
    let mut usize_3: usize = 82usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut bool_0: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut usize_4: usize = 58usize;
    let mut usize_5: usize = 28usize;
    let mut usize_6: usize = 86usize;
    let mut usize_7: usize = 27usize;
    let mut usize_8: usize = 80usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_9: usize = 76usize;
    let mut usize_10: usize = 41usize;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut usize_11: usize = 76usize;
    let mut bool_3: bool = false;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_3, mine_adjacent: usize_11, is_revealed: bool_2, is_flagged: bool_1};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut bool_4: bool = true;
    let mut cell_2: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_4);
    let mut cell_2_ref_0: &crate::minesweeper::Cell = &mut cell_2;
    let mut bool_5: bool = std::cmp::PartialEq::eq(cell_2_ref_0, cell_1_ref_0);
    let mut result_0: std::result::Result<crate::tictactoe::TicTacToe, std::convert::Infallible> = crate::tictactoe::TicTacToe::new();
    let mut tictactoe_0: crate::tictactoe::TicTacToe = std::result::Result::unwrap(result_0);
    let mut tictactoe_0_ref_0: &crate::tictactoe::TicTacToe = &mut tictactoe_0;
    let mut option_0: &std::option::Option<tictactoe::Player> = crate::tictactoe::TicTacToe::get(tictactoe_0_ref_0, usize_5, usize_4);
    let mut cell_3: crate::minesweeper::Cell = std::clone::Clone::clone(cell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2789() {
    rusty_monitor::set_test_id(2789);
    let mut bool_0: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut usize_0: usize = 35usize;
    let mut usize_1: usize = 71usize;
    let mut usize_2: usize = 71usize;
    let mut usize_3: usize = 91usize;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_3, usize_2, usize_1, usize_0);
    let mut adjacentcells_0_ref_0: &mut crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut usize_4: usize = 59usize;
    let mut usize_5: usize = 1usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_6: usize = 53usize;
    let mut usize_7: usize = 48usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_8: usize = 6usize;
    let mut usize_9: usize = 48usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut gamestate_5: minesweeper::GameState = std::clone::Clone::clone(gamestate_4_ref_0);
    let mut option_0: std::option::Option<usize> = std::iter::Iterator::next(adjacentcells_0_ref_0);
    let mut player_3: reversi::Player = crate::reversi::Player::Player0;
    let mut gamestate_6: reversi::GameState = crate::reversi::GameState::InProgress;
    let mut gamestate_5_ref_0: &minesweeper::GameState = &mut gamestate_5;
    let mut bool_1: bool = std::cmp::PartialEq::eq(gamestate_5_ref_0, gamestate_0_ref_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2852() {
    rusty_monitor::set_test_id(2852);
    let mut bool_0: bool = true;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut usize_0: usize = 96usize;
    let mut usize_1: usize = 32usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_2: usize = 83usize;
    let mut usize_3: usize = 0usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_1: gomoku::Player = crate::gomoku::Player::Player1;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_4: usize = 16usize;
    let mut usize_5: usize = 91usize;
    let mut bool_1: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_1);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_1_ref_0);
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::InvalidPosition;
    let mut player_2: gomoku::Player = crate::gomoku::Player::other(player_1);
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_4: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut player_5: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut result_0: std::result::Result<crate::reversi::Reversi, std::convert::Infallible> = crate::reversi::Reversi::new();
    let mut player_3_ref_0: &gomoku::Player = &mut player_3;
    let mut gamestate_2: tictactoe::GameState = crate::tictactoe::GameState::InProgress;
    let mut cell_2: crate::minesweeper::Cell = std::clone::Clone::clone(cell_0_ref_0);
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Tie;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut gamestate_4: reversi::GameState = crate::reversi::GameState::Tie;
    let mut gamestate_5: gomoku::GameState = crate::gomoku::GameState::InProgress;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4504() {
    rusty_monitor::set_test_id(4504);
    let mut usize_0: usize = 35usize;
    let mut usize_1: usize = 36usize;
    let mut usize_2: usize = 79usize;
    let mut usize_3: usize = 14usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut usize_4: usize = 26usize;
    let mut bool_2: bool = false;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut player_0: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_3: connect_four::GameState = crate::connect_four::GameState::Win(player_0);
    let mut gamestate_3_ref_0: &connect_four::GameState = &mut gamestate_3;
    let mut gamestate_4: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_5: usize = 18usize;
    let mut usize_6: usize = 58usize;
    let mut gamestate_4_ref_0: &minesweeper::GameState = &mut gamestate_4;
    let mut bool_3: bool = std::cmp::PartialEq::ne(gamestate_4_ref_0, gamestate_2_ref_0);
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_2, mine_adjacent: usize_4, is_revealed: bool_1, is_flagged: bool_0};
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::OccupiedPosition;
    let mut bool_4: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(cell_0_ref_0);
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_3, usize_2, usize_1, usize_0);
    let mut adjacentcells_0_ref_0: &crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut adjacentcells_1: crate::minesweeper::AdjacentCells = std::clone::Clone::clone(adjacentcells_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1459() {
    rusty_monitor::set_test_id(1459);
    let mut usize_0: usize = 79usize;
    let mut usize_1: usize = 23usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_2: usize = 31usize;
    let mut usize_3: usize = 57usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut usize_4: usize = 48usize;
    let mut usize_5: usize = 59usize;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut bool_0: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut usize_6: usize = 7usize;
    let mut bool_3: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_3, mine_adjacent: usize_6, is_revealed: bool_2, is_flagged: bool_1};
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut usize_7: usize = 15usize;
    let mut usize_8: usize = 85usize;
    let mut usize_9: usize = 37usize;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut usize_10: usize = 80usize;
    let mut usize_11: usize = 12usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_12: usize = 81usize;
    let mut usize_13: usize = 8usize;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_14: usize = 67usize;
    let mut usize_15: usize = 59usize;
    let mut vec_0: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_0, height: usize_15, width: usize_14, status: gamestate_2};
    let mut player_2: connect_four::Player = crate::connect_four::Player::Player1;
    let mut gamestate_3: gomoku::GameState = crate::gomoku::GameState::Tie;
    let mut minesweeper_0_ref_0: &mut crate::minesweeper::Minesweeper = &mut minesweeper_0;
    crate::minesweeper::Minesweeper::reveal_from(minesweeper_0_ref_0, usize_7);
    let mut player_2_ref_0: &connect_four::Player = &mut player_2;
    let mut gamestate_4: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut bool_4: bool = std::cmp::PartialEq::eq(cell_1_ref_0, cell_0_ref_0);
    let mut option_0: std::option::Option<&snafu::Backtrace> = snafu::ErrorCompat::backtrace(minesweepererror_0_ref_0);
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4109() {
    rusty_monitor::set_test_id(4109);
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut usize_0: usize = 20usize;
    let mut usize_1: usize = 77usize;
    let mut usize_2: usize = 63usize;
    let mut player_1: reversi::Player = crate::reversi::Player::Player0;
    let mut usize_3: usize = 86usize;
    let mut usize_4: usize = 5usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_5: usize = 12usize;
    let mut usize_6: usize = 67usize;
    let mut usize_7: usize = 49usize;
    let mut usize_8: usize = 92usize;
    let mut player_2: gomoku::Player = crate::gomoku::Player::Player0;
    let mut bool_0: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut bool_1: bool = false;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_1);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_9: usize = 66usize;
    let mut usize_10: usize = 83usize;
    let mut player_3: gomoku::Player = crate::gomoku::Player::Player0;
    let mut player_3_ref_0: &gomoku::Player = &mut player_3;
    let mut bool_2: bool = std::cmp::PartialEq::eq(cell_1_ref_0, cell_0_ref_0);
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut gamestate_2: gomoku::GameState = crate::gomoku::GameState::InProgress;
    let mut player_4: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_5: reversi::Player = crate::reversi::Player::Player1;
    let mut gamestate_3: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4271() {
    rusty_monitor::set_test_id(4271);
    let mut usize_0: usize = 59usize;
    let mut usize_1: usize = 53usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut player_0: gomoku::Player = crate::gomoku::Player::Player1;
    let mut player_1: gomoku::Player = crate::gomoku::Player::other(player_0);
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_1_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_1;
    let mut minesweepererror_2: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_2_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_2;
    let mut usize_2: usize = 13usize;
    let mut usize_3: usize = 88usize;
    let mut usize_4: usize = 15usize;
    let mut usize_5: usize = 22usize;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_5, usize_4, usize_3, usize_2);
    let mut adjacentcells_0_ref_0: &crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut usize_6: usize = 98usize;
    let mut usize_7: usize = 21usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_8: usize = 83usize;
    let mut usize_9: usize = 33usize;
    let mut vec_0: std::vec::Vec<(usize, usize)> = std::vec::Vec::new();
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Exploded(vec_0);
    let mut usize_10: usize = 94usize;
    let mut usize_11: usize = 80usize;
    let mut vec_1: std::vec::Vec<crate::minesweeper::Cell> = std::vec::Vec::new();
    let mut minesweeper_0: crate::minesweeper::Minesweeper = crate::minesweeper::Minesweeper {board: vec_1, height: usize_11, width: usize_10, status: gamestate_1};
    let mut adjacentcells_1: crate::minesweeper::AdjacentCells = std::clone::Clone::clone(adjacentcells_0_ref_0);
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut player_2: gomoku::Player = crate::gomoku::Player::other(player_1);
    let mut minesweepererror_3: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::GameEnded;
    let mut minesweepererror_3_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_3;
    let mut minesweeper_0_ref_0: &crate::minesweeper::Minesweeper = &mut minesweeper_0;
    let mut minesweeper_1: crate::minesweeper::Minesweeper = std::clone::Clone::clone(minesweeper_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2112() {
    rusty_monitor::set_test_id(2112);
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut gamestate_0_ref_0: &minesweeper::GameState = &mut gamestate_0;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_1_ref_0: &minesweeper::GameState = &mut gamestate_1;
    let mut bool_0: bool = false;
    let mut cell_0: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_0);
    let mut cell_0_ref_0: &crate::minesweeper::Cell = &mut cell_0;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut bool_1: bool = true;
    let mut cell_1: crate::minesweeper::Cell = crate::minesweeper::Cell::new(bool_1);
    let mut cell_1_ref_0: &crate::minesweeper::Cell = &mut cell_1;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut usize_0: usize = 77usize;
    let mut bool_4: bool = false;
    let mut cell_2: crate::minesweeper::Cell = crate::minesweeper::Cell {is_mine: bool_4, mine_adjacent: usize_0, is_revealed: bool_3, is_flagged: bool_2};
    let mut cell_2_ref_0: &crate::minesweeper::Cell = &mut cell_2;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player0;
    let mut gamestate_3: minesweeper::GameState = crate::minesweeper::GameState::Win;
    let mut usize_1: usize = 56usize;
    let mut usize_2: usize = 65usize;
    let mut gamestate_3_ref_0: &minesweeper::GameState = &mut gamestate_3;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(gamestate_3_ref_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::WrongPlayer;
    let mut player_1: tictactoe::Player = crate::tictactoe::Player::other(player_0);
    let mut bool_5: bool = std::cmp::PartialEq::ne(cell_2_ref_0, cell_1_ref_0);
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyFlagged;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut player_2: tictactoe::Player = crate::tictactoe::Player::other(player_1);
    let mut cell_3: crate::minesweeper::Cell = std::clone::Clone::clone(cell_0_ref_0);
    let mut tictactoeerror_0_ref_0: &tictactoe::TicTacToeError = &mut tictactoeerror_0;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::OccupiedPosition;
    let mut bool_6: bool = std::cmp::PartialEq::ne(gamestate_1_ref_0, gamestate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4186() {
    rusty_monitor::set_test_id(4186);
    let mut usize_0: usize = 48usize;
    let mut usize_1: usize = 4usize;
    let mut usize_2: usize = 19usize;
    let mut usize_3: usize = 73usize;
    let mut adjacentcells_0: crate::minesweeper::AdjacentCells = crate::minesweeper::AdjacentCells::new(usize_3, usize_2, usize_1, usize_0);
    let mut adjacentcells_0_ref_0: &crate::minesweeper::AdjacentCells = &mut adjacentcells_0;
    let mut usize_4: usize = 61usize;
    let mut usize_5: usize = 26usize;
    let mut gamestate_0: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_6: usize = 61usize;
    let mut usize_7: usize = 53usize;
    let mut usize_8: usize = 62usize;
    let mut usize_9: usize = 30usize;
    let mut bool_0: bool = true;
    let mut usize_10: usize = 15usize;
    let mut usize_11: usize = 27usize;
    let mut gamestate_1: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_12: usize = 95usize;
    let mut usize_13: usize = 2usize;
    let mut player_0: tictactoe::Player = crate::tictactoe::Player::Player1;
    let mut gamestate_2: minesweeper::GameState = crate::minesweeper::GameState::InProgress;
    let mut usize_14: usize = 14usize;
    let mut usize_15: usize = 82usize;
    let mut minesweepererror_0: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::TooManyMines;
    let mut minesweepererror_0_ref_0: &minesweeper::MinesweeperError = &mut minesweepererror_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(minesweepererror_0_ref_0);
    let mut gamestate_2_ref_0: &minesweeper::GameState = &mut gamestate_2;
    let mut gamestate_3: minesweeper::GameState = std::clone::Clone::clone(gamestate_2_ref_0);
    let mut gamestate_4: tictactoe::GameState = crate::tictactoe::GameState::Win(player_0);
    let mut tictactoeerror_0: tictactoe::TicTacToeError = crate::tictactoe::TicTacToeError::GameEnded;
    let mut reversierror_0: reversi::ReversiError = crate::reversi::ReversiError::GameEnded;
    let mut gomokuerror_0: gomoku::GomokuError = crate::gomoku::GomokuError::WrongPlayer;
    let mut gomokuerror_1: gomoku::GomokuError = crate::gomoku::GomokuError::OccupiedPosition;
    let mut minesweepererror_1: minesweeper::MinesweeperError = crate::minesweeper::MinesweeperError::AlreadyRevealed;
    let mut adjacentcells_1: crate::minesweeper::AdjacentCells = std::clone::Clone::clone(adjacentcells_0_ref_0);
    panic!("From RustyUnit with love");
}
}