use advent_of_code::read_file_lines;

const FIRST_DATASET_PATH: &str = "./data/5_print_queue.txt";
const SECOND_DATASET_PATH: &str = "./data/5_print_queue_2.txt";

#[derive(Debug)]
struct RuleOrderingSection(String);
#[derive(Debug)]
struct ProduceUpdateSection(String);

pub fn load_sections_from_file(path: &str) -> (Vec<String>, Vec<String>) {
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

pub fn part_one() {
    load_sections_from_file("./data/5_print_queue.txt");
}
pub fn part_two() {}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13",
        "75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47",
        0
    )]
    fn part_one_test(#[case] section_one: &str, #[case] section_two: &str, #[case] res: i32) {}

    #[rstest]
    #[case(FIRST_DATASET_PATH, 0)]
    fn test_loading_content(#[case] path: &str, #[case] res: i32) {
        let (section_one, section_two)= load_sections_from_file(path);
        println!("section one: {:#?}\nsection two: {:#?}", section_one, section_two);
    }
}
