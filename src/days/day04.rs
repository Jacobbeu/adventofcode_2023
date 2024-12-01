use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::error::Error;

pub fn solve() -> SolutionPair {
    let contents = read_to_string("input/day04_input.txt").expect("failed to parse input file");

    let solution1 = calculate_points(&contents).expect("Failed to parse points.");
    let solution2 = calculate_games(&contents).expect("Failed to parse games");

    (Solution::from(solution1), Solution::from(solution2))
}

struct ScratchGame {
    copies: u32,
    number_of_wins: u32,
}

fn calculate_games(scratch_cards: &str) -> Result<usize, Box<dyn Error>> {

    let lines = scratch_cards.lines();

    let mut games = Vec::new();

    for line in lines {
        let scratch_components = split_card(line)?;
        let winning_numbers = parse_digits(scratch_components.1)?;
        let your_numbers = parse_digits(scratch_components.2)?;

        let games_won = get_number_of_wins(&winning_numbers, &your_numbers);
        let game = ScratchGame { copies: 1, number_of_wins: games_won };
        games.push(game);
    }

    for counter in 0..games.len() {

        let split_games = &mut games.split_at_mut(counter + 1);
        let game = &split_games.0[counter];

        if game.number_of_wins == 0 {
            continue;
        }

        let wins = usize::try_from(game.number_of_wins).unwrap();

        for idx in 0..wins {
            if let Some(next_game) = split_games.1.get_mut(idx) {
                next_game.copies = next_game.copies + game.copies;
            }
        }
    }

    let result: u32 = games.iter()
        .map(|g| g.copies)
        .sum();

    Ok(usize::try_from(result).unwrap())
}

fn calculate_points(scratch_cards: &str) -> Result<usize, Box<dyn Error>> {

    let lines = scratch_cards.lines();

    let mut total_points = 0;

    for line in lines {
        let scratch_components = split_card(line)?;
        let winning_numbers = parse_digits(scratch_components.1)?;
        let your_numbers = parse_digits(scratch_components.2)?;

        let games_won = get_number_of_wins(&winning_numbers, &your_numbers);
        if games_won > 0 {
            total_points = total_points + (2usize.pow(games_won-1));
        }
    }

    Ok(total_points)
}

fn get_number_of_wins(winning_numbers: &Vec<usize>, your_numbers: &Vec<usize>) -> u32 {

    let mut games_won = 0;

    for winning_number in winning_numbers.iter() {
        for your_number in your_numbers.iter() {
            if your_number == winning_number {
                games_won = games_won + 1;
                break;
            }
        }
    }

    games_won
}

fn split_card(scratch_card: &str) -> Result<(&str, &str, &str), &'static str> {

    let colon_idx = match scratch_card.find(":") {
        Some(val) => val,
        None => return Err("Invalid scratch card, no colon."),
    };

    let pipe_idx = match scratch_card.find("|") {
        Some(val) => val,
        None => return Err("Invalid scratch card, no pipe."),
    };

    Ok((&scratch_card[0..colon_idx], &scratch_card[colon_idx+2..pipe_idx-1], &scratch_card[pipe_idx+2..]))
}

fn parse_digits(digits: &str) -> Result<Vec<usize>, Box<dyn Error>> {

    let digit_items = digits.split(" ");

    let mut result = Vec::new();

    for digit in digit_items {
        if digit.eq("") {
            continue;
        }

        result.push(digit.parse::<usize>()?);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_split_card() {
        let example = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = split_card(&example).expect("Invalid Parse");

        assert_eq!(result.0, "Card 1");
        assert_eq!(result.1, "41 48 83 86 17");
        assert_eq!(result.2, "83 86  6 31 17  9 48 53");
    }

    #[test]
    fn example_parse_digits() {
        let example = "41 48 83 86 17";
        let result = parse_digits(&example).expect("Invalid Parse");

        assert_eq!(result, vec![41, 48, 83, 86, 17]);

        let example = "83 86  6 31 17  9 48 53";
        let result = parse_digits(&example).expect("Invalid Parse");

        assert_eq!(result, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn example() {
        let example = 
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";


        let result = calculate_points(example).expect("Part Number failed to parse");

        assert_eq!(13, result);
    }

    #[test]
    fn example_two() {
        let example = 
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";


        let result = calculate_games(example).expect("Part Number failed to parse");

        assert_eq!(30, result);
    }
}