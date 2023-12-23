pub fn count_points_of_scratchcards(cards: Vec<&str>) -> i32 {
    return 0;
}

pub(crate) fn sum_all_points(points: Vec<i32>) -> i32 {
    return points.iter().fold(0, |acc, x| acc + x);
}

pub(crate) fn evaluate_card(card: &str) -> i32 {
    return 0;
}

pub(crate) fn extract_vec_numbers(numbers: &str) -> Vec<i32> {
    return numbers
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
}
