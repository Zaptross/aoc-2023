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
    return GameData::new(0, Vec::new());
}
