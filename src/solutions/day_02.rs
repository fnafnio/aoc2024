use crate::Solver;
use color_eyre::eyre::{self, Context, Result};
use itertools::Itertools;

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> Result<String> {
        Ok(solve_1(input)?.to_string())
    }

    fn part_2(&self, input: &str) -> Result<String> {
        Ok(solve_2(input)?.to_string())
    }
}

fn parse_str_of_i32(input: &str) -> Result<Vec<i32>> {
    let collect = input
        .split(" ")
        .map(|s: &str| {
            s.parse::<i32>()
                .wrap_err_with(|| format!("failed to parse {} to i32", s))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(collect)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Safety {
    SafePos,
    SafeNeg,
    Unsafe,
}

impl From<i32> for Safety {
    fn from(value: i32) -> Self {
        match value {
            -3..=-1 => Self::SafeNeg,
            0 => Self::Unsafe,
            1..=3 => Self::SafePos,
            _ => Self::Unsafe,
        }
    }
}

fn everything_safe<'a>(it: impl IntoIterator<Item = &'a i32>) -> bool {
    it.into_iter()
        .map_windows(|&[a, b]| Safety::from(a - b))
        .fold((None, true), |(kind, safe), x| match kind {
            None => (Some(x), x != Safety::Unsafe),
            Some(k) => (Some(k), safe && x == k),
        })
        .1
}

fn solve_1(input: &str) -> Result<usize> {
    let result = parse_inputs(input)?
        .iter()
        .map(|v| everything_safe((v)))
        .filter(|x| *x)
        .count();

    Ok(result)
}

fn solve_2(input: &str) -> Result<usize> {
    let input = parse_inputs(input)?;

    let result = input
        .into_iter()
        .map(|v| {
            let is_safe = everything_safe(v.iter());
            is_safe
                || v.iter()
                    .combinations(v.len() - 1)
                    .map(|comb| {
                        let x = everything_safe(comb);
                        x
                    })
                    .any(|b| b)
        })
        .filter(|x| *x)
        .count();
    Ok(result)
}

fn parse_inputs(input: &str) -> Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(parse_str_of_i32)
        .collect::<Result<Vec<Vec<_>>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_ok::assert_ok;
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    const SOLUTION_1: usize = 2;
    const SOLUTION_2: usize = 4;

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
