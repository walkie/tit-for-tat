//! 2x2 simultaneous, symmetric, cooperation/defection games, e.g. prisoner's dilemma.

use tft::prelude::norm::*;

/// In a social dilemma game, each player may either cooperate or defect.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Move {
    Cooperate,
    Defect,
}

/// Short for `Move::Cooperate`.
pub const C: Move = Move::Cooperate;

/// Short for `Move::Defect`.
pub const D: Move = Move::Defect;

/// A 2x2 simultaneous, symmetric game in which each player may cooperate or defect.
///
/// In this module, I'm calling these "social dilemma" games, though that term usually has a
/// different (but overlapping) definition in the field.
pub struct Dilemma {
    game: Normal<Move, i32, 2>,
    utils: [i32; 4],
}

impl Dilemma {
    /// Create a new social dilemma game from the utility values for player `P0`.
    pub fn new(utils: [i32; 4]) -> Self {
        let game = Normal::symmetric(vec![C, D], Vec::from(utils)).unwrap();
        Dilemma { game, utils }
    }

    /// Get the normal form representation of this game.
    pub fn as_normal(&self) -> &Normal<Move, i32, 2> {
        &self.game
    }

    // Game library

    /// Construct a canonical [prisoner's dilemma](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma)
    /// game with utility values `[2, 0, 3, 1]`.
    ///
    /// In a prisoner's dilemma, mutual cooperation is better than mutual defection, but defection
    /// is the dominant strategy for each player.
    ///
    /// This version of the prisoner's dilemma is suitable for iterated play since the combined
    /// utility of mutual cooperation exceeds the combined utility of cooperate-defect.
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::norm::*;
    /// use tft_games::dilemma::*;
    ///
    /// let g = Dilemma::prisoners_dilemma();
    /// assert_eq!(
    ///     g.as_normal().pure_nash_equilibria(),
    ///     vec![PerPlayer::new([D, D])],
    /// );
    /// ```
    pub fn prisoners_dilemma() -> Self {
        Dilemma::new([2, 0, 3, 1])
    }

    /// Construct a [friend-or-foe](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma#Friend_or_Foe?)
    /// game with utility values `[1, 0, 2, 0]`.
    ///
    /// Friend-or-foe is related to the prisoner's dilemma except that mutual defection is only a
    /// weak Nash equilibrium and so is not risk dominant.
    pub fn friend_or_foe(&self) -> Self {
        Dilemma::new([1, 0, 2, 0])
    }

    /// Construct a canonical [stag hunt](https://en.wikipedia.org/wiki/Stag_hunt) game with
    /// utility values `[3, 0, 2, 1]`.
    ///
    /// In the canonical stag hunt, mutual cooperation and mutual defection are both Nash
    /// equilibria, with mutual cooperation being payoff dominant and mutual defection being
    /// [risk dominant](https://en.wikipedia.org/wiki/Risk_dominance).
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::norm::*;
    /// use tft_games::dilemma::*;
    ///
    /// let g = Dilemma::stag_hunt();
    /// assert_eq!(
    ///     g.as_normal().pure_nash_equilibria(),
    ///     vec![PerPlayer::new([C, C]), PerPlayer::new([D, D])],
    /// );
    /// ```
    pub fn stag_hunt() -> Self {
        Dilemma::new([3, 0, 2, 1])
    }

    /// Construct a canonical assurance game with utility values `[2, 0, 1, 1]`.
    ///
    /// An assurance game is a [stag hunt](https://en.wikipedia.org/wiki/Stag_hunt) where
    /// mutual defection is not [risk dominant](https://en.wikipedia.org/wiki/Risk_dominance).
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::norm::*;
    /// use tft_games::dilemma::*;
    ///
    /// let g = Dilemma::assurance_game();
    /// assert_eq!(
    ///     g.as_normal().pure_nash_equilibria(),
    ///     vec![PerPlayer::new([C, C]), PerPlayer::new([D, D])],
    /// );
    /// ```
    pub fn assurance_game() -> Self {
        Dilemma::new([2, 0, 1, 1])
    }

