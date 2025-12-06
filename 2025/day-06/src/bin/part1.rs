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
fn part1(input: &str) -> usize {
    let mut res = 0;

    let input: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    let line_size = input[0].len();
    for i in 0..line_size {
        let mut numbers = Vec::new();
        let mut tmp_res = 0;
        for line in &input {
            info!("{}", line[i]);
            match line[i] {
                "*" => {
                    tmp_res = 1;
                    for number in &numbers {
                        tmp_res = tmp_res * number;
                    }
                }
                "+" => {
                    for number in &numbers {
                        tmp_res = tmp_res + number;
                    }
                }
                number => numbers.push(number.parse::<usize>().unwrap()),
            }
        }
        res += tmp_res;
    }

    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        let result = part1(
            "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ",
        );
        assert_eq!(result, 4277556);
    }
}
