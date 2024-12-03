use std::cmp::Ordering;

use advent_of_code::read_csv;

type Report = Vec<i32>;
fn parse_data(dataset: Vec<String>) -> Vec<Report> {
    dataset
        .iter()
        .map(|el| {
            el.split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

enum Trend {
    NotInit,
    Increasing,
    Decreasing,
}
// gradually incrasing or decreasing.
// * levels are aither all increasing or all decreasing
// * any two adjacent levels differ by alteast one and at most three
fn check_report(report: &Report) -> bool {
    let mut trend = Trend::NotInit;

    for window in report.windows(2) {
        if let [prev, next] = window {
            if !(1..4).contains(&(prev - next).abs()) {
                println!("ERR[BadAdjacent({prev}-{next})]: {report:?};");
                return false;
            }

            match trend {
                Trend::NotInit => match prev.cmp(next) {
                    Ordering::Greater => trend = Trend::Decreasing,
                    Ordering::Less => trend = Trend::Increasing,
                    Ordering::Equal => return false,
                },
                Trend::Increasing => {
                    if prev > next {
                        println!("ERR[BadTrend({prev}-{next})]: {report:?}");
                        return false;
                    }
                }
                Trend::Decreasing => {
                    if prev < next {
                        println!("ERR[BadTrend({prev}-{next})]: {report:?}");
                        return false;
                    }
                }
            }
        }
    }

    println!("SAFE: {report:?}");
    true
}

pub fn part_one() {
    let dataset = read_csv("data/2_rednosed_reports.csv");
    let mut reports = parse_data(dataset);
    reports.retain(check_report);
    eprintln!("Result: {}", reports.len());
}

pub fn part_two() {}

fn main() {
    part_one();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![
        "7 6 4 2 1".to_string(),
        "1 2 7 8 9".to_string(),
        "9 7 6 2 1".to_string(),
        "1 3 2 4 5".to_string(),
        "8 6 4 4 1".to_string(),
        "1 3 6 7 9".to_string()
    ])]
    fn tdd(#[case] inputs: Vec<String>) {
        let mut reports = parse_data(inputs);
        reports.retain(check_report);
        assert_eq!(reports.len(), 2);
    }

    #[rstest]
    #[case(1, 1, false)]
    #[case(1, 2, true)]
    #[case(1, 3, true)]
    #[case(1, 4, true)]
    fn test_mmm(#[case] prev: i32, #[case] next: i32, #[case] res: bool) {
        let diff = dbg!((prev - next).abs());
        let is_good = (1..4).contains(&diff);
        assert_eq!(is_good, res);
    }
}
