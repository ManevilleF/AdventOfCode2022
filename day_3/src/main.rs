use std::{collections::HashSet, str::FromStr};

pub type Item = char;

struct RuckSack {
    items_left: HashSet<Item>,
    items_right: HashSet<Item>,
}

impl RuckSack {
    pub fn common_items(&self) -> Vec<Item> {
        let mut res = Vec::new();
        for item in &self.items_left {
            if self.items_right.contains(item) {
                res.push(*item);
            }
        }
        res
    }

    pub fn all_items(&self) -> HashSet<Item> {
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
        let items_left = left.into_iter().copied().collect();
        let items_right = right.into_iter().copied().collect();

        Ok(Self {
            items_left,
            items_right,
        })
    }
}

fn item_value(item: Item) -> u8 {
    if ('a'..='z').contains(&item) {
        (item as u8).saturating_sub('a' as u8) + 1
    } else if ('A'..='Z').contains(&item) {
        (item as u8).saturating_sub('A' as u8) + 27
    } else {
        0
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
    let sum: u32 = common_items
        .iter()
        .copied()
        .map(|item| item_value(item) as u32)
        .sum();
    println!("Part 1: Common items sum: {sum}");
    // Part 2
    let sum: u32 = rucksacks
        .chunks(3)
        .map(|chunk| {
            let [b_items, c_items] = [chunk[1].all_items(), chunk[2].all_items()];
            let item = chunk[0]
                .all_items()
                .iter()
                .copied()
                .find(|item| b_items.contains(&item) && c_items.contains(&item))
                .unwrap();
            item_value(item) as u32
        })
        .sum();
    println!("Part 2: Badges sum: {sum}");
}
