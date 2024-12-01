use advent_of_code::read_csv;

const FIRST_DATASET_PATH: &str = "./data/1_historian_hysteria.csv";
const SECOND_DATASET_PATH: &str = "./data/1_historian_hysteria_2.csv";

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|el| el.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn clean_dataset(dataset: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::with_capacity(dataset.len());
    let mut right = Vec::with_capacity(dataset.len());

    for line in dataset.iter() {
        let p_line = parse_line(line);
        left.push(p_line[0]);
        right.push(p_line[1]);
    }

    left.sort();
    right.sort();

    (left, right)
}

fn find_smallest_distance(left: Vec<i32>, right: Vec<i32>) -> i32 {
    return left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();
}

fn find_smallest_distance_with_similarity_score(left: Vec<i32>, right: Vec<i32>) -> (i32, i32) {
    let mut similarity_score = 0;

    let distance: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| {
            let score = right.iter().filter(|el| *el == l).count();
            similarity_score += *l * score as i32;
            (l - r).abs()
        })
        .sum();

    (distance, similarity_score)
}

pub fn part_1() {
    let dataset = read_csv(FIRST_DATASET_PATH);
    let (left, right) = clean_dataset(dataset);
    let res = find_smallest_distance(left, right);
    println!("Part 1: {res}");
}

pub fn part_2() {
    let dataset = read_csv(SECOND_DATASET_PATH);
    let (left, right) = clean_dataset(dataset);
    let res = find_smallest_distance_with_similarity_score(left, right);
    println!("Part 2 [distance; similarity_score]: {res:?}");
}

fn main() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("12144 12125", vec![12144, 12125])]
    #[case("23620 85315", vec![23620, 85315])]
    #[case("96434 90834", vec![96434, 90834])]
    #[case("70853 80045", vec![70853, 80045])]
    fn test_line_parsing(#[case] line: &str, #[case] res: Vec<i32>) {
        let splited: Vec<i32> = parse_line(line);
        assert_eq!(splited, res);
    }

    #[rstest]
    #[case(vec!["8 2", "3 4", "2 1"], (vec![2, 3, 8], vec![1, 2, 4]) )]
    fn test_cleaning_dataset(#[case] inputs: Vec<&str>, #[case] res: (Vec<i32>, Vec<i32>)) {
        let inputs = inputs.iter().map(|el| el.to_string()).collect();
        let parsed = clean_dataset(inputs);
        assert_eq!(parsed, res);
    }

    #[rstest]
    #[case(vec!["3 4", "4 3", "2 5", "1 3", "3 9", "3 3"], 11)]
    fn tdd(#[case] inputs: Vec<&str>, #[case] res: i32) {
        let inputs = inputs.iter().map(|el| el.to_string()).collect();
        let (left, right) = clean_dataset(inputs);
        let my_res = find_smallest_distance(left, right);
        assert_eq!(my_res, res);
    }

    #[test]
    fn test_part_one() {
        part_1();
    }

    #[rstest]
    #[case(vec!["3 4", "4 3", "2 5", "1 3", "3 9", "3 3"], 31)]
    fn test_part_two(#[case] inputs: Vec<&str>, #[case] res: i32) {
        let inputs = inputs.iter().map(|el| el.to_string()).collect();
        let (left, right) = clean_dataset(inputs);
        let (_, similarity_score) =
            find_smallest_distance_with_similarity_score(left, right);
        assert_eq!(similarity_score, res);
    }
}
