use crate::Solver;
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;
use parser::{parse_all_ops, parse_all_pairs};
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
    let (_, r) = parse_all_pairs(input).map_err(|e| eyre!("failed to parse {e:?}"))?;
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

    use super::Operation;

    pub fn parse_mul(input: &str) -> IResult<&str, Operation> {
        let (input, (a, b)) = num_double(input)?;
        Ok((input, Operation::Mul(a, b)))
    }

    fn num_double(input: &str) -> IResult<&str, (i64, i64)> {
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

    pub fn parse_do(input: &str) -> IResult<&str, Operation> {
        let (input, _) = tag("do()")(input)?;
        Ok((input, Operation::Do))
    }
    pub fn parse_dont(input: &str) -> IResult<&str, Operation> {
        let (input, _) = tag("don't()")(input)?;
        Ok((input, Operation::Dont))
    }

    pub fn parse_operation(input: &str) -> IResult<&str, Operation> {
        let (input, op) = nom::branch::alt((parse_do, parse_dont, parse_mul))(input)?;
        Ok((input, op))
    }

    fn until_parser<'a, I, O, E, F>(mut parser: F) -> impl FnMut(I) -> IResult<I, O, E> + 'a
    where
        I: Clone + InputLength + InputIter + InputTake + 'a,
        F: Parser<I, O, E> + 'a,
        E: ParseError<I> + 'a,
    {
        move |input: I| {
            let (input, _) = many0(preceded(not(|i| parser.parse(i)), take(1u8)))(input)?;
            parser.parse(input)
        }
    }

    pub fn parse_all_pairs<'a>(input: &'a str) -> IResult<&'a str, Vec<(i64, i64)>> {
        let r = many0(until_parser(num_double))(input)?;
        Ok(r)
    }

    pub fn parse_all_ops(input: &str) -> IResult<&str, Vec<Operation>> {
        many0(until_parser(parse_operation))(input)
    }

    #[cfg(test)]
    mod tests {
        use assert_ok::assert_ok;

        use crate::solutions::day_03::{parser::parse_operation, Operation};

        use super::parse_all_ops;

        #[test]
        fn test_parse_op() {
            let o = assert_ok!(parse_operation("do()"));
            assert_eq!(o.1, Operation::Do);
            let o = assert_ok!(parse_operation("don't()"));
            assert_eq!(o.1, Operation::Dont);
            let o = assert_ok!(parse_operation("mul(123,456)"));
            assert_eq!(o.1, Operation::Mul(123, 456));
        }
    }
}

fn solve_2(input: &str) -> Result<i64> {
    let mut enabled = true;
    let mut sum = 0;
    let (rest, v) = parse_all_ops(input).map_err(|e| eyre!("failed to parse {e:?}"))?;
    for &ops in v.iter() {
        match ops {
            Operation::Mul(a, b) => {
                if enabled {
                    sum = sum + (a * b);
                }
            }
            Operation::Do => enabled = true,
            Operation::Dont => enabled = false,
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_ok::assert_ok;
    use nom::bytes::complete::take_till;
    use parser::parse_all_pairs;
    const INPUT_1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    const SOLUTION_1: i64 = 161;
    const SOLUTION_2: i64 = 48;

    #[test]
    fn test_1() {
        let r = assert_ok!(solve_1(INPUT_1));
        let r_nom = assert_ok!(solve_1_nom(INPUT_1));
        assert_eq!(SOLUTION_1, r);
        assert_eq!(SOLUTION_1, r_nom);
    }

    #[test]
    fn test_nom() {
        let (input, stuff) = assert_ok!(parse_all_pairs(INPUT_2));
        println!("{input}");
        println!("{stuff:?}");
    }

    #[test]
    fn test_2() {
        let r = assert_ok!(solve_2(INPUT_2));
        assert_eq!(SOLUTION_2, r);
    }

    #[test]
    fn test_parse_all_ops() {
        let o = assert_ok!(parse_all_ops(INPUT_2));
        use Operation::*;
        assert_eq!(
            o.1,
            vec![Mul(2, 4), Dont, Mul(5, 5), Mul(11, 8), Do, Mul(8, 5)]
        )
    }
}
