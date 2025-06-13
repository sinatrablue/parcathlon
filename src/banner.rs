use crate::{Round, TEAMS};
use rand::Rng;

pub fn ban_a_random_game(prev_rounds: &Vec<Round>) -> Option<usize> {
    let mut rng = rand::rng();
    let bannable_games = prev_rounds
        .iter()
        .map(|round| {
            let mut games = vec![];
            round.iter().for_each(|map| {
                for (key, _value) in map {
                    games.push(key.clone());
                }
            });
            games
        })
        .flatten()
        .collect::<Vec<_>>();
    if bannable_games.is_empty() {
        return None;
    }
    Some(bannable_games[rng.random_range(0..bannable_games.len())] as usize)
}

pub fn put_team_on_the_bench(already_banned_teams: &Vec<i8>) -> i8 {
    let mut rng = rand::rng();
    let bannable_teams = TEAMS
        .into_iter()
        .filter(|team| !already_banned_teams.contains(team))
        .collect::<Vec<_>>();
    bannable_teams[rng.random_range(0..bannable_teams.len())]
}
