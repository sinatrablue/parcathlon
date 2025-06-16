use std::collections::HashMap;

pub fn init_games_map(map: &mut HashMap<i8, (String, String)>) {
    map.insert(0, (String::from("Ch’ti fou mi"), String::from("Dodo")));
    map.insert(1, (String::from("Graine de star"), String::from("Aéla")));
    map.insert(
        2,
        (
            String::from("Plantons le décor"),
            String::from("Christophe"),
        ),
    );
    map.insert(
        3,
        (
            String::from("Olympic’nic’douille"),
            String::from("Baptiste"),
        ),
    );
    map.insert(4, (String::from("Copier Coller"), String::from("Assia")));
}

pub fn init_teams_map(map: &mut HashMap<i8, String>) {
    map.insert(0, String::from("Acajou"));
    map.insert(1, String::from("Fushia"));
    map.insert(2, String::from("Lilas"));
    map.insert(3, String::from("Cyan"));
    map.insert(4, String::from("Bordeau"));
    map.insert(5, String::from("Kaki"));
    map.insert(6, String::from("Or"));
    map.insert(7, String::from("TerraCota"));
    map.insert(8, String::from("Ébène"));
}
