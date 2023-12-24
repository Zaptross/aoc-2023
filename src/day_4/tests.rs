use super::{
    data::day_four_test_set,
    functions::{
        count_all_scratch_cards, count_points_of_scratchcards, evaluate_card, extract_card_id,
        extract_vec_numbers, new_cards_from_card_with_wins,
    },
};

#[test]
fn day_4_count_points_of_scratchcards() {
    assert_eq!(count_points_of_scratchcards(day_four_test_set()), 13)
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
        assert_eq!(evaluate_card(input, false), expected)
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

#[test]
fn day_4_part_2_count_all_scratch_cards() {
    assert_eq!(count_all_scratch_cards(day_four_test_set()), 30)
}

#[test]
fn day_4_part_2_evaluate_card_wins_only() {
    let cases = vec![
        (day_four_test_set()[0], 4),
        (day_four_test_set()[1], 2),
        (day_four_test_set()[2], 2),
        (day_four_test_set()[3], 1),
        (day_four_test_set()[4], 0),
        (day_four_test_set()[5], 0),
    ];

    for (input, expected) in cases {
        assert_eq!(evaluate_card(input, true), expected)
    }
}

#[test]
fn day_4_part_2_new_cards_from_card_with_wins() {
    let cases = vec![((3, 3), vec![4, 5, 6]), ((1, 0), vec![]), ((2, 1), vec![3])];

    for ((id, wins), expected) in cases {
        assert_eq!(new_cards_from_card_with_wins(id, wins), expected)
    }
}

#[test]
fn day_4_part_2_extract_card_id() {
    let cases = vec![
        (day_four_test_set()[0], 1),
        (day_four_test_set()[1], 2),
        (day_four_test_set()[2], 3),
        (day_four_test_set()[3], 4),
        (day_four_test_set()[4], 5),
        (day_four_test_set()[5], 6),
    ];

    for (input, expected) in cases {
        assert_eq!(extract_card_id(input), expected)
    }
}
