use std::fs;

fn get_first_and_last(range: &str) -> (u64, u64) {
    let (a, b) = range
        .trim()
        .split_once("-")
        .expect("Input is not as expected");

    (
        a.trim().parse::<u64>().unwrap(),
        b.trim().parse::<u64>().unwrap(),
    )
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read File");
    let (ranges, ids) = contents.split_once("\n\n").unwrap_or((&contents, ""));

    let mut fresh_ids = 0;

    let ranges_vec: Vec<(u64, u64)> = ranges
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(get_first_and_last)
        .collect();

    let ids_vec: Vec<u64> = ids
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    for id in ids_vec {
        let mut is_fresh = false;

        for &(start, end) in &ranges_vec {
            if id >= start && id <= end {
                is_fresh = true;
                break; // stop checking more ranges
            }
        }

        if is_fresh {
            fresh_ids += 1;
        }
    }

    println!("The number of fresh ids: {}", fresh_ids);
}
