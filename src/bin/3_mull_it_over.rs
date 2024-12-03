use advent_of_code::read_file_lines;
use regex::Regex;

#[derive(Debug)]
pub struct Instruction {
    name: String,
    args: Vec<i32>,
    len: usize,
    addr: usize,
}

impl Instruction {
    fn from_string(memory_slice: &str) -> Self {
        let start = memory_slice.find("(").unwrap();
        let end = memory_slice.find(")").unwrap();
        let args = &memory_slice[start + 1..end]
            .split(",")
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        Self {
            name: String::from("mul"),
            args: args.to_vec(),
            len: 0,
            addr: 0,
        }
    }
}

fn find_instruction_by_regex(memory: &str) -> Result<Vec<Instruction>, String> {
    let mut res: Vec<Instruction> = vec![];
    let re = Regex::new("mul\\([0-9]{1,3},[0-9]{1,3}\\)").unwrap();

    println!("DEBUG: {memory}; {re:?}");

    re.captures_iter(memory).for_each(|cap| {
        let _tmp: &str = &cap[0];
        res.push(Instruction::from_string(_tmp));
    });

    Ok(res)
}

// !WARN: NOT IMPLEMENTED
// validate, if substring is 'mul(xxx,yyy)'
fn find_instruction(memory: &str) -> Result<Instruction, String> {
    let mut memory_dump = memory.clone();
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
                    args: vec![
                        first_arg.join("").parse().unwrap(),
                        second_arg.join("").parse().unwrap(),
                    ],
                    len: instruction_length,
                    addr: start_indx,
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
        start_index += instr.addr + instr.len;
        println!("{instr:?}; {start_index}");
        instructions.push(instr);
    }
    // println!("Done: {instructions:?}");

    instructions
        .iter()
        .map(|instr| instr.args[0] * instr.args[1])
        .sum::<i32>()
}

fn part_one_by_regex() -> i32 {
    let memory = read_file_lines("./data/3_mull_it_over.txt").join("");
    let instructions = find_instruction_by_regex(memory.as_str()).unwrap();
    instructions
        .iter()
        .map(|instr| instr.args[0] * instr.args[1])
        .sum::<i32>()
}

fn part_two() {}

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
        assert_eq!(my_res, res);
    }

    #[rstest]
    #[case(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        161
    )]
    fn tdd(#[case] memory: &str, #[case] res: i32) {
        let instructions = find_instruction_by_regex(memory).unwrap();
        let my_res: i32 = instructions
            .iter()
            .map(|instr| instr.args[0] * instr.args[1])
            .sum();
        println!("result: {:?} ; {}", instructions, my_res);
    }
}
