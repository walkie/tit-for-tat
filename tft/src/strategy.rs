use crate::distribution::Distribution;
use crate::game::{Game, Move};
use crate::play::PlayState;

/// A strategy is a function from an intermediate game state to a move.
pub trait Strategy<G: Game<P>, const P: usize> {
    /// Get the next move to play given a particular game state.
    fn next_move(&mut self, state: &PlayState<G, P>) -> G::Move;
}

/// A pure strategy simply plays a given move regardless of the game state.
pub struct Pure<M> {
    the_move: M,
}

impl<M> Pure<M> {
    /// Construct a pure strategy that plays the given move.
    pub fn new(the_move: M) -> Self {
        Pure { the_move }
    }
}

impl<M: Move, G: Game<P, Move = M>, const P: usize> Strategy<G, P> for Pure<M> {
    fn next_move(&mut self, _state: &PlayState<G, P>) -> M {
        self.the_move
    }
}

/// A mixed strategy plays a move according to a given probability distribution.
pub struct Mixed<M> {
    dist: Distribution<M>,
}

impl<M> Mixed<M> {
    /// Construct a mixed strategy from a probability distrubtion over moves.
    pub fn new(dist: Distribution<M>) -> Self {
        Mixed { dist }
    }

    /// Construct a mixed strategy from a flat distribution over the given moves. This strategy
    /// will pick one move randomly, each with equal probability.
    ///
    /// # Errors
    ///
    /// Logs an error and returns `None` if:
    /// - The vector is empty.
    /// - The vector is longer than u32::MAX.
    pub fn flat(moves: Vec<M>) -> Option<Self> {
        Distribution::flat(moves).map(|dist| Mixed::new(dist))
    }
}

impl<M: Move, G: Game<P, Move = M>, const P: usize> Strategy<G, P> for Mixed<M> {
    fn next_move(&mut self, _state: &PlayState<G, P>) -> M {
        self.dist.sample().to_owned()
    }
}

/// A probabilistic strategy plays another strategy according to a given probability distribution.
///
/// A distribution of pure strategies is equivalent to a [Mixed] strategy.
pub struct Probabilistic<G, const P: usize> {
    dist: Distribution<Box<dyn Strategy<G, P>>>,
}

impl<G: Game<P>, const P: usize> Probabilistic<G, P> {
    /// Construct a probabilistic strategy from a distrubtion of strategies.
    pub fn new(dist: Distribution<Box<dyn Strategy<G, P>>>) -> Self {
        Probabilistic { dist }
    }
}

impl<G: Game<P>, const P: usize> Strategy<G, P> for Probabilistic<G, P> {
    fn next_move(&mut self, state: &PlayState<G, P>) -> G::Move {
        self.dist.sample_mut().next_move(state)
    }
}

/// A periodic strategy plays a sequence of strategies in order, then repeats.
pub struct Periodic<G, const P: usize> {
    strategies: Vec<Box<dyn Strategy<G, P>>>,
    next_index: usize,
}

impl<G: Game<P>, const P: usize> Periodic<G, P> {
    /// Construct a pediodic strategy that repeats the given vector of strategies in order.
    pub fn new(strategies: Vec<Box<dyn Strategy<G, P>>>) -> Self {
        Periodic {
            strategies,
            next_index: 0,
        }
    }
}

impl<G: Game<P>, const P: usize> Periodic<G, P> {
    /// Construct a pediodic strategy of pure strategies. That is, play the given moves in order
    /// and repeat indefinitely.
    pub fn of_pures(moves: &[G::Move]) -> Self {
        let strategies = Vec::from_iter(
            moves
                .iter()
                .map(|m| Box::new(Pure::new(m.to_owned())) as Box<dyn Strategy<G, P>>),
        );
        Periodic::new(strategies)
    }
}

impl<G: Game<P>, const P: usize> Strategy<G, P> for Periodic<G, P> {
    fn next_move(&mut self, state: &PlayState<G, P>) -> G::Move {
        let the_move = self.strategies[self.next_index].next_move(state);
        self.next_index = (self.next_index + 1) % self.strategies.len();
        the_move
    }
}
