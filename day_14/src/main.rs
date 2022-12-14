use std::{collections::HashSet, str::FromStr};

type Coord = [i16; 2];

const SAND_SOURCE: Coord = [500, 0];

struct Cave {
    rocks: HashSet<Coord>,
    resting_sand: HashSet<Coord>,
    current_sand_coord: Coord,
    max_y: i16,
}

impl Cave {
    const SAND_DIRS: [Coord; 3] = [[0, 1], [-1, 1], [1, 1]];

    fn tick(&mut self, floor: Option<i16>) -> bool {
        let [x, y] = self.current_sand_coord;
        if floor.is_none() && y > self.max_y {
            return false;
        }

        let mut resting = true;
        for [dx, dy] in Self::SAND_DIRS {
            let new_coord = [x + dx, y + dy];
            let floored = floor.map_or(false, |max| new_coord[1] >= max);
            if !floored
                && !self.resting_sand.contains(&new_coord)
                && !self.rocks.contains(&new_coord)
            {
                self.current_sand_coord = new_coord;
                resting = false;
                break;
            }
        }
        if resting {
            self.resting_sand.insert(self.current_sand_coord);
            if self.current_sand_coord == SAND_SOURCE {
                return false;
            }
            self.current_sand_coord = SAND_SOURCE;
        }
        true
    }

    fn print(&self, min_x: i16, max_x: i16) {
        for y in 0..=self.max_y {
            let mut line = String::new();
            for x in min_x..=max_x {
                let coord = [x, y];
                let char = match coord {
                    SAND_SOURCE => '+',
                    _ if coord == self.current_sand_coord => 'x',
                    _ if self.rocks.contains(&coord) => '█',
                    _ if self.resting_sand.contains(&coord) => '░',
                    _ => ' ',
                };
                line.push(char);
            }
            println!("{line}");
        }
    }
}

impl FromStr for Cave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rocks = HashSet::new();
        let mut max_y = 0;

        for line in s.lines() {
            let coords: Vec<Coord> = line
                .split(" -> ")
                .map(|str| {
                    let (x, y) = str
                        .split_once(',')
                        .ok_or_else(|| format!("`{str}` is not a valid coordinate"))?;
                    let x = x.parse().map_err(|_| format!("`{x}` is not a number"))?;
                    let y = y.parse().map_err(|_| format!("`{y}` is not a number"))?;
                    Ok([x, y])
                })
                .collect::<Result<_, Self::Err>>()?;
            for window in coords.windows(2) {
                let [mut start, end] = [window[0], window[1]];
                let dir = [(end[0] - start[0]).signum(), (end[1] - start[1]).signum()];
                while start != end {
                    rocks.insert(start);
                    start[0] += dir[0];
                    start[1] += dir[1];
                }
                rocks.insert(end);
                max_y = max_y.max(start[1]).max(end[1]);
            }
        }
        Ok(Self {
            rocks,
            resting_sand: HashSet::default(),
            current_sand_coord: SAND_SOURCE,
            max_y,
        })
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut cave = Cave::from_str(input).unwrap();

    // Part 1
    while cave.tick(None) {}
    cave.print(400, 600);
    println!("Part 1: {}", cave.resting_sand.len());

    // Part 2
    let mut cave = Cave::from_str(input).unwrap();
    while cave.tick(Some(cave.max_y + 2)) {}
    cave.print(400, 600);
    println!("Part 2: {}", cave.resting_sand.len());
}
