use std::collections::HashMap;

pub fn count_points_of_scratchcards(cards: Vec<&str>) -> i32 {
    return cards.iter().fold(0, |acc, c| acc + evaluate_card(c, false));
}

pub fn count_all_scratch_cards(starting_cards: Vec<&str>) -> i32 {
    // Map of <card_id, count_of_card>
    let mut card_map: HashMap<i32, i32> = HashMap::new();
    let mut total_count = 0;

    for card in starting_cards {
        let card_id = extract_card_id(card);
        let wins = evaluate_card(card, true);
        let count_of_card = 1 + *card_map.get(&card_id).or(Some(&0)).unwrap();

        for card_id in new_cards_from_card_with_wins(card_id, wins) {
            card_map
                .entry(card_id)
                .and_modify(|x| *x += count_of_card)
                .or_insert(count_of_card);
        }

        total_count += count_of_card;
    }

    return total_count;
}

pub(crate) fn new_cards_from_card_with_wins(card_id: i32, wins: i32) -> Vec<i32> {
    let mut new_cards = vec![];

    for i in 1..=wins {
        new_cards.append(&mut vec![card_id + i])
    }

    return new_cards;
}

pub(crate) fn evaluate_card(card: &str, wins_only: bool) -> i32 {
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

    if wins_only {
        return wins.try_into().unwrap();
    }

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

pub(crate) fn extract_card_id(card: &str) -> i32 {
    return card
        .split(":")
        .find(|_| true)
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse()
        .unwrap();
}
