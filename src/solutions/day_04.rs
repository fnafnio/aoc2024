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
    let b = Board::new(input);

    Ok(b.find_all_x()
        .into_iter()
        .map(|start| Direction::iterator().map(move |d| (start, d)))
        .flatten()
        .map(|(start, dir)| b.go_direction(start, dir))
        .filter(|c| *c)
        .count())
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Front,
    UpFront,
    Up,
    UpBack,
    Back,
    DownBack,
    Down,
    DownFront,
}

impl Direction {
    fn as_increment(&self, width: usize) -> isize {
        let width = width as isize;
        match self {
            Direction::Front => 1,
            Direction::UpFront => -width + 1,
            Direction::Up => -width,
            Direction::UpBack => -width - 1,
            Direction::Back => -1,
            Direction::DownBack => width - 1,
            Direction::Down => width,
            Direction::DownFront => width + 1,
        }
    }
    fn iterator() -> impl Iterator<Item = Direction> {
        [
            Direction::Front,
            Direction::UpFront,
            Direction::Up,
            Direction::UpBack,
            Direction::Back,
            Direction::DownBack,
            Direction::Down,
            Direction::DownFront,
        ]
        .iter()
        .cloned()
    }
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

    fn go_direction(&self, start: usize, dir: Direction) -> bool {
        let incr = dir.as_increment(self.width);

        for (i, xchar) in XMAS.char_indices() {
            let next = start as isize + i as isize * incr;
            if next < 0 {
                return false;
            }
            if let Some(c) = self.board.chars().nth(next as usize) {
                if c != xchar {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_ok::assert_ok;
    const INPUT_A: &str = "
..X...
.SAMX.
.A..A.
XMAS.S
.X....";

    const INPUT_B: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    const SOLUTION_1A: usize = 6;
    const SOLUTION_1B: usize = 18;
    const SOLUTION_2: usize = 0;

    #[test]
    fn test_build_1() {
        let b = dbg!(Board::new(INPUT_A));
        let v = b.find_all_x();
        assert_eq!(v, vec![2, 10, 18, 25]);
        let f: Vec<_> = v
            .iter()
            .map(|i| b.go_direction(*i, Direction::Front))
            .collect();
        assert_eq!(f, vec![false, false, true, false]);
        let f: Vec<_> = v
            .iter()
            .map(|i| b.go_direction(*i, Direction::Back))
            .collect();
        assert_eq!(f, vec![false, true, false, false]);
    }

    #[test]
    fn test_1() {
        let r = assert_ok!(solve_1(INPUT_A));
        assert_eq!(SOLUTION_1A, r);
        let r = assert_ok!(solve_1(INPUT_B));
        assert_eq!(SOLUTION_1B, r);
    }
    #[test]
    fn test_2() {
        let r = assert_ok!(solve_2(INPUT_A));
        assert_eq!(SOLUTION_2, r);
    }
}
