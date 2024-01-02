use std::fs::read_to_string;
use std::path::Path;

fn rotate_vec<T: Copy>(vec: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut new_vec = Vec::<Vec<T>>::new();
    for i in 0..vec[0].len() {
        new_vec.push(Vec::<T>::new());
        for v in vec {
            new_vec[i].push(*v.iter().skip(i).next().unwrap());
        }
    }
    return new_vec;
}

fn get_empty_rows(cosmos: &mut Vec<Vec<char>>, size: i64) -> Vec<i64> {
    let mut result = Vec::<i64>::new();
    let mut i = 0;
    while i < cosmos.len() {
        let row = &cosmos[i];
        result.push(if row.iter().all(|c| *c == '.') {
            size
        } else {
            1
        });
        i += 1;
    }
    return result;
}

fn find_galaxies(cosmos: &Vec<Vec<char>>) -> Vec<(i64, i64)> {
    let mut result = Vec::<(i64, i64)>::new();
    for (i, row) in cosmos.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                result.push((i.try_into().unwrap(), j.try_into().unwrap()));
            }
        }
    }
    return result;
}

pub fn func(path: &Path) -> i64 {
    let file_data = read_to_string(path).unwrap();
    let mut cosmos = file_data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let col = get_empty_rows(&mut cosmos, 1_000_000);
    cosmos = rotate_vec(&cosmos);
    let row = get_empty_rows(&mut cosmos, 1_000_000);
    let galaxies = find_galaxies(&cosmos);
    let mut result: i64 = 0;
    for (i, galaxy1) in galaxies.iter().enumerate() {
        for galaxy2 in galaxies.iter().skip(i) {
            for r in galaxy1.0..galaxy2.0 {
                let a: i64 = row[TryInto::<usize>::try_into(r).unwrap()];
                result += a;
            }
            let [min, max] = std::cmp::minmax(galaxy2.1, galaxy1.1);
            for c in min..max {
                let a: i64 = col[TryInto::<usize>::try_into(c).unwrap()];
                result += a;
            }
            //result += (galaxy2.0 - galaxy1.0).abs() + (galaxy2.1 - galaxy1.1).abs();
        }
    }
    return result;
}
