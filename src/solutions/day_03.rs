use crate::Solver;
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;
use regex::RegexBuilder;
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
    let re = RegexBuilder::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").build()?;

    let c = re.captures_iter(input).collect_vec();
    // it is ok to unwrap the parse result here, because the regex only returns valid digits in the capture group
    let result = re
        .captures_iter(input)
        .map(|x| x.extract())
        .map(|(_, [a, b])| (a.parse().unwrap(), b.parse().unwrap()))
        .map(|(a, b): (usize, usize)| a * b)
        .sum();

    Ok(result)
}
fn solve_2(input: &str) -> Result<usize> {
    Err(eyre!("not yet implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_ok::assert_ok;
    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SOLUTION_1: usize = 161;
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
