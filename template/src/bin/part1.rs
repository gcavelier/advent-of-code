use tracing::{debug, error, info};
use itertools::Itertools;

fn main() {
    tracing_subscriber::fmt()
        .event_format(
            tracing_subscriber::fmt::format()
                .with_target(false)
                .compact(),
        )
        .init();
    let input = include_str!("./input1.txt");
    let output = part1(input);
    info!(output);
}

fn part1(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt()
            .event_format(
                tracing_subscriber::fmt::format()
                    .with_target(false)
                    .compact(),
            )
            .init();
        let result = part1("");
        assert_eq!(result, 11);
    }
}
