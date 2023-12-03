pub mod part1;

use crate::part1::trebuchet;

use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("resources/exampleData.txt");
    // let path = Path::new("../resources/myData.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    trebuchet(file);
}
