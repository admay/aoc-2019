fn exec_instructions(mut ins: Vec<usize>) -> usize {
    let mut i: usize = 0;
    loop {
        let instruction: usize = ins[i];
        match instruction {
            1 => {
                let out_addr: usize = ins[i + 3];
                let x: usize = ins[ins[i + 1]];
                let y: usize = ins[ins[i + 2]];
                ins[out_addr] = x + y
            },
            2 => {
                let out_addr: usize = ins[i + 3];
                let x: usize = ins[ins[i + 1]];
                let y: usize = ins[ins[i + 2]];
                ins[out_addr] = x * y
            },
            99 => break,
            _ => {}
        }
        i += 4;
    }
    ins[0]
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
    let x: usize = exec_instructions(instructions);
    x
}

#[aoc(day2, part2)]
pub fn solve_p2(instructions: &Vec<usize>) -> usize {
    let mut solution = 0;
    'outer: for noun in 1..99 {
        for verb in 1..99 {
            let mut instructions = instructions.clone();
            instructions[1] = noun;
            instructions[2] = verb;
            if exec_instructions(instructions) == 19690720 {
                solution = 100 * noun + verb;
                break 'outer;
            }
        }
    }
    solution
}
