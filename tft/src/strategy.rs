use crate::distribution::Distribution;
use crate::moves::IsMove;

/// A strategy is a function from an intermediate game state to a move.
///
/// # Type variables
///
/// - `Move` -- The type of moves yielded by this strategy.
/// - `State` -- The type of the game state used to compute the next move.
pub trait Strategy<Move: IsMove, State> {
    /// Get the next move to play given a particular game state.
    fn next_move(&mut self, state: &State) -> Move;
}

/// A pure strategy simply plays a given move regardless of the game state.
pub struct Pure<Move> {
    the_move: Move,
}

impl<Move> Pure<Move> {
    /// Construct a pure strategy that plays the given move.
    pub fn new(the_move: Move) -> Self {
        Pure { the_move }
    }
}

impl<Move: IsMove, State> Strategy<Move, State> for Pure<Move> {
    fn next_move(&mut self, _game_state: &State) -> Move {
        self.the_move
    }
}

/// A mixed strategy plays a move according to a given probability distribution.
pub struct Mixed<Move> {
    dist: Distribution<Move>,
}

impl<Move> Mixed<Move> {
    /// Construct a mixed strategy from a probability distrubtion over moves.
    pub fn new(dist: Distribution<Move>) -> Self {
        Mixed { dist }
    }

    /// Construct a mixed strategy from a flat distribution over the given moves. This strategy
    /// will pick one move randomly, each with equal probability.
    pub fn flat(moves: Vec<Move>) -> Self {
        Mixed::new(Distribution::flat(moves))
    }
}

impl<Move: IsMove, State> Strategy<Move, State> for Mixed<Move> {
    fn next_move(&mut self, _game_state: &State) -> Move {
        self.dist.sample().to_owned()
    }
}

/// A probabilistic strategy plays another strategy according to a given probability distribution.
///
/// A distribution of pure strategies is equivalent to a [Mixed] strategy.
pub struct Probabilistic<Move, State> {
    dist: Distribution<Box<dyn Strategy<Move, State>>>,
}

impl<Move, State> Probabilistic<Move, State> {
    /// Construct a probabilistic strategy from a distrubtion of strategies.
    pub fn new(dist: Distribution<Box<dyn Strategy<Move, State>>>) -> Self {
        Probabilistic { dist }
    }
}

impl<Move: IsMove, State> Strategy<Move, State> for Probabilistic<Move, State> {
    fn next_move(&mut self, game_state: &State) -> Move {
        self.dist.sample_mut().next_move(game_state)
    }
}

/// A periodic strategy plays a sequence of strategies in order, then repeats.
pub struct Periodic<Move, State> {
    strategies: Vec<Box<dyn Strategy<Move, State>>>,
    next_index: usize,
}

impl<Move, State> Periodic<Move, State> {
    /// Construct a pediodic strategy that repeats the given vector of strategies in order.
    pub fn new(strategies: Vec<Box<dyn Strategy<Move, State>>>) -> Self {
        Periodic {
            strategies,
            next_index: 0,
        }
    }
}

impl<Move: IsMove, State> Periodic<Move, State> {
    /// Construct a pediodic strategy of pure strategies. That is, play the given moves in order
    /// and repeat indefinitely.
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
