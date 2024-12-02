use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

const LINE_SEPARATOR: &str = "\n";
const SPACE: &str = "   ";

fn main() {
    let input = read_input("res/input.txt").expect("String inputfile was expected");

    let init: (Vec<usize>, Vec<usize>) = (vec![], vec![]);

    let (mut left, mut right) = input
        .split(LINE_SEPARATOR)
        .map(|l| {
            l.split(SPACE)
                .take(2)
                .flat_map(|v| v.parse::<usize>())
                .collect::<Vec<usize>>()
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

    let distance = calculate_distance(&left, &right);
    let similarity = calculate_similarity(&left, &right);

    println!("The total distance between the two lists is {}", distance);
    println!("The similarity score the two lists is {}", similarity);
}

fn read_input(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn calculate_distance(left: &[usize], right: &[usize]) -> usize {
    left.into_iter()
        .zip(right.into_iter())
        .map(|e| match e {
            (l, r) if l > r => l - r,
            (l, r) => r - l,
        })
        .sum::<usize>()
}

fn calculate_similarity(left: &[usize], right: &[usize]) -> usize {
    let mut similarities: HashMap<usize, usize> = HashMap::new();

    left.into_iter()
        .map(|v| {
            let occurrences = if let Some(o) = similarities.get(v) {
                *o
            } else {
                let occurrences = find_occurrences(*v, right);
                similarities.insert(*v, occurrences);
                occurrences
            };
            *v * occurrences
        })
        .sum()
}

fn find_occurrences(value: usize, vec: &[usize]) -> usize {
    vec.into_iter().filter(|v| **v == value).count()
}
