use crate::day_1::functions::{find_and_concat_digits, find_digit, find_digit_direction};

use super::{
    data::{part_one_test_set, part_two_test_set},
    functions::{find_digit_word, sum_found_digits},
};

#[test]
fn day_1_returns_146_for_testdataset() {
    let result = sum_found_digits(part_one_test_set(), false);
    assert_eq!(result, "142")
}

#[test]
fn day_1_returns_281_for_part_two_test_dataset() {
    let result = sum_found_digits(part_two_test_set(), true);
    assert_eq!(result, "281")
}

#[test]
fn day_1_find_digit_word() {
    let cases = vec![
        ("onest", (true, '1')),
        ("fours", (true, '4')),
        ("three", (true, '3')),
        ("potat", (false, '-')),
        ("pone2", (false, '-')),
        ("zeroo", (true, '0')),
        ("s0one", (false, '-')),
    ];

    for (input, expected) in cases {
        assert_eq!(find_digit_word(input), expected)
    }
}

#[test]
fn day_1_find_digit() {
    let cases = vec![
        (("fourtwothree", 'o', 0, true), (true, '4')),
        (("1two4three", '1', 0, true), (true, '1')),
        (("onetwothree", 'o', 0, false), (false, '-')),
        (("1two4three", '1', 0, false), (true, '1')),
    ];

    for ((line, c, index, allowed), expected) in cases {
        assert_eq!(find_digit(line, c, index, allowed), expected)
    }
}

#[test]
fn day_1_find_digit_direction() {
    let cases = vec![
        (("1two3", true, false), '1'),
        (("1two3", false, false), '3'),
        (("zzzthree7twozzz", true, true), '3'),
        (("zzzthree7twozzz", false, true), '2'),
        (("zzz", true, true), '-'),
        (("zzz", false, true), '-'),
    ];

    for ((line, forward, allowed), expected) in cases {
        assert_eq!(find_digit_direction(line, forward, allowed), expected)
    }
}

#[test]
fn day_1_find_and_concat_digits() {
    let cases = vec![(("three14two", false), 14), (("three14two", true), 32)];

    for ((line, allowed), expected) in cases {
        assert_eq!(find_and_concat_digits(line, allowed), expected)
    }
}
