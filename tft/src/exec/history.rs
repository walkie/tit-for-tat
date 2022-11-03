use crate::core::PlayerIndex;

pub struct PlayedMove<Move, const N: usize> {
    player: Option<PlayerIndex<N>>,
    the_move: Move,
}

pub type Transcript<Move, const N: usize> = Vec<PlayedMove<Move, N>>;

type SimHistory = Vec<Outcome

pub struct PlayedSimIter<Move, const N: usize> {
    profile: Profile<Move, N>,
}
