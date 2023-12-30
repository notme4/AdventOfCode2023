use std::fs::read_to_string;
use std::path::Path;

fn to_vec_i32(line: &str) -> Vec<i32> {
    line.trim()
        .split(" ")
        .filter_map(|val| val.parse::<i32>().ok())
        .collect()
}

fn margin_of_error_race(z: (i32, i32)) -> f64 {
    let (time, dist) = z;
    let time = f64::from(time);
    let dist = f64::from(dist) + 0.0001;
    eprintln!("time: {}, dist: {}", time, dist);
    // zeros of (-x^2 + time*x - dist)
    let discriminant = (time * time + 4.0 * -dist).sqrt();
    eprintln!("discriminant: {}", discriminant);
    eprintln!(
        "result: {} - {} = {}",
        ((-time - discriminant) / -2.0).floor(),
        ((-time + discriminant) / -2.0).floor(),
        ((-time - discriminant) / -2.0).floor() - ((-time + discriminant) / -2.0).floor()
    );
    return ((-time - discriminant) / -2.0).floor() - ((-time + discriminant) / -2.0).floor();
    //-b +- sqrt(b^2 - 4*a*c) / 2a
    //-time +- sqrt(time*time - 4*-dist) / -2
}

pub fn find_margin_of_error(path: &Path) -> f64 {
    let file_data = read_to_string(path).unwrap();
    let (time_str, dist_str) = file_data.split_at(file_data.find("\n").unwrap() + 1);
    let times = to_vec_i32(time_str);
    let distances = to_vec_i32(dist_str);
    for t in &times {
        eprintln!("t: {}", t);
    }
    for d in &distances {
        eprintln!("d: {}", d);
    }
    return times
        .into_iter()
        .zip(distances)
        .map(margin_of_error_race)
        .fold(1.0, |val1, val2| val1 * val2);
}
