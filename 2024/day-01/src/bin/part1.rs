use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();

    for line in input.lines() {
        for (left, right) in line.split_ascii_whitespace().tuple_windows() {
            left_vec.push(left.parse::<usize>().unwrap());
            right_vec.push(right.parse::<usize>().unwrap());
        }
    }
    left_vec.sort();
    right_vec.sort();

    left_vec
        .iter()
        .zip(right_vec)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, 11);
    }
}
