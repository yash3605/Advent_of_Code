use std::collections::HashMap;
use std::fs;

fn resolve(
    wire: &str,
    instructions: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    if let Some(val) = cache.get(wire) {
        return *val;
    }

    if let Some(val) = wire.parse::<u16>().ok() {
        return val;
    }

    let tokens: Vec<&str> = instructions
        .get(wire)
        .unwrap()
        .iter()
        .map(|s| s.as_str())
        .collect();

    let result = match tokens.as_slice() {
        [a] => resolve(a, instructions, cache),
        ["NOT", a] => !resolve(a, instructions, cache),
        [a, "AND", b] => resolve(a, instructions, cache) & resolve(b, instructions, cache),
        [a, "OR", b] => resolve(a, instructions, cache) | resolve(b, instructions, cache),
        [a, "LSHIFT", b] => resolve(a, instructions, cache) << b.parse::<u16>().unwrap(),
        [a, "RSHIFT", b] => resolve(a, instructions, cache) >> b.parse::<u16>().unwrap(),
        _ => panic!("Unknown instructions"),
    };

    cache.insert(wire.to_string(), result);
    result
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read file");

    let mut cache: HashMap<String, u16> = HashMap::new();
    let mut instructions: HashMap<String, Vec<String>> = HashMap::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split("->").collect();
        let output = parts[1].trim();
        let tokens: Vec<&str> = parts[0].trim().split_whitespace().collect();

        instructions.insert(
            output.to_string(),
            tokens.iter().map(|s| s.to_string()).collect(),
        );
    }
    let res = resolve("a", &instructions, &mut cache);

    cache.clear();
    instructions.insert("b".to_string(), vec![res.to_string()]);

    let res2 = resolve("a", &instructions, &mut cache);
    println!("{}", res2);
}
