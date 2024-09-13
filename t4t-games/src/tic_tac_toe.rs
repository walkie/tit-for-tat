use t4t::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct TicTacToe;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Mark {
    X,
    O,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(usize)]
pub enum Row {
    Top = 0,
    Middle = 1,
    Bottom = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(usize)]
pub enum Col {
    Left = 0,
    Middle = 1,
    Right = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Square {
    pub row: Row,
    pub col: Col,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Board {
    squares: [[Option<Mark>; 3]; 3],
}

impl Mark {
    pub fn for_player(player: &PlayerIndex<2>) -> Mark {
        match *player {
            for2::P0 => Mark::X,
            for2::P1 => Mark::O,
            _ => unreachable!(),
        }
    }
}

impl Square {
    pub fn new(row: Row, col: Col) -> Self {
        Square { row, col }
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            squares: [[None; 3]; 3],
        }
    }

    pub fn squares(&self) -> [[Option<Mark>; 3]; 3] {
        self.squares
    }

    pub fn get_mark(&self, square: &Square) -> Option<Mark> {
        let r = square.row as usize;
        let c = square.col as usize;
        self.squares[r][c]
    }

    pub fn set_mark(&mut self, square: &Square, mark: Mark) {
        let r = square.row as usize;
        let c = square.col as usize;
        self.squares[r][c] = Some(mark);
    }

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
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Game<2> for TicTacToe {
    type Move = Square;
    type Utility = u64;
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
        if state.empty_squares().len() % 2 == 0 {
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

    fn payoff(&self, state: &Board) -> Option<Payoff<u64, 2>> {
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
