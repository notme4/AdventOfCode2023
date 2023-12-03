use std::fs::read_to_string;
use std::path::Path;

fn first_last_number(line: &str) -> usize {
    let mut first = usize::MAX;
    let mut first_pos = usize::MAX;
    let mut last = usize::MAX;
    let mut last_pos = 0;

    let search_strs = [
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
        ("0", 0),
    ];
    for p in search_strs {
        let mut i = line.find(p.0);
        first = if i.is_some_and(|x| x < first_pos) {
            unsafe {
                first_pos = i.unwrap_unchecked();
            }
            p.1
        } else {
            first
        };

        i = line.rfind(p.0);
        last = if i.is_some_and(|x| x >= last_pos) {
            unsafe {
                last_pos = i.unwrap_unchecked();
            }
            p.1
        } else {
            last
        };
    }
    return 10 * first + last;
}

pub fn trebuchet(path: &Path) -> usize {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(first_last_number)
        .sum();
}
