use crate::moves::Move;
use crate::distribution::Distribution;

/// A strategy is a function from an intermediate game context to a move.
pub trait Strategy<C, M: Move> {
    /// Get the next move to play given the current play context.
    fn next_move(&mut self, context: &C) -> M;

    // TODO: add methods (empty by default) to tell strategy we're starting a new game or a new
    // repeated game session

    fn new_game(&mut self) {}

    fn new_session(&mut self) {}
}

/// A pure strategy simply plays a given move regardless of the game context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pure<M: Move> {
    the_move: M,
}

impl<M: Move> Pure<M> {
    /// Construct a pure strategy that plays the given move.
    pub fn new(the_move: M) -> Self {
        Pure { the_move }
    }
}

impl<C, M: Move> Strategy<C, M> for Pure<M> {
    fn next_move(&mut self, _context: &C) -> M {
        self.the_move
    }
}

/// A mixed strategy plays a move according to a given probability distribution.
#[derive(Clone, Debug)]
pub struct Mixed<M: Move> {
    dist: Distribution<M>,
}

impl<M: Move> Mixed<M> {
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

impl<C, M: Move> Strategy<C, M> for Mixed<M> {
    fn next_move(&mut self, _context: &C) -> M {
        self.dist.sample().to_owned()
    }
}

/// A probabilistic strategy plays another strategy according to a given probability distribution.
///
/// A distribution of pure strategies is equivalent to a [Mixed] strategy.
pub struct Probabilistic<C, M: Move> {
    dist: Distribution<Box<dyn Strategy<C, M>>>,
}

impl<C, M: Move> Probabilistic<C, M> {
    /// Construct a probabilistic strategy from a distrubtion of strategies.
    pub fn new(dist: Distribution<Box<dyn Strategy<C, M>>>) -> Self {
        Probabilistic { dist }
    }
}

impl<C, M: Move> Strategy<C, M> for Probabilistic<C, M> {
    fn next_move(&mut self, context: &C) -> M {
        self.dist.sample_mut().next_move(context)
    }
}

/// A periodic strategy plays a sequence of strategies in order, then repeats.
pub struct Periodic<C, M: Move> {
    strategies: Vec<Box<dyn Strategy<C, M>>>,
    next_index: usize,
}

impl<C, M: Move> Periodic<C, M> {
    /// Construct a pediodic strategy that repeats the given vector of strategies in order.
    pub fn new(strategies: Vec<Box<dyn Strategy<C, M>>>) -> Self {
        Periodic {
            strategies,
            next_index: 0,
        }
    }
}

impl<C, M: Move> Periodic<C, M> {
    /// Construct a pediodic strategy of pure strategies. That is, play the given moves in order
    /// and repeat indefinitely.
    pub fn of_pures(moves: &[M]) -> Self {
        let strategies = Vec::from_iter(
            moves
                .iter()
                .map(|m| Box::new(Pure::new(m.to_owned())) as Box<dyn Strategy<C, M>>),
        );
        Periodic::new(strategies)
    }
}

impl<C, M: Move> Strategy<C, M> for Periodic<C, M> {
    fn next_move(&mut self, context: &C) -> M {
        let the_move = self.strategies[self.next_index].next_move(context);
        self.next_index = (self.next_index + 1) % self.strategies.len();
        the_move
    }
}
