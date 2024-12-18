use std::{fs, str::FromStr};

pub fn read_file_lines(path: &str) -> Vec<String> {
    println!("Reading file: {path:?}");

    fs::read_to_string(path)
        .expect("Can't read file: {path:?}")
        .lines()
        .map(String::from)
        .collect()
}

// --------------------- 5_print_queue.rs ---------------------
pub fn load_sections_from_file(path: &str) -> (Vec<String>, Vec<String>) {
    // load the file content
    let mut content = read_file_lines(path);

    let (divider_indx, _) = content
        .iter()
        .enumerate()
        .find(|(_, line)| line.is_empty())
        .expect("empty line not found");

    let mut section_two: Vec<String> = content.split_off(divider_indx);
    section_two.remove(0);

    (content, section_two)
}

pub fn parse_struct_from_str<F: FromStr>(strings: Vec<&str>) -> Result<Vec<F>, F::Err> {
    // split s and parse every string to F, then collect and return
    strings
        .iter()
        .map(|s| s.parse::<F>())
        .collect::<Result<Vec<F>, F::Err>>()
}
