// https://rosettacode.org/wiki/Permutations#Iterative
pub fn permutations(start: usize, end: usize) -> Permutations {
    Permutations {
        idxs: (start..=end).collect(),
        swaps: vec![0; end - start + 1],
        i: 0,
    }
}

pub struct Permutations {
    idxs: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
}

impl Iterator for Permutations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() {
                    return None;
                }

                if self.swaps[self.i] < self.i {
                    break;
                }

                self.swaps[self.i] = 0;
                self.i += 1;
            }

            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}

enum Parameter {
    Position(i64),
    Immediate(i64),
}

#[derive(PartialEq)]
enum Signal {
    NeedsInput,
    ProducedOutput,
    Halt,
    None,
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

struct IntCodeComputer {
    memory: Vec<i64>,
    input: Vec<i64>,
    output: Option<i64>,
    instruction_pointer: usize,
}

impl IntCodeComputer {
    fn new(mem: Vec<i64>, input: Vec<i64>) -> Self {
        IntCodeComputer {
            memory: mem,
            input: input,
            output: None,
            instruction_pointer: 0,
        }
    }

    fn get_word(&mut self) -> i64 {
        let word = self.memory[self.instruction_pointer];
        self.instruction_pointer += 1;
        word
    }

    fn _get_parameter(&mut self, mode: i64) -> Parameter {
        let word = self.get_word();
        match mode {
            0 => Parameter::Position(word),
            1 => Parameter::Immediate(word),
            _ => panic!("unknown parameter mode"),
        }
    }

    fn get_param_1(&mut self, op_code: i64) -> Parameter {
        self._get_parameter(op_code % 10)
    }

    fn get_param_2(&mut self, op_code: i64) -> (Parameter, Parameter) {
        let p1 = self.get_param_1(op_code);
        let p2 = self._get_parameter((op_code / 10) % 10);
        (p1, p2)
    }

    fn get_param_3(&mut self, op_code: i64) -> (Parameter, Parameter, Parameter) {
        let (p1, p2) = self.get_param_2(op_code);
        let p3 = self._get_parameter((op_code / 100) % 10);
        (p1, p2, p3)
    }

    fn unwrap(&self, param: Parameter) -> i64 {
        match param {
            Parameter::Immediate(val) => val,
            Parameter::Position(pos) => self.memory[pos as usize],
        }
    }

    fn store_val(&mut self, param: Parameter, val: i64) {
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

    fn feed_input(&mut self, inp: i64) {
        self.input.push(inp);
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

    fn get_output(&mut self) -> i64 {
        let ret = self.output.unwrap();
        self.output = None;
        ret
    }

    fn tick(&mut self) -> Signal {
        let inst = self.get_instruction();
        match inst {
            Instruction::Add((p_1, p_2, p_3)) => {
                let op1 = self.unwrap(p_1);
                let op2 = self.unwrap(p_2);
                self.store_val(p_3, op1 + op2);
                return Signal::None;
            }
            Instruction::Mul((p_1, p_2, p_3)) => {
                let op1 = self.unwrap(p_1);
                let op2 = self.unwrap(p_2);
                self.store_val(p_3, op1 * op2);
                return Signal::None;
            }
            Instruction::Input(p_) => {
                if self.input.is_empty() {
                    return Signal::NeedsInput;
                }
                let inp = self.input.remove(0);
                self.store_val(p_, inp);
                return Signal::None;
            }
            Instruction::Output(p_) => {
                self.emit_output(p_);
                return Signal::ProducedOutput;
            }
            Instruction::JumpIfTrue((p_1, p_2)) => {
                if self.unwrap(p_1) != 0 {
                    self.jump(p_2);
                }
                return Signal::None;
            }
            Instruction::JumpIfFalse((p_1, p_2)) => {
                if self.unwrap(p_1) == 0 {
                    self.jump(p_2);
                }
                return Signal::None;
            }
            Instruction::LessThan((p_1, p_2, p_3)) => {
                let op1 = self.unwrap(p_1);
                let op2 = self.unwrap(p_2);
                self.store_val(p_3, if op1 < op2 { 1 } else { 0 });
                return Signal::None;
            }
            Instruction::Equals((p_1, p_2, p_3)) => {
                let op1 = self.unwrap(p_1);
                let op2 = self.unwrap(p_2);
                self.store_val(p_3, if op1 == op2 { 1 } else { 0 });
                return Signal::None;
            }
            Instruction::Halt => return Signal::Halt,
        }
    }

    fn run_till_signal(&mut self, signal: Signal) {
        while self.tick() != signal {}
    }

    fn run(&mut self) -> Signal {
        let mut signal = self.tick();
        while signal == Signal::None {
            signal = self.tick();
        }
        signal
    }
}

fn get_signal(instructions: &Vec<i64>, phase: Vec<usize>) -> i64 {
    let mut amplifiers = vec![
        IntCodeComputer::new(instructions.clone(), vec![phase[0] as i64]),
        IntCodeComputer::new(instructions.clone(), vec![phase[1] as i64]),
        IntCodeComputer::new(instructions.clone(), vec![phase[2] as i64]),
        IntCodeComputer::new(instructions.clone(), vec![phase[3] as i64]),
        IntCodeComputer::new(instructions.clone(), vec![phase[4] as i64]),
    ];

    let mut out = 0;
    for i in 0..5 {
        amplifiers[i].feed_input(out);
        amplifiers[i].run_till_signal(Signal::ProducedOutput);
        out = amplifiers[i].get_output();
    }

    out
}

fn get_signal_with_feedback(instructions: &Vec<i64>, phase: Vec<usize>) -> i64 {
    let mut amplifiers = vec![
        IntCodeComputer::new(instructions.clone(), vec![phase[0] as i64, 0]),
        IntCodeComputer::new(instructions.clone(), vec![phase[1] as i64]),
        IntCodeComputer::new(instructions.clone(), vec![phase[2] as i64]),
        IntCodeComputer::new(instructions.clone(), vec![phase[3] as i64]),
        IntCodeComputer::new(instructions.clone(), vec![phase[4] as i64]),
    ];

    let mut sig: Option<i64> = None;
    for i in (0..5).cycle() {
        if let Some(s) = sig {
            amplifiers[i].feed_input(s);
        }

        match amplifiers[i].run() {
            Signal::ProducedOutput => {
                sig = Some(amplifiers[i].get_output());
            }
            Signal::Halt => {
                if i == 4 {
                    break;
                }
            }
            _ => {}
        }
    }

    sig.unwrap()
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(",")
        .filter_map(|x| x.parse::<i64>().ok())
        .collect()
}

#[aoc(day7, part1)]
fn solve_p1(instructions: &Vec<i64>) -> Option<i64> {
    let mut mx = std::i64::MIN;
    for perm in permutations(0, 4) {
        mx = ::std::cmp::max(mx, get_signal(&instructions, perm));
    }
    Some(mx)
}

#[aoc(day7, part2)]
fn solve_p2(instructions: &Vec<i64>) -> Option<i64> {
    let mut mx = std::i64::MIN;
    for perm in permutations(5, 9) {
        mx = ::std::cmp::max(mx, get_signal_with_feedback(&instructions, perm));
    }
    Some(mx)
}
