use std::fs;

fn scanner(list_item: &str) -> usize {
    let mut count_encoded = 2;

    for c in list_item.chars() {
        if c == '"' {
            count_encoded += 2;
        } else if c == '\\' {
            count_encoded += 2;
        } else {
            count_encoded += 1;
        }
    }
    return count_encoded;
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");

    let mut total_encoded = 0;
    let mut total_literal = 0;
    for line in contents.lines() {
        let encoded = scanner(&line);

        total_literal += line.len();
        total_encoded += encoded;
    }

    println!("{}", total_encoded - total_literal);
}
