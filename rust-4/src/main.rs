use std::fs::read_to_string;

const WORD: &str = "XMAS";
const WORD_REVERSE: &str = "SAMX";

fn main() {
    let input = read_to_string("res/input.txt").expect("Input read correctly.");

    let rows: Vec<&str> = input.lines().collect::<Vec<_>>();
    let columns = get_columns(&rows);
    let diag_left_to_right = get_left_to_right_diagonals(&rows);
    let diag_right_to_left = get_right_to_left_diagonals(&rows);

    let tmp: Vec<&str> = [&columns, &diag_left_to_right, &diag_right_to_left]
        .iter()
        .flat_map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
        .collect();

    let all = [tmp.as_slice(), rows.as_slice()].concat();

    let result: usize = all.iter().map(|s| count_word_in_str(s)).sum();

    println!("{}", result);
}

fn get_columns(lines: &[&str]) -> Vec<String> {
    let line_len = lines.first().map(|l| l.len()).unwrap_or(0);

    let verticals: Vec<String> = (0..line_len)
        .map(|col| lines.iter().flat_map(|row| row.chars().nth(col)).collect())
        .collect();

    verticals
}

fn get_left_to_right_diagonals(input: &[&str]) -> Vec<String> {
    let mut diagonals = Vec::new();
    let rows = input.len();
    let cols = input[0].len();

    for d in 0..rows + cols - 1 {
        let mut diagonal = String::new();
        for row in 0..=d {
            let col = d - row;
            if row < rows && col < cols {
                diagonal.push(input[row].chars().nth(col).unwrap());
            }
        }
        diagonals.push(diagonal);
    }

    diagonals
}

fn get_right_to_left_diagonals(input: &[&str]) -> Vec<String> {
    let mut diagonals = Vec::new();
    let rows = input.len();
    let cols = input[0].len();

    for d in 0..rows + cols - 1 {
        let mut diagonal = String::new();
        for row in 0..=d {
            if let Some(col) = cols.checked_sub(1 + d - row) {
                if row < rows && col < cols {
                    diagonal.push(input[row].chars().nth(col).unwrap());
                }
            }
        }
        diagonals.push(diagonal);
    }

    diagonals
}

fn count_word_in_str(s: &str) -> usize {
    s.matches(WORD).count() + s.matches(WORD_REVERSE).count()
}
