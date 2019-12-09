enum Parameter {
    Position(i32),
    Immediate(i32),
}

enum Instruction {
    Add((Parameter, Parameter, Parameter)),
    Mul((Parameter, Parameter, Parameter)),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue((Parameter, Parameter)),
    JumpIfFalse((Parameter, Parameter)),
    LessThan((Parameter, Parameter, Parameter)),
    Equals((Parameter, Parameter, Parameter)),
    Halt,
}

struct TestComputer {
    memory: Vec<i32>,
    input: Vec<i32>,
    output: Option<i32>,
    instruction_pointer: usize,
}

impl TestComputer {
    fn get_word(&mut self) -> i32 {
        let word = self.memory[self.instruction_pointer];
        self.instruction_pointer += 1;
        word
    }

    fn _get_parameter(&mut self, mode: i32) -> Parameter {
        let word = self.get_word();
        match mode {
            0 => Parameter::Position(word),
            1 => Parameter::Immediate(word),
            _ => panic!("unknown parameter mode"),
        }
    }

    fn get_param_1(&mut self, op_code: i32) -> Parameter {
        self._get_parameter(op_code % 10)
    }

    fn get_param_2(&mut self, op_code: i32) -> (Parameter, Parameter) {
        let p1 = self.get_param_1(op_code);
        let p2 = self._get_parameter((op_code / 10) % 10);
        (p1, p2)
    }

    fn get_param_3(&mut self, op_code: i32) -> (Parameter, Parameter, Parameter) {
        let (p1, p2) = self.get_param_2(op_code);
        let p3 = self._get_parameter((op_code / 100) % 10);
        (p1, p2, p3)
    }

    fn unwrap(&self, param: Parameter) -> i32 {
        match param {
            Parameter::Immediate(val) => val,
            Parameter::Position(pos) => self.memory[pos as usize],
        }
    }

    fn store_val(&mut self, param: Parameter, val: i32) {
        match param {
            Parameter::Position(out) => {
                self.memory[out as usize] = val;
            }
            _ => panic!("can not store to parameter in immediate mode"),
        }
    }

    fn emit_output(&mut self, param: Parameter) {
        self.output = Some(self.unwrap(param));
    }

    fn jump(&mut self, param: Parameter) {
        self.instruction_pointer = self.unwrap(param) as usize;
    }

    fn get_instruction(&mut self) -> Instruction {
        let mut op_code = self.get_word();
        let inst = op_code % 100;
        op_code /= 100;

        match inst {
            1 => Instruction::Add(self.get_param_3(op_code)),
            2 => Instruction::Mul(self.get_param_3(op_code)),
            3 => Instruction::Input(self.get_param_1(op_code)),
            4 => Instruction::Output(self.get_param_1(op_code)),
            5 => Instruction::JumpIfTrue(self.get_param_2(op_code)),
            6 => Instruction::JumpIfFalse(self.get_param_2(op_code)),
            7 => Instruction::LessThan(self.get_param_3(op_code)),
            8 => Instruction::Equals(self.get_param_3(op_code)),
            99 => Instruction::Halt,
            _ => panic!("unknown instruction"),
        }
    }

    fn run(&mut self) -> &mut Self {
        'program_loop: loop {
            let inst = self.get_instruction();
            match inst {
                Instruction::Add((p_1, p_2, p_3)) => {
                    let op1 = self.unwrap(p_1);
                    let op2 = self.unwrap(p_2);
                    self.store_val(p_3, op1 + op2);
                }
                Instruction::Mul((p_1, p_2, p_3)) => {
                    let op1 = self.unwrap(p_1);
                    let op2 = self.unwrap(p_2);
                    self.store_val(p_3, op1 * op2);
                }
                Instruction::Input(p_) => {
                    let inp = self.input.remove(0);
                    self.store_val(p_, inp);
                }
                Instruction::Output(p_) => {
                    self.emit_output(p_);
                }
                Instruction::JumpIfTrue((p_1, p_2)) => {
                    if self.unwrap(p_1) != 0 {
                        self.jump(p_2);
                    }
                }
                Instruction::JumpIfFalse((p_1, p_2)) => {
                    if self.unwrap(p_1) == 0 {
                        self.jump(p_2);
                    }
                }
                Instruction::LessThan((p_1, p_2, p_3)) => {
                    let op1 = self.unwrap(p_1);
                    let op2 = self.unwrap(p_2);
                    self.store_val(p_3, if op1 < op2 { 1 } else { 0 })
                }
                Instruction::Equals((p_1, p_2, p_3)) => {
                    let op1 = self.unwrap(p_1);
                    let op2 = self.unwrap(p_2);
                    self.store_val(p_3, if op1 == op2 { 1 } else { 0 })
                }
                Instruction::Halt => break 'program_loop,
            }
        }
        self
    }
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .filter_map(|x| x.parse::<i32>().ok())
        .collect()
}

#[aoc(day7, part1)]
fn solve_p1(instructions: &Vec<i32>) -> Option<i32> {
    println!("Input: {:?}", instructions);
    Some(0)
}

#[aoc(day7, part2)]
fn solve_p2(instructions: &Vec<i32>) -> Option<i32> {
    println!("Input: {:?}", instructions);
    Some(0)
}
