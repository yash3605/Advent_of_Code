use std::fs;
use std::collections::HashSet;

fn main() {
    let read_file = fs::read_to_string("input.txt")
        .expect("Failed to read");

    let mut count = 0;
    let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].into();

    for each_string in read_file.lines() {
        let mut vowel_count = 0;
        let mut prev: Option<char> = None;
        let mut has_double = false;
        let mut has_forbidden = false;

        for each_char in each_string.chars(){
            if vowels.contains(&each_char){
                vowel_count += 1
            }

            if let Some(p) = prev {
                if p == each_char {
                    has_double = true;
                }

                if (p == 'a' && each_char == 'b') ||
                   (p == 'c' && each_char == 'd') ||
                   (p == 'p' && each_char == 'q') ||
                   (p == 'x' && each_char == 'y') {
                    has_forbidden = true;
                }
            }


            prev = Some(each_char);
        }
        if vowel_count >= 3 && has_double && !has_forbidden {
            count += 1
        }
    }

    println!("Nice Count is: {count}");
}
