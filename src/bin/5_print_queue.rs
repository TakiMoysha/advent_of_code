use std::{str::FromStr, string::ParseError};

use advent_of_code::read_file_lines;

const FIRST_DATASET_PATH: &str = "./data/5_print_queue.txt";
const SECOND_DATASET_PATH: &str = "./data/5_print_queue_2.txt";

type PageNumber = u32;

#[derive(Debug, PartialEq, Eq)]
struct RuleOrderingSection(PageNumber, PageNumber);

impl FromStr for RuleOrderingSection {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let res = line
            .split('|')
            .map(|s| {
                s.parse::<u32>()
                    .expect(format!("Error while parsing: {s:?}").as_str())
            })
            .collect::<Vec<u32>>();
        Ok(RuleOrderingSection(res[0], res[1]))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ProduceUpdateSection(Vec<PageNumber>);

impl FromStr for ProduceUpdateSection {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let res = line
            .split(',')
            .map(|s| {
                s.parse::<u32>()
                    .expect(format!("Error while parsing: {s:?}").as_str())
            })
            .collect::<Vec<u32>>();
        Ok(ProduceUpdateSection(res))
    }
}

fn data_parser<F: FromStr>(s: &str) -> Result<Vec<F>, F::Err> {
    // split s and parse every string to F, then collect and return
    s.lines()
        .map(|s| s.parse::<F>())
        .collect::<Result<Vec<F>, F::Err>>()
}

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
    let (section_one, section_two) = load_sections_from_file("./data/5_print_queue.txt");
}
pub fn part_two() {}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const DATA_RULE_ORDERING: &str = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n";
    const DATA_PRODUCE: &str =
        "75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47\n";

    #[rstest]
    #[case(DATA_RULE_ORDERING, DATA_PRODUCE, 0)]
    fn test_part_one(#[case] section_one: &str, #[case] section_two: &str, #[case] res: i32) {
        let s_one = data_parser::<RuleOrderingSection>(section_one).unwrap();
        let s_two = data_parser::<ProduceUpdateSection>(section_two).unwrap();
    }

    #[rstest]
    #[case("32|41", RuleOrderingSection(32, 41))]
    #[case("52|81", RuleOrderingSection(52, 81))]
    #[case("91|21", RuleOrderingSection(91, 21))]
    fn test_rule_ordering_parser(#[case] input: &str, #[case] res: RuleOrderingSection) {
        let my_res = input.parse::<RuleOrderingSection>().unwrap();
        assert_eq!(my_res, res);
    }

    #[rstest]
    #[case("75,47,61,53,29", ProduceUpdateSection(vec![75, 47, 61, 53, 29]))]
    #[case("97,61,53,29,13", ProduceUpdateSection(vec![97, 61, 53, 29, 13]))]
    #[case("75,29,13", ProduceUpdateSection(vec![75, 29, 13]))]
    fn test_produce_parser(#[case] input: &str, #[case] res: ProduceUpdateSection) {
        let my_res = input.parse::<ProduceUpdateSection>().unwrap();
        assert_eq!(my_res, res);
    }

    #[rstest]
    #[case(FIRST_DATASET_PATH, 0)]
    fn test_loading_content(#[case] path: &str, #[case] res: i32) {
        let (section_one, section_two) = load_sections_from_file(path);
        assert!(!section_one.is_empty());
        assert!(!section_two.is_empty());
    }
}
