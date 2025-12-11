use std::cmp;
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

fn sort_ranges(ranges: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut sorted_ranges: Vec<(u64, u64)> = Vec::new();

    for i in 0..ranges.len() - 1 {
        if ranges[i].0 > ranges[i + 1].0 {
            let temp = ranges[i];
            ranges[i] = ranges[i + 1];
            ranges[i + 1] = temp;
        }

        sorted_ranges.push(ranges[i]);
    }
    sorted_ranges.push(ranges[ranges.len() - 1]);

    sorted_ranges
}

fn interval_merging(sorted_ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut sorted_ranges = sorted_ranges;
    sorted_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    merged_ranges.push(sorted_ranges[0]);

    if sorted_ranges.len() < 2 {
        return merged_ranges;
    }

    for current in sorted_ranges.iter().skip(1) {
        let last = merged_ranges.last_mut().unwrap();

        if current.0 <= last.1 {
            last.1 = cmp::max(last.1, current.1);
        } else {
            merged_ranges.push(*current);
        }
    }

    merged_ranges
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read File");
    let (ranges, _) = contents.split_once("\n\n").unwrap_or((&contents, ""));

    let mut ranges_vec: Vec<(u64, u64)> = ranges
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(get_first_and_last)
        .collect();

    let sorted_ranges = sort_ranges(&mut ranges_vec);
    let merged_ranges = interval_merging(sorted_ranges);

    let mut valid_ids = 0;

    for values in merged_ranges {
        valid_ids = valid_ids + (values.1 - values.0 + 1);
    }

    println!("Valid IDs are: {}", valid_ids);
}
