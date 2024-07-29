//! Prisoner's dilemma and related games and strategies.
//!
//! This module defines the prisoner's dilemma and several other 2x2 simultaneous, symmetric,
//! cooperation/defection games.
//!
//! The players are defined as free functions in this module that produce the described player.
//!
//! The games are defined as constructor functions attached to the
//! [`Dilemma`][crate::dilemma::Dilemma] type.
//!
//!
//! # Examples
//!
//! The following example runs several tournaments using the players and games defined in this
//! module, accumulating the scores for each player across each tournament.
//!
//! You can run this example to see the resulting scores by downloading this crate and running:
//! ```bash
//! $ cargo run --example axelrod
//! ```
//!
//! The example program is named for [Robert Axelrod](https://en.wikipedia.org/wiki/Robert_Axelrod),
//! whose famous prisoner's dilemma tournaments are one of the best known applications of
//! experimental game theory.
//!
//! ```
#![doc = include_str!("../examples/axelrod.rs")]
//! ```

use t4t::*;

/// In a social dilemma game, each player may either cooperate or defect.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Move {
    Cooperate,
    Defect,
}

impl Move {
    /// Get the opposite move.
    pub fn opposite(&self) -> Self {
        match self {
            Move::Cooperate => Move::Defect,
            Move::Defect => Move::Cooperate,
        }
    }

    /// Get this move as a single character, 'C' or 'D'.
    pub fn to_char(&self) -> char {
        match self {
            Move::Cooperate => 'C',
            Move::Defect => 'D',
        }
    }
}

/// Short for `Move::Cooperate`.
pub const C: Move = Move::Cooperate;

/// Short for `Move::Defect`.
pub const D: Move = Move::Defect;

/// A 2x2 simultaneous, symmetric game in which each player may cooperate or defect.
///
/// In this module, I'm calling these "social dilemma" games, though that term usually has a
/// different (but overlapping) definition in the field.
#[derive(Clone)]
pub struct Dilemma {
    game: Normal<Move, i64, 2>,
    utils: [i64; 4],
}

impl Dilemma {
    /// Create a new social dilemma game from the utility values for player `P0`.
    pub fn new(utils: [i64; 4]) -> Self {
        let game = Normal::symmetric(vec![C, D], Vec::from(utils)).unwrap();
        Dilemma { game, utils }
    }

    /// Convert this game into its normal-form representation.
    pub fn into_normal(self) -> Normal<Move, i64, 2> {
        self.game
    }

