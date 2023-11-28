use std::rc::Rc;

use crate::{Distribution, ErrorKind, Game, PerPlayer, PlayerIndex, Profile};

pub trait NextTurn<'g, T, G: Game<P> + 'g, const P: usize>:
    FnOnce(Rc<G::State>, T) -> Result<Turn<'g, G, P>, ErrorKind<G, P>> + 'g
{
}

impl<'g, F, T, G: Game<P> + 'g, const P: usize> NextTurn<'g, T, G, P> for F where
    F: FnOnce(Rc<G::State>, T) -> Result<Turn<'g, G, P>, ErrorKind<G, P>> + 'g
{
}

// TODO: Rename to Play or Specification?
pub struct Turn<'g, G: Game<P> + 'g, const P: usize> {
    /// The game state at this turn.
    pub state: Rc<G::State>,
    /// The action to take at this turn.
    pub action: Action<'g, G, P>,
}

/// The next action to performed while playing a game.
pub enum Action<'g, G: Game<P> + 'g, const P: usize> {
    /// One or more players play a move simultaneously.
    Players {
        to_move: Vec<PlayerIndex<P>>,
        next: Box<dyn NextTurn<'g, Vec<G::Move>, G, P>>,
    },

    /// A move of chance according to the given distribution.
    Chance {
        distribution: Distribution<G::Move>,
        next: Box<dyn NextTurn<'g, G::Move, G, P>>,
    },

    /// Award a payoff to the players and terminate the game.
    End { outcome: G::Outcome },
}

impl<'g, G: Game<P> + 'g, const P: usize> Action<'g, G, P> {
    pub fn player(to_move: PlayerIndex<P>, next: impl NextTurn<'g, G::Move, G, P>) -> Self {
        Action::players(vec![to_move], move |state, moves| {
            assert_eq!(moves.len(), 1);
            next(state, moves[0])
        })
    }

    pub fn players(
        to_move: Vec<PlayerIndex<P>>,
        next: impl NextTurn<'g, Vec<G::Move>, G, P>,
    ) -> Self {
        Action::Players {
            to_move,
            next: Box::new(next),
        }
    }

    pub fn all_players(next: impl NextTurn<'g, Profile<G::Move, P>, G, P>) -> Self {
        Action::players(PlayerIndex::all().collect(), move |state, moves| {
            assert_eq!(moves.len(), P);
            next(state, PerPlayer::new(moves.try_into().unwrap()))
        })
    }

    pub fn chance(
        distribution: Distribution<G::Move>,
        next: impl NextTurn<'g, G::Move, G, P>,
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

impl<'g, G: Game<P> + 'g, const P: usize> Turn<'g, G, P> {
    pub fn new(state: Rc<G::State>, action: Action<'g, G, P>) -> Self {
        Turn { state, action }
    }

    pub fn player(
        state: Rc<G::State>,
        player: PlayerIndex<P>,
        next: impl NextTurn<'g, G::Move, G, P>,
    ) -> Self {
        Turn::new(state, Action::player(player, next))
    }

    pub fn players(
        state: Rc<G::State>,
        players: Vec<PlayerIndex<P>>,
        next: impl NextTurn<'g, Vec<G::Move>, G, P>,
    ) -> Self {
        Turn::new(state, Action::players(players, next))
    }

    pub fn all_players(
        state: Rc<G::State>,
        next: impl NextTurn<'g, Profile<G::Move, P>, G, P>,
    ) -> Self {
        Turn::new(state, Action::all_players(next))
    }

    pub fn chance(
        state: Rc<G::State>,
        distribution: Distribution<G::Move>,
        next: impl NextTurn<'g, G::Move, G, P>,
    ) -> Self {
        Turn::new(state, Action::chance(distribution, next))
    }

    pub fn end(state: Rc<G::State>, outcome: G::Outcome) -> Self {
        Turn::new(state, Action::end(outcome))
    }
}
