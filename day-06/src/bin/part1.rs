use std::collections::HashSet;

use itertools::{iproduct, Itertools};
use tracing::{debug, error, info};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
struct Guard {
    x: isize,
    y: isize,
    direction: Direction,
    max_x: isize,
    max_y: isize,
    positions: HashSet<(isize, isize)>,
}

impl Guard {
    fn in_grid(&self) -> bool {
        self.x < self.max_x && self.y < self.max_y && self.x >= 0 && self.y >= 0
    }
    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.x -= 1,
            Direction::Right => self.y += 1,
            Direction::Down => self.x += 1,
            Direction::Left => self.y -= 1,
        }
        self.positions.insert((self.x, self.y));
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input1.txt");
    let output = part1(input);
    info!(output);
}

fn obstacle_in_front(grid: &Vec<Vec<char>>, guard: &Guard) -> bool {
    let mut x = guard.x;
    let mut y = guard.y;
    match guard.direction {
        Direction::Up => {
            if x == 0 {
                return false;
            } else {
                x -= 1
            }
        }
        Direction::Down => x += 1,
        Direction::Left => {
            if y == 0 {
                return false;
            } else {
                y -= 1
            }
        }
        Direction::Right => y += 1,
    }
    if x < guard.max_x && y < guard.max_y {
        grid[x as usize][y as usize] == '#'
    } else {
        false
    }
}
fn find_guard(grid: &Vec<Vec<char>>) -> Guard {
    let pos = grid
        .iter()
        .enumerate()
        .find_map(|(idx, line)| {
            if let Some(y) = line.iter().position(|val| *val == '^') {
                Some((idx as isize, y as isize))
            } else {
                None
            }
        })
        .unwrap();
    let mut guard = Guard {
        x: pos.0 as isize,
        y: pos.1 as isize,
        direction: Direction::Up,
        max_x: grid.len() as isize,
        max_y: grid.first().unwrap().len() as isize,
        positions: HashSet::new(),
    };
    guard.positions.insert(pos);
    guard
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut guard = find_guard(&grid);
    while guard.in_grid() {
        //info!("{},{}", guard.x, guard.y);
        if obstacle_in_front(&grid, &guard) {
            guard.turn_right()
        }
        guard.move_forward();
    }
    guard.positions.len() - 1
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
