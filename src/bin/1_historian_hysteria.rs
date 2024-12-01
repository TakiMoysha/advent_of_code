use advent_of_code::read_csv;

const DATASET_PATH: &str = "./data/1_historian_hysteria.csv";

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|el| el.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[derive(Debug, PartialEq)]
struct CleanDataset(Vec<i32>, Vec<i32>);

fn clean_dataset(dataset: Vec<String>) -> CleanDataset {
    let mut a = Vec::with_capacity(dataset.len());
    let mut b = Vec::with_capacity(dataset.len());

    for line in dataset.iter() {
        let p_line = parse_line(line);
        a.push(p_line[0]);
        b.push(p_line[1]);
    }

    a.sort();
    b.sort();
    CleanDataset(a, b)
}

fn find_smallest_distance(data: &CleanDataset) -> i32 {
    return data.0.iter().zip(data.1.iter()).map(|(l, r)| (l - r).abs()).sum();
}

fn main() {
    let dataset = read_csv(DATASET_PATH);
    let data = clean_dataset(dataset);
    let res = find_smallest_distance(&data);
    println!("{res}");
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
    #[case(vec!["8 2", "3 4", "2 1"], CleanDataset(vec![2, 3, 8], vec![1, 2, 4]))]
    fn test_cleaning_dataset(#[case] inputs: Vec<&str>, #[case] res: CleanDataset) {
        let inputs = inputs.iter().map(|el| el.to_string()).collect();
        let parsed = clean_dataset(inputs);
        assert_eq!(parsed, res);
    }

    #[rstest]
    #[case(vec!["3 4", "4 3", "2 5", "1 3", "3 9", "3 3"], 11)]
    fn tdd(#[case] inputs: Vec<&str>, #[case] res: i32) {
        let inputs = inputs.iter().map(|el| el.to_string()).collect();
        let parsed = clean_dataset(inputs);
        let my_res = find_smallest_distance(&parsed);
        assert_eq!(my_res, res);
    }
}
