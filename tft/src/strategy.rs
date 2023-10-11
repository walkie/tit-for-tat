use crate::{Context, Distribution, Game};

/// A strategy is a function from an intermediate game context to a move.
pub trait Strategy<G: Game<P>, const P: usize> {
    /// Get the next move to play given the current play context.
    fn next_move(&mut self, context: &Context<G, P>) -> G::Move;

    // TODO: add methods (empty by default) to tell strategy we're starting a new game or a new
    // repeated game session

    fn new_game(&mut self) {}

    fn new_session(&mut self) {}
}

/// A pure strategy simply plays a given move regardless of the game context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pure<G: Game<P>, const P: usize> {
    the_move: G::Move,
}

impl<G: Game<P>, const P: usize> Pure<G, P> {
    /// Construct a pure strategy that plays the given move.
    pub fn new(the_move: G::Move) -> Self {
        Pure { the_move }
    }
}

impl<G: Game<P>, const P: usize> Strategy<G, P> for Pure<G, P> {
    fn next_move(&mut self, _context: &Context<G, P>) -> G::Move {
        self.the_move
    }
}

/// A mixed strategy plays a move according to a given probability distribution.
#[derive(Clone, Debug)]
pub struct Mixed<G: Game<P>, const P: usize> {
    dist: Distribution<G::Move>,
}

impl<G: Game<P>, const P: usize> Mixed<G, P> {
    /// Construct a mixed strategy from a probability distribution over moves.
    pub fn new(dist: Distribution<G::Move>) -> Self {
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
    pub fn flat(moves: Vec<G::Move>) -> Option<Self> {
        Distribution::flat(moves).map(|dist| Mixed::new(dist))
    }
}

impl<G: Game<P>, const P: usize> Strategy<G, P> for Mixed<G, P> {
    fn next_move(&mut self, _context: &Context<G, P>) -> G::Move {
        self.dist.sample().to_owned()
    }
}

/// A probabilistic strategy plays another strategy according to a given probability distribution.
///
/// A distribution of pure strategies is equivalent to a [Mixed] strategy.
pub struct Probabilistic<G: Game<P>, const P: usize> {
    dist: Distribution<Box<dyn Strategy<G, P>>>,
}

impl<G: Game<P>, const P: usize> Probabilistic<G, P> {
    /// Construct a probabilistic strategy from a distribution of strategies.
    pub fn new(dist: Distribution<Box<dyn Strategy<G, P>>>) -> Self {
        Probabilistic { dist }
    }
}

impl<G: Game<P>, const P: usize> Strategy<G, P> for Probabilistic<G, P> {
    fn next_move(&mut self, context: &Context<G, P>) -> G::Move {
        self.dist.sample_mut().next_move(context)
    }
}

/// A periodic strategy plays a sequence of strategies in order, then repeats.
pub struct Periodic<G: Game<P>, const P: usize> {
    strategies: Vec<Box<dyn Strategy<G, P>>>,
    next_index: usize,
}

impl<G: Game<P>, const P: usize> Periodic<G, P> {
    /// Construct a periodic strategy that repeats the given vector of strategies in order.
    pub fn new(strategies: Vec<Box<dyn Strategy<G, P>>>) -> Self {
        Periodic {
            strategies,
            next_index: 0,
        }
    }
}

impl<G: Game<P> + 'static, const P: usize> Periodic<G, P> {
    /// Construct a periodic strategy of pure strategies. That is, play the given moves in order
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
    fn next_move(&mut self, context: &Context<G, P>) -> G::Move {
        let the_move = self.strategies[self.next_index].next_move(context);
        self.next_index = (self.next_index + 1) % self.strategies.len();
        the_move
    }
}

/// A conditional strategy plays one strategy if a given condition is met, and another strategy
/// otherwise.
pub struct Conditional<G: Game<P>, const P: usize> {
    condition: Box<dyn FnMut(&Context<G, P>) -> bool>,
    on_true: Box<dyn Strategy<G, P>>,
    on_false: Box<dyn Strategy<G, P>>,
}

impl<G: Game<P>, const P: usize> Conditional<G, P> {
    /// Construct a new conditional strategy that plays the `on_true` strategy if `condition`
    /// returns true for the current context, and plays the `on_false` strategy otherwise.
    pub fn new(
        condition: impl FnMut(&Context<G, P>) -> bool + 'static,
        on_true: impl Strategy<G, P> + 'static,
        on_false: impl Strategy<G, P> + 'static,
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
        mut trigger: impl FnMut(&Context<G, P>) -> bool + 'static,
        before: impl Strategy<G, P> + 'static,
        after: impl Strategy<G, P> + 'static,
    ) -> Self {
        let mut has_triggered = false;
        let condition = move |context: &Context<G, P>| {
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

impl<G: Game<P>, const P: usize> Strategy<G, P> for Conditional<G, P> {
    fn next_move(&mut self, context: &Context<G, P>) -> G::Move {
        if (self.condition)(context) {
            self.on_true.next_move(context)
        } else {
            self.on_false.next_move(context)
        }
    }
}
