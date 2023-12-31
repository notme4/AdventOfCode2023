use std::fs::read_to_string;
use std::path::Path;

fn to_vec_i32(line: &str) -> Vec<i32> {
    line.trim()
        .split(" ")
        .filter_map(|val| val.parse::<i32>().ok())
        .collect()
}

fn margin_of_error_race(z: (f64, f64)) -> f64 {
    let (time, dist) = z;
    let time = f64::from(time);
    let dist = f64::from(dist) + 0.0001;
    eprintln!("time: {}, dist: {}", time, dist);
    let discriminant = (time * time + 4.0 * -dist).sqrt();
    eprintln!("discriminant: {}", discriminant);
    eprintln!(
        "result: {} - {} = {}",
        ((-time - discriminant) / -2.0).floor(),
        ((-time + discriminant) / -2.0).floor(),
        ((-time - discriminant) / -2.0).floor() - ((-time + discriminant) / -2.0).floor()
    );
    return ((-time - discriminant) / -2.0).floor() - ((-time + discriminant) / -2.0).floor();
}

pub fn find_margin_of_error(path: &Path) -> f64 {
    let file_data = read_to_string(path).unwrap();
    let (time_str, dist_str) = file_data.split_at(file_data.find("\n").unwrap() + 1);
    let time = time_str
        .trim()
        .split(" ")
        .fold("".to_owned(), |val1, val2| val1.to_owned() + val2)
        .parse::<f64>()
        .unwrap();
    let distance = dist_str
        .trim()
        .split(" ")
        .fold("".to_owned(), |val1, val2| val1.to_owned() + val2)
        .parse::<f64>()
        .unwrap();
    eprintln!("{}", time);
    eprintln!("{}", distance);
    return margin_of_error_race((time, distance));
    todo!();
}
