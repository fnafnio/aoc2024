use std::cmp::Ordering;

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

#[derive(Debug)]
struct Board {
    board: String,
    width: usize,
    height: usize,
}

const XMAS: &str = "XMAS";
impl Board {
    fn new(board: &str) -> Self {
        let width = board.lines().next().unwrap().len();
        let height = board.lines().count();
        let board: String = board.chars().filter(|c| *c != '\n' && *c != '\r').collect();
        Self {
            board,
            width,
            height,
        }
    }

    fn find_all_x(&self) -> Vec<usize> {
        self.board
            .char_indices()
            .filter_map(|(i, c)| if c == 'X' { Some(i) } else { None })
            .collect()
    }

    fn forward(&self, start: usize) -> bool {
        self.board[start..start + XMAS.len()].cmp(XMAS) == Ordering::Equal
    }

    fn backward(&self, start: usize) -> bool {
        // start >= XMAS.len() &&
        self.board[start-XMAS.len()..=start].chars().rev()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_ok::assert_ok;
    const INPUT: &str = "
..X...
.SAMX.
.A..A.
XMAS.S
.X....";
    const SOLUTION_1: usize = 18;
    const SOLUTION_2: usize = 0;

    #[test]
    fn test_build_1() {
        let b = dbg!(Board::new(INPUT));
        let v = b.find_all_x();
        assert_eq!(v, vec![2, 10, 18, 25]);
        let f: Vec<_> = v.iter().map(|i| b.forward(*i)).collect();
        assert_eq!(f, vec![false, false, true, false]);
        let f: Vec<_> = v.iter().map(|i| b.backward(*i)).collect();
        assert_eq!(f, vec![false, true, false, false]);
    }

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
