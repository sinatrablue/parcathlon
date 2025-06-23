use crate::analyzer::have_teams_already_faced_two_times;
use crate::banner::ban_a_random_game;
use crate::picker::pick_a_random_team;
use crate::{Round, GAMES_NUMBER, TEAMS};
use std::collections::HashMap;

pub fn generate_round(
    prev_rounds: &Vec<Round>,
    _already_banned_teams: &mut Vec<u8>,
    already_banned_games: &mut Vec<usize>,
) -> Round {
    let mut round = Vec::with_capacity(GAMES_NUMBER - 1);
    let banned_game_for_round = ban_a_random_game(already_banned_games);
    already_banned_games.push(banned_game_for_round);
    let mut teams_busy_this_round: Vec<u8> = Vec::with_capacity(TEAMS.len() - 1);

    for i in 0..GAMES_NUMBER {
        if i == banned_game_for_round {
            continue;
        }

        let banned_teams_for_game = prev_rounds.iter().fold(vec![], |mut acc, round| {
            round.iter().for_each(|map| {
                for (game, teams) in map {
                    if *game == i as u8 {
                        acc.push(teams.0);
                        acc.push(teams.1);
                    }
                }
            });
            acc
        });

        let available_teams_for_game = TEAMS
            .into_iter()
            .filter(|&team| {
                //team != team_on_the_bench_for_round &&
                !teams_busy_this_round.contains(&team) && !banned_teams_for_game.contains(&team)
            })
            .collect::<Vec<_>>();

        let randomly_picked_team = pick_a_random_team(&available_teams_for_game);

        let available_teams_for_picked_team = TEAMS
            .into_iter()
            .filter(|team| {
                *team != randomly_picked_team
                    && available_teams_for_game.contains(team)
                    && !have_teams_already_faced_two_times(prev_rounds, &randomly_picked_team, team)
            })
            .collect::<Vec<_>>();

        let randomly_picked_opponent_team = pick_a_random_team(&available_teams_for_picked_team);

        let mut res = HashMap::new();
        res.insert(
            i as u8,
            (randomly_picked_team, randomly_picked_opponent_team),
        );
        round.push(res);
        teams_busy_this_round.push(randomly_picked_team);
        teams_busy_this_round.push(randomly_picked_opponent_team);
    }
    round
}
