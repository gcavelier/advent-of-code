use itertools::Itertools;
use tracing::{debug, error, info, instrument};

#[instrument(skip_all)]
fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input.txt");
    let output = part1(input);
    info!(output);
}

#[derive(Debug)]
enum Instruction {}
#[derive(Debug)]
struct Cpu {
    ip: usize,
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    output: String,
    code: Vec<(Instruction, u8)>,
}

impl Cpu {
    fn halted(&self) -> bool {
        self.ip >= self.code.len()
    }
}

#[instrument(skip_all)]
fn parse(input: &str) -> Cpu {
    let mut cpu = Cpu {
        ip: 0,
        reg_a: 0,
        reg_b: 0,
        reg_c: 0,
        output: String::new(),
        code: Vec::new(),
    };

    let mut input_iter = input.split("\n\n");
    for reg in input_iter.next().unwrap().lines() {
        let mut reg_iter = reg.split_whitespace();
        reg_iter.next(); // We discard this
        match reg_iter.next().unwrap() {
            "A:" => cpu.reg_a = reg_iter.next().unwrap().parse().unwrap(),
            "B:" => cpu.reg_b = reg_iter.next().unwrap().parse().unwrap(),
            "C:" => cpu.reg_c = reg_iter.next().unwrap().parse().unwrap(),
            _ => unreachable!(),
        }
    }

    dbg!(&cpu);
    let program_input = input_iter.next().unwrap();

    cpu
}

#[instrument(skip_all)]
fn part1(input: &str) -> String {
    let cpu = parse(input);
    String::new()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        let result = part1(
            "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
        );
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }
}
