use crate::Solver;
use color_eyre::eyre::{eyre, Result};

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> Result<String> {
        Ok(solve_1(input)?.to_string())
    }

    fn part_2(&self, input: &str) -> Result<String> {
        Ok(solve_2(input)?.to_string())
    }
}

fn solve_1(input: &str) -> Result<usize> {
    Err(eyre!("not yet implemented"))
}
fn solve_2(input: &str) -> Result<usize> {
    Err(eyre!("not yet implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_ok::assert_ok;
    const INPUT: &str = "";
    const SOLUTION_1: usize = 0;
    const SOLUTION_2: usize = 0;

    #[test]
    fn test_1() {
        let r = assert_ok!(solve_1(INPUT));
        assert_eq!(SOLUTION_1, r);
    }
    #[test]
    fn test_2() {
        let r = assert_ok!(solve_2(INPUT));
        assert_eq!(SOLUTION_2, r);
    }
}
