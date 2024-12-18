use std::{ops::Index, str::FromStr, string::ParseError};

use advent_of_code::{load_sections_from_file, read_file_lines, split_and_parse_data};

const FIRST_DATASET_PATH: &str = "./data/5_print_queue.txt";
const SECOND_DATASET_PATH: &str = "./data/5_print_queue_2.txt";

type PageNumber = u32;

#[derive(Debug, PartialEq, Eq)]
struct OrderingRule(PageNumber, PageNumber);

impl FromStr for OrderingRule {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let res = line
            .split('|')
            .map(|s| {
                s.parse::<u32>()
                    .expect(format!("Error while parsing: {s:?}").as_str())
            })
            .collect::<Vec<u32>>();
        Ok(OrderingRule(res[0], res[1]))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct UpdateSchema(Vec<PageNumber>);

impl FromStr for UpdateSchema {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let res = line
            .split(',')
            .map(|s| {
                s.parse::<u32>()
                    .expect(format!("Error while parsing: {s:?}").as_str())
            })
            .collect::<Vec<u32>>();
        Ok(UpdateSchema(res))
    }
}

fn check_produce_udpate_ordering(
    rules: &Vec<OrderingRule>,
    schema: &UpdateSchema,
) -> Result<(), String> {
    for rule in rules {
        // println!("DEBUG: Rule: {rule:?}; Produce: {schema:?}");

        if !(schema.0.contains(&rule.0) && schema.0.contains(&rule.1)) {
            continue;
        } else if schema.0.contains(&rule.0) {
            let first = schema.0.iter().position(|x| *x == rule.0).unwrap_or(0);
            let second = schema.0.iter().position(|x| *x == rule.1).unwrap_or(1);
            if first < second {
                continue;
            } else {
                return Err(format!("{}|{}", rule.0, rule.1));
            }
        }

        todo!();
    }
    Ok(())
}

pub fn part_one(section_one: Vec<&str>, section_two: Vec<&str>) -> u32 {
    let s_one = split_and_parse_data::<OrderingRule>(section_one).unwrap();
    let s_two = split_and_parse_data::<UpdateSchema>(section_two).unwrap();
    let mut sum_of_middle = 0;

    for up_schema in &s_two {
        let res = check_produce_udpate_ordering(&s_one, up_schema);
        if res.is_ok() {
            let middle_number = up_schema.0.index(up_schema.0.len() / 2);
            sum_of_middle += *middle_number;
            println!("DEBUG: Success: {up_schema:?}, Middle: {middle_number:?}");
        } else {
            println!("DEBUG: Failed: {up_schema:?}, Error: {res:?}");
        }
    }

    sum_of_middle
}
pub fn part_two() {}

fn main() {
    {
        let (one, two) = load_sections_from_file(FIRST_DATASET_PATH);
        let data_one = one.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        let data_two = two.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        println!("Part 1: {}", part_one(data_one, data_two));
    }

    {
        // println!("Part 2: {}", part_two());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const DATA_RULE_ORDERING: [&str; 21] = [
        "47|53", "97|13", "97|61", "97|47", "75|29", "61|13", "75|53", "29|13", "97|29", "53|29",
        "61|53", "97|53", "61|29", "47|13", "75|47", "97|75", "47|61", "75|61", "47|29", "75|13",
        "53|13",
    ];
    const DATA_PRODUCE: [&str; 6] = [
        "75,47,61,53,29",
        "97,61,53,29,13",
        "75,29,13",
        "75,97,47,61,53",
        "61,13,29",
        "97,13,75,29,47",
    ];

    #[rstest]
    #[case("32|41", OrderingRule(32, 41))]
    #[case("52|81", OrderingRule(52, 81))]
    #[case("91|21", OrderingRule(91, 21))]
    fn test_rule_ordering_parser(#[case] input: &str, #[case] res: OrderingRule) {
        let my_res = input.parse::<OrderingRule>().unwrap();
        assert_eq!(my_res, res);
    }

    #[rstest]
    #[case("75,47,61,53,29", UpdateSchema(vec![75, 47, 61, 53, 29]))]
    #[case("97,61,53,29,13", UpdateSchema(vec![97, 61, 53, 29, 13]))]
    #[case("75,29,13", UpdateSchema(vec![75, 29, 13]))]
    fn test_produce_parser(#[case] input: &str, #[case] res: UpdateSchema) {
        let my_res = input.parse::<UpdateSchema>().unwrap();
        assert_eq!(my_res, res);
    }

    #[rstest]
    #[case(FIRST_DATASET_PATH, 0)]
    fn test_loading_content(#[case] path: &str, #[case] res: i32) {
        let (section_one, section_two) = load_sections_from_file(path);
        assert!(!section_one.is_empty());
        assert!(!section_two.is_empty());
    }

    #[rstest]
    #[case(DATA_RULE_ORDERING.to_vec(), DATA_PRODUCE.to_vec(), 143)]
    fn test_part_one(
        #[case] section_one: Vec<&str>,
        #[case] section_two: Vec<&str>,
        #[case] res: u32,
    ) {
        let sum_of_middle = part_one(section_one.to_vec(), section_two.to_vec());
        assert_eq!(sum_of_middle, res);
    }
}
