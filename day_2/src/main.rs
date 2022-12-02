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
                Shape::Rock => Self::Paper,
                Shape::Paper => Self::Scissor,
                Shape::Scissor => Self::Rock,
            },
            Outcome::Loose => match self {
                Shape::Rock => Self::Scissor,
                Shape::Paper => Self::Rock,
                Shape::Scissor => Self::Paper,
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
    let file = include_str!("../input.txt");
    let part1_score: u32 = file
        .clone()
        .lines()
        .map(|line| {
            let (enemy_shape, my_shape): (Shape, Shape) = line
                .split_once(' ')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap();
            my_shape.score_against(enemy_shape)
        })
        .sum();
    println!("Part 1 score = {part1_score}");
    let part2_score: u32 = file
        .lines()
        .map(|line| {
            let (enemy_shape, outcome): (Shape, Outcome) = line
                .split_once(' ')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap();
            let my_shape = enemy_shape.shape_to_outcome(outcome);
            my_shape.score_against(enemy_shape)
        })
        .sum();
    println!("Part 2 score = {part2_score}");
}
