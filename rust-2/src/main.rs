use std::fs::File;
use std::io::{self, Read};

const LINE_SEPARATOR: &str = "\n";
const SPACE: &str = " ";

fn main() {
    let input = read_input("res/input.txt").expect("String inputfile was expected");

    let safe_reports = input
        .split(LINE_SEPARATOR)
        .map(|l| {
            l.split(SPACE)
                .flat_map(|v| v.parse::<isize>())
                .collect::<Vec<isize>>()
        })
        .filter(|report| is_report_safe(report))
        .count();

    println!("The count of safe reports is {}", safe_reports);
}

fn read_input(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn is_report_safe(report: &[isize]) -> bool {
    if let Some(first) = report.first() {
        let &(_, increasing, decreasing, in_range) =
            &report[1..]
                .into_iter()
                .fold((first, true, true, true), |acc, level| match acc {
                    (last_level, increasing, decreasing, in_range) => match level - last_level {
                        1..=3 => (level, increasing, false, in_range),
                        -3..=-1 => (level, false, decreasing, in_range),
                        _ => (level, increasing, decreasing, false),
                    },
                });
        (increasing || decreasing) && in_range
    } else {
        false
    }
}
