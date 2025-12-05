use std::ops::RangeInclusive;

use itertools::Itertools;
use tracing::{debug, error, info, instrument};

#[instrument(skip_all)]
fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input.txt");
    let output = part1(input);
    info!(output);
}

#[instrument(skip_all)]
fn ingredients_is_fresh(fresh_ingredients: &[RangeInclusive<usize>], ingredient: usize) -> bool {
    for range in fresh_ingredients {
        if range.contains(&ingredient) {
            return true;
        }
    }
    false
}

#[instrument(skip_all)]
fn part1(input: &str) -> usize {
    let mut input_iter = input.split("\n\n");
    let fresh_ingredients = input_iter.next().unwrap();
    let available_ingredients = input_iter.next().unwrap();

    let fresh_ingredients: Vec<_> = fresh_ingredients
        .lines()
        .map(|item| {
            let mut iter = item.split("-");
            RangeInclusive::new(
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    let available_ingredients: Vec<_> = available_ingredients
        .lines()
        .map(|item| item.parse::<usize>().unwrap())
        .collect();

    available_ingredients
        .iter()
        .filter_map(|item| {
            if ingredients_is_fresh(&fresh_ingredients, *item) {
                Some(1)
            } else {
                None
            }
        })
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        let result = part1(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        );
        assert_eq!(result, 3);
    }
}
