use std::collections::HashSet;

fn find_marker(chars: &[char], window: usize) -> Option<usize> {
    chars
        .windows(window)
        .into_iter()
        .position(|items| {
            let set: HashSet<char> = items.iter().copied().collect();
            set.len() == window
        })
        .map(|p| p + window)
}

fn main() {
    let file = include_str!("../input.txt");
    for line in file.lines() {
        let chars: Vec<_> = line.chars().collect();
        println!(
            "First marker position at {}",
            find_marker(&chars, 4).unwrap()
        );
        println!(
            "First message position at {}",
            find_marker(&chars, 14).unwrap()
        );
    }
}
