use advent_of_code::read_file_lines;
use regex::Regex;

#[derive(Debug)]
pub struct Instruction {
    name: String,
    args: Option<Vec<i32>>,
    len: Option<usize>,
    addr: Option<usize>,
}

impl Instruction {
    fn from_string(memory_slice: &str) -> Self {
        let start = memory_slice.find("(").unwrap();
        let end = memory_slice.find(")").unwrap();
        let name = String::from(&memory_slice[..start]);
        let args = if name == "mul" {
            Some(
                memory_slice[start + 1..end]
                    .split(",")
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
                    .to_vec(),
            )
        } else {
            None
        };

        Self {
            name: String::from(&memory_slice[..start]),
            args,
            len: None,
            addr: None,
        }
    }
}

pub fn find_instruction_by_regex(
    memory: &str,
    with_condition: bool,
) -> Result<Vec<Instruction>, String> {
    let mut res: Vec<Instruction> = vec![];

    let re = if with_condition {
        Regex::new("(mul\\([0-9]{1,3},[0-9]{1,3}\\)|do\\(\\)|don't\\(\\))").unwrap()
    } else {
        Regex::new("mul\\([0-9]{1,3},[0-9]{1,3}\\)").unwrap()
    };
    // println!("DEBUG: {memory}; {re:?}");

    let mut ignoring_instr = false;
    re.captures_iter(memory).for_each(|cap| {
        let _tmp: &str = &cap[0];
        let instr = Instruction::from_string(_tmp);
        match instr.name.as_str() {
            "do" => ignoring_instr = false,
            "don't" => ignoring_instr = true,
            _ => {
                if !ignoring_instr {
                    res.push(instr);
                }
            }
        }
    });

    Ok(res)
}

// !WARN: NOT IMPLEMENTED
// validate, if substring is 'mul(xxx,yyy)'
fn find_instruction(memory: &str) -> Result<Instruction, String> {
    let mut memory_dump = memory;
    while let Some(start_indx) = memory_dump.find("mul(") {
        let mut next_indx = start_indx;
        let mut is_ended = false;
        let mut instruction_length = 4;
        let mut first_arg: Vec<String> = vec![];
        let mut second_arg: Vec<String> = vec![];
        let mut x: Vec<String> = vec![];
        memory_dump = &memory_dump[start_indx..];

        for (indx, char) in memory_dump[4..].chars().enumerate() {
            // println!("iter: {char}");
            if !(char.is_numeric() || char == ',' || char == ')') {
                // break and clean if unexpected char
                break;
            }

            if char.is_numeric() && x.len() <= 3 {
                x.push(char.to_string());
                instruction_length += 1;
            } else if x.len() > 3 {
                next_indx = start_indx + "mul(".len();
                break;
            }

            if char == ',' {
                instruction_length += 1;
                first_arg.push(x.join(""));
                x = vec![];
            } else if char == ')' {
                instruction_length += 1;
                second_arg.push(x.join(""));
                x = vec![];
                is_ended = true;
            }

            if is_ended {
                return Ok(Instruction {
                    name: String::from("mul"),
                    args: Some(vec![
                        first_arg.join("").parse().unwrap(),
                        second_arg.join("").parse().unwrap(),
                    ]),
                    len: Some(instruction_length),
                    addr: Some(start_indx),
                });
            }
        }

        memory_dump = &memory_dump[next_indx + 3..];
    }

    Err(String::from("Invalid instruction"))
}

// !WARN: NOT IMPLEMENTED
fn part_one() -> i32 {
    let memory = read_file_lines("./data/3_mull_it_over.txt").join("");
    let mut start_index = 0;
    let mut instructions: Vec<Instruction> = vec![];
    while let Ok(instr) = find_instruction(&memory[start_index..]) {
        start_index += instr.addr.unwrap() + instr.len.unwrap();
        // println!("{instr:?}; {start_index}");
        instructions.push(instr);
    }
    // println!("Done: {instructions:?}");

    instructions
        .iter()
        .map(|instr| {
            let args = instr.args.as_ref().unwrap();
            args[0] * args[1]
        })
        .sum::<i32>()
}

pub fn part_one_by_regex() -> i32 {
    let memory = read_file_lines("./data/3_mull_it_over.txt").join("");
    let instructions = find_instruction_by_regex(memory.as_str(), false).unwrap();
    instructions
        .iter()
        .map(|instr| {
            if instr.name != "mul" {
                return 0;
            }

            let args = instr.args.as_ref().unwrap();
            args[0] * args[1]
        })
        .sum::<i32>()
}

pub fn part_two() -> i32 {
    let memory = read_file_lines("./data/3_mull_it_over_2.txt").join("");
    let instructions = find_instruction_by_regex(memory.as_str(), true).unwrap();
    instructions
        .iter()
        .map(|instr| {
            let args = instr.args.as_ref().unwrap();
            args[0] * args[1]
        })
        .sum::<i32>()
}

fn main() {
    part_one();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(196826776)]
    fn test_part_one(#[case] res: i32) {
        let my_res = part_one_by_regex();
        // println!("result: {:?} ; {}", my_res, res);
        assert_eq!(my_res, res);
    }

    #[rstest]
    #[case(106780429)]
    fn test_part_two(#[case] res: i32) {
        let my_res = part_two();
        // println!("result: {:?} ; {}", my_res, res);
        assert_eq!(my_res, res);
    }

    #[rstest]
    #[case(
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        48
    )]
    fn tdd_part_two(#[case] memory: &str, #[case] res: i32) {
        let instructions = find_instruction_by_regex(memory, true).unwrap();
        let my_res: i32 = instructions
            .iter()
            .map(|instr| {
                let args = instr.args.as_ref().unwrap();
                args[0] * args[1]
            })
            .sum();
        println!("result: {:?} ; {}", instructions, my_res);
        assert_eq!(my_res, res);
    }

    #[rstest]
    #[case(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        161
    )]
    fn tdd_part_one(#[case] memory: &str, #[case] res: i32) {
        let instructions = find_instruction_by_regex(memory, false).unwrap();
        let my_res: i32 = instructions
            .iter()
            .map(|instr| {
                let args = instr.args.as_ref().unwrap();
                args[0] * args[1]
            })
            .sum();
        // println!("result: {:?} ; {}", instructions, my_res);
        assert_eq!(my_res, res);
    }
}
