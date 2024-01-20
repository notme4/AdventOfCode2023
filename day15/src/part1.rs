use std::fs::read_to_string;
use std::path::Path;

fn hash(start_num: u8, s: &str) -> u8 {
    s.chars().fold(start_num, |u: u8, c: char| (u + c.try_into().unwrap()).wrapping_mult(17))
}

pub fn func(path: &Path) -> u64 {
    let file_data = read_to_string(path).unwrap();
    todo!();
}