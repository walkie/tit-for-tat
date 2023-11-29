use std::rc::Rc;

use crate::{Distribution, Move, PlayerIndex, State};

/// The strategic context in which a player makes a move during a game.
///
/// This type includes all information, besides the definition of the stage game, that a strategy
/// may use to compute its next move. It includes the player's index, the player's view of the game
/// state, a transcript of actions so far, and the current score.
#[derive(Clone, Debug, PartialEq)]
pub struct Context<S, const P: usize> {
    state: Rc<S>,
    index: PlayerIndex<P>,
}

impl<S: State, const P: usize> Context<S, P> {
    /// Construct a new context from the index of the player whose turn it is to move and that
    /// player's view of the current state.
    pub fn new(index: PlayerIndex<P>, state_view: Rc<S>) -> Self {
        Context {
            index,
            state: state_view,
        }
    }

    /// Get the player's view of the current state of the game.
    pub fn current_state(&self) -> &Rc<S> {
        &self.state
    }

    /// Get the index of the player whose turn it is to move. The method is named "my" from the
    /// perspective of the strategy that receives this context.
    pub fn my_index(&self) -> PlayerIndex<P> {
        self.index
    }
}

impl<S: State> Context<S, 2> {
    /// Get the index of the other player in a two player game. The method is named "their"
    /// (singular) from the perspective of the strategy that receives this context.
    pub fn their_index(&self) -> PlayerIndex<2> {
        self.index
    }
}

/// A strategy is a function from an intermediate game context to a move.
pub struct Strategy<S, M, const P: usize> {
    #[allow(clippy::type_complexity)]
    next_move: Box<dyn FnMut(&Context<S, P>) -> M>,
}

impl<S: State + 'static, M: Move, const P: usize> Strategy<S, M, P> {
    /// Construct a new strategy from a function that computes the next move given a strategic
    /// context.
    pub fn new(next_move: impl FnMut(&Context<S, P>) -> M + 'static) -> Self {
        Strategy {
            next_move: Box::new(next_move),
        }
    }

    /// Construct a [pure strategy](https://en.wikipedia.org/wiki/Strategy_(game_theory)#Pure_and_mixed_strategies)
    /// that always plays the same move regardless of the context.
    pub fn pure(the_move: M) -> Self {
        Strategy::new(move |_| the_move)
    }

    /// Construct a [mixed strategy](https://en.wikipedia.org/wiki/Strategy_(game_theory)#Mixed_strategy)
    /// that plays a move according to the given probability distribution over moves.
    pub fn mixed(dist: Distribution<M>) -> Self {
        Strategy::new(move |_| dist.sample().to_owned())
    }

    /// Construct a [mixed strategy](https://en.wikipedia.org/wiki/Strategy_(game_theory)#Mixed_strategy)
    /// from a flat distribution over the given moves. This strategy will pick one move randomly,
    /// each with equal probability.
    ///
    /// # Errors
    ///
    /// Logs an error and returns `None` if:
    /// - The vector is empty.
    /// - The vector is longer than u32::MAX.
    pub fn mixed_flat(moves: Vec<M>) -> Option<Self> {
        Distribution::flat(moves).map(|dist| Strategy::mixed(dist))
    }

    /// Construct a probabilistic strategy that plays another strategy according to the given
    /// probability distribution.
    ///
    /// A distribution of pure strategies is equivalent to a [mixed](Strategy::mixed) strategy.
    pub fn probabilistic(mut dist: Distribution<Strategy<S, M, P>>) -> Self {
        Strategy::new(move |context| dist.sample_mut().next_move(context))
    }

    /// Construct a periodic strategy that plays the given sequence of strategies in order, then
    /// repeats.
    pub fn periodic(mut strategies: Vec<Strategy<S, M, P>>) -> Self {
        let mut next_index = 0;
        Strategy::new(move |context| {
            let the_move = strategies[next_index].next_move(context);
            next_index = (next_index + 1) % strategies.len();
            the_move
        })
    }
    /// Construct a periodic strategy of pure strategies. That is, play the given moves in order
    /// and repeat indefinitely.
    pub fn periodic_pure(moves: &[M]) -> Self {
        let strategies = Vec::from_iter(moves.iter().map(|m| Strategy::pure(m.to_owned())));
        Strategy::periodic(strategies)
    }

    /// Construct a new conditional strategy that plays the `on_true` strategy if `condition`
    /// returns true for the current context, and plays the `on_false` strategy otherwise.
    pub fn conditional(
        mut condition: impl FnMut(&Context<S, P>) -> bool + 'static,
        mut on_true: Strategy<S, M, P>,
        mut on_false: Strategy<S, M, P>,
    ) -> Self {
        Strategy::new(move |context| {
            if (condition)(context) {
                on_true.next_move(context)
            } else {
                on_false.next_move(context)
            }
        })
    }

    /// Construct a new trigger strategy that plays the `before` strategy until `trigger` returns
    /// true, then plays the `after` strategy thereafter.
    pub fn trigger(
        mut trigger: impl FnMut(&Context<S, P>) -> bool + 'static,
        mut before: Strategy<S, M, P>,
        mut after: Strategy<S, M, P>,
    ) -> Self {
        let mut triggered = false;
        Strategy::new(move |context| {
            if !triggered {
                triggered = (trigger)(context);
            }
            if triggered {
                after.next_move(context)
            } else {
                before.next_move(context)
            }
        })
    }

    /// Get the next move to play given the current play context.
    pub fn next_move(&mut self, context: &Context<S, P>) -> M {
        (self.next_move)(context)
    }
}
