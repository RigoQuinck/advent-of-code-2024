use regex::Regex;
use std::fs::File;
use std::io::{self, Read};

const MUL_REGEX: &str = r"mul\((\d{1,3}),(\d{1,3})\)";

fn main() {
    let input = read_input("res/input.txt").expect("String inputfile was expected");

    let re: Regex = Regex::new(MUL_REGEX).unwrap();

    let muls_sum: usize = re
        .captures_iter(&input)
        .flat_map(|c| {
            let (_, [f1, f2]) = c.extract();
            let f1 = f1.parse::<usize>().ok();
            let f2 = f2.parse::<usize>().ok();
            match (f1, f2) {
                (Some(f1), Some(f2)) => Some(f1 * f2),
                _ => None,
            }
        })
        .sum();

    println!("The results of the multiplications: {}", muls_sum);
}

fn read_input(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
