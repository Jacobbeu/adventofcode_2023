use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::error::Error;
use regex::Regex;

pub fn solve() -> SolutionPair {

    let contents = read_to_string("input/day02_input.txt").expect("failed to parse input file");
    let game = Game { red: 12, green: 13, blue: 14 };

    let mut lines = contents.lines();

    let mut solution1 = 0;
    let mut solution2 = 0;

    while let Some(line) = lines.next() {
        let result = calculate_id_sum(&line, &game).expect("Failed to parse games.");
        solution1 += result;
    }

    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        let result = calculate_power(&line).expect("Failed to parse games.");
        solution2 += result;
    }

    (Solution::from(solution1), Solution::from(solution2))
}

#[derive(Debug)]
pub struct Game {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Game {
    fn max(&mut self, other: &Self) {
        self.red = std::cmp::max(self.red, other.red);
        self.blue = std::cmp::max(self.blue, other.blue);
        self.green = std::cmp::max(self.green, other.green);
    }

    fn can_fit(&self, other: &Self) -> bool {
        self.red >= other.red &&
        self.blue >= other.blue &&
        self.green >= other.green
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red &&
        self.green == other.green &&
        self.blue == other.blue
    }
}

fn calculate_id_sum(data: &str, game: &Game) -> Result<u32, Box<dyn Error>> {

    let mut total: u32 = 0;

    for datam in data.lines() {
        let result = parse_line(&datam, game)?;
        total = total + result;
    }
    
    Ok(total)
}

fn calculate_power(data: &str) -> Result<u32, Box<dyn Error>> {
    let mut total: u32 = 0;

    for datam in data.lines() {
        let result = parse_round(datam)?;
        let power: u32 = &result.red * &result.green * &result.blue;
        total = total + power;
    }

    Ok(total)
}

/// Returns the Game's ID if it can be contained in max_game.
/// Returns 0 otherwise
fn parse_line(data: &str, max_game: &Game) -> Result<u32, Box<dyn Error>> {
    let regex = Regex::new(r"(?m)^Game (\d+): (.*)$")?;

    let result = regex.captures_iter(data).next().unwrap();
    let id = result.get(1).unwrap().as_str().parse().unwrap_or(0);
    let game = parse_round(result.get(2).unwrap().as_str())?;

    if max_game.can_fit(&game) {
        Ok(id)
    }
    else {
        Ok(0)
    }
}

fn parse_round(data: &str) -> Result<Game, Box<dyn Error>> {
    let regex = Regex::new(r"(?m)(.*?;)|(.+?)$")?;

    let mut ret_game = Game { red: 0, blue: 0, green: 0 };

    let result = regex.captures_iter(data);

    for mats in result {
        if let Some(mat) = mats.get(0) {
            let game = parse_game(mat.as_str())?;
            ret_game.max(&game);
        }
    }

    Ok(ret_game)
}

fn parse_game(data: &str) -> Result<Game, Box<dyn Error>> {
    let regex = Regex::new(r"(?m)(\d+) (red|blue|green)")?;

    let mut ret_game = Game { red: 0, blue: 0, green: 0 };

    let results: Vec<(&str, &str)> = regex.captures_iter(data).map(|caps| {
        let (_, [count, color]) = caps.extract();
        (count, color)
    }).collect();

    let results_iter = results.iter();

    for item in results_iter {
        if item.1.eq("blue") {
            ret_game.blue = item.0.parse().unwrap_or(0);
        }

        if item.1.eq("green") {
            ret_game.green = item.0.parse().unwrap_or(0);
        }

        if item.1.eq("red") {
            ret_game.red = item.0.parse().unwrap_or(0);
        }
    }

    Ok(ret_game)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game_test() {
        let test1 = String::from(" 3 blue, 4 red;");
        let test2 = String::from(" 1 red, 2 green, 6 blue;");
        let test3 = String::from(" 2 green");

        let result1 = parse_game(&test1).expect("Game did not parse correctly");
        let result2 = parse_game(&test2).expect("Game did not parse correctly");
        let result3 = parse_game(&test3).expect("Game did not parse correctly");

        assert_eq!(result1, Game { red: 4, green: 0, blue: 3});
        assert_eq!(result2, Game { red: 1, green: 2, blue: 6});
        assert_eq!(result3, Game { red: 0, green: 2, blue: 0});
    }

    #[test]
    fn parse_round_test() {
        let source = String::from(" 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
         1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
         8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
         1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
         6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");

         let mut tests = source.lines();

         let result = parse_round(tests.next().unwrap()).expect("Round did not parse correctly");
         assert_eq!(Game { red: 4, blue: 6, green: 2 }, result);
         let result = parse_round(tests.next().unwrap()).expect("Round did not parse correctly");
         assert_eq!(Game { red: 1, blue: 4, green: 3 }, result);
         let result = parse_round(tests.next().unwrap()).expect("Round did not parse correctly");
         assert_eq!(Game { red: 20, blue: 6, green: 13 }, result);
         let result = parse_round(tests.next().unwrap()).expect("Round did not parse correctly");
         assert_eq!(Game { red: 14, blue: 15, green: 3 }, result);
         let result = parse_round(tests.next().unwrap()).expect("Round did not parse correctly");
         assert_eq!(Game { red: 6, blue: 2, green: 3 }, result);
    }

    #[test]
    fn parse_line_test() {
        let source = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");

         let mut tests = source.lines();
         let mut total: u32 = 0;
         let max_game = Game { red: 12, green: 13, blue: 14 };

         let result = parse_line(tests.next().unwrap(), &max_game).expect("Round did not parse correctly");
         total = total + result;
         assert_eq!(total, 1);
         let result = parse_line(tests.next().unwrap(), &max_game).expect("Round did not parse correctly");
         total = total + result;
         assert_eq!(total, 3);
         let result = parse_line(tests.next().unwrap(), &max_game).expect("Round did not parse correctly");
         total = total + result;
         assert_eq!(total, 3);
         let result = parse_line(tests.next().unwrap(), &max_game).expect("Round did not parse correctly");
         total = total + result;
         assert_eq!(total, 3);
         let result = parse_line(tests.next().unwrap(), &max_game).expect("Round did not parse correctly");
         total = total + result;
         assert_eq!(total, 8);
    }

    #[test]
    fn parse_power_test() {
        let source = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");

         let mut tests = source.lines();
         let mut total: u32 = 0;

         let result = parse_round(tests.next().unwrap()).expect("Round did not parse correctly");
         let power = &result.red * &result.green * &result.blue;
         total = total + power;
         assert_eq!(power, 48);
         assert_eq!(total, 48);
         let result = parse_round(tests.next().unwrap()).expect("Round did not parse correctly");
         let power = &result.red * &result.green * &result.blue;
         total = total + power;
         assert_eq!(power, 12);
         assert_eq!(total, 60);
         let result = parse_round(tests.next().unwrap()).expect("Round did not parse correctly");
         let power = &result.red * &result.green * &result.blue;
         total = total + power;
         assert_eq!(power, 1560);
         assert_eq!(total, 1620);
         let result = parse_round(tests.next().unwrap()).expect("Round did not parse correctly");
         let power = &result.red * &result.green * &result.blue;
         total = total + power;
         assert_eq!(power, 630);
         assert_eq!(total, 2250);
         let result = parse_round(tests.next().unwrap()).expect("Round did not parse correctly");
         let power = &result.red * &result.green * &result.blue;
         total = total + power;
         assert_eq!(power, 36);
         assert_eq!(total, 2286);
    }

    #[test]
    fn example() {
        let example = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");

        let example_game = Game {
            red: 12, 
            green: 13,
            blue: 14, 
        };

        let result = calculate_id_sum(example.as_str(), &example_game).expect("Failed to parse games");

        assert_eq!(8, result);
    }
}