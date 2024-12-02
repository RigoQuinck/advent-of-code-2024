use std::fs::File;
use std::io::{self, Read};

const LINE_SEPARATOR: &str = "\n";
const SPACE: &str = "   ";

fn main() {
    let input = read_input("res/input.txt").expect("String inputfile was expected");

    let init: (Vec<u32>, Vec<u32>) = (vec![], vec![]);

    let (mut left, mut right) = input
        .split(LINE_SEPARATOR)
        .map(|l| {
            l.split(SPACE)
                .take(2)
                .flat_map(|v| v.parse::<u32>())
                .collect::<Vec<u32>>()
        })
        .fold(init, |acc, l| match l.as_slice() {
            &[first, second] => (
                [acc.0.as_slice(), &[first]].concat(),
                [acc.1.as_slice(), &[second]].concat(),
            ),
            _ => acc,
        });

    left.sort();
    right.sort();

    let distance = left
        .into_iter()
        .zip(right.into_iter())
        .map(|e| match e {
            (l, r) if l > r => l - r,
            (l, r) => r - l,
        })
        .sum::<u32>();

    println!("The total distance between the two lists is {}", distance);
}

fn read_input(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