    /// Construct a new [hawk-dove](https://en.wikipedia.org/wiki/Evolutionary_game_theory#Hawk_dove)
    /// game.
    ///
    /// The constructed game will have utility values
    /// `[half_value, 0, half_value * 2, half_value - half_cost]` for player `P0`.
    ///
    /// In this formulation of the game, cooperation corresponds to the "dove" move, while
    /// defection corresponds to the "hawk" move.
    ///
    /// Note that the arguments to this function are *half* the value of the resource and *half*
    /// the cost of losing a fight. This is to avoid dividing the integer utility values by two and
    /// introducing rounding errors.
    ///
    /// One iteration of the hawk-dove game is equivalent to the following games, depending on the
    /// relative values of `half_value` and `half_cost`:
    /// - If `half_cost > half_value > 0` (most common), then it is an instance of
    ///   [chicken](https://en.wikipedia.org/wiki/Chicken_(game)).
    /// - If `half_value > half_cost > 0`, then it is an instance of
    ///   [prisoner's dilemma](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma).
    /// - If `half_value == half_cost > 0`, then it is an instance of
    ///   [friend-or-foe](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma#Friend_or_Foe?).
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::norm::*;
    /// use tft_games::dilemma::*;
    ///
    /// let more_dove = Dilemma::hawk_dove(2, 3);
    /// let more_hawk = Dilemma::hawk_dove(3, 2);
    ///
    /// assert!(more_dove.is_chicken());
    /// assert!(!more_dove.is_prisoners_dilemma());
    /// assert!(more_hawk.is_prisoners_dilemma());
    /// assert!(!more_hawk.is_chicken());
    ///
    /// assert_eq!(
    ///     more_dove.as_normal().pure_nash_equilibria(),
    ///     vec![PerPlayer::new([C, D]), PerPlayer::new([D, C])],
    /// );
    /// assert_eq!(
    ///     more_hawk.as_normal().pure_nash_equilibria(),
    ///     vec![PerPlayer::new([D, D])],
    /// );
    /// ```
    pub fn hawk_dove(half_value: i32, half_cost: i32) -> Self {
        Dilemma::new([half_value, 0, half_value * 2, half_value - half_cost])
    }

    /// Construct a new game of [chicken](https://en.wikipedia.org/wiki/Chicken_(game)) with the
    /// given utility value for crashing (corresponding to defect-defect).
    ///
    /// The constructed game will have utility values `[0, -1, 1, -crash]` for player `P0`.
    ///
    /// In a game of chicken, the value of crash (which will be negated) should be greater than
    /// `1`, and usually significantly so. However, this is not enforced by this function.
    ///
    /// For more, see [`Dilemma::is_chicken()`].
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::norm::*;
    /// use tft_games::dilemma::*;
    ///
    /// let g = Dilemma::chicken(100);
    ///
    /// assert_eq!(
    ///     g.as_normal().pure_nash_equilibria(),
    ///     vec![PerPlayer::new([C, D]), PerPlayer::new([D, C])],
    /// );
    /// ```
    pub fn chicken(crash: i32) -> Self {
        Dilemma::new([0, -1, 1, -crash])
    }

    /// Construct a [snowdrift](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma#Iterated_snowdrift)
    /// game with utility values `[2, 1, 3, 0]`.
    ///
    /// Snowdrift is a [chicken](https://en.wikipedia.org/wiki/Chicken_(game)) game created by
    /// swapping the defect-defect and cooperate-defect utilities from the prisoner's dilemma. It
    /// reflects scenarios where cooperating yields benefits for both players, even when one player
    /// defects (e.g. when attempting to collaborate to clear a snowdrift blocking a path).
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::norm::*;
    /// use tft_games::dilemma::*;
    ///
    /// let g = Dilemma::snowdrift();
    ///
    /// assert!(g.is_chicken());
    /// assert!(!g.is_prisoners_dilemma());
    /// assert_eq!(
    ///     g.as_normal().pure_nash_equilibria(),
    ///     vec![PerPlayer::new([C, D]), PerPlayer::new([D, C])],
    /// );
    /// ```
    pub fn snowdrift() -> Self {
        Dilemma::new([2, 1, 3, 0])
    }

    // Recognizing game forms

    /// Is this game a [prisoner's dilemma](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma)?
    ///
    /// In a prisoner's dilemma, mutual cooperation is better than mutual defection, but defection
    /// is the dominant strategy for each player.
    ///
    /// # Examples
    /// ```
    /// use tft_games::dilemma::*;
    ///
    /// assert!(Dilemma::prisoners_dilemma().is_prisoners_dilemma());
    /// assert!(Dilemma::new([3, 0, 7, 2]).is_prisoners_dilemma());
    /// assert!(!Dilemma::stag_hunt().is_prisoners_dilemma());
    /// assert!(!Dilemma::assurance_game().is_prisoners_dilemma());
    /// ```
    pub fn is_prisoners_dilemma(&self) -> bool {
        let [cc, cd, dc, dd] = self.utils;
        dc > cc && cc > dd && dd > cd
    }

