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
    let mut locks: Vec<[i32; WIDTH]> = Vec::new();
    let mut keys: Vec<[i32; WIDTH]> = Vec::new();

    for x in input.split("\n\n") {
        let mut tmp_arr = [0; WIDTH];
        let mut is_lock = false;
        let mut filter_char = '#';
        if x.lines().next().unwrap() == "#####" {
            is_lock = true;
            filter_char = '.';
        }
        for line in x.lines() {
            line.chars()
                .enumerate()
                .filter(|(_idx, c)| c == &filter_char)
                .for_each(|(idx, _c)| tmp_arr[idx] += 1);
        }
        if is_lock {
            locks.push(tmp_arr);
        } else {
            keys.push(tmp_arr);
        }
    }

    // info!(?locks);
    // info!(?keys);

    let mut count = 0;
    for lock in locks {
        for key in &keys {
            //info!("lock: {lock:?} key: {key:?}");
            let res = key.iter().zip(lock).all(|(a, b)| *a <= b);
            //info!("==> res: {res}");
            if res == true {
                count += 1;
            }
        }
    }

    count
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
