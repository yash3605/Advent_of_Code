fn process_current(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();

    let mut count = 1;

    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(chars[i - 1]);
            count = 1;
        }
    }

    result.push_str(&count.to_string());
    result.push(chars[chars.len() - 1]);

    result
}

fn main() {
    let mut current = "3113322113".to_string();

    for _ in 0..50 {
        current = process_current(&current);
    }

    println!("{}", current.len());
}
