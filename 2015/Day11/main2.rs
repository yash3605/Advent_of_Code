use std::collections::HashSet;

fn is_valid(inp: &[u8]) -> bool {
    has_straight(inp) && has_no_forbidden(inp) && has_two_pairs(inp)
}

fn has_straight(inp: &[u8]) -> bool {
    for window in inp.windows(3) {
        if window[1] == window[0] + 1 && window[2] == window[1] + 1 {
            return true;
        }
    }
    false
}

fn has_no_forbidden(inp: &[u8]) -> bool {
    for i in 0..inp.len() {
        if inp[i] == b'i' || inp[i] == b'l' || inp[i] == b'o' {
            return false;
        }
    }
    true
}

fn has_two_pairs(inp: &[u8]) -> bool {
    let mut pairs: HashSet<u8> = HashSet::new();

    let mut i = 0;
    while i + 1 < inp.len() {
        if inp[i + 1] == inp[i] {
            pairs.insert(inp[i]);
            i += 2
        } else {
            i += 1;
        }
    }

    pairs.len() >= 2
}

fn increment(inp: &mut Vec<u8>) {
    for i in (0..inp.len()).rev() {
        if inp[i] == b'z' {
            inp[i] = b'a'
        } else {
            inp[i] += 1;
            break;
        }
    }
}

fn main() {
    let mut input = b"cqjxjnds".to_vec();

    while !is_valid(&input) {
        increment(&mut input);
    }

    increment(&mut input);
    while !is_valid(&input) {
        increment(&mut input);
    }

    println!("{}", String::from_utf8(input).unwrap());
}
