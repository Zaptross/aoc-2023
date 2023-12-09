pub fn sum_found_digits(lines: Vec<&str>, number_words_allowed: bool) -> String {
    let mut sum = 0;
    for line in lines {
        sum += find_and_concat_digits(line, number_words_allowed);
    }

    return sum.to_string();
}

pub(crate) fn find_and_concat_digits(line: &str, number_words_allowed: bool) -> i32 {
    return format!(
        "{}{}",
        find_digit_direction(line, true, number_words_allowed),
        find_digit_direction(line, false, number_words_allowed)
    )
    .parse()
    .unwrap();
}

fn is_number_word_starting_character(c: char) -> bool {
    return match c {
        'o' => true,
        't' => true,
        'f' => true,
        's' => true,
        'e' => true,
        'n' => true,
        'z' => true,
        _ => false,
    };
}

pub(crate) fn find_digit_word(chars: &str) -> (bool, char) {
    return match chars {
        x if x.starts_with("one") => (true, '1'),
        x if x.starts_with("two") => (true, '2'),
        x if x.starts_with("six") => (true, '6'),
        x if x.starts_with("four") => (true, '4'),
        x if x.starts_with("five") => (true, '5'),
        x if x.starts_with("nine") => (true, '9'),
        x if x.starts_with("zero") => (true, '0'),
        x if x.starts_with("three") => (true, '3'),
        x if x.starts_with("seven") => (true, '7'),
        x if x.starts_with("eight") => (true, '8'),
        _ => (false, '-'),
    };
}

fn is_number_amount_of_characters_remaining(index: usize, len: usize) -> bool {
    return len - index >= 3;
}

pub(crate) fn find_digit_direction(line: &str, forward: bool, number_words_allowed: bool) -> char {
    if forward {
        for (index, c) in line.char_indices() {
            let (success, val) = find_digit(line, c, index, number_words_allowed);
            if success {
                return val;
            }
        }
    } else {
        for (index, c) in line.char_indices().rev() {
            let (success, val) = find_digit(line, c, index, number_words_allowed);
            if success {
                return val;
            }
        }
    }
    return '-';
}

pub(crate) fn find_digit(
    line: &str,
    c: char,
    index: usize,
    number_words_allowed: bool,
) -> (bool, char) {
    let len = line.len();
    if c.is_numeric() {
        return (true, c);
    }
    if number_words_allowed
        && c.is_alphabetic()
        && is_number_amount_of_characters_remaining(index, len)
        && is_number_word_starting_character(c)
    {
        if index + 4 >= len {
            return find_digit_word(&line[index..len]);
        }

        return find_digit_word(&line[index..index + 5]);
    }

    return (false, '-');
}
