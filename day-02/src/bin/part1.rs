use regex::Regex;

const NB_RED_CUBES: usize = 12;
const NB_GREEN_CUBES: usize = 13;
const NB_BLUE_CUBES: usize = 14;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let re = Regex::new(r"Game (?<game>\d+): (?<games>.*)$").unwrap(); // Game (\d+): (.*)$
            let caps = re.captures(line).unwrap();
            let game_number: usize = caps["game"].parse().unwrap();
            let mut res: usize = game_number;
            let games: Vec<&str> = caps["games"].split(";").collect();

            for game in games {
                let red_re = Regex::new(r"(?<red>\d+) red").unwrap();
                let green_re = Regex::new(r"(?<green>\d+) green").unwrap();
                let blue_re = Regex::new(r"(?<blue>\d+) blue").unwrap();
                let nb_red: usize = red_re
                    .captures(game)
                    .map_or_else(|| 0, |r| r["red"].parse().unwrap());
                let nb_green: usize = green_re
                    .captures(game)
                    .map_or_else(|| 0, |r| r["green"].parse().unwrap());
                let nb_blue: usize = blue_re
                    .captures(game)
                    .map_or_else(|| 0, |r| r["blue"].parse().unwrap());
                if nb_red > NB_RED_CUBES || nb_green > NB_GREEN_CUBES || nb_blue > NB_BLUE_CUBES {
                    res = 0;
                    break;
                }
            }
            res
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8);
    }
}
