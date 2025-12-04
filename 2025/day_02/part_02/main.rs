use std::fs;

fn check_id(id: i64) -> bool {
    let id_in_string = id.to_string();
    let mid = id_in_string.len() / 2;

    if id_in_string.len() % 2 != 0 {
        return true;
    } else {
        let (left, right) = id_in_string.split_at(mid);

        if left == right {
            return false;
        } else {
            return true;
        }
    }
}

fn check_id_2(id: i64) -> bool {
    let id_in_string = id.to_string();
    let doubled = format!("{}{}", id_in_string, id_in_string);

    let start = 1;
    let end = doubled.len() - 1;

    let substring = &doubled[start..end];

    let res = substring.contains(&id_in_string);

    if res {
        return false;
    } else {
        return true;
    }
}

fn main() {
    let s = fs::read_to_string("input.txt").expect("Failed to read File");

    let mut res_sum = 0;

    let ranges: Vec<(i64, i64)> = s
        .split(',')
        .map(|part| {
            let (a, b) = part.trim().split_once('-').unwrap();
            (
                a.trim().parse::<i64>().unwrap(),
                b.trim().parse::<i64>().unwrap(),
            )
        })
        .collect();

    for i in 0..ranges.len() {
        for j in ranges[i].0..=ranges[i].1 {
            let is_valid = check_id(j);
            let is_valid2 = check_id_2(j);

            if !is_valid || !is_valid2 {
                res_sum += j;
            }
        }
    }

    println!("Sum of Invalid Ids: {}", res_sum);
}
