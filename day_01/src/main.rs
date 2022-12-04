fn main() {
    let file = include_str!("../input.txt");
    let mut values = vec![0];
    let mut i = 0;

    for line in file.lines() {
        if line.is_empty() {
            values.push(0);
            i += 1;
        } else {
            let num: u32 = line.parse().unwrap();
            values[i] += num;
        }
    }

    values.sort_unstable();
    values.reverse();
    let [a, b, c] = [values[0], values[1], values[2]];
    println!("Part 1: Max value = {}", a);
    println!("Part 2: Total = {} ", a + b + c);
}
