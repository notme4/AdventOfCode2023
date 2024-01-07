use std::fs::read_to_string;
use std::path::Path;

fn is_vertical_reflection_point<T: AsRef<str> + std::cmp::PartialEq>(
    pattern: &Vec<T>,
    point: usize,
) -> bool {
    let mut count = 0;
    for (i, j) in (0..point).rev().zip(point..pattern.len()) {
        if pattern[i] != pattern[j] {
            for (c1, c2) in pattern[i].as_ref().chars().zip(pattern[j].as_ref().chars()) {
                count += if c1 != c2 { 1 } else { 0 };
            }
        }
    }
    if count == 1 {
        return true;
    }
    return false;
}

fn find_vertical_reflection_point<T: AsRef<str> + std::cmp::PartialEq>(pattern: &Vec<T>) -> u64 {
    for i in 1..pattern.len() {
        if is_vertical_reflection_point(pattern, i) {
            return i.try_into().unwrap();
        }
    }
    return 0;
}

//fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
//    assert!(!v.is_empty());
//    let len = v[0].len();
//    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
//    return (0..len)
//        .map(|_| {
//            iters
//                .iter_mut()
//                .map(|n| *n.next().unwrap())
//                .collect::<Vec<T>>()
//        })
//        .collect();
//}

fn transpose(v: &Vec<&str>) -> Vec<String> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.chars()).collect();
    return (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<String>()
        })
        .collect();
}

pub fn func(path: &Path) -> u64 {
    let file_data = read_to_string(path).unwrap();
    let patterns = file_data
        .split("\n\n")
        .map(|s| s.lines().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    //eprintln!("{:#?}", patterns);
    let horizontal_reflections = patterns
        .iter()
        .map(find_vertical_reflection_point)
        .collect::<Vec<_>>();
    eprintln!("horiz: {:?}", horizontal_reflections);
    //eprintln!("{:#?}", transpose(&patterns));
    let vertical_reflections = patterns
        .iter()
        .map(transpose)
        .map(|v| find_vertical_reflection_point(&v))
        .collect::<Vec<_>>();
    eprintln!("vert:  {:?}", vertical_reflections);
    return vertical_reflections.iter().sum::<u64>()
        + 100 * horizontal_reflections.iter().sum::<u64>();
}
