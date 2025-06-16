use rand::Rng;

pub fn pick_a_random_team(teams: &Vec<u8>) -> u8 {
    let mut rng = rand::rng();
    teams[rng.random_range(0..teams.len())]
}
