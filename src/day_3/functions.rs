// Coords are (row, column)
// Eg: (1,2) -> 6
// 123
// 456
// 789
pub(crate) type Coord = (usize, usize);

pub fn calculate_sum_of_part_numbers(schematic: Vec<&str>) -> i32 {
    return 0;
}

pub(crate) fn retrieve_adjacent_part_numbers(schematic: Vec<Vec<char>>) -> Vec<i32> {
    return vec![0];
}

pub(crate) fn find_all_symbols(schematic: Vec<Vec<char>>) -> Vec<Coord> {
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
