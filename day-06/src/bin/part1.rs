use itertools::{iproduct, Itertools};
use tracing::{debug, error, info};

fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input1.txt");
    let output = part1(input);
    info!(output);
}

fn find_guard(grid: &Vec<Vec<char>>) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .find_map(|(idx, line)| {
            if let Some(y) = line.iter().position(|val| *val == '^') {
                Some((idx, y))
            } else {
                None
            }
        })
        .unwrap()
}

fn part1(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut guard = find_guard(&grid);
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        let result = part1(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
        assert_eq!(result, 41);
    }
}
