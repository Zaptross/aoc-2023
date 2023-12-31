mod day;
mod day_1;
mod day_2;
mod day_3;
mod day_4;

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

    day::run_day(
        "Cube Conundrum",
        2,
        1,
        vec![
            || {
                day_2::functions::validate_set_of_games(
                    day_2::data::part_one_cube_set(),
                    day_2::data::day_two_test_set(),
                )
                .to_string()
            },
            || {
                day_2::functions::validate_set_of_games(
                    day_2::data::part_one_cube_set(),
                    day_2::data::day_two_data_set(),
                )
                .to_string()
            },
        ],
    );
    day::run_day(
        "Cube Conundrum",
        2,
        2,
        vec![
            || {
                day_2::functions::calculate_power_of_all_games(day_2::data::day_two_test_set())
                    .to_string()
            },
            || {
                day_2::functions::calculate_power_of_all_games(day_2::data::day_two_data_set())
                    .to_string()
            },
        ],
    );

    day::run_day(
        "Gear Ratios",
        3,
        1,
        vec![
            || {
                day_3::functions::calculate_sum_of_part_numbers(
                    day_3::data::day_three_test_set(),
                    false,
                )
                .to_string()
            },
            || {
                day_3::functions::calculate_sum_of_part_numbers(
                    day_3::data::day_three_data_set(),
                    false,
                )
                .to_string()
            },
        ],
    );
    day::run_day(
        "Gear Ratios",
        3,
        2,
        vec![
            || {
                day_3::functions::calculate_sum_of_part_numbers(
                    day_3::data::day_three_test_set(),
                    true,
                )
                .to_string()
            },
            || {
                day_3::functions::calculate_sum_of_part_numbers(
                    day_3::data::day_three_data_set(),
                    true,
                )
                .to_string()
            },
        ],
    );

    day::run_day(
        "Scratchcards",
        4,
        1,
        vec![
            || {
                day_4::functions::count_points_of_scratchcards(day_4::data::day_four_test_set())
                    .to_string()
            },
            || {
                day_4::functions::count_points_of_scratchcards(day_4::data::day_four_data_set())
                    .to_string()
            },
        ],
    );
    day::run_day(
        "Scratchcards",
        4,
        2,
        vec![
            || {
                day_4::functions::count_all_scratch_cards(day_4::data::day_four_test_set())
                    .to_string()
            },
            || {
                day_4::functions::count_all_scratch_cards(day_4::data::day_four_data_set())
                    .to_string()
            },
        ],
    );
}
