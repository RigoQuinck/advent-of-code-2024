use regex::Regex;
use std::fs::File;
use std::io::{self, Read};

const MUL_REGEX: &str = r"mul\((\d{1,3}),(\d{1,3})\)";
const DO: &str = "do()";
const DONT: &str = "don't()";

fn main() {
    let input = read_input("res/input.txt").expect("String input file was expected");

    let muls_sum = sum_mul(&input);

    match muls_sum {
        Ok(muls_sum) => println!("The results of the multiplications: {}", muls_sum),
        Err(error) => println!("{}", error.to_string()),
    }

    let do_muls_sum: usize = input
        .split(DO)
        .flat_map(|v| v.split(DONT).next())
        .flat_map(|s| sum_mul(s).ok())
        .sum();

    println!("The result obtained adding up all of the results of just the enabled multiplications is: {}", do_muls_sum);
}

// 184511516
//  90044227

fn read_input(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn sum_mul(input: &str) -> Result<usize, regex::Error> {
    let mul_regex = Regex::new(MUL_REGEX)?;

    let result: usize = mul_regex
        .captures_iter(input)
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

    Ok(result)
}
