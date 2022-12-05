use std::{collections::VecDeque, str::FromStr};

type Crate = char;

#[derive(Debug, Default, Clone)]
struct Stack {
    crates: VecDeque<Crate>,
}

#[derive(Debug, Clone)]
struct Stacks {
    stacks: Vec<Stack>,
}

#[derive(Debug)]
struct MoveInstruction {
    amount: usize,
    from_index: usize,
    to_index: usize,
}

impl Stacks {
    pub fn apply_9000(&mut self, instruction: &MoveInstruction) {
        for _ in 0..instruction.amount {
            let item = self.stacks[instruction.from_index - 1]
                .crates
                .pop_front()
                .unwrap();
            self.stacks[instruction.to_index - 1]
                .crates
                .push_front(item);
        }
    }

    pub fn apply_9001(&mut self, instruction: &MoveInstruction) {
        let items: Vec<_> = self.stacks[instruction.from_index - 1]
            .crates
            .drain(..instruction.amount)
            .rev()
            .collect();
        for item in items {
            self.stacks[instruction.to_index - 1]
                .crates
                .push_front(item);
        }
    }

    pub fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.crates.iter().next())
            .collect()
    }
}

impl FromStr for MoveInstruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str = s.replace("move", "").replace("from", "").replace("to", "");
        let parts: Vec<&str> = str.split_whitespace().collect();
        let parts: [&str; 3] = parts
            .try_into()
            .map_err(|_| format!("Expected 3 numeric values in `{str}`"))?;
        let [amount, from_index, to_index] = parts.map(|part| part.parse().unwrap());
        Ok(Self {
            amount,
            from_index,
            to_index,
        })
    }
}

impl FromStr for Stacks {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines: Vec<_> = s.lines().collect();
        let last_line = lines.pop().ok_or_else(|| "Invalid stacks".to_string())?;
        let amount = last_line.split_whitespace().count();
        let mut stacks = vec![Stack::default(); amount];
        for line in lines {
            for (index, c) in line.chars().skip(1).step_by(4).enumerate() {
                if c.is_ascii_uppercase() {
                    stacks[index].crates.push_back(c);
                }
            }
        }
        Ok(Self { stacks })
    }
}

fn main() {
    let file = include_str!("../input.txt");
    let (stacks, instructions) = file.split_once("\n\n").unwrap();
    let instructions: Vec<_> = instructions
        .lines()
        .map(MoveInstruction::from_str)
        .collect::<Result<_, _>>()
        .unwrap();
    let base_stacks = Stacks::from_str(stacks).unwrap();

    // Part 1
    let mut stacks = base_stacks.clone();
    for instruction in &instructions {
        stacks.apply_9000(instruction);
    }
    println!("Part 1: Top crates {}", stacks.top_crates());
    // Part 2
    let mut stacks = base_stacks;
    for instruction in &instructions {
        stacks.apply_9001(instruction);
    }
    println!("Part 2: Top crates {}", stacks.top_crates());
}
