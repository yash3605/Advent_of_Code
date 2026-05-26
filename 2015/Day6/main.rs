use std::fs;

fn get_state(line: &str) -> String {
    let mut i = 0;

    for ch in line.chars() {
        if ch.is_ascii_digit() {
            break;
        }
        i += 1;
    }

    line[0..i].trim().to_string()
}

fn get_from_coord(line: &str) -> (usize, usize) {
    let coords: Vec<&str> = line
        .split_whitespace()
        .filter(|w| w.contains(","))
        .collect();

    let from_parts: Vec<&str> = coords[0].split(',').collect();

    (
        from_parts[0].parse().unwrap(),
        from_parts[1].parse().unwrap(),
    )
}
fn get_to_coord(line: &str) -> (usize, usize) {
    let coords: Vec<&str> = line
        .split_whitespace()
        .filter(|w| w.contains(","))
        .collect();

    let to_parts: Vec<&str> = coords[1].split(',').collect();

    (to_parts[0].parse().unwrap(), to_parts[1].parse().unwrap())
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read file");

    let mut grid = vec![vec![false; 1000]; 1000];

    for each_line in content.lines() {
        let state = get_state(each_line);
        let from = get_from_coord(each_line);
        let to = get_to_coord(each_line);

        match state.as_str() {
            "toggle" => {
                for i in from.0..=to.0 {
                    for j in from.1..=to.1 {
                        grid[i][j] = !grid[i][j];
                    }
                }
            }
            "turn off" => {
                for i in from.0..=to.0 {
                    for j in from.1..=to.1 {
                        grid[i][j] = false;
                    }
                }
            }
            "turn on" => {
                for i in from.0..=to.0 {
                    for j in from.1..=to.1 {
                        grid[i][j] = true;
                    }
                }
            }
            _ => {}
        }
    }

    let mut count = 0;

    for i in 0..1000 {
        for j in 0..1000 {
            if grid[i][j] == true {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
