use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::error::Error;
use regex::Regex;

pub fn solve() -> SolutionPair {
    let contents = read_to_string("input/day03_input.txt").expect("failed to parse input file");

    let result = calculate_part_number(&contents).expect("Failed to parse numbers.");

    (Solution::from(result.0), Solution::from(result.1))
}

fn calculate_part_number(data: &str) -> Result<(usize,  usize), Box<dyn Error>> {
    let regex = Regex::new(r"(?m)(\d+|[^\w\r\n.])").unwrap();
    
    let mut part_numbers = Vec::new();
    let mut symbols = Vec::new();

    let mut line_number = 1;
    let mut lines = data.lines();

    while let Some(line) = lines.next() {
        let result = regex.captures_iter(line);

        for mat in result {
            let cap = mat.get(0).unwrap();
    
            match cap.as_str().parse::<usize>() {
                Ok(val) => part_numbers.push(PartNumber::new(line_number, cap.start()..cap.end(), val)),
                Err(_) => {
                    if cap.as_str().eq("*") {
                        symbols.push(Symbol { line: line_number, position: cap.start(), ratio: Some(GearRatio { part_one: None, part_two: None }) });
                    } else {
                        symbols.push(Symbol { line: line_number, position: cap.start(), ratio: None });
                    }
                },
            }
        }

        line_number = line_number + 1;
    }

    for pn in part_numbers.iter_mut() {
        for s in symbols.iter_mut() {
            pn.is_adjacent(s);
        }
    }

    let result: usize = part_numbers
        .iter()
        .filter(|pn| pn.adjacent_to_symbol)
        .map(|pn| pn.value)
        .sum();

    let gear_ratio: usize = symbols
        .iter()
        .filter(|s| s.ratio.is_some())
        .map(|s| s.ratio.as_ref().unwrap().calculate_ratio())
        .sum();

    println!("{gear_ratio}");

    Ok((result, gear_ratio))
}

#[derive(Debug)]
struct PartNumber {
    line: usize,
    position: std::ops::Range<usize>,
    value: usize,
    adjacent_to_symbol: bool,
}

impl PartNumber {
    fn new(line: usize, position: std::ops::Range<usize>, value: usize) -> Self {
        PartNumber { line, position, value, adjacent_to_symbol: false }
    }

    fn is_adjacent(&mut self, symbol: &mut Symbol) {
        if self.line < symbol.line - 1 ||
        self.line > symbol.line + 1 {
            return;
        }

        let start: isize = symbol.position as isize - 1;
        let start = std::cmp::max(start, 0) as usize;
        let test_range: std::ops::Range<usize> = start..(symbol.position + 1);

        if self.position.contains(&test_range.start) || self.position.contains(&test_range.end) || self.position.contains(&symbol.position) {
            self.adjacent_to_symbol = true;

            symbol.add_part_number(&self);
        }
    }
}

struct Symbol {
    line: usize,
    position: usize,
    ratio: Option<GearRatio>,
}

impl Symbol {
    fn add_part_number(&mut self, part_number: &PartNumber) {
        if self.ratio.is_none() {
            return;
        }

        let gr = self.ratio.as_mut().unwrap();

        if gr.part_one.is_none() {
            gr.part_one = Some(part_number.value);
        }
        else if gr.part_two.is_none() {
            gr.part_two = Some(part_number.value);
        }
        else {
            //more than 2 numbers found so this is now invalid
            self.ratio = None;
        }
    }
}

struct GearRatio {
    part_one: Option<usize>,
    part_two: Option<usize>,
}

impl GearRatio {
    fn calculate_ratio(&self) -> usize {
        if self.part_one.is_none() || self.part_two.is_none() {
            0
        }
        else {
            self.part_one.unwrap() * self.part_two.unwrap()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_number_is_adjacent() {
        let mut part1 = PartNumber::new(0, 33..35, 16);
        let mut part2 = PartNumber::new(1, 36..40, 1616);
        let mut part3 = PartNumber::new(2, 32..34, 16);
        let mut part4 = PartNumber::new(3, 33..35, 16);
        let mut symbol = Symbol { line: 2, position: 35, ratio: None };

        part1.is_adjacent(&mut symbol);
        part2.is_adjacent(&mut symbol);
        part3.is_adjacent(&mut symbol);
        part4.is_adjacent(&mut symbol);
        assert_eq!(false, part1.adjacent_to_symbol);
        assert_eq!(true, part2.adjacent_to_symbol);
        assert_eq!(false, part3.adjacent_to_symbol);
        assert_eq!(true, part4.adjacent_to_symbol);
    }
    
    #[test]
    fn part_number_is_adjacent_2() {
        let mut part1 = PartNumber::new(6, 11..12, 9);
        let mut symbol = Symbol { line: 7, position: 11, ratio: None };

        part1.is_adjacent(&mut symbol);
        assert_eq!(true, part1.adjacent_to_symbol);
    }

    #[test]
    fn day_three_example() {
        let example = String::from("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");


        let result = calculate_part_number(example.as_str()).expect("Part Number failed to parse");

        assert_eq!(4361, result.0);
        assert_eq!(467835, result.1);
    }

    #[test]
    fn example_2() {
        let example = String::from("12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56");


        let result = calculate_part_number(example.as_str()).expect("Part Number failed to parse");

        assert_eq!(413, result.0);
        assert_eq!(6756, result.1);
    }

    #[test]
    fn example_3() {
        let example = String::from("12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56");


        let result = calculate_part_number(example.as_str()).expect("Part Number failed to parse");

        assert_eq!(925, result.0);
        assert_eq!(6756, result.1);
    }
}