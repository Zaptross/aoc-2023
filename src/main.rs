mod day;
mod day_1;

fn main() {
    println!("Hello, Advent of Code 2023!");

    day::run_day(
        "Trebuchet!?",
        1,
        1,
        vec![
            || day_1::functions::sum_found_digits(day_1::data::part_one_test_set(), false),
            || day_1::functions::sum_found_digits(day_1::data::day_one_data_set(), false),
        ],
    );
    day::run_day(
        "Trebuchet!?",
        1,
        2,
        vec![
            || day_1::functions::sum_found_digits(day_1::data::part_two_test_set(), true),
            || day_1::functions::sum_found_digits(day_1::data::day_one_data_set(), true),
        ],
    );
}
