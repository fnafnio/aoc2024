use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
};

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

    Ok(b.find_all_char('X')
        .into_iter()
        .map(|start| Direction::iterator().map(move |d| (start, d)))
        .flatten()
        .map(|(start, dir)| b.check_xmas(start, dir))
        .filter(|c| *c)
        .count())
}
fn solve_2(input: &str) -> Result<usize> {
    let b = Board::new(input);

    Ok(b.find_all_char('A')
        .into_iter()
        .map(|a| b.check_cross_mas(a))
        .filter(|c| *c)
        .count())
}

#[derive(Debug)]
struct Board {
    board: Vec<Vec<char>>,
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

impl From<Direction> for Point2 {
    fn from(value: Direction) -> Self {
        let (x, y) = match value {
            Direction::Front => (1, 0),
            Direction::UpFront => (1, -1),
            Direction::Up => (0, -1),
            Direction::UpBack => (-1, -1),
            Direction::Back => (-1, 0),
            Direction::DownBack => (-1, 1),
            Direction::Down => (0, 1),
            Direction::DownFront => (1, 1),
        };
        Point2 { x, y }
    }
}

impl Direction {
    fn iterator() -> impl Iterator<Item = Direction> {
        use Direction::*;
        [Front, UpFront, Up, UpBack, Back, DownBack, Down, DownFront]
            .iter()
            .cloned()
    }

    fn opposite(&self) -> Direction {
        use Direction::*;
        match self {
            Front => Back,
            UpFront => DownBack,
            Up => Down,
            UpBack => DownFront,
            Back => Front,
            DownBack => UpFront,
            Down => Up,
            DownFront => UpBack,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point2 {
    x: i32,
    y: i32,
}

impl Add for Point2 {
    type Output = Point2;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point2 {
    type Output = Point2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for Point2 {
    type Output = Point2;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

const XMAS: &str = "XMAS";
const MAS: &str = "MAS";

impl Board {
    fn new(board: &str) -> Self {
        let width = board.lines().next().unwrap().len();
        let height = board.lines().count();
        let board = board.lines().map(|l| l.chars().collect()).collect();
        Self {
            board,
            width,
            height,
        }
    }

    fn valid_coord(&self, p: Point2) -> bool {
        p.x < self.width as i32 && p.y < self.height as i32
    }

    fn get(&self, p: &Point2) -> Option<&char> {
        if let Some(l) = self.board.get(p.y as usize) {
            if let Some(c) = l.get(p.x as usize) {
                return Some(c);
            }
        }
        None
    }

    fn find_all_char(&self, target: char) -> Vec<Point2> {
        self.board
            .iter()
            .enumerate()
            .map(|(i, l)| {
                l.iter().enumerate().filter_map(move |(j, c)| {
                    if target == *c {
                        Some(Point2 {
                            y: i as i32,
                            x: j as i32,
                        })
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect()
    }

    fn check_xmas(&self, start: Point2, dir: Direction) -> bool {
        let incr: Point2 = dir.into();

        for (i, xchar) in XMAS.char_indices() {
            let next = start + (incr * i as i32);
            if let Some(c) = self.get(&next) {
                if *c != xchar {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn check_diag_mas(&self, start: Point2, dir: Direction) -> bool {
        let d_last: Point2 = dir.opposite().into();
        let dir: Point2 = dir.into();
        if let (Some(first), Some(last)) = (self.get(&(dir + start)), self.get(&(d_last + start))) {
            *first == 'M' && *last == 'S'
        } else {
            false
        }
    }

    fn check_cross_mas(&self, start: Point2) -> bool {
        use Direction::{DownBack, DownFront, UpBack, UpFront};
        const CROSS: [Direction; 4] = [UpBack, UpFront, DownBack, DownFront];

        CROSS
            .iter()
            .map(|&dir| self.check_diag_mas(start, dir))
            .filter(|c| *c)
            .count() == 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_ok::assert_ok;
    const INPUT_1A: &str = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";

    const INPUT_1B: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    const INPUT_2: &str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

    const SOLUTION_1A: usize = 4;
    const SOLUTION_1B: usize = 18;
    const SOLUTION_2: usize = 9;

    #[test]
    fn test_build_1() {
        let b = Board::new(INPUT_1A);
        let v = b.find_all_char('X');
        assert_eq!(
            v,
            vec![
                Point2 { x: 2, y: 0 },
                Point2 { x: 4, y: 1 },
                Point2 { x: 0, y: 3 },
                Point2 { x: 1, y: 4 }
            ]
        );
        let f: Vec<_> = v
            .iter()
            .map(|i| b.check_xmas(*i, Direction::Front))
            .collect();
        assert_eq!(f, vec![false, false, true, false]);
        let f: Vec<_> = v
            .iter()
            .map(|i| b.check_xmas(*i, Direction::Back))
            .collect();
        assert_eq!(f, vec![false, true, false, false]);
    }

    #[test]
    fn test_1() {
        let r = assert_ok!(solve_1(INPUT_1B));
        assert_eq!(SOLUTION_1B, r);
        let r = assert_ok!(solve_1(INPUT_1A));
        assert_eq!(SOLUTION_1A, r);
    }

    #[test]
    fn test_2() {
        let r = assert_ok!(solve_2(INPUT_2));
        assert_eq!(SOLUTION_2, r);
    }
}
