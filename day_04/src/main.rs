use std::str::FromStr;

#[derive(Debug)]
struct Assignment {
    min: u32,
    max: u32,
}

#[derive(Debug)]
struct Pair {
    a: Assignment,
    b: Assignment,
}

impl Pair {
    pub const fn is_overlapping(&self) -> bool {
        self.a.max >= self.b.min && self.a.min <= self.b.max
    }

    pub const fn is_fully_overlapping(&self) -> bool {
        (self.a.min <= self.b.min && self.a.max >= self.b.max)
            || (self.a.min >= self.b.min && self.a.max <= self.b.max)
    }
}

impl FromStr for Assignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once('-')
            .ok_or_else(|| format!("{s} is not a valid assignment"))?;
        let min = a.parse::<u32>().map_err(|e| e.to_string())?;
        let max = b.parse::<u32>().map_err(|e| e.to_string())?;
        Ok(Self { min, max })
    }
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once(',')
            .ok_or_else(|| format!("{s} is not a valid pair"))?;
        let a = Assignment::from_str(a)?;
        let b = Assignment::from_str(b)?;
        Ok(Self { a, b })
    }
}

fn main() {
    let file = include_str!("../input.txt");
    let pairs: Vec<_> = file
        .lines()
        .map(|line| Pair::from_str(line).unwrap())
        .collect();
    let overlapped_pairs = pairs.iter().filter(|p| p.is_fully_overlapping()).count();
    println!("Part 1: {overlapped_pairs}");
    let overlapped_pairs = pairs.iter().filter(|p| p.is_overlapping()).count();
    println!("Part 2: {overlapped_pairs}");
}
