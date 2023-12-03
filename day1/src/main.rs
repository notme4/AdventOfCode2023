pub mod part2;

use crate::part2::trebuchet;

use std::path::Path;

fn main() {
    // let path = Path::new("resources/exampleData.txt");
    // let path = Path::new("resources/exampleData2.txt");
    let path = Path::new("resources/myData.txt");

    println!("{}", trebuchet(&path));
}
