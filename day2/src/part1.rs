use std::path::Path;

// returns a tuple of the max number of cubes used in each game in the order RGB
fn func(line: &str) -> (u32, u32, u32) {
    (0, 0, 0)
}

fn has_enough_cubes(r: u32, g: u32, b: u32, cubes: (u32, u32, u32)) -> bool {
    cubes.0 <= r && cubes.1 <= g && cubes.2 <= b
}

pub fn cube_conundrum(path: &Path) -> u32 {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(func)
        .filter(|x| has_enough_cubes(12, 13, 14, x))
        .sum()
}
