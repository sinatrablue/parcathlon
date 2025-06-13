mod banner;
mod init;
mod picker;

use crate::banner::{ban_a_random_game, put_team_on_the_bench};
use crate::init::{init_games_map, init_teams_map};
use crate::picker::pick_a_random_team;
use std::collections::HashMap;

const GAMES_NUMBER: usize = 5;
const TEAMS: [i8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
type Round = Vec<HashMap<i8, (i8, i8)>>;

fn generate_round(prev_rounds: &Vec<Round>, already_banned_teams: &mut Vec<i8>) -> Round {
    let mut round = Vec::with_capacity(GAMES_NUMBER);
    let banned_game_for_round = ban_a_random_game(&prev_rounds);
    let team_on_the_bench_for_round = put_team_on_the_bench(already_banned_teams);
    already_banned_teams.push(team_on_the_bench_for_round);
    // let mut teams_busy_this_round: Vec<i8> = Vec::with_capacity(TEAMS.len() - 1);

    for i in 1..GAMES_NUMBER + 1 {
        match banned_game_for_round {
            None => {}
            Some(banned_game) => {
                if i == banned_game {
                    continue;
                }
            }
        }

        let banned_teams_for_game = prev_rounds.iter().fold(vec![], |mut acc, round| {
            let mut teams_to_ban = vec![];
            round.iter().for_each(|map| {
                for (game, teams) in map {
                    if *game == i as i8 {
                        teams_to_ban.push(teams);
                    }
                }
            });
            if teams_to_ban.len() > 0 {
                teams_to_ban.iter().for_each(|&teams| {
                    let (team1, team2) = teams;
                    acc.push(team1);
                    acc.push(team2);
                })
            }
            acc
        });

        let available_teams_for_game = TEAMS
            .into_iter()
            .filter(|&team| {
                team != team_on_the_bench_for_round
                    && !banned_teams_for_game.iter().any(|&t| *t == team)
            })
            .collect::<Vec<_>>();

        let randomly_picked_team = pick_a_random_team(&available_teams_for_game);

        let banned_teams_for_picked_team = prev_rounds
            .iter()
            .flat_map(|r| {
                r.iter().fold(vec![], |acc, round| {
                    let mut teams_to_ban = acc.clone();
                    for (_game, teams) in round {
                        if teams.0 == randomly_picked_team {
                            teams_to_ban.push(teams.1);
                        } else if teams.1 == randomly_picked_team {
                            teams_to_ban.push(teams.0);
                        }
                    }
                    teams_to_ban
                })
            })
            .collect::<Vec<_>>();

        let available_teams_for_first_team = TEAMS
            .into_iter()
            .filter(|team| !banned_teams_for_picked_team.contains(team))
            .collect::<Vec<_>>();

        let randomly_picked_opponent_team = pick_a_random_team(&available_teams_for_first_team);

        let mut res = HashMap::new();
        res.insert(
            i as i8,
            (randomly_picked_team, randomly_picked_opponent_team),
        );
        round.push(res);
    }
    round
}

fn main() {
    let mut games_map: HashMap<i8, (String, String)> = HashMap::new();
    init_games_map(&mut games_map);
    let mut teams_map: HashMap<i8, String> = HashMap::new();
    init_teams_map(&mut teams_map);

    let mut rounds: Vec<Round> = Vec::with_capacity(5);
    let mut already_banned_teams: Vec<i8> = Vec::with_capacity(GAMES_NUMBER);

    for _i in 1..GAMES_NUMBER + 1 {
        let round: Round = generate_round(&rounds, &mut already_banned_teams);
        rounds.push(round);
    }
    for i in 0..rounds.len() {
        println!("==> ROUND #{}", i + 1);
        rounds[i].iter().for_each(|round| {
            for (game, teams) in round {
                let (name, person) = games_map.get(&game).unwrap();
                println!("=====> Game {} with {}", name, person);
                let (team1, team2) = teams;
                println!(
                    "========> Teams {} vs {}",
                    teams_map.get(team1).unwrap(),
                    teams_map.get(team2).unwrap()
                );
            }
        })
    }
}
