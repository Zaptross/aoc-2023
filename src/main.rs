mod day;
mod day_1;

fn main() {
    println!("Hello, Advent of Code 2023!");

    day::run_day(
        "Trebuchet!?",
        1,
        1,
        vec![
            || day_1::sum_found_digits(day_1::data::part_one_test_set()),
            || day_1::sum_found_digits(day_1::data::part_one_data_set()),
        ],
    );
}
