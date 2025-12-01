use itertools::Itertools;
use tracing::{debug, error, info, instrument};

struct Dial {
    position: isize,
}

impl Dial {
    fn new(start_position: isize) -> Self {
        Self {
            position: start_position,
        }
    }
    fn rotate_left(&mut self, nb: isize) {
        self.position = self.position - nb;
        if self.position < 0 {
            self.position = (100 + self.position) % 100;
        }
    }
    fn rotate_right(&mut self, nb: isize) {
        self.position = (self.position + nb) % 100
    }
    fn position(&self) -> isize {
        self.position
    }
}

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
    let mut dial = Dial::new(50);
    for line in input.lines() {
        match line.chars().nth(0).unwrap() {
            'R' => dial.rotate_right(line[1..].parse().unwrap()),
            'L' => dial.rotate_left(line[1..].parse().unwrap()),
            c => panic!("Unknown char : {c}"),
        }
        if dial.position() == 0 {
            res += 1;
        } else if dial.position() >= 100 {
            panic!("dial.position()={}", dial.position())
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
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        );
        assert_eq!(result, 3);
    }
}
