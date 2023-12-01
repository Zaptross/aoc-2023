mod day;

fn main() {
    println!("Hello, Advent of Code 2023!");

    day::run_day(
        "Test",
        1,
        1,
        vec![
            || "test".to_string(),
            || "main".to_string(),
        ],
    );
}
