use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();
    let mut right_hashmap = HashMap::new();
    let mut result: usize = 0;

    for line in input.split("\n") {
        for (left, right) in line.split_ascii_whitespace().tuple_windows() {
            left_vec.push(left);
            right_vec.push(right);
        }
    }

    for item in right_vec {
        right_hashmap
            .entry(item)
            .and_modify(|value| *value += 1)
            .or_insert(1);
    }

    for item in left_vec {
        let item_val: usize = item.parse().unwrap();
        result = result + (item_val * right_hashmap.get(item).unwrap_or(&0));
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, 31);
    }
}
