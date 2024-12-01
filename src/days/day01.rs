use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

pub fn solve() -> SolutionPair {

    let contents = read_to_string("input/day01_input.txt").expect("failed to parse input file");

    let mut lines = contents.lines();
    
    let mut solution1 = 0;
    let mut solution2 = 0;

    while let Some(line) = lines.next() {
        let result = get_outside_digits(&line);
        let result_val = result.0 * 10 + result.1;
        solution1 += result_val;
    }

    let mut lines = contents.lines();

    while let Some(line) = lines.next() {
        let result = get_outside_digits_including_words(&line);
        let result_val = result.0 * 10 + result.1;
        solution2 += result_val;
    }

    (Solution::from(solution1), Solution::from(solution2))
}

fn get_outside_digits(line: &str) -> (u32, u32) {
    let mut first_digit = 0;
    let mut last_digit = 0;

    let mut chars = line.chars();

    while let Some(c) = chars.next() {
        match c.to_digit(10) {
            Some(val) => {
                first_digit = val;
                break;
            },
            None => continue,
        }
    }

    let mut chars = line.chars().rev();

    while let Some(c) = chars.next() {
        match c.to_digit(10) {
            Some(val) => {
                last_digit = val;
                break;
            },
            None => continue,
        }
    }

    (first_digit, last_digit)
}

fn get_outside_digits_including_words(line: &str) -> (u32, u32) {

    let mut new_line = String::from(line);
    let numbers = vec!["zero","one","two","three","four","five","six","seven","eight","nine"];
    let chars = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for i in 0..numbers.len() {        
        match new_line.find(numbers[i]) {
            Some(idx) => new_line.insert(idx + 1, chars[i]),
            _ => (),
        }
        match new_line.rfind(numbers[i]) {
            Some(idx) => new_line.insert(idx + 1, chars[i]),
            _ => (),
        }
    }

    get_outside_digits(&new_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let source = String::from("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        let mut total: u32 = 0;
        let mut lines = source.lines();
        let answers = vec![12, 38, 15, 77];
        let rolling_total = vec![12, 50, 65, 142];
        let mut counter = 0;
        
        while let Some(line) = lines.next() {            
            let result = get_outside_digits(&line);
            let result_value = result.0 * 10 + result.1;
            total = total + result_value;
            assert_eq!(result_value, answers[counter]);
            assert_eq!(total, rolling_total[counter]);
            counter += 1;
        }
    }

    #[test]
    fn example_2() {

        let source = String::from("two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen");
        let mut total: u32 = 0;
        let mut lines = source.lines();
        let answers = vec![29, 83, 13, 24, 42, 14, 76];
        let rolling_total = vec![29, 112, 125, 149, 191, 205, 281];
        let mut counter = 0;
        
        while let Some(line) = lines.next() {            
            let result = get_outside_digits_including_words(&line);
            let result_value = result.0 * 10 + result.1;
            total = total + result_value;
            assert_eq!(result_value, answers[counter]);
            assert_eq!(total, rolling_total[counter]);
            counter += 1;
        }
    }
}