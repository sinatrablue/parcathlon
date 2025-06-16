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
