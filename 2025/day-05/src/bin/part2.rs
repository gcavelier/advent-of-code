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

    let mut fresh_ingredients: Vec<_> = fresh_ingredients
        .lines()
        .map(|item| {
            let mut iter = item.split("-");
            RangeInclusive::new(
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    fresh_ingredients.sort_by_key(|item| *item.start());

    let mut new_fresh_ingredients: Vec<RangeInclusive<usize>> = Vec::new();

    for ingredient in fresh_ingredients.into_iter() {
        if let Some(last) = new_fresh_ingredients.pop() {
            if last.contains(&ingredient.start()) {
                let (start, end) = last.into_inner();
                new_fresh_ingredients.push(RangeInclusive::new(start, *ingredient.end().max(&end)));
            } else {
                new_fresh_ingredients.push(last);
                new_fresh_ingredients.push(ingredient);
            }
        } else {
            new_fresh_ingredients.push(ingredient);
        }
    }

    new_fresh_ingredients
        .iter()
        .map(|item| item.end() - item.start() + 1)
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
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
        assert_eq!(result, 14);
    }
}

// 414685493573489 - too high
// 354149806372909
// 321958501747817 - too low
