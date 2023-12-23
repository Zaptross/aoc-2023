// Coords are (row, column)
// Eg: (1,2) -> 6
// 123
// 456
// 789
pub(crate) type Coord = (usize, usize);

pub fn calculate_sum_of_part_numbers(schematic: Vec<&str>, gear_ratios: bool) -> i32 {
    return retrieve_adjacent_part_numbers(to_vec_char(schematic), gear_ratios)
        .iter()
        .fold(0, |acc, n| acc + n);
}

pub(crate) fn retrieve_adjacent_part_numbers(
    schematic: Vec<Vec<char>>,
    gear_ratios: bool,
) -> Vec<i32> {
    let symbol_coordinates = find_all_symbols(&schematic);

    let mut part_numbers: Vec<i32> = vec![];

    for symbol in symbol_coordinates {
        if gear_ratios {
            let (r, c) = symbol;
            if schematic[r][c] == '*' {
                let result = retrieve_adjacent_part_number(symbol, &schematic);

                if result.len() == 2 {
                    part_numbers.append(&mut vec![result[0] * result[1]])
                }
            }
        } else {
            part_numbers.append(&mut retrieve_adjacent_part_number(symbol, &schematic))
        }
    }

    return part_numbers;
}

fn retrieve_adjacent_part_number(coord: Coord, schematic: &Vec<Vec<char>>) -> Vec<i32> {
    let (row, col) = coord;
    let mut row_l = 0;

    if row > 0 {
        row_l = row - 1
    }

    let mut row_u = row + 1;
    if row_u >= schematic.len() {
        row_u = schematic.len() - 1;
    }

    let mut col_l = 0;
    if col > 0 {
        col_l = col - 1;
    }

    let mut col_u = col + 1;
    if col_u >= schematic[0].len() {
        col_u = schematic[0].len() - 1;
    }

    let mut part_numbers: Vec<i32> = Vec::new();

    for r in row_l..=row_u {
        for c in col_l..=col_u {
            let cursor = schematic[r][c];

            if cursor.is_ascii_digit() {
                // try to not double count numbers and don't drop them from the edge
                if c + 1 <= col_u && schematic[r][c + 1].is_ascii_digit() {
                    continue;
                }

                part_numbers.append(&mut vec![find_number((r, c), schematic)])
            }
        }
    }

    return part_numbers;
}

fn find_number(coord: Coord, schematic: &Vec<Vec<char>>) -> i32 {
    let (row, col) = coord;

    let mut num = vec![schematic[row][col]];

    let mut left = 0;
    if col > 0 {
        left = col - 1;
    }
    while schematic[row][left].is_ascii_digit() {
        if (row, left) != coord {
            num.insert(0, schematic[row][left]);
        }

        if left == 0 {
            break;
        }

        left -= 1;
    }

    let mut right = col + 1;
    while right < schematic[row].len() && schematic[row][right].is_ascii_digit() {
        num.insert(num.len(), schematic[row][right]);
        right += 1;
    }

    let num_str: String = num.iter().collect();
    return num_str.parse().unwrap();
}

pub(crate) fn find_all_symbols(schematic: &Vec<Vec<char>>) -> Vec<Coord> {
    let mut symbol_coords: Vec<Coord> = Vec::new();

    for row in 0..schematic.len() {
        for column in 0..schematic[row].len() {
            if is_symbol(schematic[row][column]) {
                symbol_coords.append(&mut vec![(row, column)])
            }
        }
    }

    return symbol_coords;
}

fn is_symbol(c: char) -> bool {
    return c != '.' && !c.is_ascii_digit();
}

pub(crate) fn to_vec_char(input: Vec<&str>) -> Vec<Vec<char>> {
    return input.iter().map(|s| s.chars().collect()).collect();
}
