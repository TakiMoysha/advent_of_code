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

#[derive(PartialEq, Eq)]
enum DampenerStatus {
    Free,
    Taken,
}

fn _check_report_with_dampener(report: &Report) -> Result<(), usize> {
    let mut trend = Trend::NotInit;

    for window in report.windows(2).enumerate() {
        if let (indx, [prev, next]) = window {
            println!("DEBUG: {indx}: {prev}-{next}");
            let diff = prev - next;
            if !(1..=3).contains(&diff.abs()) {
                println!("ERR[BadAdjacent({indx}:{prev}-{next})]: {report:?};");
                return Err(indx);
            }

            match trend {
                Trend::NotInit => match diff.signum() {
                    1 => trend = Trend::Decreasing,
                    -1 => trend = Trend::Increasing,
                    0 => {
                        println!("DEBUG-RETURN: {indx}: {prev}-{next}");
                        return Err(indx);
                    }
                    _ => unreachable!(),
                },
                Trend::Increasing => {
                    if prev > next {
                        println!("ERR[BadTrend({indx}:{prev}-{next})]: {report:?}");
                        return Err(indx);
                    }
                }
                Trend::Decreasing => {
                    if prev < next {
                        println!("ERR[BadTrend({indx}:{prev}-{next})]: {report:?}");
                        return Err(indx);
                    }
                }
            }
        }
    }

    println!("SAFE: {report:?}");
    Ok(())
}

pub fn check_report_with_dampener(or_report: &Report) -> bool {
    let mut report = or_report.clone();
    match _check_report_with_dampener(&report) {
        Ok(_) => true,
        Err(_) => {
            for i in 0..or_report.len() {
                report.remove(i);
                if _check_report_with_dampener(&report).is_ok() {
                    return true;
                }
                report = or_report.clone();
            }
            false
        }
    }
}

// gradually incrasing or decreasing.
// * levels are aither all increasing or all decreasing
// * any two adjacent levels differ by alteast one and at most three
fn check_report(report: &Report) -> bool {
    let mut trend = Trend::NotInit;

    for window in report.windows(2) {
        if let [prev, next] = window {
            let diff = prev - next;
            if !(1..=3).contains(&diff.abs()) {
                println!("ERR[BadAdjacent({prev}-{next})]: {report:?};");
                return false;
            }

            match trend {
                Trend::NotInit => match diff.signum() {
                    1 => trend = Trend::Decreasing,
                    -1 => trend = Trend::Increasing,
                    0 => return false,
                    _ => unreachable!(),
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

pub fn part_one() -> i32 {
    let dataset = read_csv("data/2_rednosed_reports.csv");
    let mut reports = parse_data(dataset);
    reports.retain(check_report);
    reports.len() as i32
}

pub fn part_two() -> i32 {
    let dataset = read_csv("data/2_rednosed_reports_2.csv");
    let reports = parse_data(dataset);
    reports
        .iter()
        .filter(|el| check_report_with_dampener(el))
        .count() as i32
}

fn main() {
    part_one();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_answer_part_one() {
        let answer = part_one();
        println!("Part_1: {answer}");
    }

    #[test]
    fn test_answer_part_two() {
        let answer = part_two();
        println!("Part_2: {answer}");
    }

    #[rstest]
    #[case("7 6 4 2 1".to_string(), true)] // safe
    #[case("1 2 7 8 9".to_string(), false)] // unsafe
    #[case("9 7 6 2 1".to_string(), false)] // unsafe
    #[case("9 7 6 4 3".to_string(), true)] // safe
    #[case("1 3 2 4 5".to_string(), true)] // safe
    #[case("8 6 4 4 1".to_string(), true)] // safe
    #[case("1 3 6 7 9".to_string(), true)] // safe
    #[case("78 71 70 67 64 61".to_string(), true)] // safe - need removed 0 index
    #[case("26 24 24 22".to_string(), true)] // safe
    #[case("26 24 24 23".to_string(), true)] // safe
    #[case("1 4 6 7 6".to_string(), true)] // safe - removed last
    fn tdd(#[case] input: String, #[case] res: bool) {
        let report = input
            .split_whitespace()
            .map(|num_str| num_str.parse::<i32>().unwrap())
            .collect();
        let _res = check_report_with_dampener(&report);
        assert_eq!(_res, res);

        // reports.retain(check_report);
        // assert_eq!(reports.len(), 2);
    }

    #[rstest]
    #[case(1, 1, false)]
    #[case(1, 2, true)]
    #[case(1, 3, true)]
    #[case(1, 4, true)]
    fn test_contains(#[case] prev: i32, #[case] next: i32, #[case] res: bool) {
        let diff = dbg!((prev - next).abs());
        let is_good = (1..=3).contains(&diff);
        assert_eq!(is_good, res);
    }
}
