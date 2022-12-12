use std::collections::{HashSet, VecDeque};

type Coord = [usize; 2];

#[derive(Debug)]
struct HeightMap {
    map: Vec<Vec<u8>>,
    starts: Vec<Coord>,
    end: Coord,
}

impl HeightMap {
    fn get(&self, [x, y]: Coord) -> Option<&u8> {
        self.map.get(y).and_then(|row| row.get(x))
    }

    fn valid_neighbors(&self, [x, y]: Coord) -> Vec<Coord> {
        let value = match self.get([x, y]) {
            None => return vec![],
            Some(v) => v,
        };
        let mut neighbors = vec![
            [x + 1, y],
            [x.saturating_sub(1), y],
            [x, y + 1],
            [x, y.saturating_sub(1)],
        ];
        neighbors.dedup();
        neighbors
            .into_iter()
            .filter_map(|c| self.get(c).and_then(|v| (*v <= value + 1).then_some(c)))
            .collect()
    }

    fn steps(&self) -> Option<usize> {
        let mut paths = VecDeque::new();
        let mut visited = HashSet::new();
        for start in &self.starts {
            paths.push_back((*start, 0));
            visited.insert(*start);
        }
        while let Some((coord, cost)) = paths.pop_front() {
            let neighbors = self.valid_neighbors(coord);
            for c in neighbors {
                if c == self.end {
                    return Some(cost + 1);
                }
                if !visited.contains(&c) {
                    visited.insert(c);
                    paths.push_back((c, cost + 1));
                }
            }
        }
        None
    }

    fn parse(s: &str, multistart: bool) -> Self {
        let mut start = Vec::new();
        let mut end = [0; 2];
        let map = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(x, byte)| {
                        let b = match byte {
                            b'S' => {
                                start.push([x, y]);
                                b'a'
                            }
                            b'E' => {
                                end = [x, y];
                                b'z'
                            }
                            b'a' if multistart => {
                                start.push([x, y]);
                                b'a'
                            }
                            b => b,
                        };
                        b - b'a'
                    })
                    .collect()
            })
            .collect();
        Self {
            map,
            starts: start,
            end,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let map_1 = HeightMap::parse(input, false);
    let map_2 = HeightMap::parse(input, true);
    println!("Part 1: steps = {}", map_1.steps().unwrap());
    println!("Part 2: steps = {}", map_2.steps().unwrap());
}
