use itertools::Itertools;
use tracing::{debug, error, info, instrument};

const WORD_TO_FIND: &str = "XMAS";
const WORD_TO_FIND_LEN: usize = WORD_TO_FIND.len();

fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input1.txt");
    let output = part1(input);
    info!(output);
}

// Looking for WORD_TO_FIND horizontaly, to the left (backwards)
#[instrument(skip_all)]
fn hleft(
    data: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    _max_x: usize,
    _max_y: usize,
) -> usize {
    //info!("Starting {start_x}, {start_y}");

    if start_y < WORD_TO_FIND_LEN - 1 {
        return 0;
    } else {
        let mut idx: usize = 0;
        //error!("{start_y} {}", start_y + 1 - WORD_TO_FIND_LEN);
        for y in ((start_y + 1 - WORD_TO_FIND_LEN)..=start_y).rev() {
            if data[start_x][y] != WORD_TO_FIND.as_bytes()[idx] as char {
                return 0;
            }
            idx += 1;
        }
        info!("Found at {start_x}, {start_y}");
    }
    1
}

#[instrument(skip_all)]
fn hright(
    data: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    _max_x: usize,
    max_y: usize,
) -> usize {
    //info!("Starting {start_x}, {start_y}");

    if start_y + WORD_TO_FIND_LEN > max_y {
        return 0;
    } else {
        let mut idx: usize = 0;
        //error!("{start_y} {}", start_y + 1 - WORD_TO_FIND_LEN);
        for y in start_y..start_y + WORD_TO_FIND_LEN {
            if data[start_x][y] != WORD_TO_FIND.as_bytes()[idx] as char {
                return 0;
            }
            idx += 1;
        }
        info!("Found at {start_x}, {start_y}");
    }
    1
}

#[instrument(skip_all)]
fn up(
    data: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    _max_x: usize,
    _max_y: usize,
) -> usize {
    //info!("Starting {start_x}, {start_y}");

    if start_x < WORD_TO_FIND_LEN - 1 {
        return 0;
    }

    for idx in 0..WORD_TO_FIND_LEN {
        if data[start_x - idx][start_y] != WORD_TO_FIND.as_bytes()[idx] as char {
            return 0;
        }
    }

    info!("Found at {start_x}, {start_y}");
    1
}

#[instrument(skip_all)]
fn down(
    data: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    max_x: usize,
    _max_y: usize,
) -> usize {
    //info!("Starting {start_x}, {start_y}");

    if start_x + WORD_TO_FIND_LEN > max_x {
        return 0;
    } else {
        let mut idx: usize = 0;
        for x in start_x..start_x + WORD_TO_FIND_LEN {
            if data[x][start_y] != WORD_TO_FIND.as_bytes()[idx] as char {
                return 0;
            }
            idx += 1;
        }
        info!("Found at {start_x}, {start_y}");
    }
    1
}

#[instrument(skip_all)]
fn diag_ur(
    data: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    _max_x: usize,
    max_y: usize,
) -> usize {
    //info!("Starting {start_x}, {start_y}");
    if start_x < WORD_TO_FIND_LEN - 1 || start_y + WORD_TO_FIND_LEN > max_y {
        return 0;
    }

    for idx in 0..WORD_TO_FIND_LEN {
        if data[start_x - idx][start_y + idx] != WORD_TO_FIND.as_bytes()[idx] as char {
            return 0;
        }
    }
    info!("Found at {start_x}, {start_y}");

    1
}

#[instrument(skip_all)]
fn diag_dr(
    data: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    max_x: usize,
    max_y: usize,
) -> usize {
    //info!("Starting {start_x}, {start_y}");
    if start_x + WORD_TO_FIND_LEN > max_x || start_y + WORD_TO_FIND_LEN > max_y {
        return 0;
    }

    for idx in 0..WORD_TO_FIND_LEN {
        if data[start_x + idx][start_y + idx] != WORD_TO_FIND.as_bytes()[idx] as char {
            return 0;
        }
    }
    info!("Found at {start_x}, {start_y}");

    1
}

#[instrument(skip_all)]
fn diag_dl(
    data: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    max_x: usize,
    _max_y: usize,
) -> usize {
    //info!("Starting {start_x}, {start_y}");
    if start_x + WORD_TO_FIND_LEN > max_x || start_y < WORD_TO_FIND_LEN - 1 {
        return 0;
    }

    for idx in 0..WORD_TO_FIND_LEN {
        if data[start_x + idx][start_y - idx] != WORD_TO_FIND.as_bytes()[idx] as char {
            return 0;
        }
    }
    info!("Found at {start_x}, {start_y}");

    1
}

#[instrument(skip_all)]
fn diag_ul(
    data: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    _max_x: usize,
    _max_y: usize,
) -> usize {
    //info!("Starting {start_x}, {start_y}");
    if start_x < WORD_TO_FIND_LEN - 1 || start_y < WORD_TO_FIND_LEN - 1 {
        return 0;
    }

    for idx in 0..WORD_TO_FIND_LEN {
        if data[start_x - idx][start_y - idx] != WORD_TO_FIND.as_bytes()[idx] as char {
            return 0;
        }
    }
    info!("Found at {start_x}, {start_y}");

    1
}

fn xmas_count(
    data: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    max_x: usize,
    max_y: usize,
) -> usize {
    hleft(data, start_x, start_y, max_x, max_y)
        + hright(data, start_x, start_y, max_x, max_y)
        + up(data, start_x, start_y, max_x, max_y)
        + down(data, start_x, start_y, max_x, max_y)
        + diag_ur(data, start_x, start_y, max_x, max_y)
        + diag_dr(data, start_x, start_y, max_x, max_y)
        + diag_dl(data, start_x, start_y, max_x, max_y)
        + diag_ul(data, start_x, start_y, max_x, max_y)
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let max_x = data[0].len();
    let max_y = data.len();

    info!("max_x={max_x} max_y={max_y}");

    // For each position, we are going to find the number of "XMAS" found
    for x in 0..max_x {
        for y in 0..max_y {
            //info!("{x}, {y}");
            if data[x][y] == 'X' {
                sum += xmas_count(&data, x, y, max_x, max_y);
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        let result = part1(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(result, 18);
    }
}

// 2520 : too low
