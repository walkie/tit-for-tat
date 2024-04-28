# Tit-for-tat: a game theory toolbox

Tit-for-tat (t4t) is a [game theory](https://en.wikipedia.org/wiki/Game_theory) library with a
focus on experimentation over formal analysis. It supports defining games and strategies, then
executing them repeatedly in order to collect and observe the results.

The companion crate [t4t-games](https://crates.io/crates/t4t-games) provides some example games
and strategies.


## Design goal: expressiveness over performance

This library prioritizes expressiveness over performance. It aims to provide a powerful set of
abstractions for building arbitrary games and strategies.

This tradeoff is easy to see in the representation of [normal-form games](crate::Normal), which
are represented not as a matrix of payoffs, but as a function from generic move profiles to
generic payoffs. This enables normal-form games of arbitrary size, between arbitrary numbers of
players, and with arbitrary move and utility values, but is somewhat less efficient than a
simple payoff matrix of numbers.

A subtler but more extreme case of this tradeoff is in how games are executed. The [Game] trait
is quite generic, and implementers of this trait do not implement the execution of their game
directly, but rather produce a description of how their game is executed. This is much less
efficient, but enables generically transforming the execution of a game, for example, defining
new games that modify the behavior of existing games.

An example of this capability in action is the [Repeated] type, which transforms any game into
a repeated game, modifying the original game's state and execution to enable players of the
game to see the history of games played so far.

Of course, all things being equal, I'd still like things to run as fast as possible! However,
if your application deals only with 2x2 numeric, normal-form games, and you need to run
billions of iterations, this might not be the library you're looking for.


## This is a work in progress!

The library is very much a work-in-progress and will continue expanding and evolving.

The type- and method-level documentation is very good in places, minimal in others. The
top-level documentation that you're reading now is sorely missing some good examples to get you
started--sorry!

[Normal-form games](crate::Normal) are in good shape, and [repeated games](crate::Repeated) are
solid for perfect-information games. You can define [players](crate::Player) and
[strategies](crate::Strategy) for these games, and they can be played.

There is a lot of infrastructure in place for sequential and state-based types, but the library
is still missing the main top-level types to make this convenient to use.

Still to come and low-hanging fruit are convenient mechanisms to run "tournaments" (e.g. play
a game with all combinations of players drawn from a set of entrants). Long-term, I'd like to
add ways to visualize game executions and build games and strategies interactively, but we'll
see!

License: MIT
