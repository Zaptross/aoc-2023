use super::{
    data::day_three_test_set,
    functions::{
        calculate_sum_of_part_numbers, find_all_symbols, retrieve_adjacent_part_numbers,
        to_vec_char, Coord,
    },
};

#[test]
fn day_3_calculate_sum_of_part_numbers() {
    assert_eq!(calculate_sum_of_part_numbers(day_three_test_set()), 4361)
}

#[test]
fn day_3_find_all_symbols() {
    let cases = vec![(
        to_vec_char(vec!["*.#", ".+.", "+.$"]),
        vec![(0, 0), (0, 2), (1, 1), (2, 0), (2, 2)],
    )];

    for (input, expected) in cases {
        let result = find_all_symbols(&input);

        for coord in expected {
            assert!(result.contains(&coord));
        }
    }
}

#[test]
fn day_3_retrieve_adjacent_part_numbers() {
    let cases = vec![
        (to_vec_char(vec!["1*.", "...", ".*.", "111"]), vec![1, 111]),
        (to_vec_char(vec!["111", ".*.", "222"]), vec![111, 222]),
        (
            to_vec_char(vec![".1.22", "33*44", "....."]),
            vec![1, 22, 33, 44],
        ),
        (
            to_vec_char(day_three_test_set()),
            vec![467, 35, 633, 617, 592, 755, 664, 598],
        ),
    ];

    for (input, expected) in cases {
        let result = retrieve_adjacent_part_numbers(input);

        for part in expected {
            assert!(result.contains(&part))
        }
    }
}
