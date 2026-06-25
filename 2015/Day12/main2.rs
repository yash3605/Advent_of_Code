use std::fs;

enum ParsedValue {
    Sum(i32),
    String(String),
}

fn parse_value(chars: &[char], i: &mut usize) -> ParsedValue {
    if chars[*i] == '{' {
        parse_object(chars, i)
    } else if chars[*i] == '[' {
        parse_array(chars, i)
    } else if chars[*i] == '"' {
        parse_string(chars, i)
    } else {
        parse_number(chars, i)
    }
}

fn parse_number(chars: &[char], i: &mut usize) -> ParsedValue {
    let mut num_str = String::new();

    if chars[*i] == '-' {
        num_str.push('-');
        *i += 1;
    }

    while *i < chars.len() && chars[*i].is_ascii_digit() {
        num_str.push(chars[*i]);
        *i += 1;
    }

    ParsedValue::Sum(num_str.parse::<i32>().unwrap())
}

fn parse_string(chars: &[char], i: &mut usize) -> ParsedValue {
    if chars[*i] == '"' {
        *i += 1;
    }

    let mut string = String::new();

    while *i < chars.len() && chars[*i] != '"' {
        string.push(chars[*i]);
        *i += 1;
    }

    if chars[*i] == '"' {
        *i += 1;
    }

    return ParsedValue::String(string);
}

fn parse_array(chars: &[char], i: &mut usize) -> ParsedValue {
    if chars[*i] == '[' {
        *i += 1;
    }

    let mut sum = 0;

    while *i < chars.len() && chars[*i] != ']' {
        let value = parse_value(chars, i);

        match value {
            ParsedValue::Sum(x) => {
                sum += x;
            }

            ParsedValue::String(_) => {}
        }

        if chars[*i] == ',' {
            *i += 1;
        }
    }

    if chars[*i] == ']' {
        *i += 1;
    }

    return ParsedValue::Sum(sum);
}

fn parse_object(chars: &[char], i: &mut usize) -> ParsedValue {
    if chars[*i] == '{' {
        *i += 1;
    }

    let mut sum = 0;
    let mut has_red = false;

    while *i < chars.len() && chars[*i] != '}' {
        let _ = parse_string(chars, i);

        if chars[*i] == ':' {
            *i += 1;
        }

        let value = parse_value(chars, i);

        match value {
            ParsedValue::String(s) => {
                if s == "red" {
                    has_red = true;
                }
            }

            ParsedValue::Sum(x) => {
                sum += x;
            }
        }

        if chars[*i] == ',' {
            *i += 1;
        }
    }

    if chars[*i] == '}' {
        *i += 1;
    }

    if has_red {
        return ParsedValue::Sum(0);
    }

    return ParsedValue::Sum(sum);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let chars: Vec<char> = contents.chars().collect();
    let mut index = 0;

    let answer = parse_value(&chars, &mut index);

    if let ParsedValue::Sum(x) = answer {
        println!("{}", x);
    }
}
