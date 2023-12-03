use std::fs::read_to_string;
use std::path::Path;

fn first_last_number(s: &str) -> u32 {
    let mut first = u32::max_value();
    let mut last = u32::max_value();
    for c in s.chars() {
        if c.is_numeric() {
            if first == u32::max_value() {
                first = c.to_digit(10).unwrap();
            }
            last = c.to_digit(10).unwrap();
        }
    }
    return 10 * first + last;
}

pub fn trebuchet(path: &Path) -> u32 {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(first_last_number)
        .sum();
}
