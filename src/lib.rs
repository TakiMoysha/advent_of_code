use std::fs;

pub fn read_csv(path: &str) -> Vec<String> {
    println!("Reading csv: {path:?}");

    let content: Vec<String> = fs::read_to_string(path)
        .expect("Can't read file: {path:?}")
        .lines()
        .map(String::from)
        .collect();

    content.into()
}
