use crate::{Distribution, Game, Payoff, PlayerIndex, Profile};

// TODO: Rename to Play or Specification?
pub struct Turn<G: Game<P>, const P: usize> {
    pub state: G::State,
    pub action: Action<G, P>,
}

/// The next action to performed while playing a game.
pub enum Action<G: Game<P>, const P: usize> {
    /// A turn taken by a single player.
    // Player {
    //     player: PlayerIndex<P>,
    //     next: Box<dyn FnOnce(G::State, G::Move) -> Turn<G, P>>,
    // },

    /// A turn taken by one or more players simultaneously.
    Players {
        players: Vec<PlayerIndex<P>>,
        next: Box<dyn FnOnce(G::State, Vec<G::Move>) -> Turn<G, P>>,
    },

    /// A turn taken by all players simultaneously.
    // AllPlayers {
    //     next: Box<dyn FnOnce(G::State, Profile<G::Move, P>) -> Turn<G, P>>,
    // },

    /// A move of chance according to the given distribution.
    Chance {
        distribution: Distribution<G::Move>,
        next: Box<dyn FnOnce(G::State, G::Move) -> Turn<G, P>>,
    },

    /// Award a payoff to the players and possibly terminate the game.
    Payoff {
        payoff: Payoff<G::Utility, P>,
        next: Option<dyn FnOnce(G::State) -> Turn<G, P>>,
    },
}

impl<G: Game<P>, const P: usize> Action<G, P> {
    pub fn player(
        player: PlayerIndex<P>,
        next: impl FnOnce(G::State, G::Move) -> Turn<G, P>,
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
        next: impl FnOnce(G::State, Vec<G::Move>) -> Turn<G, P>,
    ) -> Self {
        Action::Players {
            players,
            next: Box::new(next),
        }
    }

    pub fn all_players(next: impl FnOnce(G::State, Profile<G::Move, P>) -> Turn<G, P>) -> Self {
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
        next: impl FnOnce(G::State, G::Move) -> Turn<G, P>,
    ) -> Self {
        Action::Chance {
            distribution,
            next: Box::new(next),
        }
    }

    pub fn terminal_payoff(payoff: Payoff<G::Utility, P>) -> Self {
        Action::Payoff { payoff, next: None }
    }

    pub fn intermediate_payoff(
        payoff: Payoff<G::Utility, P>,
        next: impl FnOnce(G::State) -> Turn<G, P>,
    ) -> Self {
        Action::Payoff {
            payoff,
            next: Some(Box::new(next)),
        }
    }
}

impl<G: Game<P>, const P: usize> Turn<G, P> {
    pub fn new(state: G::State, action: Action<G, P>) -> Self {
        Turn { state, action }
    }

    pub fn player(
        state: G::State,
        player: PlayerIndex<P>,
        next: impl FnOnce(G::State, G::Move) -> Turn<G, P>,
    ) -> Self {
        Turn::new(state, Action::player(player, next))
    }

    pub fn players(
        state: G::State,
        players: Vec<PlayerIndex<P>>,
        next: impl FnOnce(G::State, Vec<G::Move>) -> Turn<G, P>,
    ) -> Self {
        Turn::new(state, Action::players(players, next))
    }

    pub fn all_players(
        state: G::State,
        next: impl FnOnce(G::State, Profile<G::Move, P>) -> Turn<G, P>,
    ) -> Self {
        Turn::new(state, Action::all_players(next))
    }

    pub fn chance(
        state: G::State,
        distribution: Distribution<G::Move>,
        next: impl FnOnce(G::State, G::Move) -> Turn<G, P>,
    ) -> Self {
        Turn::new(state, Action::chance(distribution, next))
    }

    pub fn terminal_payoff(state: G::State, payoff: Payoff<G::Utility, P>) -> Self {
        Turn::new(state, Action::terminal_payoff(payoff))
    }

    pub fn intermediate_payoff(
        state: G::State,
        payoff: Payoff<G::Utility, P>,
        next: impl FnOnce(G::State) -> Turn<G, P>,
    ) -> Self {
        Turn::new(state, Action::intermediate_payoff(payoff, next))
    }
}
