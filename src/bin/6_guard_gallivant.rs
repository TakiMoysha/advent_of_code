const DIRECTIONS: [char; 4] = ['^', '>', 'v', '<'];
const OBSTACLE: char = '#';
const FOOTPRINT: char = 'X';

#[derive(Clone, Copy, PartialEq, Eq)]
struct Guardian {
    pos: (usize, usize),
    direction: usize,
}

impl Guardian {
    fn new(x: usize, y: usize, dir: char) -> Self {
        let direction: usize = DIRECTIONS.iter().position(|el| *el == dir).unwrap();
        Self {
            pos: (x, y),
            direction,
        }
    }

    fn turn_rith(&mut self) {
        self.direction += 1;
    }

    fn move_forward(&mut self) {
        match self.direction {
            0 => self.pos.1 -= 1,
            1 => self.pos.0 += 1,
            2 => self.pos.1 += 1,
            3 => self.pos.0 -= 1,
            _ => unreachable!("move_forward->direction match error"),
        }
    }
}

struct Field<'a> {
    width: usize,
    height: usize,
    map: Vec<&'a str>,
    steps: u32,
    guard: Guardian,
}

impl<'a> From<&'a str> for Field<'a> {
    fn from(s: &'a str) -> Self {
        let map = s.lines().collect::<Vec<&str>>();
        let width = map[0].len();
        let height = map.len();

        let mut guard_position: Option<(usize, usize)> = None;
        for (line_indx, line) in map.iter().enumerate() {
            if let Some(pos) = line.chars().position(|c| DIRECTIONS.contains(&c)) {
                guard_position = Some((pos, line_indx));
                break;
            }
        }

        let (guard_x, guard_y) = guard_position.expect("Guard position not found!");
        let guard_direction = map[guard_y].chars().nth(guard_x).unwrap();
        let guard = Guardian::new(guard_x, guard_y, guard_direction);

        Field {
            width,
            height,
            map,
            steps: 0,
            guard,
        }
    }
}

impl Iterator for Field<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
// guard rules:
//  * turn right 90, if there is no obstacle
//  * otherwise take a step forward

fn main() {}

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
    fn test_part_one(#[case] input: &str, #[case] steps_for_leave: u32) {
        let field = Field::from(input);
        // assert_eq!(my_res, steps_for_leave);
    }
}
