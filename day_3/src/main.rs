use std::{collections::HashSet, str::FromStr};

struct RuckSack {
    items_left: HashSet<u8>,
    items_right: HashSet<u8>,
}

impl RuckSack {
    pub fn common_items(&self) -> Vec<u8> {
        let mut res = Vec::new();
        for item in &self.items_left {
            if self.items_right.contains(item) {
                res.push(*item);
            }
        }
        res
    }

    pub fn all_items(&self) -> HashSet<u8> {
        let mut res = self.items_right.clone();
        res.extend(self.items_left.iter().copied());
        res
    }
}

impl FromStr for RuckSack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<_> = s.chars().collect();
        let (left, right) = chars.split_at(chars.len() / 2);

        let value = |item: &char| {
            if ('a'..='z').contains(item) {
                Ok((*item as u8).saturating_sub(b'a') + 1)
            } else if ('A'..='Z').contains(item) {
                Ok((*item as u8).saturating_sub(b'A') + 27)
            } else {
                Err(format!("{item} is not a valid item"))
            }
        };

        let items_left = left.iter().map(value).collect::<Result<_, _>>()?;
        let items_right = right.iter().map(value).collect::<Result<_, _>>()?;

        Ok(Self {
            items_left,
            items_right,
        })
    }
}

fn main() {
    let file = include_str!("../input.txt");
    let rucksacks: Vec<_> = file
        .lines()
        .map(|line| RuckSack::from_str(line).unwrap())
        .collect();
    // Part 1
    let common_items: Vec<_> = rucksacks.iter().flat_map(RuckSack::common_items).collect();
    let sum: u32 = common_items.iter().copied().map(u32::from).sum();
    println!("Part 1: Common items sum: {sum}");
    // Part 2
    let sum: u32 = rucksacks
        .chunks(3)
        .map(|chunk| {
            let [b_items, c_items] = [chunk[1].all_items(), chunk[2].all_items()];
            u32::from(
                chunk[0]
                    .all_items()
                    .iter()
                    .copied()
                    .find(|item| b_items.contains(item) && c_items.contains(item))
                    .unwrap(),
            )
        })
        .sum();
    println!("Part 2: Badges sum: {sum}");
}
