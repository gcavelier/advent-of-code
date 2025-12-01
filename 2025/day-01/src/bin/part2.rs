use itertools::Itertools;
use tracing::{debug, error, info, instrument};

#[derive(Debug)]
struct Dial {
    position: isize,
    nb_zeros: usize,
}

impl Dial {
    fn new(start_position: isize) -> Self {
        Self {
            position: start_position,
            nb_zeros: 0,
        }
    }
    #[instrument]
    fn rotate_left(&mut self, nb: isize) {
        self.nb_zeros += (nb / 100) as usize;
        let nb = nb % 100;
        self.position -= nb;
        if self.position < 0 {
            self.position = 100 - self.position.abs();
            self.nb_zeros += 1;
        }
    }
    #[instrument]
    fn rotate_right(&mut self, nb: isize) {
        self.nb_zeros += (nb / 100) as usize;
        let nb = nb % 100;
        self.position = self.position + nb;
        if self.position > 99 {
            self.position = self.position % 100;
            self.nb_zeros += 1;
        }
    }
    fn nb_zeros(&self) -> usize {
        self.nb_zeros
    }
}

#[instrument(skip_all)]
fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input.txt");
    let output = part2(input);
    info!(output);
}

#[instrument(skip_all)]
fn part2(input: &str) -> usize {
    let mut dial = Dial::new(50);
    for line in input.lines() {
        match line.chars().nth(0).unwrap() {
            'R' => dial.rotate_right(line[1..].parse().unwrap()),
            'L' => dial.rotate_left(line[1..].parse().unwrap()),
            c => panic!("Unknown char : {c}"),
        }
    }
    dial.nb_zeros()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        tracing_subscriber::fmt::init();
        let result = part2(
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
        assert_eq!(result, 6);
        // 6913 => nope!
        // 7023 => nope!
        // 6925 => nope!
        // 6963 => nope!
        // 7129 => nope!
        // 6571 => too low
        // 4426 => too low
        // 3033 => too low
    }
}
