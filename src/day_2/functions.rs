pub struct GameData {
    pub id: i32,
    pub turns: Vec<CubeSet>,
}

impl GameData {
    pub fn new(id: i32, turns: Vec<CubeSet>) -> GameData {
        GameData { id, turns }
    }
}

pub struct CubeSet {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl CubeSet {
    pub fn new(red: i32, green: i32, blue: i32) -> CubeSet {
        CubeSet { red, blue, green }
    }
}

pub fn validate_set_of_games(set: CubeSet, games: Vec<&str>) -> i32 {
    return 0;
}

pub fn extract_game_data(game: &str) -> GameData {
    let split_game_id_and_games: Vec<&str> = game.split(": ").collect();
    let game_id: i32 = split_game_id_and_games[0]
        .split(" ")
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let turns = split_game_id_and_games[1]
        .split("; ")
        .map(|t| extract_cubes_from_turn(t))
        .collect();

    return GameData::new(game_id, turns);
}

pub fn extract_cubes_from_turn(turn: &str) -> CubeSet {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    let split: Vec<Vec<&str>> = turn.split(", ").map(|c| c.split(" ").collect()).collect();

    for number_and_color in split {
        let number: i32 = number_and_color[0].parse().unwrap();
        let color = number_and_color[1];

        match color {
            "red" => red += number,
            "green" => green += number,
            "blue" => blue += number,
            _ => continue,
        }
    }

    return CubeSet::new(red, green, blue);
}
