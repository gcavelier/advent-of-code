use std::ops::BitAnd;

use itertools::Itertools;
use tracing::{debug, error, info, instrument};

#[derive(Debug, PartialEq)]
enum Operator {
    Sum,
    Mul,
}
#[derive(Debug)]
struct Equation {
    value: usize,
    numbers: Vec<usize>,
}

fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input.txt");
    let output = part1(input);
    info!(output);
}

#[instrument]
fn is_valid(equation: &Equation) -> bool {
    // We have to test all possible cases :(
    let max_nb_loop = 2_usize.pow((equation.numbers.len() - 1) as u32);
    //info!("max_nb_loop={max_nb_loop}");
    for curr_loop in 0..max_nb_loop {
        let mut operators: Vec<Operator> =
            (1..equation.numbers.len()).map(|_| Operator::Sum).collect();

        //info!("curr_loop={curr_loop}");

        let mut tmp = curr_loop;
        let mut idx = 0;
        while tmp > 0 {
            if tmp & 1 == 1 {
                operators[idx] = Operator::Mul;
            }
            idx += 1;
            tmp = tmp >> 1;
        }

        //info!(?operators);
        let mut num_iter = equation.numbers.iter();
        let mut res: usize = *num_iter.next().unwrap();
        for operator in &operators {
            match operator {
                Operator::Sum => res = res + num_iter.next().unwrap(),
                Operator::Mul => res = res * num_iter.next().unwrap(),
            }
        }
        if res == equation.value {
            //info!("OK!!! {operators:?}");
            return true;
        }
    }

    false
}

fn part1(input: &str) -> usize {
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(": ");
            Equation {
                value: line_iter.next().unwrap().parse().unwrap(),
                numbers: line_iter
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect(),
            }
        })
        .collect();

    equations
        .iter()
        .filter(|equation| is_valid(equation))
        .map(|equation| equation.value)
        // .inspect(|item| {
        //     info!("{item}");
        // })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        let result = part1(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );
        assert_eq!(result, 3749);
    }
}
