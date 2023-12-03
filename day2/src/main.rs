pub mod part1;
pub mod part2;

use crate::part2::cube_conundrum;

use std::path::Path;

fn main() {
    // let path = Path::new("resources/example_data.txt");
    let path = Path::new("resources/my_data.txt");

    println!("{}", cube_conundrum(path));
}
