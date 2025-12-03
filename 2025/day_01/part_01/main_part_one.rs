use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to open File");

    let mut track_dial: i32 = 50;
    let mut zero_left_count = 0;

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    for i in 0..lines.len() {
        let dir = lines[i].chars().nth(0);
        let num_in_line = lines[i][1..].parse::<i32>().unwrap();
        match dir {
            Some('L') => track_dial = (track_dial - num_in_line + 100) % 100,
            Some('R') => track_dial = (track_dial + num_in_line) % 100,
            _ => continue,
        }

        if track_dial == 0 {
            zero_left_count += 1;
        }
    }

    println!("Password is: {}", zero_left_count);
}

// println!("{:?}", lines[i].chars().nth(0));
// println!("{:?}", &lines[i][1..].parse::<i32>());
