pub trait Game {
    /// The type of moves in this game.
    type Move;

    /// The type of utility value awarded to each player in the payoff.
    type Utility;
}
