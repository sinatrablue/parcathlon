mod analyzer;
mod banner;
mod generator;
mod init;
mod pdf_exporter;
mod picker;

use crate::generator::generate_round;
use crate::init::{init_games_map, init_teams_map};
use crate::pdf_exporter::export_rounds_to_pdf;
use std::collections::HashMap;

const GAMES_NUMBER: usize = 5;
const TEAMS: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
type Round = Vec<HashMap<u8, (u8, u8)>>;

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
