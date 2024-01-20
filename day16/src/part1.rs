use std::fs::read_to_string;
use std::path::Path;

pub fn fun(path: &Path) -> u64 {
    let file_data = read_to_string(path).unwrap();
    todo!();
}