use itertools::Itertools;
use tracing::{debug, error, info, instrument};

const WIDTH: usize = 5;

#[instrument(skip_all)]
fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input.txt");
    let output = part1(input);
    info!(output);
}

#[instrument(skip_all)]
fn part1(input: &str) -> usize {
    let mut is_lock = false;
    for x in input.split("\n\n") {
        let mut tmp_arr = [0; WIDTH];
        for line in x.lines() {
            if line == "#####" {
                is_lock = true;
                continue;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        let result = part1(
            "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####",
        );
        assert_eq!(result, 3);
    }
}
