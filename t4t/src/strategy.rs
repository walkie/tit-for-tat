use crate::{Distribution, Move, PlayerIndex, State};

/// The strategic context in which a player makes a move during a game.
///
/// This type includes all information, besides the definition of the stage game, that a strategy
/// may use to compute its next move. It includes the player's index and the player's view of the
/// game state.
#[derive(Clone, Debug, PartialEq)]
pub struct Context<V, const P: usize> {
    index: PlayerIndex<P>,
    state_view: V,
}

impl<V: State, const P: usize> Context<V, P> {
    /// Construct a new context from the index of the player whose turn it is to move and that
    /// player's view of the current state.
    pub fn new(index: PlayerIndex<P>, state_view: V) -> Self {
        Context { index, state_view }
    }

    /// Get the player's view of the current state of the game.
    pub fn state_view(&self) -> &V {
        &self.state_view
    }

    /// Get the index of the player whose turn it is to move. The method is named "my" from the
    /// perspective of the strategy that receives this context.
    pub fn my_index(&self) -> PlayerIndex<P> {
        self.index
    }
}

impl<V: State> Context<V, 2> {
    /// Get the index of the other player in a two player game. The method is named "their"
    /// (singular) from the perspective of the strategy that receives this context.
    pub fn their_index(&self) -> PlayerIndex<2> {
        self.index.next()
    }
}

/// A strategy is a function from an intermediate game context to a move.
pub struct Strategy<V, M, const P: usize> {
    #[allow(clippy::type_complexity)]
    next_move: Box<dyn FnMut(&Context<V, P>) -> M + Send + Sync>,
}

impl<V: State + 'static, M: Move, const P: usize> Strategy<V, M, P> {
    /// Construct a new strategy from a function that computes the next move given a strategic
    /// context.
    pub fn new(next_move: impl FnMut(&Context<V, P>) -> M + Send + Sync + 'static) -> Self {
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
    pub fn probabilistic(mut dist: Distribution<Strategy<V, M, P>>) -> Self {
        Strategy::new(move |context| dist.sample_mut().next_move(context))
    }

    /// Construct a periodic strategy that plays the given sequence of strategies in order, then
    /// repeats.
    pub fn periodic(mut strategies: Vec<Strategy<V, M, P>>) -> Self {
        let mut next_index = 0;
        Strategy::new(move |context| {
            let the_move = strategies[next_index].next_move(context);
            next_index = (next_index + 1) % strategies.len();
            the_move
        })
    }
    /// Construct a periodic strategy of pure strategies. That is, play the given moves in order
    /// and repeat indefinitely.
    pub fn periodic_pure(moves: Vec<M>) -> Self {
        let strategies = Vec::from_iter(moves.into_iter().map(|m| Strategy::pure(m)));
        Strategy::periodic(strategies)
    }

    /// Construct a new conditional strategy that plays the `on_true` strategy if `condition`
    /// returns true for the current context, and plays the `on_false` strategy otherwise.
    pub fn conditional(
        mut condition: impl FnMut(&Context<V, P>) -> bool + Send + Sync + 'static,
        mut on_true: Strategy<V, M, P>,
        mut on_false: Strategy<V, M, P>,
    ) -> Self {
        Strategy::new(move |context| {
            if condition(context) {
                on_true.next_move(context)
            } else {
                on_false.next_move(context)
            }
        })
    }

    /// Construct a new trigger strategy that plays the `before` strategy until `trigger` returns
    /// true, then plays the `after` strategy thereafter.
    pub fn trigger(
        mut trigger: impl FnMut(&Context<V, P>) -> bool + Send + Sync + 'static,
        mut before: Strategy<V, M, P>,
        mut after: Strategy<V, M, P>,
    ) -> Self {
        let mut triggered = false;
        Strategy::new(move |context| {
            if !triggered {
                triggered = trigger(context);
            }
            if triggered {
                after.next_move(context)
            } else {
                before.next_move(context)
            }
        })
    }

    /// Get the next move to play given the current play context.
    pub fn next_move(&mut self, context: &Context<V, P>) -> M {
        (self.next_move)(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use impls::impls;
    use test_log::test;

    #[test]
    fn strategy_is_send_sync() {
        assert!(impls!(Strategy<(), u8, 2>: Send & Sync));
    }
}
