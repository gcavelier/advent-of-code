use std::collections::HashMap;

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
enum Gate {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Computation {
    input1: String,
    input2: String,
    gate: Gate,
    output: String,
}

#[instrument(skip_all)]
fn compute(computation: &Computation, wire_values: &mut HashMap<String, u8>) -> Option<u8> {
    //info!(?computation);
    match (
        wire_values.get(&computation.input1),
        wire_values.get(&computation.input2),
    ) {
        (Some(a), Some(b)) => match computation.gate {
            Gate::And => {
                if *a == 1 && *b == 1 {
                    Some(1)
                } else {
                    Some(0)
                }
            }
            Gate::Or => {
                if *a == 0 && *b == 0 {
                    Some(0)
                } else {
                    Some(1)
                }
            }
            Gate::Xor => {
                if a == b {
                    Some(0)
                } else {
                    Some(1)
                }
            }
        },
        _ => return None,
    }
}

#[instrument(skip_all)]
fn part1(input: &str) -> u64 {
    let mut wire_values: HashMap<String, u8> = HashMap::new();
    let mut computations: Vec<Computation> = Vec::new();
    let mut later_computations: Vec<Computation> = Vec::new();
    let mut input_iter = input.split("\n\n");

    // Initialize {wire_value}
    for x in input_iter.next().unwrap().lines() {
        let mut a_iter = x.split(": ");
        wire_values.insert(
            a_iter.next().unwrap().to_string(),
            a_iter.next().unwrap().parse().unwrap(),
        );
    }

    // Initialize {computations}
    for x in input_iter.next().unwrap().lines() {
        let mut x_iter = x.split_whitespace();
        let input1 = x_iter.next().unwrap().to_string();
        let gate = match x_iter.next().unwrap() {
            "XOR" => Gate::Xor,
            "OR" => Gate::Or,
            "AND" => Gate::And,
            _ => unreachable!(),
        };
        let input2 = x_iter.next().unwrap().to_string();
        x_iter.next(); // We don't need this ("->")
        let output = x_iter.next().unwrap().to_string();
        computations.push(Computation {
            input1,
            input2,
            gate,
            output,
        });
    }

    while !computations.is_empty() {
        //info!("Let's compute! {}", computations.len());
        later_computations = computations
            .into_iter()
            .filter_map(|computation| {
                let res = compute(&computation, &mut wire_values);
                if let Some(val) = res {
                    wire_values.insert(computation.output.to_string(), val);
                    None
                } else {
                    Some(computation)
                }
            })
            .collect();
        computations = later_computations;
    }

    let mut res: u64 = 0;
    wire_values
        .iter()
        .filter(|(k, _v)| k.starts_with('z'))
        .sorted()
        .enumerate()
        .map(|(idx, (_k, v))| {
            //info!("res={res} idx={idx} {v}");
            res += (*v as u64) << idx;
        })
        .count();

    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        let result = part1(
            "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj",
        );
        assert_eq!(result, 2024);
    }
}
