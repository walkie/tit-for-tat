use crate::distribution::Distribution;
use crate::moves::IsMove;

/// A strategy is a function from the an intermediate game state to a move.
///
/// # Type variables
///
/// - `Move` -- The type of moves yielded by this strategy.
/// - `State` -- The type of the game state used to compute the next move.
pub trait Strategy<Move: IsMove, State> {
    /// Get the next move to play given a particular game state.
    fn next_move(&mut self, state: &State) -> Move;
}

pub struct Pure<Move> {
    the_move: Move,
}

impl<Move> Pure<Move> {
    pub fn new(the_move: Move) -> Self {
        Pure { the_move }
    }
}

impl<Move: IsMove, State> Strategy<Move, State> for Pure<Move> {
    fn next_move(&mut self, _game_state: &State) -> Move {
        self.the_move
    }
}

pub struct Mixed<Move> {
    dist: Distribution<Move>,
}

impl<Move: IsMove> Mixed<Move> {
    pub fn new(dist: Distribution<Move>) -> Self {
        Mixed { dist }
    }
}

impl<Move: IsMove, State> Strategy<Move, State> for Mixed<Move> {
    fn next_move(&mut self, _game_state: &State) -> Move {
        self.dist.sample().to_owned()
    }
}

pub struct Periodic<Move, State> {
    strategies: Vec<Box<dyn Strategy<Move, State>>>,
    next_index: usize,
}

impl<Move: IsMove, State> Periodic<Move, State> {
    pub fn new(strategies: Vec<Box<dyn Strategy<Move, State>>>) -> Self {
        Periodic {
            strategies,
            next_index: 0,
        }
    }

    pub fn of_pures(moves: &[Move]) -> Self {
        let strategies = Vec::from_iter(
            moves
                .iter()
                .map(|m| Box::new(Pure::new(m.to_owned())) as Box<dyn Strategy<Move, State>>),
        );
        Periodic::new(strategies)
    }
}

impl<Move: IsMove, State> Strategy<Move, State> for Periodic<Move, State> {
    fn next_move(&mut self, game_state: &State) -> Move {
        let the_move = self.strategies[self.next_index].next_move(game_state);
        self.next_index = (self.next_index + 1) % self.strategies.len();
        the_move
    }
}
