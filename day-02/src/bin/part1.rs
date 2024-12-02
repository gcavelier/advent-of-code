use std::usize;

use itertools::Itertools;

#[derive(Debug)]
enum Order {
    Asc,
    Desc,
}

const MAX_DIFF: usize = 3;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn is_valid_report(line: &str) -> bool {
    let mut line_iter = line.split_whitespace().peekable();

    // Get the first 2 items to determine the Order to use
    let item1 = line_iter.next().unwrap().parse::<usize>().unwrap();
    //let item2 = line_iter.next().unwrap().parse::<usize>().unwrap();
    let item2 = line_iter.peek().unwrap().parse::<usize>().unwrap();

    let order = if item1 > item2 {
        Order::Desc
    } else if item1 < item2 {
        Order::Asc
    } else {
        // The first 2 items are equal => invalid report
        return false;
    };

    if item1.abs_diff(item2) > MAX_DIFF {
        return false;
    }

    for (a, b) in line_iter.tuple_windows() {
        let a = a.parse::<usize>().unwrap();
        let b = b.parse::<usize>().unwrap();

        match order {
            Order::Asc => {
                if a > b {
                    return false;
                }
            }
            Order::Desc => {
                if a < b {
                    return false;
                }
            }
        }
        let diff = a.abs_diff(b);
        if diff > MAX_DIFF || diff == 0 {
            return false;
        }
    }

    true
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|report| match is_valid_report(report) {
            true => 1,
            false => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, 2);
    }
}
