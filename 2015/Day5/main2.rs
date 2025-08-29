use std::fs;
use std::collections::HashMap;

fn main() {
    let read_file = fs::read_to_string("input.txt")
        .expect("Failed to read");

    let mut count = 0;

    for each_string in read_file.lines() {
        let mut has_pair_twice = false;
        let mut has_repeat_with_gap = false;

        let mut pairs: HashMap<String, usize> = HashMap::new();
        let chars: Vec<char> = each_string.chars().collect();

        for i in 0..chars.len() {
            if i > 0 {
                let pair: String = vec![chars[i - 1], chars[i]].into_iter().collect();

                if let Some(&prev_index) = pairs.get(&pair) {
                    if i - prev_index > 1 {
                        has_pair_twice = true;
                    }
                } else {
                    pairs.insert(pair, i);
                }
            }

            if i > 1 && chars[i] == chars[i - 2] {
                has_repeat_with_gap = true;
            }
        }

        if has_pair_twice && has_repeat_with_gap {
            count += 1;
        }
    }

    println!("Nice Count is: {count}");
}
