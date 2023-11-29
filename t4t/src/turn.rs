use std::rc::Rc;

use crate::{Distribution, ErrorKind, Move, PerPlayer, PlayerIndex, Profile};

pub trait NextTurn<'g, T, S, M, O, const P: usize>:
    FnOnce(Rc<S>, T) -> Result<Turn<'g, S, M, O, P>, ErrorKind<M, P>> + 'g
{
}

impl<'g, F, T, S, M, O, const P: usize> NextTurn<'g, T, S, M, O, P> for F where
    F: FnOnce(Rc<S>, T) -> Result<Turn<'g, S, M, O, P>, ErrorKind<M, P>> + 'g
{
}

// TODO: Rename to Play or Specification?
pub struct Turn<'g, S, M, O, const P: usize> {
    /// The game state at this turn.
    pub state: Rc<S>,
    /// The action to take at this turn.
    pub action: Action<'g, S, M, O, P>,
}

/// The next action to performed while playing a game.
pub enum Action<'g, S, M, O, const P: usize> {
    /// One or more players play a move simultaneously.
    Players {
        to_move: Vec<PlayerIndex<P>>,
        next: Box<dyn NextTurn<'g, Vec<M>, S, M, O, P>>,
    },

    /// A move of chance according to the given distribution.
    Chance {
        distribution: Distribution<M>,
        next: Box<dyn NextTurn<'g, M, S, M, O, P>>,
    },

    /// Award a payoff to the players and terminate the game.
    End { outcome: O },
}

impl<'g, S, M: Move, O, const P: usize> Action<'g, S, M, O, P> {
    pub fn player(to_move: PlayerIndex<P>, next: impl NextTurn<'g, M, S, M, O, P>) -> Self {
        Action::players(vec![to_move], move |state, moves| {
            assert_eq!(moves.len(), 1);
            next(state, moves[0])
        })
    }

    pub fn players(
        to_move: Vec<PlayerIndex<P>>,
        next: impl NextTurn<'g, Vec<M>, S, M, O, P>,
    ) -> Self {
        Action::Players {
            to_move,
            next: Box::new(next),
        }
    }

    pub fn all_players(next: impl NextTurn<'g, Profile<M, P>, S, M, O, P>) -> Self {
        Action::players(PlayerIndex::all().collect(), move |state, moves| {
            assert_eq!(moves.len(), P);
            next(state, PerPlayer::new(moves.try_into().unwrap()))
        })
    }

    pub fn chance(distribution: Distribution<M>, next: impl NextTurn<'g, M, S, M, O, P>) -> Self {
        Action::Chance {
            distribution,
            next: Box::new(next),
        }
    }

    pub fn end(outcome: O) -> Self {
        Action::End { outcome }
    }
}

impl<'g, S, M: Move, O, const P: usize> Turn<'g, S, M, O, P> {
    pub fn new(state: Rc<S>, action: Action<'g, S, M, O, P>) -> Self {
        Turn { state, action }
    }

    pub fn player(
        state: Rc<S>,
        player: PlayerIndex<P>,
        next: impl NextTurn<'g, M, S, M, O, P>,
    ) -> Self {
        Turn::new(state, Action::player(player, next))
    }

    pub fn players(
        state: Rc<S>,
        players: Vec<PlayerIndex<P>>,
        next: impl NextTurn<'g, Vec<M>, S, M, O, P>,
    ) -> Self {
        Turn::new(state, Action::players(players, next))
    }

    pub fn all_players(state: Rc<S>, next: impl NextTurn<'g, Profile<M, P>, S, M, O, P>) -> Self {
        Turn::new(state, Action::all_players(next))
    }

    pub fn chance(
        state: Rc<S>,
        distribution: Distribution<M>,
        next: impl NextTurn<'g, M, S, M, O, P>,
    ) -> Self {
        Turn::new(state, Action::chance(distribution, next))
    }

    pub fn end(state: Rc<S>, outcome: O) -> Self {
        Turn::new(state, Action::end(outcome))
    }
}
