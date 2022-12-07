use std::{collections::HashMap, str::FromStr};

const DISK_SPACE: usize = 70_000_000;
const REQUIRED_SPACE: usize = 30_000_000;

#[derive(Debug, Default)]
struct Dir {
    file_size: usize,
    dirs: Dirs,
}

#[derive(Debug, Default)]
struct Dirs(pub HashMap<String, Dir>);

impl Dir {
    pub fn total_size(&self) -> usize {
        self.file_size + self.dirs.total_size()
    }

    pub fn push_entry(&mut self, entry: &str) -> Result<(), String> {
        let (prefix, name) = entry
            .split_once(' ')
            .ok_or_else(|| format!("`{entry}` is not a valid entry"))?;
        match prefix {
            "dir" => {
                self.dirs.0.insert(name.to_string(), Dir::default());
            }
            _ => {
                let size: usize = prefix
                    .parse()
                    .map_err(|_| format!("{prefix} is not a valid file size"))?;
                self.file_size += size;
            }
        }
        Ok(())
    }
}

impl Dirs {
    pub fn get_dir_mut(&mut self, depth: &[String]) -> Option<&mut Dir> {
        let dir = self.0.get_mut(&depth[0])?;
        if depth.len() > 1 {
            dir.dirs.get_dir_mut(&depth[1..])
        } else {
            Some(dir)
        }
    }

    pub fn total_size(&self) -> usize {
        self.0.values().map(Dir::total_size).sum()
    }

    pub fn part1(&self) -> usize {
        self.0
            .values()
            .map(|dir| {
                let total_size = dir.total_size();
                let size = if total_size <= 100_000 { total_size } else { 0 };
                size + dir.dirs.part1()
            })
            .sum()
    }

    pub fn all_sizes(&self) -> Vec<usize> {
        self.0
            .values()
            .flat_map(|dir| {
                let mut sizes = vec![dir.total_size()];
                sizes.extend(dir.dirs.all_sizes());
                sizes
            })
            .collect()
    }

    pub fn part2(&self) -> usize {
        let available_space = DISK_SPACE.saturating_sub(self.total_size());
        let expected_size = REQUIRED_SPACE.saturating_sub(available_space);
        let mut sizes = self.all_sizes();
        sizes.sort_unstable();
        sizes
            .into_iter()
            .find(|size| *size >= expected_size)
            .unwrap()
    }
}

impl FromStr for Dirs {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dirs = Self::default();
        dirs.0.insert("/".to_string(), Dir::default());

        let mut current_depth = vec![];

        for section in s.lines() {
            match section {
                "$ ls" => {}
                "$ cd .." => {
                    current_depth.pop();
                }
                section if section.starts_with("$ cd ") => {
                    current_depth.push(section[5..].to_string());
                }
                _ => {
                    let dir = dirs.get_dir_mut(&current_depth).ok_or_else(|| {
                        format!("There is no valid directory at {}", current_depth.join("/"))
                    })?;
                    dir.push_entry(section)?;
                }
            }
        }
        Ok(dirs)
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let hierarchy = Dirs::from_str(input).unwrap();
    // println!("{hierarchy:#?}");
    println!("total size {}", hierarchy.total_size());
    println!("Part 1: {}", hierarchy.part1());
    println!("Part 2: {}", hierarchy.part2());
}
