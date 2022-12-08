use std::str::FromStr;

struct Map {
    trees: Vec<Vec<u8>>,
}

impl Map {
    fn visible_trees(&self) -> usize {
        let max_y = self.trees.len();
        self.trees
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(x, v)| {
                        let left = &line[..*x];
                        let right = &line[(*x + 1)..];
                        let mut up = (0..y).map(|i| self.trees[i][*x]);
                        let mut down = ((y + 1)..max_y).map(|i| self.trees[i][*x]);

                        up.all(|n| n < **v)
                            || down.all(|n| n < **v)
                            || left.iter().all(|n| n < *v)
                            || right.iter().all(|n| n < *v)
                    })
                    .count()
            })
            .sum()
    }

    fn score(v: u8, mut iter: impl Iterator<Item = u8>, default: usize) -> usize {
        iter.position(|n| n >= v).unwrap_or(default) + 1
    }

    fn max_scenic_score(&self) -> Option<usize> {
        let max_y = self.trees.len();
        self.trees
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().copied().enumerate().map(move |(x, v)| {
                    if y == 0 || x == 0 || x >= line.len() - 1 || y >= max_y - 1 {
                        return 0;
                    }

                    let left = &line[..x];
                    let right = &line[(x + 1)..];
                    let up = (0..y).map(|i| self.trees[i][x]);
                    let down = ((y + 1)..max_y).map(|i| self.trees[i][x]);

                    Self::score(v, up.rev(), y.saturating_sub(1))
                        * Self::score(v, down, max_y.saturating_sub(y + 2))
                        * Self::score(v, left.iter().rev().copied(), x.saturating_sub(1))
                        * Self::score(v, right.iter().copied(), line.len().saturating_sub(x + 2))
                })
            })
            .max()
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees = s
            .lines()
            .map(|line| line.as_bytes().iter().map(|v| *v - b'0').collect())
            .collect();
        Ok(Self { trees })
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let map = Map::from_str(input).unwrap();
    println!("Part 1: {}", map.visible_trees());
    println!("Part 2: {}", map.max_scenic_score().unwrap());
}
