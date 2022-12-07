use std::{collections::HashMap, str::FromStr};

const DISK_SPACE: usize = 70_000_000;
const REQUIRED_SPACE: usize = 30_000_000;

#[derive(Debug)]
struct Dirs(pub HashMap<String, usize>);

impl Dirs {
    pub fn part1(&self) -> usize {
        self.0
            .values()
            .copied()
            .filter(|size| *size <= 100_000)
            .sum()
    }

    pub fn part2(&self) -> usize {
        let available_space = DISK_SPACE.saturating_sub(self.0["/"]);
        let expected_size = REQUIRED_SPACE.saturating_sub(available_space);
        let mut sizes: Vec<_> = self.0.values().collect();
        sizes.sort_unstable();
        sizes
            .into_iter()
            .find(|size| **size >= expected_size)
            .copied()
            .unwrap()
    }
}

impl FromStr for Dirs {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dirs = HashMap::new();
        let mut path = Vec::new();

        for section in s.lines() {
            match section {
                "$ ls" => {}
                "$ cd .." => {
                    path.pop();
                }
                section if section.starts_with("$ cd ") => {
                    let dir_name = section[5..].to_string();
                    path.push(dir_name);
                }
                _ => {
                    let (prefix, name) = section
                        .split_once(' ')
                        .ok_or_else(|| format!("`{section}` is not a valid entry"))?;
                    if prefix == "dir" {
                        let path_str = format!("{}{name}", path.join(""));
                        dirs.entry(path_str).or_insert(0);
                    } else {
                        let size: usize = prefix
                            .parse()
                            .map_err(|_| format!("{prefix} is not a valid file size"))?;
                        let mut dir_path = String::new();
                        for p in &path {
                            dir_path.push_str(p);
                            *dirs.entry(dir_path.clone()).or_default() += size;
                        }
                    }
                }
            }
        }
        Ok(Self(dirs))
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let hierarchy = Dirs::from_str(input).unwrap();
    // println!("{hierarchy:#?}");
    println!("Part 1: {}", hierarchy.part1());
    println!("Part 2: {}", hierarchy.part2());
}
