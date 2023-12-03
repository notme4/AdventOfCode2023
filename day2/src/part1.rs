use core::cmp::max;
use std::fs::read_to_string;
use std::path::Path;

fn to_number(s: &str) -> u32 {
    s.chars()
        .map(|x| x.to_digit(10).unwrap())
        .fold(0, |acc, x| 10 * acc + x)
}

fn color_cubes(set: &str) -> (u32, u32, u32) {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let cubes = set.split(",");

    for color in cubes {
        let smth: Vec<_> = color.split(" ").collect();
        let c = smth[2].chars().next().unwrap();
        if c == 'r' {
            r = to_number(smth[1]);
        } else if c == 'g' {
            g = to_number(smth[1]);
        } else if c == 'b' {
            b = to_number(smth[1]);
        }
    }
    (r, g, b)
}

// returns a tuple of the max number of cubes used in each game in the order RGB
fn max_color_cubes(line: &str) -> (u32, (u32, u32, u32)) {
    let game: Vec<_> = line.split(":").collect();
    let game_id = to_number(&game[0][5..]);

    let sets = game[1].split(";");
    let max = sets.map(color_cubes).fold((0, 0, 0), |acc, x| {
        (max(acc.0, x.0), max(acc.1, x.1), max(acc.2, x.2))
    });

    (game_id, max)
}

fn has_enough_cubes(r: u32, g: u32, b: u32, cubes: &(u32, (u32, u32, u32))) -> bool {
    cubes.1 .0 <= r && cubes.1 .1 <= g && cubes.1 .2 <= b
}

pub fn cube_conundrum(path: &Path) -> u32 {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(max_color_cubes)
        .filter(|x| has_enough_cubes(12, 13, 14, x))
        .map(|x| x.0)
        .sum()
}
