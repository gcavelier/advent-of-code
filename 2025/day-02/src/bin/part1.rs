use std::ops::Shr;

use itertools::Itertools;
use tracing::{debug, error, info, instrument};

#[instrument(skip_all)]
fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input.txt");
    let output = part1(input);
    info!(output);
}

fn is_valid_id(id: usize) -> bool {
    let id = format!("{id}");
    let id_len = id.len();

    if id_len.rem_euclid(2) != 0 {
        return true;
    }

    id[..(id_len / 2)] != id[(id_len / 2)..]
}

fn sum_of_invalid_ids_in_range(start_range: usize, end_range: usize) -> usize {
    let mut sum = 0;

    for i in start_range..=end_range {
        if !is_valid_id(i) {
            sum += i;
        }
    }

    sum
}

#[instrument(skip_all)]
fn part1(input: &str) -> usize {
    input
        .split(',')
        .map(|item| {
            let mut iter = item.split('-');
            let start_range = iter.next().unwrap();
            let end_range = iter.next().unwrap().trim();
            //info!("start_range={start_range} end_range={end_range}");
            let start_range = start_range.parse().unwrap();
            let end_range = end_range.parse().unwrap();
            (start_range, end_range)
        })
        .map(|(start_range, end_range)| sum_of_invalid_ids_in_range(start_range, end_range))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        let result = part1(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_ids() {
        assert_eq!(false, is_valid_id(55));
        assert_eq!(false, is_valid_id(6464));
        assert_eq!(false, is_valid_id(123123));
        assert_eq!(true, is_valid_id(101));
        assert_eq!(false, is_valid_id(1188511885));
        assert_eq!(true, is_valid_id(1188511880));
    }
}
