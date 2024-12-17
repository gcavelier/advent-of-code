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
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}
#[derive(Debug)]
struct Cpu {
    ip: usize, // instruction pointer
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
    fn execute(&mut self) {
        let (instruction, value) = &self.code[self.ip];
        info!("instruction: {instruction:?} value: {value}");
        match instruction {
            Instruction::Adv => {
                let numerator = self.reg_a;
                let denominator = 2_u32.pow(self.get_combo_operand(value) as u32);
                self.reg_a = numerator.div_euclid(denominator as usize);
                info!("Dividing {numerator} by {denominator} => {}", self.reg_a);
                self.ip += 1;
            }
            Instruction::Bxl => {
                self.reg_b = self.reg_b ^ (*value as usize);
                self.ip += 1;
            }
            Instruction::Bst => {
                self.reg_b = ((self.get_combo_operand(value) as u32) % 8) as usize;
                self.ip += 1;
            }
            Instruction::Jnz => {
                if self.reg_a == 0 {
                    self.ip += 1;
                } else {
                    self.ip = *value as usize;
                }
            }
            Instruction::Bxc => {
                self.reg_b = self.reg_b ^ self.reg_c;
                self.ip += 1;
            }
            Instruction::Out => {
                let out = self.get_combo_operand(value) % 8;
                if self.output.len() > 0 {
                    self.output.push(',');
                }
                self.output.push(format!("{out}").chars().next().unwrap());
                self.ip += 1;
            }
            Instruction::Bdv => todo!(),
            Instruction::Cdv => {
                let numerator = self.reg_a;
                let denominator = 2_u32.pow(self.get_combo_operand(value) as u32);
                self.reg_c = numerator.div_euclid(denominator as usize);
                info!("Dividing {numerator} by {denominator} => {}", self.reg_c);
                self.ip += 1;
            }
        }
    }
    fn get_combo_operand(&self, value: &u8) -> usize {
        match value {
            0..=3 => *value as usize,
            4 => self.reg_a as usize,
            5 => self.reg_b as usize,
            6 => self.reg_c as usize,
            _ => unreachable!(),
        }
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

    let mut program_input_iter = input_iter.next().unwrap().split_whitespace();
    program_input_iter.next().unwrap(); // We discard this ("Program: ")
    let bytes: Vec<u8> = program_input_iter
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();
    for i in 0..bytes.len() {
        if i & 1 == 1 {
            let instruction = match bytes[i - 1] {
                0 => Instruction::Adv,
                1 => Instruction::Bxl,
                2 => Instruction::Bst,
                3 => Instruction::Jnz,
                4 => Instruction::Bxc,
                5 => Instruction::Out,
                6 => Instruction::Bdv,
                7 => Instruction::Cdv,
                _ => unreachable!(),
            };
            let value = bytes[i];
            cpu.code.push((instruction, value));
        }
    }

    cpu
}

#[instrument(skip_all)]
fn part1(input: &str) -> String {
    let mut cpu = parse(input);
    while !cpu.halted() {
        cpu.execute();
    }
    cpu.output
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
