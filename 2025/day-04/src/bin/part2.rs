use itertools::Itertools;
use pathfinding::grid::{self, Grid};
use tracing::{debug, error, info, instrument};

#[instrument(skip_all)]
fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input.txt");
    let output = part1(input);
    info!(output);
}

#[instrument(skip_all)]
fn part1(input: &str) -> usize {
    let mut res = 0;

    let mut grid = Grid::new(
        input.lines().nth(0).unwrap().chars().count(),
        input.lines().count(),
    );
    grid.enable_diagonal_mode();

    for (line_idx, line) in input.lines().enumerate() {
        for (char_idx, char_value) in line.chars().enumerate() {
            if char_value == '@' {
                grid.add_vertex((char_idx, line_idx));
            }
        }
    }

    loop {
        let mut rolls_to_remove = Vec::new();

        for roll in grid.iter() {
            if grid.neighbours(roll).iter().count() < 4 {
                res += 1;
                rolls_to_remove.push(roll);
            }
        }

        // remove found rolls
        for vertex in rolls_to_remove.iter() {
            grid.remove_vertex(*vertex);
        }

        if rolls_to_remove.is_empty() {
            break;
        }
    }

    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        let result = part1(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        );
        assert_eq!(result, 43);
    }
}
