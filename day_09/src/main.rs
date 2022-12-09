use std::collections::HashSet;

type Coords = [i16; 2];

struct Rope {
    head: Coords,
    tail: Coords,
    history: HashSet<Coords>,
}

impl Rope {
    pub fn new() -> Self {
        Self {
            head: [0; 2],
            tail: [0; 2],
            history: vec![[0; 2]].into_iter().collect(),
        }
    }

    pub fn move_head(&mut self, [x, y]: Coords) {
        let original_pos = self.head;
        self.head[0] += x;
        self.head[1] += y;
        if self.head[0].abs_diff(self.tail[0]) > 1 || self.head[1].abs_diff(self.tail[1]) > 1 {
            self.tail = original_pos;
            self.history.insert(self.tail);
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut rope = Rope::new();
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
            rope.move_head(dir);
        }
    }
    println!("Part 1: {}", rope.history.len());
}
