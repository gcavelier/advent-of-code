use itertools::Itertools;
use tracing::{debug, error, info, instrument};

#[instrument(skip_all)]
fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input.txt");
    let output = part2(input);
    info!(output);
}

fn pattern_matching_till_the_end(pattern: &str, rest: &str) -> bool {
    let pattern_len = pattern.len();
    if rest.len().rem_euclid(pattern_len) != 0 {
        return false;
    }

    //println!("pattern={pattern} rest={rest}");

    for i in 0..rest.len() / pattern_len {
        //println!("i={i} {}->{}", i * pattern_len, i + 1 * pattern_len);
        if &rest[i * pattern_len..(i + 1) * pattern_len] != pattern {
            return false;
        }
    }

    true
}

fn is_valid_id(id: usize) -> bool {
    let id = format!("{id}");
    let id_len = id.len();

    for i in 0..id_len / 2 {
        let pattern = &id[..i + 1];
        if pattern_matching_till_the_end(pattern, &id[i + 1..]) {
            return false;
        }
    }

    true
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
fn part2(input: &str) -> usize {
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
    fn part2_works() {
        tracing_subscriber::fmt::init();
        let result = part2(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn check_invalid() {
        //assert_eq!(is_valid_id(11), false);
        assert_eq!(is_valid_id(111), false);
        assert_eq!(is_valid_id(1188511885), false);
        assert_eq!(is_valid_id(222222), false);
        assert_eq!(is_valid_id(824824824), false);
        assert_eq!(is_valid_id(1234123), true);
    }
}
