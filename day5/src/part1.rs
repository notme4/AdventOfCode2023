use std::fs::read_to_string;
use std::path::Path;

type Mapping = (u32, u32, u32); // start from, start to, len

fn in_mapping(id: u32, mapping: Mapping) -> bool {
    return mapping.0 < id && id < mapping.0 + mapping.2;
}

fn to_vec_u32(line: &str) -> Vec<u32> {
    line.split(" ")
        .filter_map(|val| val.parse::<u32>().ok())
        .collect()
}

fn get_seed_numbers(seed_str: &str) -> Vec<(u32, u32)> {
    let (_, seed_str) = seed_str.split_at(seed_str.find("\n").unwrap() + 1);
    return to_vec_u32(seed_str).into_iter().map(|val| (val, val)).collect();
}

pub fn lowest_seed_location(path: &Path) -> u32 {
    let file_data = read_to_string(path).unwrap();
    let categories: Vec<_> = file_data.split("\n\n").collect();
    let seed_numbers = get_seed_numbers(categories[0]);
    for c in categories {
        eprintln!("category: {}\n", c);
    }
    todo!()
}
