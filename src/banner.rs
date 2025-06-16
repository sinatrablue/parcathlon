use crate::GAMES_NUMBER;
use rand::Rng;

pub fn ban_a_random_game(already_banned_games: &Vec<usize>) -> usize {
    let mut rng = rand::rng();
    if already_banned_games.is_empty() {
        rng.random_range(0..GAMES_NUMBER)
    } else {
        let bannable_games = (0..GAMES_NUMBER)
            .filter(|game| !already_banned_games.contains(game))
            .collect::<Vec<usize>>();
        bannable_games[rng.random_range(0..bannable_games.len())]
    }
}

/*pub fn put_team_on_the_bench(already_banned_teams: &Vec<u8>) -> u8 {
    let mut rng = rand::rng();
    let bannable_teams = TEAMS
        .into_iter()
        .filter(|team| !already_banned_teams.contains(team))
        .collect::<Vec<_>>();
    bannable_teams[rng.random_range(0..bannable_teams.len())]
}*/
