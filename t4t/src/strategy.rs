use crate::{Distribution, Move, PlayerIndex, State};

/// The strategic context in which a player makes a move during a game.
///
/// This type includes all information, besides the definition of the stage game, that a strategy
/// may use to compute its next move. It includes the player's index, the player's view of the game
/// state, a transcript of actions so far, and the current score.
#[derive(Clone, Debug, PartialEq)]
pub struct Context<S, const P: usize> {
    index: PlayerIndex<P>,
    state: S,
}

impl<S: State, const P: usize> Context<S, P> {
    /// Construct a new context from the index of the player whose turn it is to move and that
    /// player's view of the current state.
    pub fn new(index: PlayerIndex<P>, state_view: S) -> Self {
        Context {
            index,
            state: state_view,
        }
    }

    /// Get the index of the player whose turn it is to move. The method is named ("my") from the
    /// perspective of the strategy that receives this context.
    pub fn my_index(&self) -> PlayerIndex<P> {
        self.index
    }

    /// Get the player's view of the current state of the game.
    pub fn current_state(&self) -> &S {
        &self.state
    }
}

/// A strategy is a function from an intermediate game context to a move.
pub trait Strategy<S, M, const P: usize> {
    /// Get the next move to play given the current play context.
    fn next_move(&mut self, context: &Context<S, P>) -> M;
}

/// A pure strategy simply plays a given move regardless of the game context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pure<M> {
    the_move: M,
}

impl<M: Move> Pure<M> {
    /// Construct a pure strategy that plays the given move.
    pub fn new(the_move: M) -> Self {
        Pure { the_move }
    }
}

impl<S: State, M: Move, const P: usize> Strategy<S, M, P> for Pure<M> {
    fn next_move(&mut self, _context: &Context<S, P>) -> M {
        self.the_move
    }
}

/// A mixed strategy plays a move according to a given probability distribution.
#[derive(Clone, Debug)]
pub struct Mixed<M> {
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

impl<S: State, M: Move, const P: usize> Strategy<S, M, P> for Mixed<M> {
    fn next_move(&mut self, _context: &Context<S, P>) -> M {
        self.dist.sample().to_owned()
    }
}

/// A probabilistic strategy plays another strategy according to a given probability distribution.
///
/// A distribution of pure strategies is equivalent to a [Mixed] strategy.
pub struct Probabilistic<S: State, M: Move, const P: usize> {
    dist: Distribution<Box<dyn Strategy<S, M, P>>>,
}

impl<S: State, M: Move, const P: usize> Probabilistic<S, M, P> {
    /// Construct a probabilistic strategy from a distribution of strategies.
    pub fn new(dist: Distribution<Box<dyn Strategy<S, M, P>>>) -> Self {
        Probabilistic { dist }
    }
}

impl<S: State, M: Move, const P: usize> Strategy<S, M, P> for Probabilistic<S, M, P> {
    fn next_move(&mut self, context: &Context<S, P>) -> M {
        self.dist.sample_mut().next_move(context)
    }
}

/// A periodic strategy plays a sequence of strategies in order, then repeats.
pub struct Periodic<S: State, M: Move, const P: usize> {
    strategies: Vec<Box<dyn Strategy<S, M, P>>>,
    next_index: usize,
}

impl<S: State, M: Move, const P: usize> Periodic<S, M, P> {
    /// Construct a periodic strategy that repeats the given vector of strategies in order.
    pub fn new(strategies: Vec<Box<dyn Strategy<S, M, P>>>) -> Self {
        Periodic {
            strategies,
            next_index: 0,
        }
    }
}

impl<S: State, M: Move + 'static, const P: usize> Periodic<S, M, P> {
    /// Construct a periodic strategy of pure strategies. That is, play the given moves in order
    /// and repeat indefinitely.
    pub fn of_pures(moves: &[M]) -> Self {
        let strategies = Vec::from_iter(
            moves
                .iter()
                .map(|m| Box::new(Pure::new(m.to_owned())) as Box<dyn Strategy<S, M, P>>),
        );
        Periodic::new(strategies)
    }
}

impl<S: State, M: Move, const P: usize> Strategy<S, M, P> for Periodic<S, M, P> {
    fn next_move(&mut self, context: &Context<S, P>) -> M {
        let the_move = self.strategies[self.next_index].next_move(context);
        self.next_index = (self.next_index + 1) % self.strategies.len();
        the_move
    }
}

/// A conditional strategy plays one strategy if a given condition is met, and another strategy
/// otherwise.
pub struct Conditional<S: State, M: Move, const P: usize> {
    #[allow(clippy::type_complexity)]
    condition: Box<dyn FnMut(&Context<S, P>) -> bool>,
    on_true: Box<dyn Strategy<S, M, P>>,
    on_false: Box<dyn Strategy<S, M, P>>,
}

impl<S: State, M: Move, const P: usize> Conditional<S, M, P> {
    /// Construct a new conditional strategy that plays the `on_true` strategy if `condition`
    /// returns true for the current context, and plays the `on_false` strategy otherwise.
    pub fn new(
        condition: impl FnMut(&Context<S, P>) -> bool + 'static,
        on_true: impl Strategy<S, M, P> + 'static,
        on_false: impl Strategy<S, M, P> + 'static,
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
        mut trigger: impl FnMut(&Context<S, P>) -> bool + 'static,
        before: impl Strategy<S, M, P> + 'static,
        after: impl Strategy<S, M, P> + 'static,
    ) -> Self {
        let mut has_triggered = false;
        let condition = move |context: &Context<S, P>| {
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

impl<S: State, M: Move, const P: usize> Strategy<S, M, P> for Conditional<S, M, P> {
    fn next_move(&mut self, context: &Context<S, P>) -> M {
        if (self.condition)(context) {
            self.on_true.next_move(context)
        } else {
            self.on_false.next_move(context)
        }
    }
}
