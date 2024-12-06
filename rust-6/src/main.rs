use std::fs;

const GUARD_UP: char = '^';
const GUARD_DOWN: char = 'v';
const GUARD_RIGHT: char = '>';
const GUARD_LEFT: char = '<';
const OBSTACLE: char = '#';
const VISITED: char = 'X';

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn next(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Guard(usize, usize, Direction);

fn main() {
    let input = fs::read_to_string("res/input.txt").expect("Input read correctly.");
    let mut matrix = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut guard = get_guard_position(&matrix).expect("Guard present in matrix");
    let mut in_matrix = true;
    let mut visited_count = 0;

    while in_matrix {
        let Guard(row, col, dir) = guard;
        if not_visited(&matrix, row, col) {
            matrix[row][col] = VISITED;
            visited_count += 1;
        }

        let moved_guard = match dir {
            Direction::Up if row > 0 => move_guard(&matrix, &guard, row - 1, col),
            Direction::Right => move_guard(&matrix, &guard, row, col + 1),
            Direction::Down => move_guard(&matrix, &guard, row + 1, col),
            Direction::Left if col > 0 => move_guard(&matrix, &guard, row, col - 1),
            _ => None,
        };

        if let Some(moved_guard) = moved_guard {
            guard = moved_guard;
        } else {
            in_matrix = false;
        }
    }

    println!(
        "How many distinct positions will the guard visit before leaving the mapped area?: {}",
        visited_count
    );
}

fn get_guard_position(matrix: &[Vec<char>]) -> Option<Guard> {
    for (i, line) in matrix.iter().enumerate() {
        for (j, curr) in line.iter().enumerate() {
            match *curr {
                GUARD_UP => return Some(Guard(i, j, Direction::Up)),
                GUARD_DOWN => return Some(Guard(i, j, Direction::Down)),
                GUARD_RIGHT => return Some(Guard(i, j, Direction::Right)),
                GUARD_LEFT => return Some(Guard(i, j, Direction::Left)),
                _ => (),
            }
        }
    }
    None
}

fn get_cell(matrix: &[Vec<char>], row: usize, col: usize) -> Option<&char> {
    matrix.get(row).map(|r| r.get(col)).flatten()
}

fn not_visited(matrix: &[Vec<char>], row: usize, col: usize) -> bool {
    match get_cell(&matrix, row, col) {
        Some(c) if c != &VISITED => true,
        _ => false,
    }
}

fn move_guard(
    matrix: &[Vec<char>],
    guard: &Guard,
    next_row: usize,
    next_col: usize,
) -> Option<Guard> {
    let Guard(row, col, d) = guard;
    match get_cell(&matrix, next_row, next_col) {
        Some(&OBSTACLE) => Some(Guard(*row, *col, d.next())),
        Some(_) => Some(Guard(next_row, next_col, *d)),
        _ => None,
    }
}
