use std::ops::Index;

use advent_of_code_2023::read_csv;

#[derive(Debug)]
struct ReplacedSlice {
    index_in_digit: usize,
    index_in_input: usize,
}

fn str_preprocess(input_val: String) -> String {
    let mut val = input_val.clone();
    let digit_nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // find all replaced
    let mut replaceable: Vec<ReplacedSlice> = Vec::new();
    for (indx, _str_num) in digit_nums.iter().enumerate() {
        if input_val.contains(_str_num) {
            replaceable.push(ReplacedSlice {
                index_in_digit: indx,
                index_in_input: input_val.find(_str_num).unwrap(),
            });
        }
    }

    // if no need to replace
    if replaceable.is_empty() {
        return val;
    };

    // select first and last replaced
    let mut start_replace = &replaceable[0];
    let mut end_replace = &replaceable[0];

    let _ = replaceable
        .iter()
        .map(|v| {
            if v.index_in_input < start_replace.index_in_input {
                start_replace = v;
            }

            if v.index_in_input > end_replace.index_in_input {
                end_replace = v;
            }
        })
        .collect::<()>();

    // if first correct, replace only last
    let mut it = val.chars().filter_map(|c| c.to_digit(10));
    // ?? if error => not correct the code
    let first_digit_num = it.next().expect("No number in string").to_string();

    println!("val: {val}, first_digit_num: {}, index: {:?}", first_digit_num, val.find(first_digit_num.as_str()) );

    // if i_first_digit_num < start_replace.index_in_input {
    //     val = val.replace(
    //         digit_nums.get(end_replace.index_in_digit).unwrap(),
    //         format!("{}", end_replace.index_in_digit + 1).as_str(),
    //     );
    //     return val;
    // }
    // let last_num = it.last().unwrap_or(first_num);

    // replace  
    val = val.replace(
        digit_nums.get(start_replace.index_in_digit).unwrap(),
        format!("{}", start_replace.index_in_digit + 1).as_str(),
    );
    if end_replace.index_in_input != start_replace.index_in_input {
        val = val.replace(
            digit_nums.get(end_replace.index_in_digit).unwrap(),
            format!("{}", end_replace.index_in_digit + 1).as_str(),
        );
    }
    println!("{}", val);
    val
}

fn find_number_in_val(val: &str) -> Result<i32, &str> {
    let _val = str_preprocess(val.to_string());
    let mut it = _val.chars().filter_map(|c| c.to_digit(10));

    let first_num = it.next().expect("No number in string");
    let last_num = it.last().unwrap_or(first_num);
    let res = format!("{first_num}{last_num}")
        .parse::<i32>()
        .expect("Can't parse number");

    println!("val: {}, res: {}", val, res);
    Ok(res)
}

fn part_1() -> i32 {
    let dataset = read_csv("./src/data/1_trebuchet_part_1.csv");
    dataset.iter().map(|v| find_number_in_val(v).unwrap()).sum()
}

fn part_2() -> i32 {
    let dataset = read_csv("./src/data/1_trebuchet_part_2.csv");
    dataset.iter().map(|v| find_number_in_val(v).unwrap()).sum()
}

fn main() {
    println!("{}", part_2());
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::*;

    pub fn ex_data_part_1<'a>() -> Vec<&'a str> {
        vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
    }
    pub fn ex_numbers_part_1() -> Vec<i32> {
        vec![12, 38, 15, 77]
    }
    pub fn ex_data_part_2<'a>() -> Vec<&'a str> {
        let data: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
7twonen
";
        return data.lines().collect();
    }
    pub fn ex_numbers_part_2() -> Vec<i32> {
        vec![29, 83, 13, 24, 42, 14, 76, 71]
    }

    #[test]
    fn example_find_correct_numbers_part_1() {
        let data = ex_data_part_1();
        let nums = ex_numbers_part_1();
        let _ = zip(data, nums)
            .map(|(d, n)| {
                let r_v = find_number_in_val(d).unwrap();
                assert_eq!(n, r_v);
            })
            .collect::<()>();
    }

    #[test]
    fn example_find_correct_numbers_part_2() {
        let data = ex_data_part_2();
        let nums = ex_numbers_part_2();
        let _ = zip(data, nums)
            .map(|(d, n)| {
                let r_v = find_number_in_val(d).unwrap();
                assert_eq!(n, r_v);
            })
            .collect::<()>();
    }
}
