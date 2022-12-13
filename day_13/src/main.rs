use std::{cmp::Ordering, fmt::Display, fs::read_to_string, str::FromStr};

type Items = Vec<Item>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Num(u8),
    List(Items),
}

#[derive(Debug)]
struct Pair {
    left: Item,
    right: Item,
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match [self, other] {
            [Self::Num(a), Self::Num(b)] => a.partial_cmp(b),
            [Self::Num(a), Self::List(b)] => [Self::Num(*a)].as_slice().partial_cmp(b.as_slice()),
            [Self::List(a), Self::Num(b)] => a.as_slice().partial_cmp([Self::Num(*b)].as_slice()),
            [Self::List(a), Self::List(b)] => a.partial_cmp(b),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut queues: Vec<Items> = Vec::new();
        let mut current_num_str = String::new();

        for c in s.chars() {
            let push_num = match c {
                ',' | ']' => true,
                '[' => {
                    queues.push(Items::new());
                    false
                }
                c => {
                    current_num_str.push(c);
                    false
                }
            };
            if push_num && !current_num_str.is_empty() {
                let last = queues
                    .last_mut()
                    .ok_or_else(|| format!("`{s}` is invalid"))?;
                let num = u8::from_str(&current_num_str)
                    .map_err(|_| format!("`{current_num_str}` is not a valid number"))?;
                current_num_str.clear();
                last.push(Self::Num(num));
            }
            if c == ']' && queues.len() > 1 {
                let items = queues
                    .pop()
                    .ok_or_else(|| format!("`{s}` has an invalid `{c}` char"))?;
                let last = queues.last_mut().unwrap();
                last.push(Self::List(items));
            }
        }
        if queues.len() != 1 {
            return Err(format!("`{s}` has invalid bracket hierarchy: {queues:#?}"));
        }
        Ok(Self::List(queues.remove(0)))
    }
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let (left_str, right_str) = lines
            .next()
            .zip(lines.next())
            .ok_or_else(|| format!("{s} is not a valid Pair"))?;

        let left = Item::from_str(left_str)?;
        let right = Item::from_str(right_str)?;

        // debug assert
        assert_eq!(left.to_string(), left_str.to_string());
        assert_eq!(right.to_string(), right_str.to_string());

        Ok(Self { left, right })
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Num(n) => n.to_string(),
            Self::List(l) => {
                let chars: Vec<_> = l.iter().map(ToString::to_string).collect();
                format!("[{}]", chars.join(","))
            }
        };
        write!(f, "{}", str)
    }
}

fn main() {
    let mut input = read_to_string("input.txt").unwrap();

    // Part 1
    let pairs: Vec<Pair> = input
        .split("\n\n")
        .map(Pair::from_str)
        .collect::<Result<_, _>>()
        .unwrap();
    let ordered_pairs: usize = pairs
        .into_iter()
        .enumerate()
        .filter_map(|(i, pair)| (pair.left < pair.right).then_some(i + 1))
        .sum();
    println!("Part 1: {}", ordered_pairs);
    // Part 2
    input.push_str("[[2]]");
    input.push('\n');
    input.push_str("[[6]]");
    let mut items: Items = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Item::from_str)
        .collect::<Result<_, _>>()
        .unwrap();
    items.sort_unstable();
    let items: Vec<_> = items.into_iter().map(|i| i.to_string()).collect();
    let a = items.iter().position(|s| s == "[[2]]").unwrap() + 1;
    let b = items.iter().position(|s| s == "[[6]]").unwrap() + 1;
    println!("Part 2: {a} * {b} = {}", a * b);
}
