use std::fs::read_to_string;
use std::path::Path;

pub fn func(path: &Path) -> i64 {
    read_to_string(path).unwrap();
    todo!()
}
