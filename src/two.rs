use super::util;

fn exec_program(program: &mut Vec<usize>) -> usize {
    let mut i = 0;
    loop {
        // get the instruction
        let instruction = program[i];

        // x_pointer and y_pointer
        let x_pointer = program[i + 1];
        let y_pointer = program[i + 2];

        let x = program[x_pointer];
        let y = program[y_pointer];

        // out pointerition
        let out = program[i + 3];

        // match for instructions
        match instruction {
            1 => {
                program[out] = x + y;
            },
            2 => {
                program[out] = x * y;
            },
            99 => {
                break;
            },
            _ => {}
        }

        i += 4;
    }

    program[0]
}

pub fn day_two(path: String) {
    let program: Vec<usize> = util::split_lines_from_file(path)
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    // part 1
    // let mut program = program.clone();
    // program[1] = 12;
    // program[2] = 2;

    // part 2
    for noun in 1..99 {
        for verb in 1..99 {
            let mut program = program.clone();
            program[1] = noun;
            program[2] = verb;

            let solution = exec_program(&mut program);

            if solution == 19690720 {
                let code = 100 * noun + verb;
                println!("Solution: {}", code);
            }
        }
    }
}
