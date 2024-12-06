use core::num;
use std::collections::{HashMap, HashSet};

use crate::Solver;
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;
use rayon::{self, iter::ParallelIterator, str::ParallelString};

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
    let (updates, rule_map) = parse_input(input)?;

    let valid = updates
        .iter()
        .map(|update| {
            if rule_map.validate_update(&update) {
                update[update.len() / 2]
            } else {
                0
            }
        })
        .sum();
    Ok(valid)
}

fn parse_input(input: &str) -> Result<(Vec<Vec<usize>>, RuleMap)> {
    let (rules, updates) = input
        .split_once("\n\n")
        .ok_or(eyre!("cannot find empty line"))?;
    let rule_map = RuleMap::parse_rules(rules)?;
    let updates = updates
        .lines()
        .map(parse_update)
        .collect::<Result<Vec<Vec<usize>>>>()?;
    Ok((updates, rule_map))
}

fn solve_2(input: &str) -> Result<usize> {
    let (updates, rule_map) = parse_input(input)?;

    updates
        .iter()
        .filter(|u| !rule_map.validate_update(&u))
        .map(|u| {
            // now we have an invalid update to fix
        });

    Err(eyre!("not yet implemented"))
}

#[derive(Debug)]
struct RuleMap {
    rule_map: HashMap<usize, HashSet<usize>>,
}

impl RuleMap {
    fn parse_rules(input: &str) -> Result<Self> {
        let mut rule_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        for s in input.lines().take_while(|l| !l.is_empty()) {
            let (a, b) = s.split_once("|").ok_or(eyre!("rule elimiter not found"))?;
            let a: usize = a.parse()?;
            let b: usize = b.parse()?;

            if let Some(set) = rule_map.get_mut(&a) {
                set.insert(b);
            } else {
                let mut set = HashSet::new();
                set.insert(b);
                rule_map.insert(a, set);
            }
        }
        Ok(Self { rule_map })
    }

    fn validate_update(&self, update: &[usize]) -> bool {
        for (i, u) in update.iter().enumerate().rev() {
            if let Some(rules) = self.rule_map.get(u) {
                // there exist rules, get the unchecked part
                let unchecked = &update[..i];
                if rules.iter().any(|r| unchecked.contains(r)) {
                    return false;
                }
            }
        }
        true
    }

    fn fix_update(&self, update: &[usize]) -> Result<Vec<usize>> {
        // sort by the count of rules applying for a number

        let result = update
            .iter()
            .enumerate()
            .map(|(i, u)| {
                let rules = self
                    .rule_map
                    .get(u)
                    .ok_or(eyre!("no rules for {u} found"))?;
                let other = update[i..].into_iter().map(|x| *x).collect();

                let num_rules = rules.intersection(&other).count();
                // rules.iter().inter
                Ok((i, u, num_rules))
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&a.2, &b.2))
            .map(|(_, u, _)| *u)
            .collect_vec();
        Ok(result)
    }
}

fn parse_update(input: &str) -> Result<Vec<usize>> {
    let r: Vec<usize> = input
        .split(",")
        .map(|n| n.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_ok::assert_ok;
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    const SOLUTION_1: usize = 143;
    const SOLUTION_2: usize = 0;

    #[test]
    fn test_parser() {
        let r = assert_ok!(RuleMap::parse_rules(INPUT));
        dbg!(&r);
        // assert_eq!(SOLUTION_1, r);
    }

    #[test]
    fn test_fix_update() {
        let r = assert_ok!(RuleMap::parse_rules(INPUT));
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
