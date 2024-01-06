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

fn to_regex(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c == '?' {
            result.push_str(r"[\.#]");
        } else if c == '.' {
            result.push_str("\\.+");
        } else {
            result.push(c);
        }
    }
    return result;
}
#[derive(Debug, Clone)]
struct SpringConditions {
    string: String,
    regex_springs: Regex,
    consecutive_bad_springs: Vec<u64>,
}

impl SpringConditions {
    fn new(line: &str) -> SpringConditions {
        let (conditions, consecutives) = line.split_at(line.find(" ").unwrap());
        // removes redundant '.'
        eprintln!("original str: {conditions}");
        let mut string = str_to_vec_str(conditions.trim(), ".")
            .iter()
            .fold(String::from("."), |s, v| format!("{s}{v}."));
        if line.chars().next().unwrap() != '.' {
            let mut chars = string.chars();
            let _ = chars.next();
            string = chars.as_str().to_string();
        }
        if line.chars().last().unwrap() != '.' {
            let mut chars = string.chars();
            let _ = chars.next_back();
            string = chars.as_str().to_string();
        }
        string = format!("{string}?").repeat(5);
        let mut chars = string.chars();
        let _ = chars.next_back();
        string = chars.as_str().to_string();
        string.push('.');
        let conditions = to_regex(&string);
        let consecutives = str_to_vec::<u64>(consecutives.trim(), ",").repeat(5);
        let regex_springs = Regex::new(&conditions).unwrap();
        return SpringConditions {
            string: string,
            regex_springs: regex_springs,
            consecutive_bad_springs: consecutives,
        };
    }
}

struct Replacer<'a> {
    fills: &'a Vec<u64>,
    gaps: Vec<u64>,
    done: bool,
}

impl<'a> Replacer<'a> {
    fn new(fills: &'a Vec<u64>, total_size: u64) -> Replacer {
        let mut gaps = Vec::<u64>::with_capacity(fills.len() + 1);
        gaps.resize(fills.len() + 1, 1);
        let filled: u64 = fills.iter().sum();
        let initial_gapped: u64 = fills.len().try_into().unwrap();
        gaps[0] = total_size.saturating_sub(filled + initial_gapped);
        return Replacer {
            fills: fills,
            gaps: gaps,
            done: false,
        };
    }

    fn get_possible_replacement(&self) -> String {
        let mut s = String::new();
        let mut gaps = self.gaps.iter();
        s.push_str(&".".repeat((*gaps.next().unwrap()).try_into().unwrap()));

        for (fill, gap) in self.fills.iter().zip(gaps) {
            s.push_str(&"#".repeat((*fill).try_into().unwrap()));
            s.push_str(&".".repeat((*gap).try_into().unwrap()));
        }
        return s;
    }
}

impl<'a> Iterator for Replacer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let res = self.get_possible_replacement();
        //eprintln!("res: {res}");

        let mut it = self.gaps.iter();
        let first = it.next().unwrap();
        if (*first == 0 || *first == 1) && it.all(|u| *u == 1) {
            self.done = true;
            return Some(res);
        }

        let gaps_size: u64 = self.gaps.iter().sum();
        //eprintln!("gaps: {:?}", self.gaps);
        let mut i = 0;
        while (i == 0 && self.gaps[0] == 0) || (i != 0 && self.gaps[i] == 1) {
            i += 1;
        }

        if i == self.gaps.len() - 1 {
            self.done = true;
            return Some(res);
        }

        //self.gaps[i] -= 1;
        self.gaps[i + 1] += 1;

        self.gaps[0] = 0;
        for j in 1..(i + 1) {
            self.gaps[j] = 1;
        }
        self.gaps[0] += gaps_size - self.gaps.iter().sum::<u64>();

        if gaps_size != self.gaps.iter().sum() {
            eprintln!("I was an idiot fixing gaps");
        }

        return Some(res);
    }
}

fn get_possible_replacements(spring_conditions: SpringConditions) -> u64 {
    eprintln!("string: {}", spring_conditions.string);
    eprintln!("regex: {}", spring_conditions.regex_springs);
    eprintln!(
        "consecutives: {:?}",
        spring_conditions.consecutive_bad_springs
    );
    let replacer = Replacer::new(
        &spring_conditions.consecutive_bad_springs,
        spring_conditions.string.len().try_into().unwrap(),
    );

    return replacer
        //.map(eprint_through)
        .filter(|r| spring_conditions.regex_springs.is_match_at(r, 0))
        .map(|v| {
            eprintln!("{v} <<<");
            return v;
        })
        .count()
        .try_into()
        .unwrap();
}

#[allow(dead_code)]
fn eprint_through<T: std::fmt::Debug>(t: T) -> T {
    eprint!("\n{:?}", t);
    return t;
}

pub fn func(path: &Path) -> u64 {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(SpringConditions::new)
        .map(get_possible_replacements)
        .sum();
}
