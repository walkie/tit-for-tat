# Tit-for-tat: a game theory toolbox in Rust

[![GitHub](https://img.shields.io/badge/github-walkie%2Ftit--for--tat-mediumorchid?logo=github)
][github-repo]
[![crates.io](https://img.shields.io/crates/v/t4t?label=crates.io)
][t4t-crates]
[![docs.rs/t4t](https://img.shields.io/badge/docs.rs-t4t-blue?logo=docs.rs)
][t4t-docs]
[![docs.rs/t4t-games](https://img.shields.io/badge/docs.rs-t4t--games-blue?logo=docs.rs)
][games-docs]
[![GitHub actions workflow status](https://img.shields.io/github/actions/workflow/status/walkie/tit-for-tat/rust.yml?logo=rust)
][github-build]

Tit-for-tat (t4t) is a [game theory](https://en.wikipedia.org/wiki/Game_theory) library with a
focus on experimentation over formal analysis, and expressiveness over performance. It supports
defining games and strategies, then executing them repeatedly in order to collect and observe the
results.

This repository hosts two crates:

- **[t4t][t4t-crates]:** The library itself.
- **[t4t-games][games-crates]:** A collection of games and strategies implemented using t4t.


[github-repo]: https://github.com/walkie/tit-for-tat
[github-build]: https://github.com/walkie/tit-for-tat/actions
[t4t-crates]: https://crates.io/crates/t4t
[t4t-docs]: https://docs.rs/t4t
[games-crates]: https://crates.io/crates/t4t-games
[games-docs]: https://docs.rs/t4t-games