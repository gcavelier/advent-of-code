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
fn blink(stones: Vec<usize>) -> Vec<usize> {
    let new_stones: Vec<usize> = stones
        .iter()
        .flat_map(|stone| {
            let mut tmp_vec = Vec::new();
            if *stone == 0 {
                tmp_vec.push(1);
                tmp_vec
            } else if stone.to_string().len() % 2 == 0 {
                let tmp_str = stone.to_string();
                let (a, b) = tmp_str.split_at(stone.to_string().len() / 2);
                tmp_vec.push(a.parse().unwrap());
                tmp_vec.push(b.parse().unwrap());
                tmp_vec
            } else {
                tmp_vec.push(stone * 2024);
                tmp_vec
            }
        })
        .collect();
    new_stones
}

#[instrument(skip_all)]
fn part1(input: &str) -> usize {
    let mut stones: Vec<usize> = input
        .split_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect();

    for _ in 0..25 {
        stones = blink(stones);
    }

    stones.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        let result = part1("125 17");
        assert_eq!(result, 55312);
    }
}
