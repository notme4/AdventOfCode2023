pub mod part1;
pub mod part2;

use crate::part2::gear_ratios;

use std::path::Path;

fn main() {
    // let path = Path::new("resources/example_data.txt");
    let path = Path::new("resources/my_data.txt");
    // let path = Path::new("resources/test_data.txt");

    println!("{}", gear_ratios(path));
}
