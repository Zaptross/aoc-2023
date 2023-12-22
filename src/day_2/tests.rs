use super::{
    data::{part_one_cube_set, part_one_test_set},
    functions::{extract_game_data, validate_set_of_games, CubeSet, GameData},
};

#[test]
fn day_2_validates_set_of_games() {
    assert_eq!(
        validate_set_of_games(part_one_cube_set(), part_one_test_set()),
        8
    )
}

#[test]
fn day_2_extract_game_data() {
    let cases = vec![
        (
            "Game 1: 3 blue, 4 red",
            GameData::new(1, vec![CubeSet::new(4, 0, 3)]),
        ),
        (
            "Game 2: 3 red, 4 green, 7 blue",
            GameData::new(2, vec![CubeSet::new(3, 7, 4)]),
        ),
        (
            "Game 2: 3 red; 4 green; 7 blue",
            GameData::new(
                2,
                vec![
                    CubeSet::new(3, 0, 0),
                    CubeSet::new(0, 4, 0),
                    CubeSet::new(0, 0, 7),
                ],
            ),
        ),
    ];

    for (input, expected) in cases {
        let result = extract_game_data(input);

        assert_eq!(result.id, expected.id);
        assert_eq!(result.turns.len(), expected.turns.len());

        if result.turns.len() == expected.turns.len() {
            for t in 0..result.turns.len() {
                assert_eq!(result.turns[t].red, expected.turns[t].red);
                assert_eq!(result.turns[t].green, expected.turns[t].green);
                assert_eq!(result.turns[t].blue, expected.turns[t].blue);
            }
        }
    }
}
