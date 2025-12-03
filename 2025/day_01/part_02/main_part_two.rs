use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to open File");

    let mut track_dial: i32 = 50;
    let mut zero_left_count = 0;

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    for i in 0..lines.len() {
        let dir = lines[i].chars().nth(0);
        let num = lines[i][1..].parse::<i32>().unwrap();

        let loops = num / 100;
        let mut remaining = num;

        if num >= 100 {
            zero_left_count += loops;
            remaining = num % 100;
        }

        if dir == Some('L') && track_dial != 0 && (track_dial - remaining) <= 0 {
            zero_left_count += 1;
        }

        if dir == Some('R') && track_dial != 0 && (track_dial + remaining) >= 100 {
            zero_left_count += 1;
        }

        match dir {
            Some('L') => track_dial = (track_dial - remaining + 100) % 100,
            Some('R') => track_dial = (track_dial + remaining) % 100,
            _ => continue,
        };
    }

    println!("Zero Encounter: {}", zero_left_count);
}
