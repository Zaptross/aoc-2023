pub mod data;

pub fn sum_found_digits(lines: Vec<&str>) -> String {
    let mut sum = 0;
    for line in lines {
        sum += find_and_concat_digits(line)
    }

    return sum.to_string();
}

fn find_and_concat_digits(line: &str) -> i32 {
    return format!("{}{}", find_digit(line, true), find_digit(line, false))
        .parse()
        .unwrap();
}

fn find_digit(line: &str, forward: bool) -> char {
    if !forward {
        for char in line.chars().rev() {
            if char.is_numeric() {
                return char;
            }
        }
    } else {
        for char in line.chars() {
            if char.is_numeric() {
                return char;
            }
        }
    }
    return 'X';
}
