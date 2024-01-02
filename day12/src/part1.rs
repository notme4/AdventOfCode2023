use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

fn split_formats(line: &str) -> (Vec<&str>, Vec<u64>) {
    //eprint!("\nstart split_formats: ");
    let (conditions, consecutives) = line.split_at(line.find(" ").unwrap());

    return (
        conditions.split(".").filter(|val| val.len() > 0).collect(),
        consecutives
            .trim()
            .split(",")
            .map(|val| val.parse::<u64>().unwrap())
            .collect(),
    );
}

fn reduce_fmt(mut fmt: (Vec<&str>, Vec<u64>)) -> (Vec<&str>, Vec<u64>) {
    //eprint!("start reduce_fmts: ");
    while fmt.1.len() != 0 {
        if fmt.0[0].len() != fmt.1[0].try_into().unwrap() {
            break;
        }
        fmt.0.remove(0);
        fmt.1.remove(0);
    }
    while fmt.1.len() != 0 {
        if fmt.0.last().unwrap().len() != fmt.1.last().copied().unwrap().try_into().unwrap() {
            break;
        }
        fmt.0.remove(fmt.0.len() - 1);
        fmt.1.remove(fmt.1.len() - 1);
    }
    return fmt;
}

fn combine_and_regex(fmt: (Vec<&str>, Vec<u64>)) -> (String, Regex, u64) {
    //eprint!("start combine_and_regex: ");
    let r = fmt.1.iter().fold(r"\.+".to_string(), |mut s, v| {
        s.push_str(r"#{");
        s.push_str(&v.to_string());
        s.push_str(r"}\.+");
        s
    });
    return (
        fmt.0.iter().fold(".".to_string(), |mut s, v| {
            s.push_str(*v);
            s.push_str(".");
            s
        }),
        Regex::new(&r).unwrap(),
        fmt.1.iter().sum(),
    );
}

// gets the next highest value with the same number of ones set
// implementation adjusted from https://www.geeksforgeeks.org/next-higher-number-with-same-number-of-set-bits/
fn snoob(x: u64) -> u64 {
    if x == 0 {
        return 0;
    }
    let x = x as i64;
    let right_one = x & -x;
    let next_higher_one_bit = x + right_one;
    let mut right_ones_pattern = x ^ next_higher_one_bit;
    right_ones_pattern /= right_one;
    right_ones_pattern >>= 2;
    return (next_higher_one_bit | right_ones_pattern) as u64;
}

fn get_possible_replacements(sr: (String, Regex, u64)) -> Vec<String> {
    if sr.2 == 0 {
        return vec![sr.0];
    }
    //eprint!("start get_possible_replacements: ");
    let mut i = 2_u64.pow(
        <u64 as TryInto<u32>>::try_into(sr.2).unwrap()
            - <usize as TryInto<u32>>::try_into(sr.0.chars().filter(|c| *c == '#').count())
                .unwrap(),
    ) - 1;
    if i == 0 {
        return vec![sr.0];
    }
    let max = 2_u64.pow(
        sr.0.chars()
            .filter(|c| *c == '?')
            .count()
            .try_into()
            .unwrap(),
    );
    let mut v = Vec::<String>::new();
    eprintln!("i: {i} max: {max}");
    while i < max.into() {
        //eprint!(" i: {i}");
        let mut j = 1;
        let mut s = sr.0.clone().chars().collect::<Vec<char>>();
        let mut k = s.len() - 1;
        loop {
            if s[k] == '?' {
                if j & i != 0 {
                    s[k] = '#';
                } else {
                    s[k] = '.';
                }
                j <<= 1;
            }
            if k == 0 {
                break;
            }
            k -= 1;
        }
        v.push(s.iter().collect());
        i = snoob(i);
    }
    eprintln!("{:?}", v);
    return v
        .iter()
        .filter_map(|s| {
            if sr.1.is_match_at(s, 0) {
                Some(s.to_owned())
            } else {
                None
            }
        })
        .collect();
}

#[allow(dead_code)]
fn eprintln_through<T: std::fmt::Debug>(t: T) -> T {
    eprint!("{:?}\n", t);
    return t;
}

pub fn func(path: &Path) -> u64 {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(split_formats)
        //.map(eprintln_through)
        //.map(reduce_fmt)
        //.map(eprintln_through)
        .map(combine_and_regex)
        .map(eprintln_through)
        .map(get_possible_replacements)
        .map(eprintln_through)
        .map(|vec| vec.len() as u64)
        .sum();
}
