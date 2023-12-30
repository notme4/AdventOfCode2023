use std::fs::read_to_string;
use std::path::Path;

type MappingZone = (u32, u32, u32); // start from, start to, len
type Id<'a> = (u32, u32, &'a str); // seedNum, current id, id type

struct Mapping<'a> {
    from: &'a str,
    to: &'a str,

    mapping_zones: Vec<MappingZone>,
}

fn in_mapping_zone(id: u32, mapping: &MappingZone) -> bool {
    return mapping.0 < id && (id as u64) < (mapping.0 as u64) + (mapping.2 as u64);
}

fn to_vec_u32(line: &str) -> Vec<u32> {
    line.split(" ")
        .filter_map(|val| val.parse::<u32>().ok())
        .collect()
}

fn get_seed_numbers(seed_str: &str) -> Vec<(u32, u32, &str)> {
    let (_, seed_str) = seed_str.split_at(seed_str.find("\n").unwrap() + 1);
    return to_vec_u32(seed_str)
        .into_iter()
        .map(|val| (val, val, "seed"))
        .collect();
}

fn get_mapping_zone(line: &str) -> Option<MappingZone> {
    let v = to_vec_u32(line);
    if v.len() < 3 {
        return None;
    }
    return Some((v[1], v[0], v[2])); // thought it was ordered the other way, this is the easy fix
}

fn get_mapping<'a>(mapping_str: &&'a str) -> Mapping<'a> {
    let mut mapping_iter = mapping_str.split("\n");
    let from_to_str = mapping_iter.next().unwrap();
    let (from_to_str, _) = from_to_str.split_at(from_to_str.find(" ").unwrap());
    let (from, to) = from_to_str.split_at(from_to_str.find("-").unwrap());
    let (_, to) = to.split_at(4);
    return Mapping {
        from: from,
        to: to,
        mapping_zones: mapping_iter.filter_map(get_mapping_zone).collect(),
    };
}

fn convert_id<'a>(id: &Id, mapping: &Mapping<'a>) -> Id<'a> {
    if id.2 != mapping.from {
        panic!(
            "invalid conversion attempt: id: {}, mapping: {}, {}",
            id.2, mapping.from, mapping.to
        );
    }
    let map: Vec<_> = mapping
        .mapping_zones
        .iter()
        .filter(|val| in_mapping_zone(id.1, val))
        .collect();
    if map.len() == 0 {
        eprintln!(
            "({}, {}, {}) -> ({}, {}, {})",
            id.0, id.1, id.2, id.0, id.1, mapping.to
        );
        return (id.0, id.1, mapping.to);
    }
    let new_id = id.1 - map[0].0 + map[0].1;
    eprintln!("map: {}, {}, {}", map[0].0, map[0].1, map[0].2);
    eprintln!(
        "({}, {}, {}) -> ({}, {}, {})",
        id.0, id.1, id.2, id.0, new_id, mapping.to
    );
    return (id.0, new_id, mapping.to);
}

pub fn lowest_seed_location(path: &Path) -> u64 {
    let file_data = read_to_string(path).unwrap();
    let categories: Vec<&str> = file_data.split("\n\n").collect();
    let seed_numbers = get_seed_numbers(categories[0]);
    let mappings: Vec<Mapping> = categories[1..].iter().map(get_mapping).collect();
    let mut seeds = seed_numbers
        .iter()
        .map(|seed| {
            let mut s = *seed;
            for mapping in &mappings {
                s = convert_id(&s, &mapping);
            }
            return s;
        })
        .collect::<Vec<_>>();
    seeds.sort_by(|val1, val2| val1.1.partial_cmp(&val2.1).unwrap());
    return seeds[0].1.into();
}
