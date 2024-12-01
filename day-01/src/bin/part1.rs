use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();
    let mut result: usize = 0;

    for line in input.split("\n") {
        for (left, right) in line.split_ascii_whitespace().tuple_windows() {
            left_vec.push(left);
            right_vec.push(right);
        }
    }
    left_vec.sort();
    right_vec.sort();

    for (left, right) in left_vec.iter().zip(right_vec) {
        let num_left: usize = left.parse().unwrap();
        let num_right: usize = right.parse().unwrap();
        let diff = num_left.abs_diff(num_right);
        result += diff;
    }
    result
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
