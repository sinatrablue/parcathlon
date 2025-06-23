use crate::{Round, TEAMS};

pub fn deduce_team_on_bench(round: &Round) -> Option<u8> {
    let teams = round.iter().fold(vec![], |acc, game_pairs| {
        let mut acc_teams = acc.clone();
        game_pairs.iter().for_each(|(_game, teams)| {
            acc_teams.push(teams.0);
            acc_teams.push(teams.1);
        });
        acc_teams
    });
    TEAMS.into_iter().find(|team| !teams.contains(team))
}

pub fn have_teams_already_faced_two_times(
    prev_rounds: &Vec<Round>,
    team1: &u8,
    team2: &u8,
) -> bool {
    2 <= prev_rounds.iter().fold(0, |mut acc, round| {
        let acc_teams = &mut acc;
        round.iter().for_each(|game_pairs| {
            for (_game, teams) in game_pairs.iter() {
                if vec![team1, team2].contains(&&teams.0) && vec![team1, team2].contains(&&teams.1)
                {
                    *acc_teams += 1;
                }
            }
        });
        *acc_teams
    })
}
