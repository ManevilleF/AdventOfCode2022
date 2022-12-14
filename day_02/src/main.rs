use std::str::FromStr;

#[derive(Clone, Copy)]
#[repr(u32)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(Clone, Copy)]
#[repr(u32)]
enum Outcome {
    Loose = 0,
    Draw = 3,
    Win = 6,
}

impl Shape {
    pub const fn shape_to_outcome(self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => match self {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissor,
                Self::Scissor => Self::Rock,
            },
            Outcome::Loose => match self {
                Self::Rock => Self::Scissor,
                Self::Paper => Self::Rock,
                Self::Scissor => Self::Paper,
            },
            Outcome::Draw => self,
        }
    }

    pub const fn outcome_against(self, other: Self) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Scissor)
            | (Self::Scissor, Self::Paper)
            | (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Rock, Self::Paper)
            | (Self::Scissor, Self::Rock)
            | (Self::Paper, Self::Scissor) => Outcome::Loose,
            _ => Outcome::Draw,
        }
    }

    pub const fn score_against(self, other: Self) -> u32 {
        let outcome = self.outcome_against(other);
        outcome as u32 + self as u32
    }
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rcs = match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => return Err(format!("{s} is not a valis RCS action")),
        };
        Ok(rcs)
    }
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outcome = match s {
            "X" => Self::Loose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => return Err(format!("{s} is not a valis RCS outcome")),
        };
        Ok(outcome)
    }
}

fn main() {
    let file: Vec<(&str, &str)> = include_str!("../input.txt")
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .collect();
    let part1_score: u32 = file
        .iter()
        .map(|(a, b)| {
            let enemy_shape = Shape::from_str(a).unwrap();
            let my_shape = Shape::from_str(b).unwrap();
            my_shape.score_against(enemy_shape)
        })
        .sum();
    println!("Part 1 score = {part1_score}");
    let part2_score: u32 = file
        .iter()
        .map(|(a, b)| {
            let enemy_shape = Shape::from_str(a).unwrap();
            let outcome = Outcome::from_str(b).unwrap();
            let my_shape = enemy_shape.shape_to_outcome(outcome);
            my_shape.score_against(enemy_shape)
        })
        .sum();
    println!("Part 2 score = {part2_score}");
}
