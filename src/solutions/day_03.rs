use crate::Solver;
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;
use parser::parse_all_mult;
use regex::RegexBuilder;
pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> Result<String> {
        Ok(solve_1_nom(input)?.to_string())
    }

    fn part_2(&self, input: &str) -> Result<String> {
        Ok(solve_2(input)?.to_string())
    }
}

fn solve_1(input: &str) -> Result<i64> {
    let re = RegexBuilder::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").build()?;

    let c = re.captures_iter(input).collect_vec();
    // it is ok to unwrap the parse result here, because the regex only returns valid digits in the capture group
    let result = re
        .captures_iter(input)
        .map(|x| x.extract())
        .map(|(_, [a, b])| (a.parse().unwrap(), b.parse().unwrap()))
        .map(|(a, b): (i64, i64)| a * b)
        .sum();

    Ok(result)
}

fn solve_1_nom(input: &str) -> Result<i64> {
    let (_, r) = parse_all_mult(input).map_err(|e| eyre!("failed to parse {e:?}"))?;
    Ok(r.iter().map(|(a, b)| a * b).sum())
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Operation {
    Mul(i64, i64),
    Do,
    Dont,
}

mod parser {
    use nom::{
        self,
        bytes::complete::{tag, take, take_till},
        character::complete::digit1,
        combinator::{map, map_res, not},
        error::ParseError,
        multi::{many0, many0_count},
        sequence::{delimited, preceded, separated_pair, tuple},
        IResult, InputIter, InputLength, InputTake, Parser,
    };

    pub fn parse_mul(input: &str) -> IResult<&str, (i64, i64)> {
        let (input, (a, b)) = delimited(
            tag("mul("),
            separated_pair(
                map_res(digit1, str::parse),
                tag(","),
                map_res(digit1, str::parse),
            ),
            tag(")"),
        )(input)?;
        Ok((input, (a, b)))
    }
    /*
    pub fn many0<I, O, E, F>(mut f: F) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
    where
      I: Clone + InputLength,
      F: Parser<I, O, E>,
      E: ParseError<I>,
    {
       */
    fn until_parser<'a, I, O, E, F>(mut parser: F) -> impl FnMut(I) -> IResult<I, O, E> + 'a
    where
        I: Clone + InputLength + InputIter + InputTake + 'a,
        F: Parser<I, O, E> + 'a,
        E: ParseError<I> + 'a,
    {
        move |input: I| {
            let (input, _) = many0_count(preceded(not(|i| parser.parse(i)), take(1u8)))(input)?;
            parser.parse(input)
        }
    }

    pub fn parse_all_mult<'a>(input: &'a str) -> IResult<&'a str, Vec<(i64, i64)>> {
        let r = many0(until_parser(parse_mul))(input)?;
        Ok(r)
    }
}

fn solve_2(input: &str) -> Result<i64> {
    Err(eyre!("not yet implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_ok::assert_ok;
    use nom::bytes::complete::take_till;
    use parser::parse_all_mult;
    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SOLUTION_1: i64 = 161;
    const SOLUTION_2: i64 = 0;

    #[test]
    fn test_nom() {
        let (input, stuff) = assert_ok!(parse_all_mult(INPUT));
        println!("{input}");
        println!("{stuff:?}");
    }

    #[test]
    fn test_1() {
        let r = assert_ok!(solve_1(INPUT));
        let r_nom = assert_ok!(solve_1_nom(INPUT));
        assert_eq!(SOLUTION_1, r);
        assert_eq!(SOLUTION_1, r_nom);
    }
    #[test]
    fn test_2() {
        let r = assert_ok!(solve_2(INPUT));
        assert_eq!(SOLUTION_2, r);
    }
}
