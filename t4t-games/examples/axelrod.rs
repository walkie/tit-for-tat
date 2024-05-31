use std::sync::Arc;
use t4t::*;
use t4t_games::dilemma::*;

/// Run a tournament with the given game and players. Prints the scores and also adds them to
/// the current overall scores.
fn run_tournament(game: Dilemma, players: &[Arc<DilemmaPlayer>], overall: &mut Score<i64>) {
    let tournament = Tournament::combinations_with_replacement(
        Arc::new(Repeated::new(Arc::new(game), 100)),
        players,
    );
    let result = tournament.play();
    assert!(!result.has_errors());
    result.score().print_best_to_worst();
    overall.add_all(result.score());
}

/// Runs several tournaments of twenty different players competing in various repeated
/// dilemma games.
pub fn main() {
    let players = vec![
        Arc::new(cooperator()),
        Arc::new(defector()),
        Arc::new(periodic(vec![C, D])),
        Arc::new(periodic(vec![D, C])),
        Arc::new(periodic(vec![C, C, D])),
        Arc::new(periodic(vec![D, D, C])),
        Arc::new(periodic(vec![C, C, D, D])),
        Arc::new(random()),
        Arc::new(random_ccd()),
        Arc::new(tit_for_tat()),
        Arc::new(suspicious_tit_for_tat()),
        Arc::new(tit_for_n_tats(2)),
        Arc::new(tit_for_n_tats(3)),
        Arc::new(n_tits_for_tat(2)),
        Arc::new(n_tits_for_tat(3)),
        Arc::new(generous_tit_for_tat()),
        Arc::new(probing_tit_for_tat()),
        Arc::new(firm_but_fair()),
        Arc::new(pavlov()),
        Arc::new(grim_trigger()),
    ];

    let mut overall = Score::new();

    println!("== Prisoner's Dilemma ==");
    run_tournament(Dilemma::prisoners_dilemma(), &players, &mut overall);

    println!("\n== Friend-or-Foe ==");
    run_tournament(Dilemma::friend_or_foe(), &players, &mut overall);

    println!("\n== Stag Hunt ==");
    run_tournament(Dilemma::stag_hunt(), &players, &mut overall);

    println!("\n== Assurance Game ==");
    run_tournament(Dilemma::assurance_game(), &players, &mut overall);

    println!("\n== Hawk-Dove (2/3) ==");
    run_tournament(Dilemma::hawk_dove(2, 3), &players, &mut overall);

    println!("\n== Hawk-Dove (3/2) ==");
    run_tournament(Dilemma::hawk_dove(3, 2), &players, &mut overall);

    println!("\n== Chicken ==");
    run_tournament(Dilemma::chicken(5), &players, &mut overall);

    println!("\n== Snowdrift ==");
    run_tournament(Dilemma::snowdrift(), &players, &mut overall);

    println!("\n== Overall scores ==");
    overall.print_best_to_worst();
}
