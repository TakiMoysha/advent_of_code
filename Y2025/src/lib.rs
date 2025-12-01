#![allow(non_snake_case)]

use std::fs;
use std::path::Path;

mod macros;

pub fn read_file_lines<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let canonical_path = fs::canonicalize(path).expect("Path {path:?} is invalid");
    fs::read_to_string(canonical_path)
        .expect("Can't read file: {path:?}")
        .lines()
        .map(String::from)
        .collect()
}
