use std::collections::{HashMap, HashSet};
use std::fs;

fn get_data_sep(line: &str) -> (String, String, u32) {
    let data_vec: Vec<&str> = line.split_whitespace().collect();
    let src = data_vec[0].to_string();
    let des = data_vec[2].to_string();
    let distance = data_vec[4].parse::<u32>().unwrap();
    return (src, des, distance);
}

fn permutation_cities(
    cities_vec: &Vec<String>,
    current_route: &mut Vec<String>,
    visited: &mut Vec<bool>,
    distance_map: &HashMap<(String, String), u32>,
    current_distance: u32,
    best: &mut u32,
) {
    if current_route.len() == cities_vec.len() {
        if current_distance < *best {
            *best = current_distance;
        }
        return;
    }

    for i in 0..cities_vec.len() {
        if !visited[i] {
            let new_distance = if current_route.is_empty() {
                current_distance
            } else {
                let prev_city = current_route.last().unwrap();

                let key = (prev_city.clone(), cities_vec[i].clone());

                if !distance_map.contains_key(&key) {
                    println!("Missing key: {:?}", key);
                }

                let edge_dist = distance_map.get(&key).unwrap();

                current_distance + *edge_dist
            };

            current_route.push(cities_vec[i].clone());
            visited[i] = true;

            permutation_cities(
                cities_vec,
                current_route,
                visited,
                distance_map,
                new_distance,
                best,
            );

            visited[i] = false;
            current_route.pop();
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to Read file");

    let mut cities: HashSet<String> = HashSet::new();
    let mut distance_map: HashMap<(String, String), u32> = HashMap::new();

    for line in contents.lines() {
        let (src, des, distance) = get_data_sep(&line);

        cities.insert(src.clone());
        cities.insert(des.clone());

        distance_map.insert((src.clone(), des.clone()), distance);
        distance_map.insert((des, src), distance);
    }

    let cities_vec: Vec<String> = cities.into_iter().collect();
    let mut current_route: Vec<String> = Vec::new();
    let mut visited = vec![false; cities_vec.len()];
    let mut best = u32::MAX;

    permutation_cities(
        &cities_vec,
        &mut current_route,
        &mut visited,
        &distance_map,
        0,
        &mut best,
    );

    println!("{}", best);
}
