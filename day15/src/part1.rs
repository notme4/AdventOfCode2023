use std::fs::read_to_string;
use std::path::Path;

fn hash(start_num: u8, s: &str) -> u8 {
    s.chars().fold(start_num, |u: u8, c: char| {
        u.wrapping_add(<char as TryInto<u8>>::try_into(c).expect("Ascii"))
            .wrapping_mul(17)
    })
}

pub fn func(path: &Path) -> u64 {
    read_to_string(path)
        .unwrap()
        .split(",")
        .map(|s| hash(0, s) as u64)
        .sum()
}
