use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to Read File");

    let mut x = 0;
    let mut y = 0;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((x, y));

    for c in contents.trim().chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => continue,
        }
        visited.insert((x, y));
    }
    println!("House having atleast one present: {}", visited.len());
}
