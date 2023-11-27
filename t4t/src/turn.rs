use std::rc::Rc;

use crate::{Distribution, Game, PerPlayer, PlayerIndex, Profile};

// TODO: Rename to Play or Specification?
pub struct Turn<G: Game<P>, const P: usize> {
    /// The game state at this turn.
    pub state: Rc<G::State>,
    /// The action to take at this turn.
    pub action: Action<G, P>,
}

/// The next action to performed while playing a game.
pub enum Action<G: Game<P>, const P: usize> {
    /// One or more players play a move simultaneously.
    #[allow(clippy::type_complexity)]
    Players {
        to_move: Vec<PlayerIndex<P>>,
        next: Box<dyn FnOnce(Rc<G::State>, Vec<G::Move>) -> Turn<G, P>>,
    },

    /// A move of chance according to the given distribution.
    #[allow(clippy::type_complexity)]
    Chance {
        distribution: Distribution<G::Move>,
        next: Box<dyn FnOnce(Rc<G::State>, G::Move) -> Turn<G, P>>,
    },

    /// Award a payoff to the players and terminate the game.
    End { outcome: G::Outcome },
}

impl<G: Game<P>, const P: usize> Action<G, P> {
    pub fn player(
        to_move: PlayerIndex<P>,
        next: impl FnOnce(Rc<G::State>, G::Move) -> Turn<G, P> + 'static,
    ) -> Self {
        Action::players(vec![to_move], move |state, moves| {
            assert_eq!(moves.len(), 1);
            next(state, moves[0])
        })
    }

    pub fn players(
        to_move: Vec<PlayerIndex<P>>,
        next: impl FnOnce(Rc<G::State>, Vec<G::Move>) -> Turn<G, P> + 'static,
    ) -> Self {
        Action::Players {
            to_move,
            next: Box::new(next),
        }
    }

    pub fn all_players(
        next: impl FnOnce(Rc<G::State>, Profile<G::Move, P>) -> Turn<G, P> + 'static,
    ) -> Self {
        // Action::AllPlayers {
        //     next: Box::new(next),
        // }
        Action::players(PlayerIndex::all().collect(), move |state, moves| {
            assert_eq!(moves.len(), P);
            next(state, PerPlayer::new(moves.try_into().unwrap()))
        })
    }

    pub fn chance(
        distribution: Distribution<G::Move>,
        next: impl FnOnce(Rc<G::State>, G::Move) -> Turn<G, P> + 'static,
    ) -> Self {
        Action::Chance {
            distribution,
            next: Box::new(next),
        }
    }

    pub fn end(outcome: G::Outcome) -> Self {
        Action::End { outcome }
    }
}

impl<G: Game<P>, const P: usize> Turn<G, P> {
    pub fn new(state: Rc<G::State>, action: Action<G, P>) -> Self {
        Turn { state, action }
    }

    pub fn player(
        state: Rc<G::State>,
        player: PlayerIndex<P>,
        next: impl FnOnce(Rc<G::State>, G::Move) -> Turn<G, P> + 'static,
    ) -> Self {
        Turn::new(state, Action::player(player, next))
    }

    pub fn players(
        state: Rc<G::State>,
        players: Vec<PlayerIndex<P>>,
        next: impl FnOnce(Rc<G::State>, Vec<G::Move>) -> Turn<G, P> + 'static,
    ) -> Self {
        Turn::new(state, Action::players(players, next))
    }

    pub fn all_players(
        state: Rc<G::State>,
        next: impl FnOnce(Rc<G::State>, Profile<G::Move, P>) -> Turn<G, P> + 'static,
    ) -> Self {
        Turn::new(state, Action::all_players(next))
    }

    pub fn chance(
        state: Rc<G::State>,
        distribution: Distribution<G::Move>,
        next: impl FnOnce(Rc<G::State>, G::Move) -> Turn<G, P> + 'static,
    ) -> Self {
        Turn::new(state, Action::chance(distribution, next))
    }

    pub fn end(state: Rc<G::State>, outcome: G::Outcome) -> Self {
        Turn::new(state, Action::end(outcome))
    }
}
