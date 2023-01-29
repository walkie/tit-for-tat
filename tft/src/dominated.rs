/// Captures a domination relationship between moves in a simultaneous move game.
///
/// A move `m1` is *strictly dominated* by another move `m2` for player `p` if, for any possible
/// moves played by other players, changing from `m1` to `m2` increases `p`'s utility.
///
/// A move `m1` is *weakly dominated* by another move `m2` for player `p` if, for any possible
/// moves played by other players, changing from `m1` to `m2` does not decrease `p`'s utility.
///
/// Note that `m1` and `m2` may weakly dominate each other if the two moves are equivalent, that
/// is, if they always yield the same utility in otherwise identical profiles.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Dominated<Move> {
    /// The move that is dominated, i.e. yields a worse utility.
    pub dominated: Move,
    /// The move that is dominates the dominated move, i.e. yields a better utility.
    pub dominator: Move,
    /// Is the domination relationship strict? If `true`, the `dominator` always yields a greater
    /// utility. If `false`, the `dominator` always yields a greater or equal utility.
    pub is_strict: bool,
}

impl<Move> Dominated<Move> {
    /// Construct a strict domination relationship.
    pub fn strict(dominated: Move, dominator: Move) -> Self {
        Dominated {
            dominated,
            dominator,
            is_strict: true,
        }
    }

    /// Construct a weak domination relationship.
    pub fn weak(dominated: Move, dominator: Move) -> Self {
        Dominated {
            dominated,
            dominator,
            is_strict: false,
        }
    }
}
