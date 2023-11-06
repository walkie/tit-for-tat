use std::rc::Rc;

use crate::{Game, Payoff, Transcript};

/// Execution context.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Exec<G: Game<P>, const P: usize> {
    pub(crate) game_state: G::State,
    pub(crate) transcript: Rc<Transcript<G::Move, P>>,
    pub(crate) score: Payoff<G::Utility, P>,
}

impl<G: Game<P>, const P: usize> Exec<G, P> {
    pub(crate) fn new(initial_state: G::State) -> Self {
        Exec {
            game_state: initial_state,
            transcript: Rc::new(Transcript::new()),
            score: Payoff::zeros(),
        }
    }
}
