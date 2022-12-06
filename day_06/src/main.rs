use std::collections::HashSet;

fn find_marker(str: &str, window: usize) -> Option<usize> {
    str.as_bytes()
        .windows(window)
        .into_iter()
        .position(|items| {
            let set: HashSet<_> = items.iter().collect();
            set.len() == window
        })
        .map(|p| p + window)
}

fn main() {
    let file = include_str!("../input.txt");
    for line in file.lines() {
        println!("First marker at {}", find_marker(line, 4).unwrap());
        println!("First message at {}", find_marker(line, 14).unwrap());
    }
}
