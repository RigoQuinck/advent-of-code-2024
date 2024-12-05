use std::collections::HashMap;
use std::fs;

use regex::Regex;

const RULE_REGEX: &str = r"(\d+)\|(\d+)";
const UPDATE_REGEX: &str = r"\d+(,\d+)+";
const COMMA: &str = ",";

fn main() {
    let input = fs::read_to_string("res/input.txt").expect("Input read correctly.");
    let rules = get_rules(&input);
    let updates = get_updates(&input);

    let valid_middle_pages_sum: usize = updates
        .iter()
        .filter(|update| is_update_in_order(update, &rules))
        .map(|update| middle_value(update))
        .sum();

    let non_valid_middle_pages_sum: usize = updates
        .iter()
        .filter(|update| !is_update_in_order(update, &rules))
        .map(|update| order_udpate(update, &rules))
        .map(|update| middle_value(&update))
        .sum();

    println!("Valid middle pages sum: {}", valid_middle_pages_sum);
    println!("Non valid middle pages sum: {}", non_valid_middle_pages_sum);
}

fn get_rules(input: &str) -> HashMap<usize, Vec<usize>> {
    let rule_regex = Regex::new(RULE_REGEX).expect("rule regex processed");

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    for (_, [preceed, follow]) in rule_regex.captures_iter(input).map(|c| c.extract()) {
        if let (Ok(preceed), Ok(follow)) = (preceed.parse::<usize>(), follow.parse::<usize>()) {
            let followers = match rules.get(&preceed) {
                Some(followers) => [followers.as_slice(), &[follow]].concat(),
                _ => vec![follow],
            };
            rules.insert(preceed, followers);
        }
    }
    rules
}

fn get_updates(input: &str) -> Vec<Vec<usize>> {
    let re = Regex::new(UPDATE_REGEX).expect("rule regex processed");

    re.find_iter(input)
        .map(|m| {
            m.as_str()
                .split(COMMA)
                .flat_map(|v| v.parse::<usize>())
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn is_update_in_order(update: &[usize], rules: &HashMap<usize, Vec<usize>>) -> bool {
    for (i, v) in update.iter().enumerate() {
        if let Some(followers) = rules.get(v) {
            let precedents: &[usize] = &update[0..i];
            let follower_before = precedents.iter().any(|p| followers.contains(p));
            if follower_before {
                return false;
            }
        }
    }
    true
}

fn middle_value(update: &[usize]) -> usize {
    update[update.len() / 2]
}

fn order_udpate(update: &[usize], rules: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut ordered_update: Vec<usize> = vec![];

    for value in update.iter() {
        if let Some(followers) = rules.get(value) {
            let first_follower_index = ordered_update.iter().enumerate().find_map(|(i, v)| {
                if followers.contains(v) {
                    Some(i)
                } else {
                    None
                }
            });
            if let Some(index) = first_follower_index {
                // TODO
                ordered_update.insert(index, *value);
            } else {
                ordered_update.push(*value);
            }
        } else {
            ordered_update.push(*value);
        }
    }

    ordered_update
}
