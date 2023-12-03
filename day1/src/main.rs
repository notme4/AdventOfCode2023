pub mod part1;

use crate::part1::trebuchet;

use std::path::Path;

fn main() {
    // let path = Path::new("resources/exampleData.txt");
    let path = Path::new("resources/myData.txt");

    println!("{}", trebuchet(&path));
}
