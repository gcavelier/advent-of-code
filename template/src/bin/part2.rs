use itertools::Itertools;
use tracing::{debug, error, info};

fn main() {
    tracing_subscriber::fmt::init();

    // let input = include_str!("./input2.txt");
    // let output = part2(input);
    // info!(output);
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        tracing_subscriber::fmt::init();
        let result = part2(xx);
        assert_eq!(result, xx);
    }
}
