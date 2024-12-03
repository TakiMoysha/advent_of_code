use std::fs;

pub fn read_file_lines(path: &str) -> Vec<String> {
    println!("Reading file: {path:?}");

    fs::read_to_string(path)
        .expect("Can't read file: {path:?}")
        .lines()
        .map(String::from)
        .collect()
}
