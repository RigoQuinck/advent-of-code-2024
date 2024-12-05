use std::fs::read_to_string;

const WORD: &str = "XMAS";
const WORD_REVERSE: &str = "SAMX";
const MAS: &str = "MAS";
const SAM: &str = "SAM";

fn main() {
    let input = read_to_string("res/input.txt").expect("Input read correctly.");

    let matrix: Vec<&str> = input.lines().collect::<Vec<_>>();

    let word_count_in_all_directions = count_word_in_all_directions(&matrix);

    println!(
        "XMAS count in all directions {}",
        word_count_in_all_directions
    );

    let x_word_count = count_x_word(&matrix, MAS.len());
    println!("X-MAS count {}", x_word_count);
}

fn count_word_in_all_directions(matrix: &[&str]) -> usize {
    let columns = get_columns(matrix);
    let diag_left_to_right = get_left_to_right_diagonals(matrix);
    let diag_right_to_left = get_right_to_left_diagonals(matrix);

    let non_rows: Vec<&str> = [&columns, &diag_left_to_right, &diag_right_to_left]
        .iter()
        .flat_map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
        .collect();

    let all = [non_rows.as_slice(), matrix].concat();

    all.iter().map(|s| count_word_in_str(s)).sum()
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

fn get_columns(lines: &[&str]) -> Vec<String> {
    let line_len = lines.first().map(|l| l.len()).unwrap_or(0);

    let verticals: Vec<String> = (0..line_len)
        .map(|col| lines.iter().flat_map(|row| row.chars().nth(col)).collect())
        .collect();

    verticals
}

fn count_x_word(matrix: &[&str], size: usize) -> usize {
    let last_row = matrix.len() - size;
    let last_col: usize = matrix.first().map(|r| r.len()).unwrap_or(0) - size;

    let mut count: usize = 0;
    for i in 0..=last_row {
        let rows: &[&str] = &matrix[i..i + size];
        for j in 0..=last_col {
            let sub_matrix = rows.iter().map(|r| &r[j..j + size]).collect::<Vec<_>>();
            if is_x_word(&sub_matrix, MAS, SAM) {
                count += 1;
            }
        }
    }
    count
}

fn is_x_word(matrix: &[&str], word: &str, word_rev: &str) -> bool {
    let size = word.len();

    let (d1, d2) = (0..size).fold((String::new(), String::new()), |acc, i| {
        let ch1 = matrix.get(i).map(|l| l.chars().nth(i)).flatten();
        let ch2 = matrix.get(i).map(|l| l.chars().nth(size - 1 - i)).flatten();
        match (acc, ch1, ch2) {
            ((d1, d2), Some(ch1), Some(ch2)) => {
                let tmp = (format!("{}{}", d1, ch1), format!("{}{}", d2, ch2));
                tmp
            }
            (acc, _, _) => acc,
        }
    });

    let d1_match = d1.contains(word) || d1.contains(word_rev);
    let d2_match = d2.contains(word) || d2.contains(word_rev);
    d1_match && d2_match
}