    /// Get the normal-form representation of this game.
    pub fn as_normal(&self) -> &Normal<Move, i64, 2> {
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
    /// use t4t::*;
    /// use t4t_games::dilemma::*;
    ///
    /// let g = Dilemma::prisoners_dilemma();
    ///
    /// let nash = g.as_normal().pure_nash_equilibria_parallel();
    /// assert_eq!(nash, vec![Profile::new([D, D])]);
    ///
    /// let mut pareto = g.as_normal().pareto_optimal_solutions_parallel();
    /// pareto.sort();
    /// assert_eq!(pareto, vec![Profile::new([C, C]), Profile::new([C, D]), Profile::new([D, C])]);
    /// ```
    pub fn prisoners_dilemma() -> Self {
        Dilemma::new([2, 0, 3, 1])
    }

    /// Construct a [friend-or-foe](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma#Friend_or_Foe?)
    /// game with utility values `[1, 0, 2, 0]`.
    ///
    /// Friend-or-foe is related to the prisoner's dilemma except that mutual defection is only a
    /// weak Nash equilibrium and so is not
    /// [risk dominant](https://en.wikipedia.org/wiki/Risk_dominance).
    pub fn friend_or_foe() -> Self {
        Dilemma::new([1, 0, 2, 0])
    }

    /// Construct a canonical [stag hunt](https://en.wikipedia.org/wiki/Stag_hunt) game with
    /// utility values `[3, 0, 2, 1]`.
    ///
    /// In the canonical stag hunt, mutual cooperation and mutual defection are both Nash
    /// equilibria, with mutual cooperation being payoff dominant
    /// (i.e. [Pareto optimal](https://en.wikipedia.org/wiki/Pareto_efficiency))
    /// and mutual defection being [risk dominant](https://en.wikipedia.org/wiki/Risk_dominance).
    ///
    /// # Examples
    /// ```
    /// use t4t::*;
    /// use t4t_games::dilemma::*;
    ///
    /// let g = Dilemma::stag_hunt();
    ///
    /// let nash = g.as_normal().pure_nash_equilibria_parallel();
    /// assert_eq!(nash, vec![Profile::new([C, C]), Profile::new([D, D])]);
    ///
    /// let mut pareto = g.as_normal().pareto_optimal_solutions_parallel();
    /// pareto.sort();
    /// assert_eq!(pareto, vec![Profile::new([C, C])]);
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
    /// use t4t::*;
    /// use t4t_games::dilemma::*;
    ///
    /// let g = Dilemma::assurance_game();
    /// assert_eq!(
    ///     g.as_normal().pure_nash_equilibria(),
    ///     vec![Profile::new([C, C]), Profile::new([D, D])],
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
    /// use t4t::*;
    /// use t4t_games::dilemma::*;
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
    ///     vec![Profile::new([C, D]), Profile::new([D, C])],
    /// );
    /// assert_eq!(
    ///     more_hawk.as_normal().pure_nash_equilibria(),
    ///     vec![Profile::new([D, D])],
    /// );
    /// ```
    pub fn hawk_dove(half_value: i64, half_cost: i64) -> Self {
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
    /// use t4t::*;
    /// use t4t_games::dilemma::*;
    ///
    /// let g = Dilemma::chicken(100);
    ///
    /// assert_eq!(
    ///     g.as_normal().pure_nash_equilibria(),
    ///     vec![Profile::new([C, D]), Profile::new([D, C])],
    /// );
    /// ```
    pub fn chicken(crash: i64) -> Self {
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
    /// use t4t::*;
    /// use t4t_games::dilemma::*;
    ///
    /// let g = Dilemma::snowdrift();
    ///
    /// assert!(g.is_chicken());
    /// assert!(!g.is_prisoners_dilemma());
    /// assert_eq!(
    ///     g.as_normal().pure_nash_equilibria(),
    ///     vec![Profile::new([C, D]), Profile::new([D, C])],
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
    /// use t4t_games::dilemma::*;
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
    /// use t4t_games::dilemma::*;
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
    /// use t4t_games::dilemma::*;
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
    /// use t4t_games::dilemma::*;
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
    /// use t4t_games::dilemma::*;
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

impl Game<2> for Dilemma {
    type Move = Move;
    type Utility = i64;
    type State = ();
    type View = ();
    fn state_view(&self, _state: &(), _player: PlayerIndex<2>) {}
}

impl Playable<2> for Dilemma {
    type Outcome = SimultaneousOutcome<Move, i64, 2>;

    fn into_game_tree(self) -> GameTree<(), Move, i64, SimultaneousOutcome<Move, i64, 2>, 2> {
        self.into_normal().into_game_tree()
    }
}

// Strategies

/// A player in a repeated social dilemma game.
pub type DilemmaPlayer = Player<Repeated<Dilemma, 2>, 2>;

/// The strategic context in a repeated social dilemma game.
pub type DilemmaContext = Context<RepeatedState<Dilemma, 2>, 2>;

/// A player that always cooperates.
pub fn cooperator() -> DilemmaPlayer {
    Player::new("Cooperator".to_string(), || Strategy::pure(C))
}

/// A player that always defects.
pub fn defector() -> DilemmaPlayer {
    Player::new("Defector".to_string(), || Strategy::pure(D))
}

/// Construct a player that plays a periodic sequence of moves.
pub fn periodic(moves: Vec<Move>) -> DilemmaPlayer {
    let name = format!(
        "Periodic ({})*",
        String::from_iter(moves.iter().map(|m| m.to_char()))
    );

    Player::new(name, move || Strategy::periodic_pure(moves.clone()))
}

/// A player that plays randomly with a 1:1 expected ratio of cooperation to defection.
pub fn random() -> DilemmaPlayer {
    Player::new("Random 1:1".to_string(), || {
        Strategy::mixed_flat(vec![C, D]).unwrap()
    })
}

/// A player that plays randomly with a 2:1 expected ratio of cooperation to defection.
pub fn random_ccd() -> DilemmaPlayer {
    Player::new("Random 2:1".to_string(), || {
        Strategy::mixed_flat(vec![C, C, D]).unwrap()
    })
}

/// A player that cooperates on the first move then copies the opponent's previous move.
///
/// Tit-for-Tat can be thought of as a strategy that prefers to cooperate but will retaliate if the
/// opponent defects.
pub fn tit_for_tat() -> DilemmaPlayer {
    Player::new("Tit-for-Tat".to_string(), || {
        Strategy::new(|context: &DilemmaContext| {
            context
                .state_view()
                .history()
                .moves_for_player(context.their_index())
                .last()
                .unwrap_or(C)
        })
    })
}

/// A player that defects on the first move then copies the opponent's last move.
///
/// Like [Tit-for-Tat](tit_for_tat) but defects on the first move.
pub fn suspicious_tit_for_tat() -> DilemmaPlayer {
    Player::new("Suspicious Tit-for-Tat".to_string(), || {
        Strategy::new(|context: &DilemmaContext| {
            context
                .state_view()
                .history()
                .moves_for_player(context.their_index())
                .last()
                .unwrap_or(D)
        })
    })
}

/// A player that cooperates unless the opponent has defected in *each* of the last `n` games.
///
/// Like [Tit-for-Tat](tit_for_tat) but more lenient: it only retaliates if the opponent has
/// defected `n` times in a row.
pub fn tit_for_n_tats(n: usize) -> DilemmaPlayer {
    Player::new(format!("Tit-for-{}-Tats", n), move || {
        Strategy::new(move |context: &DilemmaContext| {
            let last_n: Vec<Move> = context
                .state_view()
                .history()
                .moves_for_player(context.their_index())
                .rev()
                .take(n)
                .collect();

            if last_n.len() == n && last_n.iter().all(|m| *m == D) {
                D
            } else {
                C
            }
        })
    })
}

/// A player that cooperates unless the opponent has defected in *any* of the last `n` moves.
///
/// Like [Tit-for-Tat](tit_for_tat) but more vindictive: it retaliates to a single defection by
/// defecting `n` times in a row.
pub fn n_tits_for_tat(n: usize) -> DilemmaPlayer {
    Player::new(format!("{}-Tits-for-Tat", n), move || {
        Strategy::new(move |context: &DilemmaContext| {
            let last_n: Vec<Move> = context
                .state_view()
                .history()
                .moves_for_player(context.their_index())
                .rev()
                .take(n)
                .collect();

            if last_n.contains(&D) {
                D
            } else {
                C
            }
        })
    })
}

/// A player that plays [Tit-for-Tat](tit_for_tat) but draws its moves from an `on_cooperate`
/// distribution when it would cooperate and from an `on_defect` distribution when it would defect.
///
/// The `on_cooperate` distribution should typically be weighted toward cooperation with a small
/// chance of defection, and vice versa for the `on_defect` distribution.
///
/// The `name_suffix` argument is appended to the player's name to enable playing multiple
/// Probabilistic Tit-for-Tat players with different distributions.
pub fn probabilistic_tit_for_tat(
    on_cooperate: Distribution<Move>,
    on_defect: Distribution<Move>,
    name_suffix: &str,
) -> DilemmaPlayer {
    Player::new(
        format!("Probabilistic Tit-for-Tat {}", name_suffix),
        move || {
            Strategy::conditional(
                |context: &DilemmaContext| {
                    context
                        .state_view()
                        .history()
                        .moves_for_player(context.their_index())
                        .last()
                        .map_or(false, |m| m == D)
                },
                Strategy::mixed(on_defect.clone()),
                Strategy::mixed(on_cooperate.clone()),
            )
        },
    )
}

/// A [Probabilistic Tit-for-Tat](probabilistic_tit_for_tat) player that always cooperates when it
/// should but has a 10% chance of cooperating when it would otherwise defect.
pub fn generous_tit_for_tat() -> DilemmaPlayer {
    probabilistic_tit_for_tat(
        Distribution::singleton(C),
        Distribution::new(vec![(D, 0.9), (C, 0.1)]).unwrap(),
        "- Generous (10%)",
    )
}

/// A [Probabilistic Tit-for-Tat](probabilistic_tit_for_tat) player that always defects when it
/// should but has a 10% chance of defecting when it would otherwise cooperate.
pub fn probing_tit_for_tat() -> DilemmaPlayer {
    probabilistic_tit_for_tat(
        Distribution::new(vec![(C, 0.9), (D, 0.1)]).unwrap(),
        Distribution::singleton(D),
        "- Probing (10%)",
    )
}

/// A player that cooperates unless it was the "sucker" (it cooperated but the opponent defected)
/// in the previous game.
///
/// Different from [Tit-for-Tat](tit_for_tat) since it will cooperate after mutual defection.
pub fn firm_but_fair() -> DilemmaPlayer {
    Player::new("Firm-but-Fair".to_string(), || {
        Strategy::new(|context: &DilemmaContext| {
            context
                .state_view()
                .history()
                .profiles()
                .last()
                .map_or(C, |profile| {
                    if profile[context.my_index()] == C && profile[context.their_index()] == D {
                        D
                    } else {
                        C
                    }
                })
        })
    })
}

/// A player that cooperates on the first move, thereafter it repeats its previous move if the
/// opponent cooperated or else flips its move if the opponent defected.
pub fn pavlov() -> DilemmaPlayer {
    Player::new("Pavlov".to_string(), || {
        Strategy::new(|context: &DilemmaContext| {
            context
                .state_view()
                .history()
                .profiles()
                .last()
                .map_or(C, |profile| {
                    let my_move = profile[context.my_index()];
                    match profile[context.their_index()] {
                        Move::Cooperate => my_move,
                        Move::Defect => my_move.opposite(),
                    }
                })
        })
    })
}

/// A player that cooperates until the opponent defects once, then defects forever after.
pub fn grim_trigger() -> DilemmaPlayer {
    Player::new("Grim Trigger".to_string(), || {
        Strategy::trigger(
            |context: &DilemmaContext| {
                context
                    .state_view()
                    .history()
                    .moves_for_player(context.their_index())
                    .last()
                    .map_or(false, |m| m == D)
            },
            Strategy::pure(C),
            Strategy::pure(D),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defector_vs_cooperator() {
        let g = Repeated::new(Dilemma::prisoners_dilemma(), 100);
        let matchup = Matchup::from_players([defector(), cooperator()]);
        let history = g.play(&matchup).unwrap();
        assert_eq!(history.score(), &Payoff::from([300, 0]));
    }

    #[test]
    fn defector_vs_tit_for_tat() {
        let g = Repeated::new(Dilemma::prisoners_dilemma(), 100);
        let matchup = Matchup::from_players([defector(), tit_for_tat()]);
        let history = g.play(&matchup).unwrap();
        assert_eq!(history.score(), &Payoff::from([102, 99]));
    }

    #[test]
    fn tit_for_tat_vs_tit_for_tat() {
        let g = Repeated::new(Dilemma::prisoners_dilemma(), 100);
        let matchup = Matchup::from_players([tit_for_tat(), tit_for_tat()]);
        let history = g.play(&matchup).unwrap();
        assert_eq!(history.score(), &Payoff::from([200, 200]));
    }

    #[test]
    fn tit_for_tat_vs_suspicious_tit_for_tat() {
        let g = Repeated::new(Dilemma::prisoners_dilemma(), 100);
        let matchup = Matchup::from_players([tit_for_tat(), suspicious_tit_for_tat()]);
        let history = g.play(&matchup).unwrap();
        assert_eq!(history.score(), &Payoff::from([150, 150]));
    }

    #[test]
    fn tit_for_two_tats_vs_suspicious_tit_for_tat() {
        let g = Repeated::new(Dilemma::prisoners_dilemma(), 100);
        let matchup = Matchup::from_players([tit_for_n_tats(2), suspicious_tit_for_tat()]);
        let history = g.play(&matchup).unwrap();
        assert_eq!(history.score(), &Payoff::from([198, 201]));
    }
}
