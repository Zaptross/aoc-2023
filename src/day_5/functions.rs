
// Create type brands to help with understanding
type MappableId = i32;
type SeedId = i32;
type MapKey<'lt> = &'lt str;

pub fn find_nearest_soil_location<'lt>(almanac: Vec<&str>) -> i32 {
    return 0;
}

pub(crate) fn split_almanac_into_seeds_and_maps(almanac: Vec<&str>) -> (&str, Vec<Vec<&str>>) {
    return ("", vec![vec![]]);
}

pub(crate) fn extract_seeds(list: &str) -> Vec<SeedId> {
    return vec![];
}

pub(crate) fn extract_map<'lt>(
    map: &Vec<&str>,
) -> (MapKey<'lt>, MapKey<'lt>, fn(&MappableId) -> &MappableId) {
    return ("", "", |x| x);
}

pub(crate) fn create_map_fn(map_params: Vec<Vec<i32>>) -> fn(&MappableId) -> &MappableId {
    return |x| x;
}

pub(crate) fn run_procedure(seed_id: &i32, procedure: &Vec<fn(&i32) -> &i32>) -> i32 {
    return 0;
}
