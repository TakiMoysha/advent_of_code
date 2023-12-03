use advent_of_code_2023::read_csv;

fn find_number_in_val(line: &str) -> Result<u32, &str> {
    let mut number_iter = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            Some(1)
        } else if reduced_line.starts_with("two") {
            Some(2)
        } else if reduced_line.starts_with("three") {
            Some(3)
        } else if reduced_line.starts_with("four") {
            Some(4)
        } else if reduced_line.starts_with("five") {
            Some(5)
        } else if reduced_line.starts_with("six") {
            Some(6)
        } else if reduced_line.starts_with("seven") {
            Some(7)
        } else if reduced_line.starts_with("eight") {
            Some(8)
        } else if reduced_line.starts_with("nine") {
            Some(9)
        } else {
            reduced_line.chars().next().unwrap().to_digit(10)
        };

        result
    });
    let first_num = number_iter.next().expect("Undefined error, needed debug");
    let last_num = number_iter.last().unwrap_or(first_num);
    Ok(first_num * 10 + last_num)
}

fn main() {
    let dataset = read_csv("./src/data/1_trebuchet_dataset.csv");
    let res: u32 = dataset.iter().map(|line| find_number_in_val(line).unwrap()).sum();
    println!("Result: {res}");
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn part_1_example_find_correct_numbers(#[case] line: &str, #[case] res: u32) {
        assert_eq!(res, find_number_in_val(line).unwrap());
    }

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("7twonen", 71)]
    #[case("fivezg8jnf6hrxnhgxxttwoneg", 51)]
    fn part_2_example_find_correct_numbers(#[case] line: &str, #[case] res: u32) {
        assert_eq!(res, find_number_in_val(line).unwrap());
    }
}
