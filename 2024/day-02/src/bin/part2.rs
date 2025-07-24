use itertools::Itertools;

#[derive(Debug)]
enum Order {
    Asc,
    Desc,
}

const MAX_DIFF: usize = 3;
fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn is_valid_report(levels: &Vec<usize>) -> bool {
    let mut levels_iter = levels.iter().peekable();

    // Get the first 2 items to determine the Order to use
    let item1 = *levels_iter.next().unwrap();
    let item2 = **levels_iter.peek().unwrap();

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

    for (a, b) in levels_iter.tuple_windows() {
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
        let diff = a.abs_diff(*b);
        if diff > MAX_DIFF || diff == 0 {
            return false;
        }
    }

    println!("{levels:?}");
    true
}

fn part2(input: &str) -> usize {
    let mut lines_ok = 0;
    let reports: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    for report in reports {
        if is_valid_report(&report) {
            lines_ok += 1;
        } else {
            println!("failed w/ {report:?}");
            for idx_to_remove in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(idx_to_remove);
                println!("Trying w/ {new_report:?}");
                if is_valid_report(&new_report) {
                    lines_ok += 1;
                    break;
                }
            }
        }
    }
    lines_ok
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, 4);
    }
}
