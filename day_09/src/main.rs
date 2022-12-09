use std::collections::HashSet;

type Coords = [i16; 2];

struct Rope<const HIST: usize = 1> {
    head_history: [Coords; HIST],
    tail_history: HashSet<Coords>,
}

impl<const HIST: usize> Rope<HIST> {
    pub fn new() -> Self {
        Self {
            head_history: [[0; 2]; HIST],
            tail_history: vec![[0; 2]].into_iter().collect(),
        }
    }

    pub fn move_head(&mut self, [x, y]: Coords) {
        self.head_history[0][0] += x;
        self.head_history[0][1] += y;
        for i in 1..HIST {
            let prev = self.head_history[i - 1];
            let coord = &mut self.head_history[i];
            let [diff_x, diff_y] = [prev[0] - coord[0], prev[1] - coord[1]];

            if diff_x.abs() <= 1 && diff_y.abs() <= 1 {
                return;
            }
            coord[0] += diff_x.signum();
            coord[1] += diff_y.signum();
        }
        self.tail_history.insert(self.head_history[HIST - 1]);
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut rope_1 = Rope::<2>::new();
    let mut rope_2 = Rope::<10>::new();
    for line in input.lines() {
        let (dir, count) = line.split_once(' ').unwrap();
        let dir = match dir {
            "R" => [1, 0],
            "L" => [-1, 0],
            "U" => [0, 1],
            "D" => [0, -1],
            _ => panic!("{dir} is not a valid direction"),
        };
        let count: usize = count.parse().unwrap();
        for _ in 0..count {
            rope_1.move_head(dir);
            rope_2.move_head(dir);
        }
    }
    println!("Part 1: {}", rope_1.tail_history.len());
    println!("Part 2: {}", rope_2.tail_history.len());
}
