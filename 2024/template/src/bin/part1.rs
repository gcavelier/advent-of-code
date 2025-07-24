use itertools::Itertools;
use tracing::{debug, error, info, instrument};

#[instrument(skip_all)]
fn main() {
    tracing_subscriber::fmt::init();

    // let input = include_str!("./input.txt");
    // let output = part1(input);
    // info!(output);
}

#[instrument(skip_all)]
fn part1(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        let result = part1(xx);
        assert_eq!(result, xx);
    }
}
