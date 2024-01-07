use std::fs::read_to_string;
use std::path::Path;

fn transpose<T: Copy>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    return (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| *n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect();
}

fn count_load_row(platform: &Vec<char>) -> u64 {
    let mut start = 0;
    let mut num_round = 0;
    let mut result = 0;
    for (ind, c) in platform.iter().enumerate() {
        match c {
            '.' => assert!(true),
            'O' => num_round += 1,
            '#' => {
                for i in 0..num_round {
                    result += platform.len() - (start + i);
                }
                num_round = 0;
                start = ind + 1;
            }
            _ => panic!("invalid character"),
        }
    }
    for i in 0..num_round {
        result += platform.len() - (start + i);
    }
    return result.try_into().unwrap();
}

fn eprintln_through<T: std::fmt::Debug>(t: T) -> T {
    eprintln!("{:?}", t);
    return t;
}

pub fn func(path: &Path) -> u64 {
    let file_data = read_to_string(path).unwrap();
    let platform = file_data
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let platform = transpose(&platform);
    return platform
        .iter()
        .map(count_load_row)
        .map(eprintln_through)
        .sum();
}
