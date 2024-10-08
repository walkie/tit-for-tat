//! The classic game of [tic-tac-toe](https://en.wikipedia.org/wiki/Tic-tac-toe), a.k.a. noughts and
//! crosses.
//!
//! Two players take turns marking empty squares in a 3x3 grid. The first player marks `X`, the
//! second player marks `O`. If a player marks all three squares in a row, column, or one of the
//! two diagonals, they win!
//!
//! The game is implemented by defining the state of the board and then implementing t4t's
//! [Combinatorial](t4t::Combinatorial) trait to define the game's rules, i.e. how players make
//! moves to alter the state, and when and how the game ends.
//!
//! # Example
//!
//! The following example runs a small tournament among three tic-tac-toe players, accumulating the
//! scores for each player and checking that the optimal strategy never loses a game.
//!
//! You can run this example to see the final board for each game and the resulting scores by
//! downloading this crate and running:
//! ```bash
//! $ cargo run --example tic_tac_toe
//! ```
//!
//! ```
#![doc = include_str!("../examples/tic_tac_toe.rs")]
//! ```

use t4t::*;

/// A zero-size struct representing the game of tic-tac-toe.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct TicTacToe;

/// A player's mark in the game. The first player uses `X`, the second player uses `O`.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Mark {
    X,
    O,
}

/// A row in a tic-tac-toe board.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(usize)]
pub enum Row {
    Top = 0,
    Middle = 1,
    Bottom = 2,
}

/// A column in a tic-tac-toe board.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(usize)]
pub enum Col {
    Left = 0,
    Middle = 1,
    Right = 2,
}

/// A square in a tic-tac-toe board is identified by a row and a column.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Square {
    pub row: Row,
    pub col: Col,
}

/// A tic-tac-toe board is a 3x3 grid of squares, each of which may be empty or contain a mark.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Board {
    squares: [[Option<Mark>; 3]; 3],
}

impl Mark {
    /// Get the mark corresponding to the given player.
    pub fn for_player(player: &PlayerIndex<2>) -> Mark {
        match *player {
            for2::P0 => Mark::X,
            for2::P1 => Mark::O,
            _ => unreachable!(),
        }
    }
}

impl Square {
    /// Construct an identifier for the square at the given row and column.
    pub fn new(row: Row, col: Col) -> Self {
        Square { row, col }
    }
}

impl Board {
    /// Construct a new, empty tic-tac-toe board.
    pub fn new() -> Self {
        Board {
            squares: [[None; 3]; 3],
        }
    }

    /// Get the current state of the board as a 2D array of optional marks.
    pub fn state(&self) -> [[Option<Mark>; 3]; 3] {
        self.squares
    }

    /// Get the mark in the given square, if any.
    pub fn get_mark(&self, square: &Square) -> Option<Mark> {
        let r = square.row as usize;
        let c = square.col as usize;
        self.squares[r][c]
    }

    /// Set the mark in the given square.
    pub fn set_mark(&mut self, square: &Square, mark: Mark) {
        let r = square.row as usize;
        let c = square.col as usize;
        self.squares[r][c] = Some(mark);
    }

    /// Get a list of all empty squares on the board.
    pub fn empty_squares(&self) -> Vec<Square> {
        let mut empty = Vec::new();
        for r in [Row::Top, Row::Middle, Row::Bottom] {
            for c in [Col::Left, Col::Middle, Col::Right] {
                let square = Square::new(r, c);
                if self.get_mark(&square).is_none() {
                    empty.push(square);
                }
            }
        }
        empty
    }

    /// Get the state of the board organized into the 8 possible lines: 3 rows, 3 columns, and
    /// 2 diagonals.
    pub fn lines(&self) -> [[Option<Mark>; 3]; 8] {
        [
            [self.squares[0][0], self.squares[0][1], self.squares[0][2]],
            [self.squares[1][0], self.squares[1][1], self.squares[1][2]],
            [self.squares[2][0], self.squares[2][1], self.squares[2][2]],
            [self.squares[0][0], self.squares[1][0], self.squares[2][0]],
            [self.squares[0][1], self.squares[1][1], self.squares[2][1]],
            [self.squares[0][2], self.squares[1][2], self.squares[2][2]],
            [self.squares[0][0], self.squares[1][1], self.squares[2][2]],
            [self.squares[0][2], self.squares[1][1], self.squares[2][0]],
        ]
    }

    /// Get a list of all winning moves for the given player.
    pub fn winning_moves_for(&self, player: PlayerIndex<2>) -> Vec<Square> {
        let mark = Mark::for_player(&player);
        let mut winning_moves = Vec::new();
        for square in self.empty_squares() {
            let mut next_board = self.clone();
            next_board.set_mark(&square, mark);
            if next_board.check_winner() == Some(mark) {
                winning_moves.push(square);
            }
        }
        winning_moves
    }

    /// Check if the game has a winner, and if so, return the mark of the winning player.
    pub fn check_winner(&self) -> Option<Mark> {
        for line in self.lines().iter() {
            for mark in [Mark::X, Mark::O] {
                if line.iter().all(|&m| m == Some(mark)) {
                    return Some(mark);
                }
            }
        }
        None
    }

    /// Print the current state of the board to stdout.
    pub fn print(&self) {
        let mark = |r: usize, c: usize| match self.squares[r][c] {
            None => ' ',
            Some(Mark::X) => 'X',
            Some(Mark::O) => 'O',
        };

        println!("{}|{}|{}", mark(0, 0), mark(0, 1), mark(0, 2));
        println!("-----");
        println!("{}|{}|{}", mark(1, 0), mark(1, 1), mark(1, 2));
        println!("-----");
        println!("{}|{}|{}", mark(2, 0), mark(2, 1), mark(2, 2));
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Game<2> for TicTacToe {
    type Move = Square;
    type Utility = i8;
    type State = Board;
    type View = Board;

    fn state_view(&self, state: &Self::State, _player: PlayerIndex<2>) -> Self::View {
        state.clone()
    }
}

impl Combinatorial<2> for TicTacToe {
    fn initial_state(&self) -> Board {
        Board::new()
    }

    fn whose_turn(&self, state: &Board) -> PlayerIndex<2> {
        if state.empty_squares().len() % 2 == 1 {
            for2::P0
        } else {
            for2::P1
        }
    }

    fn next_state(&self, board: Board, square: Square) -> PlayResult<Board, Board, Square, 2> {
        let player = self.whose_turn(&board);
        if board.get_mark(&square).is_some() {
            return Err(InvalidMove::new(board.clone(), player, square));
        }

        let mut next_board = board;
        next_board.set_mark(&square, Mark::for_player(&player));

        Ok(next_board)
    }

    fn payoff(&self, state: &Board) -> Option<Payoff<i8, 2>> {
        match state.check_winner() {
            Some(Mark::X) => Some(Payoff::zero_sum_winner(for2::P0)),
            Some(Mark::O) => Some(Payoff::zero_sum_winner(for2::P1)),
            None => {
                if state.empty_squares().is_empty() {
                    Some(Payoff::zeros())
                } else {
                    None
                }
            }
        }
    }
}

impl Finite<2> for TicTacToe {
    fn possible_moves(&self, _player: PlayerIndex<2>, state: &Board) -> PossibleMoves<Square> {
        let vec = if self.is_game_end(state) {
            Vec::new()
        } else {
            state.empty_squares()
        };
        PossibleMoves::from_vec(vec)
    }
}
