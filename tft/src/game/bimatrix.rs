//! Bimatrix games. 2-player, normal-form games.

use crate::prelude::*;

pub struct Bimatrix<RowMove, ColMove, Util, const ROWS: usize, const COLS: usize> {
    moves_p0: [RowMove; ROWS],
    moves_p1: [ColMove; COLS],
    payoffs: [[Payoff<Util, 2>; COLS]; ROWS],
}

impl<RowMove: IsMove, ColMove: IsMove, Util: IsUtility, const ROWS: usize, const COLS: usize>
    Bimatrix<RowMove, ColMove, Util, ROWS, COLS>
{
    pub fn new(
        moves_p0: [RowMove; ROWS],
        moves_p1: [ColMove; COLS],
        payoff_matrix: [[(Util, Util); COLS]; ROWS],
    ) -> Self {
        let payoffs: [[Payoff<Util, 2>; COLS]; ROWS] =
            payoff_matrix.map(|row| row.map(|col| Payoff::from([col.0, col.1])));
        Bimatrix {
            moves_p0,
            moves_p1,
            payoffs,
        }
    }

    pub fn from_matrices(
        moves_p0: [RowMove; ROWS],
        moves_p1: [ColMove; COLS],
        utils_p0: [[Util; COLS]; ROWS],
        utils_p1: [[Util; COLS]; ROWS],
    ) -> Self {
        let mut rows = Vec::with_capacity(ROWS);
        for (row_p0, row_p1) in utils_p0.into_iter().zip(utils_p1) {
            let mut row = Vec::with_capacity(COLS);
            for (u0, u1) in row_p0.into_iter().zip(row_p1) {
                row.push(Payoff::from([u0, u1]));
            }
            rows.push(row.try_into().unwrap());
        }
        Bimatrix {
            moves_p0,
            moves_p1,
            payoffs: rows.try_into().unwrap(),
        }
    }

    pub fn available_moves_p0<'a>(
        &'a self
    ) -> std::iter::Copied<std::slice::Iter<'a, for2::Move<RowMove, ColMove>>> {
        self.moves_p0.map(|m| for2::Move::P0(m)).iter().copied()
    }

    pub fn available_moves_p1<'a>(
        &'a self
    ) -> std::iter::Copied<std::slice::Iter<'a, for2::Move<RowMove, ColMove>>> {
        self.moves_p1.map(|m| for2::Move::P1(m)).iter().copied()
    }
}

impl<Move: IsMove, Util: IsUtility, const SIZE: usize> Bimatrix<Move, Move, Util, SIZE, SIZE> {
    pub fn symmetric(moves: [Move; SIZE], utils: [[Util; SIZE]; SIZE]) -> Self {
        let mut rows = Vec::with_capacity(SIZE);
        for util_row in utils.into_iter() {
            let mut row = Vec::with_capacity(SIZE);
            for util in util_row.into_iter() {
                row.push(Payoff::from([util, util]));
            }
            rows.push(row.try_into().unwrap());
        }
        Bimatrix {
            moves_p0: moves,
            moves_p1: moves,
            payoffs: rows.try_into().unwrap(),
        }
    }
}

impl<RowMove: IsMove, ColMove: IsMove, Util: IsUtility, const ROWS: usize, const COLS: usize>
    Game<2> for Bimatrix<RowMove, ColMove, Util, ROWS, COLS>
{
    type Move = for2::Move<RowMove, ColMove>;
    type Utility = Util;
    type State = ();

    fn initial_state(&self) {}

    fn is_valid_move_for_player_at_state(
        &self,
        player: PlayerIndex<2>,
        _state: &(),
        the_move: for2::Move<RowMove, ColMove>,
    ) -> bool {
        match the_move {
            for2::Move::P0(m) => player == for2::P0 && self.moves_p0.contains(&m),
            for2::Move::P1(m) => player == for2::P1 && self.moves_p1.contains(&m),
        }
    }
}

impl<RowMove: IsMove, ColMove: IsMove, Util: IsUtility, const ROWS: usize, const COLS: usize>
    Finite<2> for Bimatrix<RowMove, ColMove, Util, ROWS, COLS>
{
    fn available_moves_for_player_at_state(
        &self,
        player: PlayerIndex<2>,
        _state: &(),
    ) -> MoveIter<for2::Move<RowMove, ColMove>> {
        if player == for2::P0 {
            MoveIter::new(self.available_moves_p0())
        } else {
            MoveIter::new(self.available_moves_p1())
        }
    }
}
