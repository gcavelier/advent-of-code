use std::collections::HashMap;

use tracing::{debug, error, info, instrument};

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

    info!(input);

    for (idx, c) in input.chars().enumerate() {
        hm.entry(c)
            .and_modify(|entry: &mut Vec<usize>| entry.push(idx))
            .or_insert(vec![idx]);
    }

    info!("hm={hm:#?}");

    let max_key = hm.keys().max().unwrap();
    let min_idx = hm.get(max_key).unwrap().iter().min().unwrap();

    (*min_idx, *max_key)
}

#[instrument(skip_all)]
fn get_max_joltage(input: &str) -> usize {
    info!(input);

    let (first_max_idx, first_max) = get_first_max(&input[..input.len() - 1]);
    let (_, second_max) = get_first_max(&input[first_max_idx + 1..]);

    let res = format!("{first_max}{second_max}").parse().unwrap();
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
        let result = part1(
            "987654321111111
811111111111119
234234234234278
818181911112111",
        );
        assert_eq!(result, 357);

        assert_eq!(
            get_max_joltage(
                "2729222898333435221331344323326433312213237351265464322422255231323342221332324224626312434133312222"
            ),
            99
        );
    }
}
