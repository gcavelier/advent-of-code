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
    let input = include_str!("./input2.txt");
    let output = part2(input);
    info!(output);
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        tracing_subscriber::fmt()
            .event_format(
                tracing_subscriber::fmt::format()
                    .with_target(false)
                    .compact(),
            )
            .init();
        let result = part2("");
        assert_eq!(result, 31);
    }
}
