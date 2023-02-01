use crate::moves::IsMove;
use crate::payoff::IsUtil;
use crate::payoff::Payoff;
use crate::per_player::PlayerIndex;
use crate::player::Players;

/// An error caused by a player playing an invalid move.
pub struct InvalidMove<Move, const N: usize> {
    pub player: PlayerIndex<N>,
    pub the_move: Move,
}

/// An abstract interface for playing games.
pub trait Game<const N: usize> {
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
        players: &Players<Self::Move, Self::State, N>,
    ) -> Result<Payoff<Self::Util, N>, InvalidMove<Self::Move, N>>;
}
