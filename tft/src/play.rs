use crate::moves::IsMove;
use crate::normal::Normal;
use crate::payoff::IsUtil;
use crate::payoff::Payoff;
use crate::per_player::{PerPlayer, PlayerIndex};
use crate::player::Players;
use crate::simultaneous::Simultaneous;

/// An error caused by a player playing an invalid move.
pub struct InvalidMove<Move, const N: usize> {
    pub player: PlayerIndex<N>,
    pub the_move: Move,
}

/// An abstract interface for playing games.
pub trait Playable<const N: usize> {
    /// The type of moves played by players in this game.
    type Move: IsMove;

    /// The type of utility values awarded to each player at the end of the game.
    type Util: IsUtil;

    /// The type of the intermediate game state used while playing this game. This state can be
    /// used by the players of the game to implement their strategies.
    type State;

    /// Play the game with the given players, yielding a payoff if the game completed successfully,
    /// or an error, otherwise.
    fn play(
        &self,
        players: &mut Players<Self::Move, Self::State, N>,
    ) -> Result<Payoff<Self::Util, N>, InvalidMove<Self::Move, N>>;
}

impl<Move: IsMove, Util: IsUtil, const N: usize> Playable<N> for Simultaneous<Move, Util, N> {
    type Move = Move;
    type Util = Util;
    type State = ();

    fn play(
        &self,
        players: &mut Players<Self::Move, Self::State, N>,
    ) -> Result<Payoff<Self::Util, N>, InvalidMove<Self::Move, N>> {
        let profile = PerPlayer::generate(|i| players[i].next_move(&()));
        for i in PlayerIndex::all_indexes() {
            if !self.is_valid_move_for_player(i, profile[i]) {
                return Err(InvalidMove { player: i, the_move: profile[i] });
            }
        }
        Ok(self.payoff(profile))
    }
}

impl<Move: IsMove, Util: IsUtil, const N: usize> Playable<N> for Normal<Move, Util, N> {
    type Move = Move;
    type Util = Util;
    type State = ();

    fn play(
        &self,
        players: &mut Players<Self::Move, Self::State, N>,
    ) -> Result<Payoff<Self::Util, N>, InvalidMove<Self::Move, N>> {
        self.as_simultaneous().play(players)
    }
}
