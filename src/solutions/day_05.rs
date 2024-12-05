use std::collections::HashMap;

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
    let (rules, updates) = input
        .split_once("\n\n")
        .ok_or(eyre!("cannot find empty line"))?;
    let rule_map = RuleMap::parse_rules(rules)?;

    let mut valid = 0;
    for update in updates.lines() {
        let update = parse_update(update)?;
        if rule_map.validate_update(&update) {
            valid += update[update.len() / 2];
        }
    }
    Ok(valid)
}
fn solve_2(input: &str) -> Result<usize> {
    Err(eyre!("not yet implemented"))
}

#[derive(Debug)]
struct RuleMap {
    rule_map: HashMap<usize, Vec<usize>>,
}

impl RuleMap {
    fn parse_rules(input: &str) -> Result<Self> {
        let mut rule_map: HashMap<usize, Vec<usize>> = HashMap::new();
        for s in input.lines().take_while(|l| !l.is_empty()) {
            let (a, b) = s.split_once("|").ok_or(eyre!("rule elimiter not found"))?;
            let a: usize = a.parse()?;
            let b: usize = b.parse()?;

            if let Some(v) = rule_map.get_mut(&a) {
                v.push(b);
            } else {
                rule_map.insert(a, vec![b]);
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
