use std::fs;

fn check_current_battery(battery: &str) -> u32 {
    let digits: Vec<u32> = battery.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let n = digits.len();
    let mut max_joltage = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let joltage = digits[i] * 10 + digits[j];
            if joltage > max_joltage {
                max_joltage = joltage;
            }
        }
    }
    max_joltage
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read File");
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    let mut total_output_joltage = 0;

    for i in 0..lines.len() {
        let chosen_bank = check_current_battery(&lines[i]);
        total_output_joltage += chosen_bank;
    }

    println!("Answer is: {}", total_output_joltage);
}
