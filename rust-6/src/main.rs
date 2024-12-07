use std::fs;

const GUARD_UP: char = '^';
const GUARD_DOWN: char = 'v';
const GUARD_RIGHT: char = '>';
const GUARD_LEFT: char = '<';
const OBSTACLE: char = '#';

#[derive(Copy, Clone, PartialEq)]
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

    fn char(&self) -> char {
        match *self {
            Direction::Up => GUARD_UP,
            Direction::Right => GUARD_RIGHT,
            Direction::Down => GUARD_DOWN,
            Direction::Left => GUARD_LEFT,
        }
    }
}

#[derive(PartialEq)]
struct Guard(usize, usize, Direction);

#[derive(Debug, PartialEq)]
struct Location(usize, usize);

fn main() {
    let input = fs::read_to_string("res/input.txt").expect("Input read correctly.");
    let mut matrix = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut guard = get_guard_position(&matrix).expect("Guard present in matrix");
    let mut in_matrix = true;
    let mut visited_count = 1;
    let mut stuck_count = 0;
    let mut location_changed = false;

    while in_matrix {
        let Guard(row, col, dir) = guard;
        if not_visited(&matrix, row, col) {
            matrix[row][col] = dir.char();
            visited_count += 1;
        }
        if location_changed && can_stuck(&matrix, Location(row, col), dir.next()) {
            println!("{:?}", next_location(row, col, dir));
            stuck_count += 1;
        }

        let guard_in_matrix = match dir {
            Direction::Up if row > 0 => move_guard(&matrix, &guard, row - 1, col),
            Direction::Right => move_guard(&matrix, &guard, row, col + 1),
            Direction::Down => move_guard(&matrix, &guard, row + 1, col),
            Direction::Left if col > 0 => move_guard(&matrix, &guard, row, col - 1),
            _ => None,
        };

        if let Some(guard_in_matrix) = guard_in_matrix {
            location_changed = dir == guard_in_matrix.2;
            guard = guard_in_matrix;
        } else {
            in_matrix = false;
        }
    }

    println!(
        "How many distinct positions will the guard visit before leaving the mapped area?: {}",
        visited_count
    );
    println!(
        "How many different positions could you choose for this obstruction?: {}",
        stuck_count
    );

    print_matrix(&matrix);
}

fn can_stuck(matrix: &[Vec<char>], loc: Location, dir: Direction) -> bool {
    let Location(row, col) = loc;
    let dir_c = dir.char();
    let cell = get_cell(matrix, row, col);
    match cell {
        Some(&OBSTACLE) => false,
        Some(c) if *c == dir_c => true,
        Some(_) => next_location(row, col, dir)
            .map(|loc| can_stuck(matrix, loc, dir))
            .unwrap_or(false),
        None => false,
    }
}

fn next_location(row: usize, col: usize, dir: Direction) -> Option<Location> {
    match dir {
        Direction::Up if row > 0 => Some(Location(row - 1, col)),
        Direction::Right => Some(Location(row, col + 1)),
        Direction::Down => Some(Location(row + 1, col)),
        Direction::Left if col > 0 => Some(Location(row, col - 1)),
        _ => None,
    }
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
    get_cell(&matrix, row, col)
        .map(|c| direction_from_direction(c))
        .flatten()
        .is_none()
}

fn direction_from_direction(c: &char) -> Option<Direction> {
    match *c {
        GUARD_UP => Some(Direction::Up),
        GUARD_DOWN => Some(Direction::Down),
        GUARD_RIGHT => Some(Direction::Right),
        GUARD_LEFT => Some(Direction::Left),
        _ => None,
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

fn print_matrix(matrix: &[Vec<char>]) {
    println!("##### MATRIX START #####");
    for line in matrix.iter() {
        for ch in line.iter() {
            print!("{}", ch);
        }
        println!();
    }
    println!("##### MATRIX END #####");
}
