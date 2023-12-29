use std::fs::read_to_string;
use std::path::Path;

pub fn func(path: &Path) -> u32 {
    let _ = read_to_string(path);
    todo!()
}
