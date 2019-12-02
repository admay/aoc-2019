#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    ADD,
    MULT,
}

impl Instruction {
    fn new(from: usize) -> Option<Instruction> {
        match from {
            1 => Some(Instruction::ADD),
            2 => Some(Instruction::MULT),
            _ => None,
        }
    }
}

fn exec_instructions(mut ins: Vec<usize>) -> Option<usize> {
    let mut i: usize = 0;

    while let Some(instruction) = Instruction::new(ins[i]) {
        let out_addr = ins[i + 3];
        ins[out_addr] = match instruction {
            Instruction::ADD => ins[ins[i + 1]] + ins[ins[i + 2]],
            Instruction::MULT => ins[ins[i + 1]] * ins[ins[i + 2]],
        };
        i += 4;
    }

    ins.get(0).copied()
}

#[aoc_generator(day2)]
pub fn parse_program(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_p1(instructions: &Vec<usize>) -> usize {
    let mut instructions = instructions.clone();
    instructions[1] = 12;
    instructions[2] = 2;
    exec_instructions(instructions).ok_or(0).unwrap()
}

#[aoc(day2, part2)]
pub fn solve_p2(instructions: &Vec<usize>) -> usize {
    let mut solution = 0;
    'outer: for noun in 1..99 {
        for verb in 1..99 {
            let mut instructions = instructions.clone();
            instructions[1] = noun;
            instructions[2] = verb;
            let output = exec_instructions(instructions).ok_or(0).unwrap();
            if output == 19690720 {
                solution = 100 * noun + verb;
                break 'outer;
            }
        }
    }
    solution
}
