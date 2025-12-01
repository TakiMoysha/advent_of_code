// 2687 is too high

use Y2025::{dataset_path, read_file_lines};

static FIRST_INPUT_DATA: &str = dataset_path!("01_secret_entrance.csv");

struct SafeDial {
    cross_zero_count: i32,
    current_pos: i32,
}

impl SafeDial {
    fn new() -> Self {
        Self {
            cross_zero_count: 0,
            current_pos: 0,
        }
    }

    fn turn(&mut self, toward: &str) {
        let (direction, distance) = parse_input(toward);

        match direction {
            "R" => {
                let _mod = self.current_pos + distance;
                self.current_pos = _mod % 100;
                if _mod / 100 > 0 {
                    self.cross_zero_count += 1;
                }
            }
            "L" => {
                let _mod = self.current_pos + 100 - distance;
                self.current_pos = _mod % 100;
                if _mod <= 100 {
                    self.cross_zero_count += 1;
                }
            }
            _ => panic!("Invalid direction"),
        };
    }
}

pub fn parse_input(input: &str) -> (&str, i32) {
    (input.split_at(1).0, input[1..].parse::<i32>().unwrap())
}

fn process_safe_dial(commands: Vec<String>) {
    let mut dial = SafeDial::new();
    for command in commands {
        dial.turn(&command);
    }
    println!("Result: {}", dial.cross_zero_count);
}

fn main() {
    let data = read_file_lines(FIRST_INPUT_DATA);
    process_safe_dial(data);
}

// ===========================================================
// tests
// ===========================================================

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("L68", ("L", 68))]
    #[case("L30", ("L", 30))]
    #[case("R48", ("R", 48))]
    #[case("L4", ("L", 4))]
    #[case("R60", ("R", 60))]
    #[case("L1", ("L", 1))]
    #[case("L99", ("L", 99))]
    fn test_main(#[case] input: &str, #[case] res: (&str, i32)) {
        assert_eq!(parse_input(input), res)
    }

    #[rstest]
    #[case("R1", 1)]
    #[case("R41", 41)]
    #[case("L81", 19)]
    #[case("L81", 19)]
    fn test_turn(#[case] toward: &str, #[case] res: i32) {
        let mut dial = SafeDial::new();
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
        let mut dial = SafeDial::new();
        for instr in inputs {
            dial.turn(instr);
        }
        assert_eq!(dial.cross_zero_count, res);
    }
}
