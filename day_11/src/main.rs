use std::{collections::HashMap, str::FromStr};

type Item = u64;
type ThrownItems = HashMap<usize, Vec<Item>>;

#[derive(Debug, Copy, Clone)]
enum OpSign {
    Add,
    Mul,
}

#[derive(Debug, Copy, Clone)]
enum OpVal {
    Old,
    Val(u64),
}

#[derive(Debug, Copy, Clone)]
struct Operation {
    sign: OpSign,
    val: OpVal,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    divisible_by: u64,
    to_monkey_true: usize,
    to_monkey_false: usize,
    inspections: usize,
}

struct Monkeys(Vec<Monkey>);

impl Monkeys {
    fn run(&mut self, amount: usize, divide: bool) {
        for _ in 0..amount {
            for i in 0..self.0.len() {
                let thrown_items = self.0[i].run_turn(divide);
                for (id, items) in thrown_items {
                    self.0[id].items.extend(items);
                }
            }
        }
    }
}

impl Operation {
    pub fn resolve(&self, old: Item) -> Item {
        let val = match self.val {
            OpVal::Old => old,
            OpVal::Val(v) => v,
        };
        match self.sign {
            OpSign::Add => old + val,
            OpSign::Mul => old * val,
        }
    }
}

impl Monkey {
    pub fn run_turn(&mut self, divide: bool) -> ThrownItems {
        let mut items = ThrownItems::new();
        self.inspections += self.items.len();
        for item in self.items.drain(..) {
            let mut item = self.operation.resolve(item);
            if divide {
                item = item / 3;
            }
            let target = if item % self.divisible_by == 0 {
                self.to_monkey_true
            } else {
                self.to_monkey_false
            };
            items.entry(target).or_default().push(item);
        }
        items
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, op) = s
            .split_once('=')
            .ok_or_else(|| format!("{s} is not a valid operation"))?;
        let elems: Vec<_> = op.split_whitespace().collect();
        if elems.len() != 3 {
            return Err(format!("{op} should have two elements and an operator"))?;
        }
        let sign = match elems[1] {
            "+" => OpSign::Add,
            "*" => OpSign::Mul,
            v => return Err(format!("{v} is not a valid operator")),
        };
        let val = match elems[2] {
            "old" => OpVal::Old,
            v => OpVal::Val(
                v.parse()
                    .map_err(|_| format!("{v} is not a valid number"))?,
            ),
        };
        Ok(Self { sign, val })
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().skip(1);

        let get_right = |s: Option<&str>| -> Result<String, Self::Err> {
            let s = s.ok_or_else(|| String::from("Missing line"))?;
            let (_, right) = s.split_once(':').ok_or_else(|| format!("{s} is invalid"))?;
            Ok(right.trim().to_owned())
        };
        let get_las_num =
            |s: &str| -> Option<usize> { s.split_whitespace().last().and_then(|v| v.parse().ok()) };

        let items_str = get_right(lines.next())?;
        let items = items_str
            .split(',')
            .map(str::trim)
            .map(Item::from_str)
            .collect::<Result<_, _>>()
            .map_err(|_| format!("{items_str} is invalid"))?;

        let operation = get_right(lines.next()).map(|s| Operation::from_str(&s))??;

        let divisible_by = lines
            .next()
            .and_then(get_las_num)
            .ok_or_else(|| String::from("Invalid divisible by"))?
            as Item;

        let to_monkey_true = lines
            .next()
            .and_then(get_las_num)
            .ok_or_else(|| String::from("Invalid TRUE target monkey"))?;
        let to_monkey_false = lines
            .next()
            .and_then(get_las_num)
            .ok_or_else(|| String::from("Invalid FALSE target monkey"))?;
        Ok(Self {
            items,
            operation,
            divisible_by,
            to_monkey_false,
            to_monkey_true,
            inspections: 0,
        })
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(Monkey::from_str)
        .collect::<Result<_, _>>()
        .unwrap();
    let mut monkeys_1 = Monkeys(monkeys.clone());
    let mut monkeys_2 = Monkeys(monkeys);

    // Part 1
    monkeys_1.run(20, true);
    let mut inspections: Vec<_> = monkeys_1.0.iter().map(|m| m.inspections).collect();
    inspections.sort_unstable();
    inspections.reverse();
    println!(
        "Part 1: {} * {} = {}",
        inspections[0],
        inspections[1],
        inspections[0] * inspections[1]
    );
    // part 2
    monkeys_2.run(1000, false);
    let mut inspections: Vec<_> = monkeys_2.0.iter().map(|m| m.inspections).collect();
    inspections.sort_unstable();
    inspections.reverse();
    println!(
        "Part 2: {} * {} = {}",
        inspections[0],
        inspections[1],
        inspections[0] * inspections[1]
    );
}
