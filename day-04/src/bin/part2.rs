use itertools::Itertools;
use tracing::{debug, error, info};

fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input2.txt");
    let output = part2(input);
    info!(output);
}

fn x_mas_count(
    data: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    max_x: usize,
    max_y: usize,
) -> usize {
    if start_x < 1 || start_y < 1 || start_x + 1 >= max_x || start_y + 1 >= max_y {
        return 0;
    }

    info!("Starting {start_x}, {start_y}");
    // check first diagonal
    if (data[start_x - 1][start_y - 1] == 'M' && data[start_x + 1][start_y + 1] == 'S')
        || (data[start_x - 1][start_y - 1] == 'S' && data[start_x + 1][start_y + 1] == 'M')
    {
        // check second diagonal
        if (data[start_x - 1][start_y + 1] == 'M' && data[start_x + 1][start_y - 1] == 'S')
            || (data[start_x - 1][start_y + 1] == 'S' && data[start_x + 1][start_y - 1] == 'M')
        {
            return 1;
        }
    }

    0
}

fn part2(input: &str) -> usize {
    let mut sum = 0;
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let max_x = data[0].len();
    let max_y = data.len();

    info!("max_x={max_x} max_y={max_y}");

    // For each position, we are going to find the number of "XMAS" found
    for x in 0..max_x {
        for y in 0..max_y {
            //info!("{x}, {y}");
            if data[x][y] == 'A' {
                sum += x_mas_count(&data, x, y, max_x, max_y);
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        tracing_subscriber::fmt::init();
        let result = part2(
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
        assert_eq!(result, 9);
    }
}
