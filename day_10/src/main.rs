#[derive(Debug)]
struct Register {
    value: i32,
    add_queue: Vec<i32>,
}

impl Register {
    pub const fn new() -> Self {
        Self {
            value: 1,
            add_queue: Vec::new(),
        }
    }

    pub fn is_done(&self) -> bool {
        self.add_queue.is_empty()
    }

    pub fn push(&mut self, add_value: i32) {
        if add_value != 0 {
            self.add_queue.push(0);
        }
        self.add_queue.push(add_value);
    }

    pub fn cycle(&mut self) {
        if self.is_done() {
            return;
        }
        self.value += self.add_queue.remove(0);
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let checks = [20, 60, 100, 140, 180, 220];
    let mut register = Register::new();

    for line in input.lines() {
        let add_value = if let Some(("addx", v)) = line.split_once(' ') {
            v.parse().unwrap()
        } else {
            0
        };
        register.push(add_value);
    }

    let mut cycle = 1;
    let mut strength_sum = 0;
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
            strength_sum += register.value * cycle;
        }
        register.cycle();
        cycle += 1;
    }
    println!("Part 1: {strength_sum}");
    println!("Part 2: \n{crt}");
}
