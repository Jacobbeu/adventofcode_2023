use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

pub fn solve() -> SolutionPair {

    let contents = read_to_string("input/day01_input.txt").expect("failed to parse input file");

    let mut lines = contents.lines();

    while let Some(_line) = lines.next() {

    }

    let solution1 = 0;
    let solution2 = 0;

    (Solution::from(solution1), Solution::from(solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_example() {
        let _ = solve();
    }
}