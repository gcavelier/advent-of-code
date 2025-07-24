use itertools::Itertools;
use tracing::{debug, error, info, instrument};

#[derive(Debug)]
struct Rule {
    first: usize,
    second: usize,
}

fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input1.txt");
    let output = part1(input);
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

#[instrument]
fn get_middle_page_number(update: &Vec<usize>) -> usize {
    *update.get(update.len().div_euclid(2)).unwrap()
}

fn part1(input: &str) -> usize {
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
    let valid_updates: Vec<Vec<usize>> = input_iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .filter(|update| is_valid_update(update, &rules))
        .collect();

    valid_updates
        .iter()
        .map(|update| get_middle_page_number(update))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        let result = part1(
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
        assert_eq!(result, 143);
    }
}
