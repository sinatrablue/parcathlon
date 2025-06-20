mod analyzer;
mod banner;
mod init;
mod pdf_exporter;
mod picker;

use crate::banner::ban_a_random_game;
use crate::init::{init_games_map, init_teams_map};
use crate::pdf_exporter::export_rounds_to_pdf;
use crate::picker::pick_a_random_team;
use std::collections::HashMap;

const GAMES_NUMBER: usize = 5;
const TEAMS: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
type Round = Vec<HashMap<u8, (u8, u8)>>;

fn generate_round(
    prev_rounds: &Vec<Round>,
    _already_banned_teams: &mut Vec<u8>,
    already_banned_games: &mut Vec<usize>,
) -> Round {
    let mut round = Vec::with_capacity(GAMES_NUMBER - 1);
    let banned_game_for_round = ban_a_random_game(already_banned_games);
    already_banned_games.push(banned_game_for_round);
    //let team_on_the_bench_for_round = put_team_on_the_bench(already_banned_teams);
    //println!("team on the bench {}", team_on_the_bench_for_round);
    //already_banned_teams.push(team_on_the_bench_for_round);
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
                    //&& *team != team_on_the_bench_for_round
                    //&& !banned_teams_for_picked_team.contains(team)
                    && available_teams_for_game.contains(team)
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

fn main() {
    let mut games_map: HashMap<u8, (String, String)> = HashMap::new();
    init_games_map(&mut games_map);
    let mut teams_map: HashMap<u8, String> = HashMap::new();
    init_teams_map(&mut teams_map);

    let mut rounds: Vec<Round> = Vec::with_capacity(5);
    let mut already_banned_teams: Vec<u8> = Vec::with_capacity(GAMES_NUMBER - 1);
    let mut already_banned_games: Vec<usize> = Vec::with_capacity(GAMES_NUMBER - 1);

    for _i in 1..GAMES_NUMBER + 1 {
        let round: Round = generate_round(
            &rounds,
            &mut already_banned_teams,
            &mut already_banned_games,
        );
        rounds.push(round);
    }
    match export_rounds_to_pdf(&rounds, &games_map, &teams_map) {
        Ok(_) => {
            println!("Rounds exported");
        }
        Err(_) => {
            println!("Error exporting");
        }
    }
}
