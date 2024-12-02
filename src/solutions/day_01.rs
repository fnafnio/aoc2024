use crate::Solver;
use color_eyre::eyre::Result;
use itertools::{multiunzip, Itertools, MultiUnzip};

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> Result<String> {
        let z = solve_1(input);
        Ok(z.to_string())
    }

    fn part_2(&self, input: &str) -> Result<String> {
        todo!()
    }
}

fn solve_1(input: &str) -> isize {
    let (a, b) = Day::parse_input(input);
    let a = a.iter().sorted();
    let b = b.iter().sorted();
        
    let x = 1i32;
    let y = 5i32;

    let z = a.zip(b).fold(0, |acc, (&a,&b)| acc + a.abs_diff(b)) as isize;
    z
}

impl Day {
    fn parse_input(input: &str) -> (Vec<isize>, Vec<isize>) {
        let v: Vec<(isize, isize)> = input
            .lines()
            .filter_map(|s| s.split_once("   "))
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect();
        let v = multiunzip(v);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_parse() {
        let v = Day::parse_input(INPUT);
        println!("{v:?}");
    }

    #[test]
    fn test_solve_1() {
      assert_eq!(solve_1(INPUT), 11)
    }
}
