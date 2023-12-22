pub(crate) struct GameData {
    pub id: i32,
    pub turns: Vec<CubeSet>,
}

impl GameData {
    pub(crate) fn new(id: i32, turns: Vec<CubeSet>) -> GameData {
        GameData { id, turns }
    }
}

pub(crate) struct CubeSet {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl CubeSet {
    pub(crate) fn new(red: i32, green: i32, blue: i32) -> CubeSet {
        CubeSet { red, blue, green }
    }
}

pub fn calculate_power_of_all_games(games: Vec<&str>) -> i32 {
    return 0;
}

pub(crate) fn calculate_power_of_game(game: GameData) -> i32 {
    return 0;
}

pub fn validate_set_of_games(set: CubeSet, games: Vec<&str>) -> i32 {
    let mut result = 0;

    for game in games {
        result += validate_game(&set, extract_game_data(game))
    }

    return result;
}

pub(crate) fn validate_game(set: &CubeSet, game: GameData) -> i32 {
    for turn in game.turns {
        // if any set of cubes exceed the maximum allowed
        if turn.red > set.red || turn.green > set.green || turn.blue > set.blue {
            // it is not valid and should not be counted
            return 0;
        }
    }

    // if it's valid, it should be counted
    return game.id;
}

pub(crate) fn extract_game_data(game: &str) -> GameData {
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

pub(crate) fn extract_cubes_from_turn(turn: &str) -> CubeSet {
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
