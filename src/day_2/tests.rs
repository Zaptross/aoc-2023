use super::{
    data::{part_one_cube_set, part_one_test_set},
    functions::{
        extract_cubes_from_turn, extract_game_data, validate_game, validate_set_of_games, CubeSet,
        GameData,
    },
};

#[test]
fn day_2_validates_set_of_games() {
    assert_eq!(
        validate_set_of_games(part_one_cube_set(), part_one_test_set()),
        8
    )
}

#[test]
fn day_2_validate_game() {
    let cases = vec![
        (GameData::new(1, vec![CubeSet::new(1, 1, 1)]), 1),
        (GameData::new(2, vec![CubeSet::new(100, 0, 0)]), 0),
        (
            GameData::new(
                3,
                vec![
                    CubeSet::new(1, 0, 0),
                    CubeSet::new(0, 1, 0),
                    CubeSet::new(0, 0, 1),
                ],
            ),
            0,
        ),
    ];

    let set = &CubeSet::new(1, 1, 1);

    for (input, expected) in cases {
        assert_eq!(validate_game(set, input), expected);
    }
}

#[test]
fn day_2_extract_game_data() {
    let cases = vec![
        (
            "Game 1: 3 blue, 4 red",
            GameData::new(1, vec![CubeSet::new(4, 0, 3)]),
        ),
        (
            "Game 2: 3 red, 7 green, 4 blue",
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

#[test]
fn day_2_extract_cubes_from_turn() {
    let cases = vec![
        ("1 red, 2 green, 3 blue", CubeSet::new(1, 2, 3)),
        (
            "1 red, 1 red, 1 red, 1 green, 1 green, 1 blue",
            CubeSet::new(3, 2, 1),
        ),
    ];

    for (input, expected) in cases {
        let result = extract_cubes_from_turn(input);

        assert_eq!(result.red, expected.red);
        assert_eq!(result.green, expected.green);
        assert_eq!(result.blue, expected.blue);
    }
}
