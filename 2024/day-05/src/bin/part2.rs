use itertools::Itertools;
use tracing::{debug, error, info, instrument};

#[derive(Debug)]
struct Rule {
    first: usize,
    second: usize,
}

fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input2.txt");
    let output = part2(input);
    info!(output);
}

#[instrument(skip_all)]
fn is_valid_update(update: &Vec<usize>, rules: &Vec<Rule>) -> bool {
    // Check each Rule on {update}
    for rule in rules {
        let first_idx = update.iter().position(|val| val == &rule.first);
        let second_idx = update.iter().position(|val| val == &rule.second);
        match (first_idx, second_idx) {
            (Some(first_idx), Some(second_idx)) => {
                if first_idx > second_idx {
                    return false;
                }
            }
            (_, _) => {}
        }
    }
    true
}

#[instrument(skip_all)]
// If this function returns true, we have to run it again!
fn fix_invalid_updates(invalid_updates: &mut Vec<Vec<usize>>, rules: &Vec<Rule>) -> bool {
    let mut res = false;
    for update in invalid_updates {
        for rule in rules {
            let first_idx = update.iter().position(|val| val == &rule.first);
            let second_idx = update.iter().position(|val| val == &rule.second);
            match (first_idx, second_idx) {
                (Some(first_idx), Some(second_idx)) => {
                    if first_idx > second_idx {
                        update.swap(first_idx, second_idx);
                        res = true;
                    }
                }
                (_, _) => {}
            }
        }
    }
    res
}

#[instrument]
fn get_middle_page_number(update: &Vec<usize>) -> usize {
    *update.get(update.len().div_euclid(2)).unwrap()
}

fn part2(input: &str) -> usize {
    let mut input_iter = input.split("\n\n");
    let rules: Vec<Rule> = input_iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut iter = line.split('|');
            Rule {
                first: iter.next().unwrap().parse().unwrap(),
                second: iter.next().unwrap().parse().unwrap(),
            }
        })
        .collect();
    let mut updates: Vec<Vec<usize>> = input_iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .filter(|update| !is_valid_update(update, &rules))
        .collect();

    while fix_invalid_updates(&mut updates, &rules) {}

    updates
        .iter()
        .map(|update| get_middle_page_number(update))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        tracing_subscriber::fmt::init();
        let result = part2(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(result, 123);
    }
}
