use super::{
    data::day_five_test_set,
    functions::{
        create_map_fn, extract_map, extract_seeds, find_nearest_soil_location, run_procedure,
        split_almanac_into_seeds_and_maps,
    },
};

#[test]
fn day_5_find_nearest_soil_location() {
    assert_eq!(find_nearest_soil_location(day_five_test_set()), 35);
}

#[test]
fn day_5_split_almanac_into_seeds_and_maps() {
    let case = vec![
        "seeds: 79 14 55 13",
        "",
        "seed-to-location map:",
        "0 1 2",
        "",
        "fertilizer-to-water map:",
        "2 1 0",
    ];

    let expected = (
        case[0],
        vec![vec![case[2], case[3]], vec![case[5], case[6]]],
    );

    assert_eq!(split_almanac_into_seeds_and_maps(case), expected);
}

#[test]
fn day_5_extract_seeds() {
    let cases = vec![("1 2 3", vec![1, 2, 3]), ("55 66 77", vec![55, 66, 77])];

    for (input, expected) in cases {
        assert_eq!(extract_seeds(input), expected)
    }
}

#[test]
fn day_5_extract_map() {
    let fne: fn(&i32) -> &i32 = |n: &i32| n;
    let inp = vec!["seed-to-soil map:", "0 1 2"];
    let cases = vec![(&inp, ("seed", "soil", fne))];

    for (input, expected) in cases {
        assert_eq!(extract_map(input), expected)
    }
}

#[test]
fn day_5_create_map_fn() {
    let cases = vec![((vec![vec![10, 0, 5]], 1), 11)];

    for ((map_input, fn_input), expected) in cases {
        assert_eq!(create_map_fn(map_input)(&fn_input), &expected);
    }
}

#[test]
fn day_5_run_procedure() {
    let pfn: fn(&i32) -> &i32 = |n: &i32| n;
    let cases = vec![((1, vec![pfn]), 1)];

    for ((seed_input, procedure), expected) in cases {
        assert_eq!(run_procedure(&seed_input, &procedure), expected)
    }
}
