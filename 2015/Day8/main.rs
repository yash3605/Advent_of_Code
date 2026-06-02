use std::fs;

fn scanner(list_item: &str) -> usize {
    let mut count_memory = 0;
    let mut i = 1;
    let char_vec: Vec<char> = list_item.chars().collect();

    while i < char_vec.len() - 1 {
        if char_vec[i] != '\\' {
            count_memory += 1;
            i += 1;
        } else if char_vec[i] == '\\' && (char_vec[i + 1] == '\\' || char_vec[i + 1] == '"') {
            count_memory += 1;
            i += 2;
        } else if char_vec[i] == '\\' && char_vec[i + 1] == 'x' {
            count_memory += 1;
            i += 4;
        } else {
            return 0;
        }
    }
    return count_memory;
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");

    let mut total_memory = 0;
    let mut total_literal = 0;
    for line in contents.lines() {
        let memory_count = scanner(&line);

        total_literal += line.len();
        total_memory += memory_count;
    }

    println!("{}", total_literal - total_memory);
}
