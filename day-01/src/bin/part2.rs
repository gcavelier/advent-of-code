fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    let mut res = String::new();

    input
        .lines()
        .map(|line| {
            line.replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .map(|line| {
            let digit1_idx = line.find(|c: char| c.is_ascii_digit()).unwrap();
            let digit2_idx = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
            res = format!(
                "{}{}",
                line.chars().nth(digit1_idx).unwrap(),
                line.chars().nth(digit2_idx).unwrap()
            );
            res.parse::<usize>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}
