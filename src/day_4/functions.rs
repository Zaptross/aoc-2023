pub fn count_points_of_scratchcards(cards: Vec<&str>) -> i32 {
    return 0;
}

pub(crate) fn sum_all_points(points: Vec<i32>) -> i32 {
    return points.iter().fold(0, |acc, x| acc + x);
}

pub(crate) fn evaluate_card(card: &str) -> i32 {
    let removed_card_num: Vec<&str> = card.split(":").collect();
    let numbers_sections: Vec<Vec<i32>> = removed_card_num[1]
        .split("|")
        .map(|x| extract_vec_numbers(x))
        .collect();

    let wins = numbers_sections[1]
        .iter()
        .map(|n| {
            if numbers_sections[0].contains(n) {
                return 1;
            }
            return 0;
        })
        .fold(0, |acc, x| acc + x);

    if wins <= 0 {
        return 0;
    }

    let base: i32 = 2;
    return base.pow(wins - 1);
}

pub(crate) fn extract_vec_numbers(numbers: &str) -> Vec<i32> {
    return numbers
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
}
