use rand::Rng;

pub fn pick_a_random_team(teams: &Vec<i8>) -> i8 {
    let mut rng = rand::rng();
    teams[rng.random_range(0..teams.len())]
}
