use std::fs::read_to_string;
use std::path::Path;

pub fn func(path: &Path) -> u64 {
    eprintln!("{:?}", read_to_string(path).unwrap().split("\n\n"));
    todo!();
}
