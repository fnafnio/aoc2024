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
            if rule_map.is_update_valid(&update) {
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

    let sum = updates
        .iter()
        .filter(|u| rule_map.is_update_invalid(u))
        .map(|u| {
            let fixed = rule_map.fix_update(u);
            fixed[u.len() / 2]
        })
        .sum();

    Ok(sum)
}

#[derive(Debug)]
struct RuleMap {
    forward_map: HashMap<usize, HashSet<usize>>,
    backward_map: HashMap<usize, HashSet<usize>>,
}

impl RuleMap {
    fn parse_rules(input: &str) -> Result<Self> {
        let mut forward_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut backward_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        for s in input.lines().take_while(|l| !l.is_empty()) {
            let (a, b) = s.split_once("|").ok_or(eyre!("rule elimiter not found"))?;
            let a: usize = a.parse()?;
            let b: usize = b.parse()?;

            if let Some(set) = forward_map.get_mut(&a) {
                set.insert(b);
            } else {
                let mut set = HashSet::new();
                set.insert(b);
                forward_map.insert(a, set);
            }
            if let Some(set) = backward_map.get_mut(&a) {
                set.insert(b);
            } else {
                let mut set = HashSet::new();
                set.insert(b);
                backward_map.insert(a, set);
            }
        }
        Ok(Self {
            forward_map,
            backward_map,
        })
    }

    fn is_update_valid(&self, update: &[usize]) -> bool {
        for (i, u) in update.iter().enumerate().rev() {
            if let Some(rules) = self.forward_map.get(u) {
                // there exist rules, get the unchecked part
                let unchecked = &update[..i];
                if rules.iter().any(|r| unchecked.contains(r)) {
                    return false;
                }
            }
        }
        true
    }

    fn is_update_invalid(&self, update: &[usize]) -> bool {
        !self.is_update_valid(update)
    }

    fn fix_update(&self, update: &[usize]) -> Vec<usize> {
        // sort by the count of rules applying for a number

        let result = update
            .iter()
            .enumerate()
            .map(|(i, u)| {
                let num_rules = match self.backward_map.get(u) {
                    Some(rules) => {
                        let other = update[i..].into_iter().map(|x| *x).collect();

                        rules.intersection(&other).count()
                    }
                    _ => 0,
                };

                (i, *u, num_rules)
            })
            .sorted_by(|a, b| Ord::cmp(&a.2, &b.2))
            .map(|(_, u, _)| u)
            .collect_vec();
        result
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
    const SOLUTION_2: usize = 123;

    #[test]
    fn test_parser() {
        let r = assert_ok!(RuleMap::parse_rules(INPUT));
        dbg!(&r);
    }

    #[test]
    fn test_fix_update() {
        let correct = vec![
            vec![97, 75, 47, 61, 53],
            vec![61, 29, 13],
            vec![97, 75, 47, 29, 13],
        ];
        let (updates, rules) = assert_ok!(parse_input(INPUT));

        for (u, c) in updates
            .iter()
            .filter(|u| rules.is_update_invalid(u))
            .zip(correct.iter())
            .map(|x| x)
        {
            let fixed = rules.fix_update(u);
            assert_eq!(c[c.len() / 2], fixed[fixed.len() / 2]);
        }
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
