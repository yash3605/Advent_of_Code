use std::fs;

fn main() {
    let contents = fs::read_to_string("input_test.txt").expect("Failed to read file");

    println!("{}", contents);
}
