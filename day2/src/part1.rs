use std::fs::read_to_string;
use std::path::Path;

fn color_cubes(hand: &str) -> (u32, u32, u32) {
    
}

// returns a tuple of the max number of cubes used in each game in the order RGB
fn max_color_cubes(line: &str) -> (u32, (u32, u32, u32)) {
    let game: Vec<_> = line.split(":").collect();
    let game_id = &game[0][5..]
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .fold(0, |acc, x| 10 * acc + x);

    let hands: Vec<_> = &game[1].split(";").collect();
    let max = hands
    println!("game_id: {}", game_id);
    (game_id, (0, 0, 0))
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
