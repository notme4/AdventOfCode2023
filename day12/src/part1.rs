use regex::Regex;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

fn str_to_vec_str<'a>(s: &'a str, sep: &str) -> Vec<&'a str> {
    return s.split(sep).filter(|val| val.len() > 0).collect();
}

fn str_to_vec<T: FromStr>(s: &str, sep: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    return str_to_vec_str(s, sep)
        .into_iter()
        .map(|val| {
            val.parse::<T>().unwrap() //_or_else(|_| panic!("invalid parse unwrapping"))
        })
        .collect();
}

#[derive(Debug, Clone)]
struct SpringConditions {
    springs: String,
    consecutive_bad_springs: Vec<usize>,
    regex: Regex,
}

impl SpringConditions {
    fn new(line: &str) -> SpringConditions {
        let (conditions, consecutives) = line.split_at(line.find(" ").unwrap());
        // removes redundant '.'
        let conditions = str_to_vec_str(conditions.trim(), ".")
            .into_iter()
            .fold(".".to_string(), |s, v| format!("{s}{v}."));
        let consecutives = str_to_vec::<usize>(consecutives.trim(), ",");
        let regex_string = consecutives
            .iter()
            .fold(r"\.+".to_string(), |s, v| format!("{s}#{{{v}}}\\.+"));
        let regex = Regex::new(&regex_string).unwrap();
        return SpringConditions {
            springs: conditions,
            consecutive_bad_springs: consecutives,
            regex: regex,
        };
    }
}

// gets the next highest value with the same number of ones set
// implementation adjusted from https://www.geeksforgeeks.org/next-higher-number-with-same-number-of-set-bits/
struct Snoob {
    i: u64,
    max: u64,
}

impl Snoob {
    fn new(num_set_bits: u8, max: u64) -> Snoob {
        let i = 2_u64.pow(num_set_bits as u32) - 1;
        return Snoob { i: i, max: max };
    }
}

impl Iterator for Snoob {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            return None;
        }
        let old = self.i;
        let x = self.i as i64;
        let right_one = x & -x;
        let next_higher_one_bit = x + right_one;
        let mut right_ones_pattern = x ^ next_higher_one_bit;
        right_ones_pattern /= right_one;
        right_ones_pattern >>= 2;
        self.i = (next_higher_one_bit | right_ones_pattern) as u64;
        if old <= self.max {
            return Some(old);
        }
        return None;
    }
}

fn get_next_possible_replacement(s: String, i: u64) -> String {
    let mut j = 1;
    let mut s = s.chars().collect::<Vec<char>>();
    for c in s.iter_mut().rev() {
        if *c == '?' {
            if j & i != 0 {
                *c = '#';
            } else {
                *c = '.';
            }
            j <<= 1;
        }
    }
    return s.iter().collect();
}

fn get_possible_replacements(spring_conditions: SpringConditions) -> u64 {
    if !spring_conditions.springs.contains("?") {
        return 1;
    }
    let num_hashes = spring_conditions
        .springs
        .chars()
        .filter(|c| *c == '#')
        .count();
    let num_needed_hashes: usize = spring_conditions.consecutive_bad_springs.iter().sum();
    //eprint!("start get_possible_replacements: ");
    let num_set_bits = <usize as TryInto<u8>>::try_into(num_needed_hashes - num_hashes).unwrap();
    if num_set_bits == 0 {
        return 1;
    }
    let num_q_marks = spring_conditions
        .springs
        .chars()
        .filter(|c| *c == '?')
        .count();
    let max = 2_u64.pow(num_q_marks.try_into().unwrap());
    let mut result = 0;
    eprintln!("num_set_bits: {num_set_bits}, max: {max}");
    for i in Snoob::new(num_set_bits, max) {
        //eprint!(" i: {i}");
        let s = get_next_possible_replacement(spring_conditions.springs.clone(), i);
        eprint!("s: {s}");
        if spring_conditions.regex.is_match_at(&s, 0) {
            eprint!(" <<< ");
            result += 1;
        }
        eprint!("\n");
    }
    return result;
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
        .map(SpringConditions::new)
        //.map(eprintln_through)
        //.map(reduce_fmt)
        //.map(eprintln_through)
        .map(eprintln_through)
        .map(get_possible_replacements)
        .map(eprintln_through)
        .sum();
}
