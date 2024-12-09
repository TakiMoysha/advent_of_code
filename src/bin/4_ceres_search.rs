use advent_of_code::read_file_lines;

// available solutions for ceres search:
// * defined directions (up, down, left, right, up-left, up-right, down-left, down-right)
// *
//
// good points:
// * find all X positions (as start)

#[derive(Debug)]
struct Direction(isize, isize);

impl Direction {
    const UP: Self = Self(-1, 0);
    const DOWN: Self = Self(1, 0);
    const LEFT: Self = Self(0, -1);
    const RIGHT: Self = Self(0, 1);
    const UP_LEFT: Self = Self(-1, -1);
    const UP_RIGHT: Self = Self(-1, 1);
    const DOWN_LEFT: Self = Self(1, -1);
    const DOWN_RIGHT: Self = Self(1, 1);
}

const DIRECTIONS: [Direction; 8] = [
    Direction::LEFT,
    Direction::UP_LEFT,
    Direction::UP,
    Direction::UP_RIGHT,
    Direction::RIGHT,
    Direction::DOWN_RIGHT,
    Direction::DOWN_LEFT,
    Direction::DOWN,
];

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

    pub fn find_line_potential_starts(line: &Vec<char>) -> Vec<usize> {
        line.iter()
            .enumerate()
            .filter_map(|(indx, ch)| if *ch == 'X' { Some(indx) } else { None })
            .collect()
    }

    pub fn search_xmas_by_point(point: (usize, usize), data: &Vec<Vec<char>>) -> i32 {
        let mut occurrences = 0;
        let world = ['X', 'M', 'A', 'S'];
        for direction in DIRECTIONS {
            // validate world
            for (symbol_indx, symbol) in world.iter().enumerate() {
                let indx = symbol_indx as isize;
                let line_indx = point.0 as isize + direction.0 * indx;
                let char_indx = point.1 as isize + direction.1 * indx;

                if !(0 <= line_indx && line_indx < data.len() as isize)
                    || !(0 <= char_indx && char_indx < data[line_indx as usize].len() as isize)
                {
                    // eprintln!("Out of range: {line_indx}, {char_indx}");
                    break;
                }

                if &data[line_indx as usize][char_indx as usize] != symbol {
                    // eprintln!("Not match: {symbol}, {line_indx}, {char_indx}");
                    break;
                }

                if symbol_indx == world.len() - 1 {
                    // println!("\tCorrect: {symbol}, {line_indx}, {char_indx}!");
                    occurrences += 1;
                }
            }
        }
        occurrences
    }

    pub fn find_all_occurrences(self) -> i32 {
        let points: Vec<(usize, usize)> = self
            .data
            .iter()
            .enumerate()
            .flat_map(|(line_indx, line)| {
                Self::find_line_potential_starts(line)
                    .into_iter()
                    .map(|ch_indx| (line_indx, ch_indx))
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
    let data = read_file_lines("./data/4_ceres_search.txt");
    let prepared_data = data.iter().map(|el| el.as_str()).collect();
    let ceres = CeresListVec::from_str(prepared_data);
    ceres.find_all_occurrences()
}

pub fn part_two() {}

fn main() {
    let one_result = part_one();
    println!("PartONE: {one_result:?}");
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
    ], 0)]
    fn test_part_one(#[case] input: Vec<&str>, #[case] res: i32) {
        let ceres = CeresListVec::from_str(input);
        ceres.find_all_occurrences();
    }

    #[rstest]
    #[case(vec![], 0)]
    fn test_part_two(#[case] input: Vec<&str>, #[case] res: i32) {}

    // #[rstest]
    // #[case(vec!["MMMSXXMASM",])]
    // fn test_line_potential_starts(#[case] input: Vec<&str>, #[case] res: i32) {}
}
