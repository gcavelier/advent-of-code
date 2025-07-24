use itertools::Itertools;
use regex::Regex;
use tracing::{debug, error, info};

enum Inst {
    Do,
    Dont,
}
fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input2.txt");
    let output = part2(input);
    info!(output);
}

fn part2(input: &str) -> usize {
    let mut inst = Inst::Do;
    let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\))").unwrap();

    re.find_iter(input)
        //.inspect(|m| info!("{}", m.as_str()))
        .map(|m| {
            let match_str = m.as_str();
            match match_str {
                "do()" => {
                    inst = Inst::Do;
                    0
                }
                "don't()" => {
                    inst = Inst::Dont;
                    0
                }
                _ => match inst {
                    Inst::Do => {
                        let mut nb_iter = match_str
                            .strip_prefix("mul(")
                            .unwrap()
                            .strip_suffix(")")
                            .unwrap()
                            .split(",");
                        let a = nb_iter.next().unwrap().parse::<usize>().unwrap();
                        let b = nb_iter.next().unwrap().parse::<usize>().unwrap();
                        a * b
                    }
                    Inst::Dont => 0,
                },
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        tracing_subscriber::fmt::init();
        let result =
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, 48);
    }
}
