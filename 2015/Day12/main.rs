use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let vec_inp: Vec<char> = contents.chars().collect();

    let mut i = 0;
    let mut total = 0;
    while i < vec_inp.len() {
        let ch = vec_inp[i];

        if ch.is_ascii_digit()
            || (i + 1 < vec_inp.len() && ch == '-' && vec_inp[i + 1].is_ascii_digit())
        {
            let mut num_str = String::new();
            if ch == '-' {
                num_str.push('-');
                i += 1;
            }
            while i < vec_inp.len() && vec_inp[i].is_ascii_digit() {
                num_str.push(vec_inp[i]);
                i += 1;
            }
            total += num_str.parse::<i32>().unwrap();
        } else {
            i += 1;
        }
    }

    println!("{}", total);
}
