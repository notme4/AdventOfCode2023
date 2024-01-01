use std::fs::read_to_string;
use std::path::Path;

fn to_vec_i64(line: &str) -> Vec<i64> {
    line.trim()
        .split(" ")
        .filter_map(|val| val.parse::<i64>().ok())
        .collect()
}

fn get_next(hist: Vec<i64>) -> i64 {
    if hist.iter().all(|val| *val == 0) {
        return 0;
    }
    let mut a = hist.iter();
    let first = a.next().unwrap();
    //let last = hist.last().unwrap();
    let seq = get_next(
        hist.iter()
            .zip(a)
            .map(|val| val.1 - val.0)
            .collect::<Vec<i64>>(),
    );

    //eprintln!("last: {first}");
    return first - seq;
}

pub fn func(path: &Path) -> i64 {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(to_vec_i64)
        .map(get_next)
        .sum()
}
