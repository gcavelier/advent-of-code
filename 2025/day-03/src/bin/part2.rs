use std::collections::HashMap;

use tracing::{debug, error, info, instrument};

const NB_BATTERIES: usize = 12;

#[instrument(skip_all)]
fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input.txt");
    let output = part1(input);
    info!(output);
}

#[instrument(skip_all)]
fn get_first_max(input: &str) -> (usize, char) {
    let mut hm = HashMap::new();

    //info!(input);

    for (idx, c) in input.chars().enumerate() {
        hm.entry(c)
            .and_modify(|entry: &mut Vec<usize>| entry.push(idx))
            .or_insert(vec![idx]);
    }

    //info!("hm={hm:#?}");

    let max_key = hm.keys().max().unwrap();
    let min_idx = hm.get(max_key).unwrap().iter().min().unwrap();

    (*min_idx, *max_key)
}

#[instrument(skip_all)]
fn get_max_joltage(input: &str) -> usize {
    info!(input);

    let mut res = String::new();
    let mut current_idx = 0;

    for i in 0..11 {
        info!(i, slice = &input[current_idx..input.len() - 11 + i]);
        let (first_max_idx, first_max) = get_first_max(&input[current_idx..input.len() - 11 + i]);
        current_idx += first_max_idx + 1;
        res.push(first_max);
    }
    let (_, second_max) = get_first_max(&input[current_idx..]);
    res.push(second_max);

    let res = format!("{res}").parse().unwrap();
    info!(res);
    res
}

#[instrument(skip_all)]
fn part1(input: &str) -> usize {
    input.lines().map(|item| get_max_joltage(item)).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(get_max_joltage("987654321111111"), 987654321111);
        assert_eq!(get_max_joltage("811111111111119"), 811111111119);
        assert_eq!(get_max_joltage("234234234234278"), 434234234278);
        assert_eq!(get_max_joltage("818181911112111"), 888911112111);
        let result = part1(
            "987654321111111
        811111111111119
        234234234234278
        818181911112111",
        );
        assert_eq!(result, 3121910778619);
    }
}
