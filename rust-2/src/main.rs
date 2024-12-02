use std::fs::File;
use std::io::{self, Read};

const LINE_SEPARATOR: &str = "\n";
const SPACE: &str = " ";

fn main() {
    let input = read_input("res/input.txt").expect("String inputfile was expected");

    let reports = &input
        .split(LINE_SEPARATOR)
        .map(|l| {
            l.split(SPACE)
                .flat_map(|v| v.parse::<isize>())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();

    let safe_reports = reports
        .into_iter()
        .filter(|report| is_report_safe(report))
        .count();

    let safe_reports_removing_any_level = reports
        .into_iter()
        .filter(|report| is_report_safe_removing_any_level(report))
        .count();

    println!("The count of safe reports is {}", safe_reports);
    println!(
        "The count of safe reports removing any val is {}",
        safe_reports_removing_any_level
    );
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

fn is_report_safe_removing_any_level(report: &[isize]) -> bool {
    if is_report_safe(report) {
        true
    } else {
        for i in 0..report.len() {
            let without_i = [&report[0..i], &report[(i + 1)..]].concat();
            if is_report_safe(&without_i) {
                return true;
            }
        }
        false
    }
}
