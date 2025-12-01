use std::{collections::HashSet, ops::IndexMut};

use Y2024::read_file_lines;

const DIRECTIONS: [char; 4] = ['^', '>', 'v', '<'];
const OBSTACLE: char = '#';
const FOOTPRINT: char = 'X';

type GuardPosition = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Guardian {
    pos: GuardPosition,
    direction: char,
}

impl Guardian {
    fn new(x: usize, y: usize, dir: char) -> Self {
        let _ = DIRECTIONS
            .iter()
            .position(|el| *el == dir)
            .expect("Guard direction not found!");
        Self {
            pos: (x, y),
            direction: dir,
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            _ => unreachable!("turn_right->direction match error"),
        };
    }

    fn move_forward(&self, position: &GuardPosition) -> GuardPosition {
        match self.direction {
            '^' => (position.0, position.1 - 1),
            '>' => (position.0 + 1, position.1),
            'v' => (position.0, position.1 + 1),
            '<' => (position.0 - 1, position.1),
            _ => unreachable!("move_forward->direction match error"),
        }
    }
}

#[derive(Clone)]
struct Laboratory<'a> {
    width: usize,
    height: usize,
    field: Vec<&'a str>, // can be replaced with char, but I wanted to delve into lifetime
    steps: u32,
    guard: Guardian,
    path: HashSet<GuardPosition>,
}

impl<'a> From<&'a str> for Laboratory<'a> {
    fn from(s: &'a str) -> Self {
        let field = s.lines().collect::<Vec<&str>>();
        let width = field[0].len() - 1;
        let height = field.len() - 1;

        let mut guard_position: Option<GuardPosition> = None;
        for (line_idx, line) in field.iter().enumerate() {
            if let Some(pos) = line.chars().position(|c| DIRECTIONS.contains(&c)) {
                guard_position = Some((pos, line_idx));
                break;
            }
        }

        let (guard_x, guard_y) = guard_position.expect("Guard position not found!");
        let guard_direction = field[guard_y].chars().nth(guard_x).unwrap();
        let guard = Guardian::new(guard_x, guard_y, guard_direction);

        Laboratory {
            width,
            height,
            field,
            steps: 0,
            guard,
            path: HashSet::new(),
        }
    }
}

impl Iterator for Laboratory<'_> {
    type Item = GuardPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_pos = self.guard.move_forward(&self.guard.pos);

        // check if guard left the field
        println!("DEBUG: {}; {:?};", self.steps, self.guard.pos);
        if new_pos.0 > self.width || new_pos.1 > self.height || new_pos.0 < 0 || new_pos.1 < 0 {
            println!("RES: {};", self.path.len());
            return None;
        }

        // check if there is an obstacle
        if self.field[new_pos.1].chars().nth(new_pos.0).unwrap() == OBSTACLE {
            self.guard.turn_right();
            new_pos = self.guard.move_forward(&self.guard.pos);
        }

        self.path.insert(self.guard.pos);
        self.guard.pos = new_pos;
        self.steps += 1;
        Some(self.guard.pos)
    }
}

// guard rules:
//  * turn right 90, if there is no obstacle
//  * otherwise take a step forward

fn part_one(input: &str) {
    Laboratory::from(input).for_each(|_| {});
}

fn main() {
    let input = read_file_lines(dataset_path!("6_guard_gallivant.txt")).join("\n");
    part_one(input.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    const TEST_FIELD: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[rstest]
    #[case(TEST_FIELD, 41)]
    fn test_part_one(#[case] input: &str, #[case] res: u32) {
        let mut field = Laboratory::from(input);
        let c_field = field.clone();
        field.for_each(|_| {});
        // assert_eq!(my_res, res);
    }
}
