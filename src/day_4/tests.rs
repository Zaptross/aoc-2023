use crate::day_4::functions::{evaluate_card, extract_vec_numbers};

use super::{
    data::day_four_test_set,
    functions::{count_points_of_scratchcards, sum_all_points},
};

#[test]
fn day_4_count_points_of_scratchcards() {
    assert_eq!(count_points_of_scratchcards(day_four_test_set()), 13)
}

#[test]
fn day_4_sum_all_points() {
    let cases = vec![
        (vec![1, 2, 3, 4], 10),
        (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 55),
    ];

    for (input, expected) in cases {
        assert_eq!(sum_all_points(input), expected)
    }
}

#[test]
fn day_4_evaluate_card() {
    let cases = vec![
        (day_four_test_set()[0], 8),
        (day_four_test_set()[1], 2),
        (day_four_test_set()[2], 2),
        (day_four_test_set()[3], 1),
        (day_four_test_set()[4], 0),
        (day_four_test_set()[5], 0),
    ];

    for (input, expected) in cases {
        assert_eq!(evaluate_card(input), expected)
    }
}

#[test]
fn day_4_extract_vec_numbers() {
    let cases = vec![
        ("1 2  3 4", vec![1, 2, 3, 4]),
        ("91 92 3 94", vec![91, 92, 3, 94]),
    ];

    for (input, expected) in cases {
        assert_eq!(extract_vec_numbers(input), expected)
    }
}