    /// Is this game a [prisoner's dilemma](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma)
    /// that is suitable for iterated play?
    ///
    /// In an iterated prisoner's dilemma, the total utility of mutual cooperation (i.e. the sum of
    /// both player's utility) should exceed the total utility of one player cooperating while the
    /// other defects. This rewards coordinating on mutual cooperation over coordinating on
    /// alternating cooperate-defect cycles.
    ///
    /// # Examples
    /// ```
    /// use tft_games::dilemma::*;
    ///
    /// assert!(Dilemma::prisoners_dilemma().is_iterated_prisoners_dilemma());
    /// assert!(!Dilemma::new([3, 0, 7, 2]).is_iterated_prisoners_dilemma());
    /// assert!(!Dilemma::stag_hunt().is_iterated_prisoners_dilemma());
    /// assert!(!Dilemma::assurance_game().is_iterated_prisoners_dilemma());
    /// ```
    pub fn is_iterated_prisoners_dilemma(&self) -> bool {
        let [cc, cd, dc, _dd] = self.utils;
        self.is_prisoners_dilemma() && 2 * cc > cd + dc
    }

    /// Is this game a [stag hunt](https://en.wikipedia.org/wiki/Stag_hunt)?
    ///
    /// In a stag hunt, both cooperate-cooperate and defect-defect are Nash equilibria, but
    /// cooperate-cooperate yields a higher utility for each.
    ///
    /// # Examples
    /// ```
    /// use tft_games::dilemma::*;
    ///
    /// assert!(Dilemma::stag_hunt().is_stag_hunt());
    /// assert!(Dilemma::assurance_game().is_stag_hunt());
    /// assert!(!Dilemma::prisoners_dilemma().is_stag_hunt());
    /// ```
    pub fn is_stag_hunt(&self) -> bool {
        let [cc, cd, dc, dd] = self.utils;
        cc > dc && dc >= dd && dd > cd
    }

    /// Is this game an assurance game?
    ///
    /// An assurance game is a special case of a [stag hunt](https://en.wikipedia.org/wiki/Stag_hunt)
    /// where mutual defection is not risk-dominant, i.e. the utility earned by defecting is the
    /// same regardless of whether the other player cooperates. This increases the incentive to
    /// cooperate.
    ///
    /// # Examples
    /// ```
    /// use tft_games::dilemma::*;
    ///
    /// assert!(Dilemma::assurance_game().is_assurance_game());
    /// assert!(!Dilemma::stag_hunt().is_assurance_game());
    /// assert!(!Dilemma::prisoners_dilemma().is_assurance_game());
    /// ```
    pub fn is_assurance_game(&self) -> bool {
        let [cc, cd, dc, dd] = self.utils;
        cc > dc && dc == dd && dd > cd
    }

    /// Is this a game of [chicken](https://en.wikipedia.org/wiki/Chicken_(game))?
    ///
    /// In chicken, the outcomes can be described from the perspective of player `P0` as:
    /// - cooperate-cooperate -- "tie"
    /// - cooperate-defect -- "lose"
    /// - defect-cooperate -- "win"
    /// - defect-defect -- "crash"
    ///
    /// These outcomes should be ranked as "win" > "tie" > "lose" > "crash".
    ///
    /// In classical chicken, the utility value of "crash" will be much lower than "lose"
    /// (capturing the idea that death is much worse than shame). However, any game that satisfies
    /// the basic relationships described above qualifies as a game of chicken.
    ///
    /// # Examples
    /// ```
    /// use tft_games::dilemma::*;
    ///
    /// assert!(Dilemma::chicken(100).is_chicken());
    /// assert!(Dilemma::hawk_dove(1, 2).is_chicken());
    /// assert!(Dilemma::snowdrift().is_chicken());
    /// assert!(!Dilemma::prisoners_dilemma().is_chicken());
    /// assert!(!Dilemma::stag_hunt().is_chicken());
    /// ```
    pub fn is_chicken(&self) -> bool {
        let [tie, lose, win, crash] = self.utils;
        win > tie && tie > lose && lose > crash
    }
}
