use std::fs::read_to_string;
use std::path::Path;
use std::str::Lines;

pub fn scratch_cards(path: &Path) -> u32 {
    let data = read_to_string(path).unwrap();
    let lines = data.lines();
    return 1;
}