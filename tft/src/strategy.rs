use crate::{Distribution, Move};

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
    /// Construct a mixed strategy from a probability distribution over moves.
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
    /// Construct a probabilistic strategy from a distribution of strategies.
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
    /// Construct a periodic strategy that repeats the given vector of strategies in order.
    pub fn new(strategies: Vec<Box<dyn Strategy<C, M>>>) -> Self {
        Periodic {
            strategies,
            next_index: 0,
        }
    }
}

impl<C, M: Move> Periodic<C, M> {
    /// Construct a periodic strategy of pure strategies. That is, play the given moves in order
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

/// A conditional strategy plays one strategy if a given condition is met, and another strategy
/// otherwise.
pub struct Conditional<C, M> {
    condition: Box<dyn FnMut(&C) -> bool>,
    on_true: Box<dyn Strategy<C, M>>,
    on_false: Box<dyn Strategy<C, M>>,
}

impl<C, M: Move> Conditional<C, M> {
    /// Construct a new conditional strategy that plays the `on_true` strategy if `condition`
    /// returns true for the current context, and plays the `on_false` strategy otherwise.
    pub fn new(
        condition: impl FnMut(&C) -> bool + 'static,
        on_true: impl Strategy<C, M> + 'static,
        on_false: impl Strategy<C, M> + 'static,
    ) -> Self {
        Conditional {
            condition: Box::new(condition),
            on_true: Box::new(on_true),
            on_false: Box::new(on_false),
        }
    }

    /// Construct a new trigger strategy that plays the `before` strategy until `trigger` returns
    /// true, then plays the `after` strategy thereafter.
    pub fn trigger(
        mut trigger: impl FnMut(&C) -> bool + 'static,
        before: impl Strategy<C, M> + 'static,
        after: impl Strategy<C, M> + 'static,
    ) -> Self {
        let mut has_triggered = false;
        let condition = move |context: &C| {
            if has_triggered {
                true
            } else {
                has_triggered = (trigger)(context);
                has_triggered
            }
        };
        Conditional::new(condition, before, after)
    }
}

impl<C, M: Move> Strategy<C, M> for Conditional<C, M> {
    fn next_move(&mut self, context: &C) -> M {
        if (self.condition)(context) {
            self.on_true.next_move(context)
        } else {
            self.on_false.next_move(context)
        }
    }
}
