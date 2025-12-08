use std::fs;

fn check_current_battery(battery_bank: &str) -> u128 {
    let digits: Vec<u8> = battery_bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let n = digits.len();
    let num_of_picks = 12;

    if n < num_of_picks {
        return 0;
    }

    let mut result_digits = Vec::with_capacity(num_of_picks);
    let mut current_start_index = 0;

    for k in 0..num_of_picks {
        let remaining_to_pick = num_of_picks - 1 - k;

        let search_end_index = n - remaining_to_pick;

        let mut max_in_window = 0;
        let mut max_digit_index = current_start_index;

        for i in current_start_index..search_end_index {
            if digits[i] > max_in_window {
                max_in_window = digits[i];
                max_digit_index = i;
            }
        }

        result_digits.push(max_in_window);
        current_start_index = max_digit_index + 1;
    }

    let mut joltage_value: u128 = 0;

    for &digits in &result_digits {
        joltage_value = joltage_value * 10 + (digits as u128)
    }

    joltage_value
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
