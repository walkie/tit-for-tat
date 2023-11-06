use std::rc::Rc;

use crate::{Distribution, Game, Payoff, PlayerIndex, Profile};

// TODO: Rename to Play or Specification?
pub struct Turn<G: Game<P>, const P: usize> {
    pub state: Rc<G::State>,
    pub action: Action<G, P>,
}

/// The next action to performed while playing a game.
pub enum Action<G: Game<P>, const P: usize> {
    /// A turn taken by a single player.
    // Player {
    //     player: PlayerIndex<P>,
    //     next: Box<dyn FnOnce(Rc<G::State>, G::Move) -> Turn<G, P>>,
    // },

    /// A turn taken by one or more players simultaneously.
    Players {
        players: Vec<PlayerIndex<P>>,
        next: Box<dyn FnOnce(Rc<G::State>, Vec<G::Move>) -> Turn<G, P>>,
    },

    /// A turn taken by all players simultaneously.
    // AllPlayers {
    //     next: Box<dyn FnOnce(Rc<G::State>, Profile<G::Move, P>) -> Turn<G, P>>,
    // },

    /// A move of chance according to the given distribution.
    Chance {
        distribution: Distribution<G::Move>,
        next: Box<dyn FnOnce(Rc<G::State>, G::Move) -> Turn<G, P>>,
    },

    /// Award a payoff to the players and terminate the game.
    Payoff {
        payoff: Payoff<G::Utility, P>,
        outcome: Box<dyn FnOnce(Rc<G::State>, Payoff<G::Utility, P>) -> G::Outcome>,
    },
}

impl<G: Game<P>, const P: usize> Action<G, P> {
    pub fn player(
        player: PlayerIndex<P>,
        next: impl FnOnce(Rc<G::State>, G::Move) -> Turn<G, P>,
    ) -> Self {
        // Action::Player {
        //     player,
        //     next: Box::new(next),
        // }
        Action::players(vec![player], move |state, moves| {
            assert_eq!(moves.len(), 1);
            next(state, moves[0])
        })
    }

    pub fn players(
        players: Vec<PlayerIndex<P>>,
        next: impl FnOnce(Rc<G::State>, Vec<G::Move>) -> Turn<G, P>,
    ) -> Self {
        Action::Players {
            players,
            next: Box::new(next),
        }
    }

    pub fn all_players(next: impl FnOnce(Rc<G::State>, Profile<G::Move, P>) -> Turn<G, P>) -> Self {
        // Action::AllPlayers {
        //     next: Box::new(next),
        // }
        Action::players(PlayerIndex::all().collect(), move |state, moves| {
            assert_eq!(moves.len(), P);
            next(state, Profile::try_from(moves).unwrap())
        })
    }

    pub fn chance(
        distribution: Distribution<G::Move>,
        next: impl FnOnce(Rc<G::State>, G::Move) -> Turn<G, P>,
    ) -> Self {
        Action::Chance {
            distribution,
            next: Box::new(next),
        }
    }

    pub fn payoff(
        payoff: Payoff<G::Utility, P>,
        outcome: impl FnOnce(Rc<G::State>, Payoff<G::Utility, P>) -> G::Outcome,
    ) -> Self {
        Action::Payoff {
            payoff,
            outcome: Box::new(outcome),
        }
    }
}

impl<G: Game<P>, const P: usize> Turn<G, P> {
    pub fn new(state: Rc<G::State>, action: Action<G, P>) -> Self {
        Turn { state, action }
    }

    pub fn player(
        state: Rc<G::State>,
        player: PlayerIndex<P>,
        next: impl FnOnce(Rc<G::State>, G::Move) -> Turn<G, P>,
    ) -> Self {
        Turn::new(state, Action::player(player, next))
    }

    pub fn players(
        state: Rc<G::State>,
        players: Vec<PlayerIndex<P>>,
        next: impl FnOnce(Rc<G::State>, Vec<G::Move>) -> Turn<G, P>,
    ) -> Self {
        Turn::new(state, Action::players(players, next))
    }

    pub fn all_players(
        state: Rc<G::State>,
        next: impl FnOnce(Rc<G::State>, Profile<G::Move, P>) -> Turn<G, P>,
    ) -> Self {
        Turn::new(state, Action::all_players(next))
    }

    pub fn chance(
        state: Rc<G::State>,
        distribution: Distribution<G::Move>,
        next: impl FnOnce(Rc<G::State>, G::Move) -> Turn<G, P>,
    ) -> Self {
        Turn::new(state, Action::chance(distribution, next))
    }

    pub fn payoff(
        state: Rc<G::State>,
        payoff: Payoff<G::Utility, P>,
        outcome: impl FnOnce(Rc<G::State>, Payoff<G::Utility, P>) -> G::Outcome,
    ) -> Self {
        Turn::new(state, Action::payoff(payoff, outcome))
    }
}
