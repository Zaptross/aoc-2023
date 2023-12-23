use super::{
    data::day_three_test_set,
    functions::{
        calculate_sum_of_part_numbers, create_jagged_vec_from_vec_str, find_all_symbols,
        retrieve_adjacent_part_numbers, Coord,
    },
};

#[test]
fn day_3_calculate_sum_of_part_numbers() {
    assert_eq!(calculate_sum_of_part_numbers(day_three_test_set()), 4361)
}

#[test]
fn day_3_create_jagged_vec_from_vec_str() {
    let input = vec!["123", "456", "789"];

    let result = create_jagged_vec_from_vec_str(input);

    assert_eq!(result[0][0], '1');
    assert_eq!(result[2][1], '6');
    assert_eq!(result[2][2], '9');
}

#[test]
fn day_3_find_all_symbols() {
    let cases: Vec<(Vec<Vec<char>>, Vec<Coord>)> = vec![(
        create_jagged_vec_from_vec_str(vec!["*.#", ".+.", "+.$"]),
        vec![(0, 0), (2, 0), (1, 1), (2, 0), (2, 2)],
    )];

    for (input, expected) in cases {
        let result = find_all_symbols(input);

        for coord in expected {
            assert!(!result.contains(&coord));
        }
    }
}

#[test]
fn day_3_retrieve_adjacent_part_numbers() {
    let cases = vec![(
        create_jagged_vec_from_vec_str(vec!["1*.", "...", ".*.", "111"]),
        vec![1, 111],
    )];

    for (input, expected) in cases {
        let result = retrieve_adjacent_part_numbers(input);

        for part in expected {
            assert!(result.contains(&part))
        }
    }
}
