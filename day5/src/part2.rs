use std::fs::read_to_string;
use std::path::Path;

type MappingRange = (u64, u64, u64, u64); // start src, end src, start dest end dest
type IdRange<'a> = (u64, u64, &'a str); // min id, max id, id type

struct Mapping<'a> {
    from: &'a str,
    to: &'a str,

    mapping_ranges: Vec<MappingRange>,
}

fn to_vec_u64(line: &str) -> Vec<u64> {
    line.split(" ")
        .filter_map(|val| val.parse::<u64>().ok())
        .collect()
}

fn id_range_sort(idr1: &IdRange, idr2: &IdRange) -> std::cmp::Ordering {
    return idr1.0.partial_cmp(&idr2.0).unwrap();
}

fn mapping_range_sort(m1: &MappingRange, m2: &MappingRange) -> std::cmp::Ordering {
    return m1.0.partial_cmp(&m2.0).unwrap();
}

fn minimize_id_ranges(v: &mut Vec<IdRange>) {
    return;
    v.sort_by(id_range_sort);
    let mut i = 0;
    while i + 1 < v.len() {
        //eprintln!("{} >= {} -> {}", v[i].1, v[i+1].0, v[i].1 >= v[i + 1].0);
        if v[i].1 >= v[i + 1].0 {
            v[i].1 = v[i + 1].1;
            v.remove(i + 1);
            continue;
        }
        i += 1;
    }
}

fn get_seed_id_ranges(seed_str: &str) -> Vec<IdRange> {
    let (_, seed_str) = seed_str.split_at(seed_str.find("\n").unwrap() + 1);
    let seed_vals = to_vec_u64(seed_str);
    let mut result = Vec::<IdRange>::new();
    let mut i = 0;
    while i < seed_vals.len() {
        result.push((seed_vals[i], seed_vals[i] + seed_vals[i + 1] - 1, "seed"));
        i += 2;
    }
    minimize_id_ranges(&mut result);
    return result;
}

fn get_mapping_range(line: &str) -> Option<MappingRange> {
    let v = to_vec_u64(line);
    if v.len() < 3 {
        return None;
    }
    return Some((v[1], v[1] + v[2] - 1, v[0], v[0] + v[2] - 1));
}

fn get_mapping<'a>(mapping_str: &&'a str) -> Mapping<'a> {
    let mut mapping_iter = mapping_str.split("\n");
    let from_to_str = mapping_iter.next().unwrap();
    let (from_to_str, _) = from_to_str.split_at(from_to_str.find(" ").unwrap());
    let (from, to) = from_to_str.split_at(from_to_str.find("-").unwrap());
    let (_, to) = to.split_at(4);
    let mut mapping = Mapping {
        from: from,
        to: to,
        mapping_ranges: mapping_iter.filter_map(get_mapping_range).collect(),
    };
    mapping.mapping_ranges.sort_by(mapping_range_sort);
    return mapping;
}

fn id_range_in_mapping_range(id: &IdRange, map_range: &MappingRange) -> bool {
    eprintln!("  id: ({}, {}, {})", id.0, id.1, id.2);
    eprintln!(
        "  map_range: ({}, {}, {}, {})",
        map_range.0, map_range.1, map_range.2, map_range.3
    );
    eprintln!(
        "  id_in_mapping_range(id.0, map_range)-> {}",
        id_in_mapping_range(id.0, map_range)
    );
    eprintln!(
        "  id_in_mapping_range(id.1, map_range) -> {}",
        id_in_mapping_range(id.1, map_range)
    );
    eprintln!(
        "  (id.0 < map_range.0 && map_range.1 < id.1) -> {}",
        (id.0 < map_range.0 && map_range.1 < id.1)
    );
    return id_in_mapping_range(id.0, map_range)
        || id_in_mapping_range(id.1, map_range)
        || (id.0 < map_range.0 && map_range.1 < id.1);
}

fn id_in_mapping_range(id: u64, map_range: &MappingRange) -> bool {
    return map_range.0 <= id && id <= map_range.1;
}

fn map_id_ranges<'a>(ids: Vec<IdRange>, mapping: Mapping<'a>) -> Vec<IdRange<'a>> {
    if mapping.from != ids[0].2 {
        panic!(
            "invalid mapping attempt: id: {}, mapping.from: {}, mapping.to {}",
            ids[0].2, mapping.from, mapping.to
        );
    }

    let map_ranges = mapping.mapping_ranges;
    let mut result = Vec::<IdRange>::new();
    for id in ids {
        let mut ranges = map_ranges
            .iter()
            .filter(|val| id_range_in_mapping_range(&id, *val));
        let mut i = id.0;
        let mut next = ranges.next();
        while i < id.1 {
            match next {
                None => {
                    result.push((i, id.1, mapping.to));
                    break;
                }
                Some(range) => {
                    //eprintln!("range: ({}, {}, {}, {})", range.0, range.1, range.2, range.3);
                    if id_in_mapping_range(i, range) {
                        let end = std::cmp::min(id.1, range.1);
                        result.push((i - range.0 + range.2, end - range.0 + range.2, mapping.to));
                        next = ranges.next();
                        i = end;
                    } else {
                        result.push((i, range.0, mapping.to));
                        i = range.0;
                    }
                }
            }
            eprintln!(
                "end: ({}, {}, {})",
                result.last().unwrap().0,
                result.last().unwrap().1,
                result.last().unwrap().2
            );
        }
    }
    minimize_id_ranges(&mut result);
    return result;
}

fn eprint_id_ranges(ids: &Vec<IdRange>) {
    for id in ids {
        eprintln!("({}, {}, {})", id.0, id.1, id.2);
    }
}

pub fn lowest_seed_location(path: &Path) -> u64 {
    let file_data = read_to_string(path).unwrap();
    let categories: Vec<&str> = file_data.split("\n\n").collect();
    let mut seeds = get_seed_id_ranges(categories[0]);
    minimize_id_ranges(&mut seeds);
    let mappings: Vec<Mapping> = categories[1..].iter().map(get_mapping).collect();
    eprint_id_ranges(&seeds);
    for mapping in mappings {
        seeds = map_id_ranges(seeds, mapping);
        eprint_id_ranges(&seeds);
    }
    seeds.sort_by(id_range_sort);
    return seeds[0].0;
}
