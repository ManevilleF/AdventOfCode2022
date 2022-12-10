#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
struct Register {
    value: i32,
    instructions: Vec<Instruction>,
}

impl Register {
    pub const fn new() -> Self {
        Self {
            value: 1,
            instructions: Vec::new(),
        }
    }

    pub fn is_done(&self) -> bool {
        self.instructions.is_empty()
    }

    pub fn push(&mut self, instruction: Instruction) {
        if matches!(instruction, Instruction::Addx(_)) {
            self.instructions.push(Instruction::Noop);
        }
        self.instructions.push(instruction);
    }

    pub fn cycle(&mut self) {
        if self.is_done() {
            return;
        }
        match self.instructions.remove(0) {
            Instruction::Noop => (),
            Instruction::Addx(v) => self.value += v,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let checks = [20, 60, 100, 140, 180, 220];
    let mut register = Register::new();

    for line in input.lines() {
        let instruction = if line == "noop" {
            Instruction::Noop
        } else {
            match line.split_once(' ') {
                Some(("addx", v)) => Instruction::Addx(v.parse().unwrap()),
                _ => panic!("{line} is not a valid instruction"),
            }
        };
        register.push(instruction);
    }

    let mut cycle = 1;
    let mut sum = 0;
    let mut crt = String::new();
    while !register.is_done() {
        let reg_value = register.value;
        let crt_pos = cycle % 40 - 1;
        if reg_value == crt_pos || reg_value + 1 == crt_pos || reg_value - 1 == crt_pos {
            crt.push('#');
        } else {
            crt.push(' ');
        }
        if cycle % 40 == 0 {
            crt.push('\n');
        }
        if checks.contains(&cycle) {
            sum += register.value * cycle;
        }
        register.cycle();
        cycle += 1;
    }
    println!("Part 1: {sum}");
    println!("Part 2: \n{crt}");
}
