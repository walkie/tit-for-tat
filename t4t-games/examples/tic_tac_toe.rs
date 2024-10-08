use std::sync::Arc;
use t4t::*;
use t4t_games::tic_tac_toe::*;

/// Returns (wins, draws, losses) for the matchup indicated by the given player names, from the
/// perspective of player `P0` (who plays `X`).
fn count_results(
    player_names: PerPlayer<String, 2>,
    results: &TournamentResult<TicTacToe, 2>,
) -> (u32, u32, u32) {
    let mut wins = 0;
    let mut losses = 0;
    let mut draws = 0;
    for result in results.matchup_results(&player_names).unwrap().iter() {
        let payoff = *result.clone().unwrap().payoff();
        if payoff.is_zero_sum_winner(for2::P0) {
            wins += 1;
        } else if payoff.is_zero_sum_winner(for2::P1) {
            losses += 1;
        } else {
            draws += 1;
        }
    }
    (wins, draws, losses)
}

pub fn main() {
    // A player that plays randomly.
    let random = Player::new("Random".to_string(), Strategy::randomly);

    // A player that plays optimally, using the generic minimax algorithm.
    let minimax = Player::new("Minimax".to_string(), Strategy::total_minimax);

    // A player that plays opportunistically. If it can win in one move, it will. If the opponent
    // can win in one move, it will block. Otherwise, it will play randomly.
    let opportunist = Player::new("Opportunist".to_string(), || {
        Strategy::new(|context: Context<TicTacToe, 2>| {
            let board = context.state_view();
            // If we can win in one move, do so.
            if let Some(winning_move) = board.winning_moves_for(context.my_index()).first() {
                return *winning_move;
            }
            // If we can stop them from winning in one move, do so.
            if let Some(blocking_move) = board.winning_moves_for(context.their_index()).first() {
                return *blocking_move;
            }
            // Otherwise, play randomly.
            Strategy::randomly().next_move(context)
        })
    });

    // Each player plays each other player 10 times going first, and 10 times going second.
    let results = Tournament::permutations_without_replacement(
        Arc::new(TicTacToe),
        &[Arc::new(random), Arc::new(minimax), Arc::new(opportunist)],
    )
    .repeat(10)
    .play();

    // Check that there were no errors.
    assert!(!results.has_errors());

    // For reach matchup, report the wins, losses, and draws for each matchup.
    for player in ["Random", "Opportunist", "Minimax"].iter() {
        println!("{}", player);
        for opponent in ["Random", "Opportunist", "Minimax"].iter() {
            if player == opponent {
                continue;
            }
            let (x_wins, x_draws, x_losses) = count_results(
                PerPlayer::new([player.to_string(), opponent.to_string()]),
                &results,
            );
            let (o_losses, o_draws, o_wins) = count_results(
                PerPlayer::new([opponent.to_string(), player.to_string()]),
                &results,
            );
            println!("  vs. {}", opponent);
            println!(
                "    as X's: {} wins, {} losses, {} draws",
                x_wins, x_losses, x_draws
            );
            println!(
                "    as O's: {} wins, {} losses, {} draws",
                o_wins, o_losses, o_draws
            );
        }
    }
}
