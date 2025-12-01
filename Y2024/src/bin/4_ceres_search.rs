use std::collections::HashMap;

use Y2024::{dataset_path, read_file_lines};

// available solutions for ceres search:
// * defined directions (up, down, left, right, up-left, up-right, down-left, down-right)
// *
//
// good points:
// * find all X positions (as start)

#[derive(Debug)]
struct CeresListVec {
    data: Vec<Vec<char>>,
    line_len: usize,
    starts: Vec<(usize, usize)>,
}

impl CeresListVec {
    fn from_str(inputs: Vec<&str>) -> Self {
        let line_len = inputs[0].len();
        let data = inputs.iter().map(|line| line.chars().collect()).collect();
        let starts = vec![];
        Self {
            data,
            line_len,
            starts,
        }
    }

    fn find_index_by_symbol_in_line(line: &Vec<char>, symbol: &char) -> Vec<usize> {
        line.iter()
            .enumerate()
            .filter_map(|(idx, ch)| if *ch == *symbol { Some(idx) } else { None })
            .collect()
    }

    // !TODO: problem with orientations of M and S
    fn validate_shape_mas_by_point(point: (usize, usize), data: &Vec<Vec<char>>) -> i32 {
        println!("P: {point:?}");
        const WORLDS_SCHEMA: [(char, isize, isize); 4] =
            [('M', -1, -1), ('M', 1, -1), ('S', -1, 1), ('S', 1, 1)];

        for schema in WORLDS_SCHEMA {
            let current_char = &schema.0;
            let line_idx = point.0 as isize + schema.1;
            let char_idx = point.1 as isize + schema.2;

            if !(0 <= line_idx && line_idx < data.len() as isize)
                || !(0 <= char_idx && char_idx < data[line_idx as usize].len() as isize)
            {
                eprintln!("Out of range: {line_idx}, {char_idx}");
                break;
            }

            if &data[line_idx as usize][char_idx as usize] != current_char {
                break;
            }
        }

        1
    }

    fn search_xmas_by_point(point: (usize, usize), data: &Vec<Vec<char>>) -> i32 {
        const DIRECTIONS: [(isize, isize); 8] = [
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
        ];
        const WORLD: [char; 4] = ['X', 'M', 'A', 'S'];

        let mut occurrences = 0;
        for direction in DIRECTIONS {
            // validate world
            for (symbol_idx, symbol) in WORLD.iter().enumerate() {
                let idx = symbol_idx as isize;
                let line_idx = point.0 as isize + direction.0 * idx;
                let char_idx = point.1 as isize + direction.1 * idx;

                if !(0 <= line_idx && line_idx < data.len() as isize)
                    || !(0 <= char_idx && char_idx < data[line_idx as usize].len() as isize)
                {
                    // eprintln!("Out of range: {line_idx}, {char_idx}");
                    break;
                }

                if &data[line_idx as usize][char_idx as usize] != symbol {
                    // eprintln!("Not match: {symbol}, {line_idx}, {char_idx}");
                    break;
                }

                if symbol_idx == WORLD.len() - 1 {
                    // println!("\tCorrect: {symbol}, {line_idx}, {char_idx}!");
                    occurrences += 1;
                }
            }
        }
        occurrences
    }

    pub fn find_all_shape_mas_occurrences(self) -> i32 {
        let points: Vec<(usize, usize)> = self
            .data
            .iter()
            .enumerate()
            .flat_map(|(line_idx, line)| {
                Self::find_index_by_symbol_in_line(line, &'A')
                    .into_iter()
                    .map(|ch_idx| (line_idx, ch_idx))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();

        let res: Vec<i32> = points
            .iter()
            .map(|point| Self::validate_shape_mas_by_point(*point, &self.data))
            .collect();

        println!("res: {res:?}");
        0
    }

    pub fn find_all_xmas_occurrences(self) -> i32 {
        let points: Vec<(usize, usize)> = self
            .data
            .iter()
            .enumerate()
            .flat_map(|(line_idx, line)| {
                Self::find_index_by_symbol_in_line(line, &'X')
                    .into_iter()
                    .map(|ch_idx| (line_idx, ch_idx))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();

        points
            .iter()
            .map(|point| Self::search_xmas_by_point(*point, &self.data))
            .sum()
    }
}

pub fn part_one() -> i32 {
    let data = read_file_lines(dataset_path!("4_ceres_search.txt"));
    let prepared_data = data.iter().map(|el| el.as_str()).collect();
    let ceres = CeresListVec::from_str(prepared_data);
    ceres.find_all_xmas_occurrences()
}

pub fn part_two() -> i32 {
    let data = read_file_lines(dataset_path!("4_ceres_search_2.txt"));
    let prepared_data = data.iter().map(|el| el.as_str()).collect();
    let ceres = CeresListVec::from_str(prepared_data);
    ceres.find_all_shape_mas_occurrences()
}

fn main() {
    let one_result = part_one();
    println!("PartONE: {one_result:?}");
    let two_result = part_two();
    println!("PartTWO: {two_result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![
        "MMMSXXMASM",
        "MSAMXMSMSA",
        "AMXSXMAAMM",
        "MSAMASMSMX",
        "XMASAMXAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMMM",
        "MXMXAXMASX",
    ], 18)]
    fn test_part_one(#[case] input: Vec<&str>, #[case] res: i32) {
        let ceres = CeresListVec::from_str(input);
        assert_eq!(ceres.find_all_xmas_occurrences(), res);
    }

    #[rstest]
    #[case(vec![
        "MMMSXXMASM",
        "MSAMXMSMSA",
        "AMXSXMAAMM",
        "MSAMASMSMX",
        "XMASAMXAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMMM",
        "MXMXAXMASX",
    ], 9)]
    fn test_part_two(#[case] input: Vec<&str>, #[case] res: i32) {
        let ceres = CeresListVec::from_str(input);
        let my_res = ceres.find_all_shape_mas_occurrences();
        println!("res: {:?}", my_res);
        assert_eq!(my_res, res);
    }

    // #[rstest]
    // #[case(vec!["MMMSXXMASM",])]
    // fn test_line_potential_starts(#[case] input: Vec<&str>, #[case] res: i32) {}
}
