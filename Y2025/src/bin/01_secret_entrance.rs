// incorrect: 3415
use Y2025::{dataset_path, read_file_lines};

static FIRST_INPUT_DATA: &str = dataset_path!("01_secret_entrance.csv");
// static SECOND_INPUT_DATA: &str = dataset_path!("01_secret_entrance_2.csv");

#[derive(Debug, PartialEq)]
enum Direction {
    Left(i32),
    Right(i32),
}

#[derive(Debug)]
struct SafeDial {
    cross_zero_count: i32,
    set_at_zero: i32,
    current_pos: i32,
}

impl SafeDial {
    fn new(init_pos: i32) -> Self {
        Self {
            cross_zero_count: 0,
            set_at_zero: 0,
            current_pos: init_pos,
        }
    }

    fn turn(&mut self, code: &str) {
        let direction = parse_input(code);
        let mut new_pos = self.current_pos;

        match direction {
            Direction::Right(num) => {
                let _mod = (self.current_pos + num).rem_euclid(100);
                new_pos = _mod % 100;
                if _mod / 100 > 0 {
                    self.cross_zero_count += 1;
                }
            }
            Direction::Left(num) => {
                let _mod = (self.current_pos - num).rem_euclid(100);
                new_pos = _mod % 100;
                if _mod <= 100 {
                    self.cross_zero_count += 1;
                }
            }
            _ => unreachable!("Invalid input operator: {code}"),
        };

        if new_pos == 0 {
            self.set_at_zero += 1;
        }
        self.current_pos = new_pos;
    }
}

fn parse_input(input: &str) -> Direction {
    match input.split_at(1).0 {
        "R" => Direction::Right(input[1..].parse::<i32>().unwrap()),
        "L" => Direction::Left(input[1..].parse::<i32>().unwrap()),
        _ => unreachable!("Invalid input operator: {input}"),
    }
}

fn process_safe_dial(init_dial_pos: i32, commands: Vec<String>) {
    let mut dial = SafeDial::new(init_dial_pos);
    for command in commands {
        dial.turn(&command);
    }
    println!("Result: {dial:?} ({})", dial.set_at_zero + dial.cross_zero_count);
}

fn main() {
    let data = read_file_lines(FIRST_INPUT_DATA);
    process_safe_dial(50, data);
}

// ===========================================================
// tests
// ===========================================================

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("L68", Direction::Left(68))]
    #[case("L30", Direction::Left(30))]
    #[case("R48", Direction::Right(48))]
    #[case("L4", Direction::Left(4))]
    #[case("R60", Direction::Right(60))]
    #[case("L1", Direction::Left(1))]
    #[case("L99", Direction::Left(99))]
    fn test_main(#[case] input: &str, #[case] res: Direction) {
        assert_eq!(parse_input(input), res)
    }

    #[rstest]
    #[case("R1", 1)]
    #[case("R41", 41)]
    #[case("L81", 19)]
    #[case("L81", 19)]
    fn test_turn(#[case] toward: &str, #[case] res: i32) {
        let mut dial = SafeDial::new(0);
        dial.turn(toward);
        assert_eq!(dial.current_pos, res);
    }

    #[rstest]
    #[case("R99", 0)]
    #[case("L1", 1)]
    #[case("R8\nL12", 1)]
    #[case("R99\nR1", 1)]
    #[case("L4\nR4\nR4\nR3", 2)]
    fn test_cross_zero(#[case] input: &str, #[case] res: i32) {
        let inputs = input.lines().collect::<Vec<&str>>();
        let mut dial = SafeDial::new(0);
        inputs.iter().for_each(|instr| dial.turn(instr));
        assert_eq!(dial.cross_zero_count, res);
    }

    #[rstest]
    #[case("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82", 3)]
    fn should_return_coun_set_at_zero(#[case] input: &str, #[case] res: i32) {
        let inputs = input.lines().collect::<Vec<&str>>();
        let mut dial = SafeDial::new(50);
        inputs.iter().for_each(|instr| dial.turn(instr));
        assert_eq!(dial.set_at_zero, res);
    }

    #[rstest]
    #[case((84, 842), (42, 26))]
    fn test_circle_math(
        #[case] (start_pos, num): (i32, i32),
        #[case] (l_end_pos, r_end_pos): (i32, i32),
    ) {
        let l_new_pos = (start_pos - num).rem_euclid(100);
        let r_new_pos = (start_pos + num).rem_euclid(100);
        assert_eq!(l_new_pos, l_end_pos);
        assert_eq!(r_new_pos, r_end_pos);
    }
}
