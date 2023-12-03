fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let mut res = String::new();

    input
        .lines()
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
    fn part1_works() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
