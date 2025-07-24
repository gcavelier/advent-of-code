use itertools::Itertools;
use regex::Regex;
use tracing::{debug, error, info, instrument};

fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input1.txt");
    let output = part1(input);
    info!(output);
}

fn part1(input: &str) -> usize {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    re.find_iter(input)
        .inspect(|m| info!("{m:?}"))
        .map(|m| {
            let mut nb_iter = m
                .as_str()
                .strip_prefix("mul(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split(",");
            let a = nb_iter.next().unwrap().parse::<usize>().unwrap();
            let b = nb_iter.next().unwrap().parse::<usize>().unwrap();
            a * b
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        let result =
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, 161);
    }
}
