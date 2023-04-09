use crate::moves::IsMove;
use crate::normal::Normal;
use crate::outcome::Outcome;
use crate::payoff::{IsUtil, Payoff};
use crate::per_player::{PerPlayer, PlayerIndex};
use crate::player::Players;
use crate::simultaneous::Simultaneous;

/// The outcome of a successfully played game.
///
/// The builtin instances of this trait are:
/// - For simultaneous games: [Outcome](outcome::Outcome)
/// - For sequential games: [Transcript](transcript::Transcript)
/// - For iterated games: [History](iterated::History)
pub trait HasPayoff<Util: IsUtil, const N: usize> {
    /// Get the payoff associated with this outcome.
    fn payoff(&self) -> Payoff<Util, N>;
}

impl<Move, Util: IsUtil, const N: usize> HasPayoff<Util, N> for Outcome<Move, Util, N> {
    fn payoff(&self) -> Payoff<Util, N> {
        self.payoff
    }
}

/// An error caused by a player playing an invalid move.
pub struct InvalidMove<Move, State, const N: usize> {
    /// The player who made the invalid move.
    pub player: PlayerIndex<N>,
    /// The offending move.
    pub the_move: Move,
    /// The game state at the time the invalid move was played.
    pub state: State,
}

/// The result of playing a game. Either the outcome or an error if the game was aborted due to an
/// invalid move.
pub type PlayResult<Outcome, Move, State, const N: usize> =
    Result<Outcome, InvalidMove<Move, State, N>>;

/// An interface for playing games.
pub trait Playable<const N: usize> {
    /// The type of moves played by players in this game.
    type Move: IsMove;

    /// The type of utility values awarded to each player at the end of the game.
    type Util: IsUtil;

    /// The result of successfully playing a game.
    type Outcome: HasPayoff<Self::Util, N>;

    /// The type of the intermediate game state used while playing this game. This state can be
    /// used by the players of the game to implement their strategies.
    type State;

    /// Play the game with the given players, yielding a payoff if the game completed successfully,
    /// or an error, otherwise.
    fn play(
        &self,
        players: &mut Players<Self::Move, Self::State, N>,
    ) -> PlayResult<Self::Outcome, Self::Move, Self::State, N>;
}

impl<Move: IsMove, Util: IsUtil, const N: usize> Playable<N> for Simultaneous<Move, Util, N> {
    type Move = Move;
    type Util = Util;
    type Outcome = Outcome<Move, Util, N>;
    type State = ();

    fn play(
        &self,
        players: &mut Players<Move, (), N>,
    ) -> PlayResult<Outcome<Move, Util, N>, Move, (), N> {
        let profile = PerPlayer::generate(|i| players[i].next_move(&()));
        for i in PlayerIndex::all_indexes() {
            if !self.is_valid_move_for_player(i, profile[i]) {
                return Err(InvalidMove {
                    player: i,
                    the_move: profile[i],
                    state: (),
                });
            }
        }
        Ok(Outcome::new(profile, self.payoff(profile)))
    }
}

impl<Move: IsMove, Util: IsUtil, const N: usize> Playable<N> for Normal<Move, Util, N> {
    type Move = Move;
    type Util = Util;
    type Outcome = Outcome<Move, Util, N>;
    type State = ();

    fn play(
        &self,
        players: &mut Players<Move, (), N>,
    ) -> PlayResult<Outcome<Move, Util, N>, Move, (), N> {
        self.as_simultaneous().play(players)
    }
}
