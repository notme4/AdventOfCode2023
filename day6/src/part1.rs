use std::fs::read_to_string;
use std::path::Path;

pub fn find_margin_of_error(path: &Path) -> u64 {
    read_to_string(path).unwrap();
    todo!()
}