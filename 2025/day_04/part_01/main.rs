use std::fs;

fn parse_matrix(file_path: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(file_path).expect("Failed to read files");

    let matrix = contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    matrix
}

fn walk(r: usize, c: usize, matrix: &Vec<Vec<char>>) -> Vec<char> {
    let mut result = Vec::new();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dr, dc) in dirs {
        let nr = r as isize + dr;
        let nc = c as isize + dc;

        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
            result.push(matrix[nr as usize][nc as usize]);
        }
    }

    result
}

fn main() {
    let file_path = "input.txt";
    let matrix = parse_matrix(&file_path);

    let mut valid_rolls = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '@' {
                let mut count = 0;
                let neighbors = walk(i, j, &matrix);

                for elements in neighbors {
                    if elements == '@' {
                        count += 1;
                    }
                }

                if count < 4 {
                    valid_rolls += 1;
                }
            } else {
                continue;
            }
        }
    }

    println!("Valid Rolls is: {}", valid_rolls);
}
