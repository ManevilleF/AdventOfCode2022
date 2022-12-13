use std::{fmt::Display, str::FromStr};

type Items = Vec<Item>;

#[derive(Debug, Clone)]
enum Item {
    Char(u8),
    List(Items),
}

#[derive(Debug)]
struct Pair {
    left: Item,
    right: Item,
}

impl Item {
    fn is_ordered(self, other: Self) -> Option<bool> {
        println!("Comparing:\n{self}\n{other}");
        match [self, other] {
            [Self::Char(a), Self::Char(b)] => {
                if a == b {
                    None
                } else {
                    Some(a < b)
                }
            }
            [Self::Char(a), Self::List(b)] => {
                Self::List(vec![Self::Char(a)]).is_ordered(Self::List(b))
            }
            [Self::List(a), Self::Char(b)] => {
                Self::List(a).is_ordered(Self::List(vec![Self::Char(b)]))
            }
            [Self::List(a), Self::List(b)] => {
                let mut iter_a = a.into_iter();
                let mut iter_b = b.into_iter();
                loop {
                    match [iter_a.next(), iter_b.next()] {
                        [None, None] => return None,
                        [None, Some(_)] => return Some(true),
                        [Some(_), None] => return Some(false),
                        [Some(item), Some(other)] => {
                            if let Some(v) = item.is_ordered(other) {
                                return Some(v);
                            }
                        }
                    }
                }
            }
        }
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
                last.push(Self::Char(num));
            }
            if c == ']' {
                if queues.len() > 1 {
                    let items = queues
                        .pop()
                        .ok_or_else(|| format!("`{s}` has an invalid `{c}` char"))?;
                    let last = queues.last_mut().unwrap();
                    last.push(Item::List(items));
                }
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
            Self::Char(c) => c.to_string(),
            Self::List(l) => {
                let chars: Vec<_> = l.iter().map(ToString::to_string).collect();
                format!("[{}]", chars.join(","))
            }
        };
        write!(f, "{}", str)
    }
}

fn main() {
    let input = include_str!("../test_input.txt");
    let pairs: Vec<Pair> = input
        .split("\n\n")
        .map(Pair::from_str)
        .collect::<Result<_, _>>()
        .unwrap();
    // println!("{pairs:#?}");
    let ordered_pairs: usize = pairs
        .into_iter()
        .enumerate()
        .filter_map(|(i, pair)| {
            let is_ordered = pair.left.is_ordered(pair.right);
            println!("{is_ordered:?}\n");
            is_ordered.unwrap_or(false).then_some(i + 1)
        })
        .sum();
    println!("Part 1: {}", ordered_pairs);
}
