use std::fs::read_to_string;
use std::path::Path;

pub fn lowest_seed_location(path: &Path) -> u32 {
    read_to_string(path)
        .unwrap()
        .lines();
    todo!()
}
