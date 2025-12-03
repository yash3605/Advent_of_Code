use std::collections::HashSet;
use std::fs;

fn main() {
    let data = fs::read_to_string("input2.txt").expect("Failed to read Files");

    let mut santa_x = 0;
    let mut santa_y = 0;

    let mut robo_x = 0;
    let mut robo_y = 0;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((santa_x, santa_y));

    for (i, c) in data.trim().chars().enumerate() {
        if i % 2 == 1 {
            match c {
                '^' => santa_y += 1,
                'v' => santa_y -= 1,
                '>' => santa_x += 1,
                '<' => santa_x -= 1,
                _ => continue,
            }
            visited.insert((santa_x, santa_y));
        }
    }

    for (i, c) in data.trim().chars().enumerate() {
        if i % 2 == 0 {
            match c {
                '^' => robo_y += 1,
                'v' => robo_y -= 1,
                '>' => robo_x += 1,
                '<' => robo_x -= 1,
                _ => continue,
            }
            visited.insert((robo_x, robo_y));
        }
    }

    println!("House having atleast one present: {}", visited.len());
}
