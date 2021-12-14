// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Polymer(u8);

impl fmt::Debug for Polymer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

// Parser

pub mod parser {
    use crate::Polymer;
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::{bytes, character, combinator, multi::*, IResult};
    use std::io::BufRead;

    type Rule = ((Polymer, Polymer), Polymer);

    pub fn polymer(input: &str) -> IResult<&str, Polymer> {
        let (input, p) = character::complete::satisfy(|c| c.is_alphabetic())(input)?;
        Ok((input, Polymer(p as u8)))
    }

    pub fn template(input: &str) -> IResult<&str, Vec<Polymer>> {
        let (input, ps) = many1(polymer)(input)?;
        let (input, _) = character::complete::char('\n')(input)?;
        Ok((input, ps))
    }

    pub fn pairrules(input: &str) -> IResult<&str, Rule> {
        let (input, p1) = polymer(input)?;
        let (input, p2) = polymer(input)?;
        let (input, _) = bytes::complete::tag(" -> ")(input)?;
        let (input, pr) = polymer(input)?;
        let (input, _) = character::complete::char('\n')(input)?;
        Ok((input, ((p1, p2), pr)))
    }

    pub fn all(input: &str) -> IResult<&str, (Vec<Polymer>, Vec<Rule>)> {
        let (input, t) = template(input)?;
        let (input, _) = character::complete::char('\n')(input)?;
        let (input, rules) = many1(pairrules)(input)?;
        Ok((input, (t, rules)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Vec<Polymer>, Vec<Rule>)> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(all)(&input);
        match result {
            Ok((_, info)) => Ok(info),
            Err(e) => Err(anyhow!("error reading input: {:?}", e)),
        }
    }
}

// Main functions

fn countall(
    rules: &HashMap<(Polymer, Polymer), Polymer>,
    target: Polymer,
    p1: Polymer,
    p2: Polymer,
    cache: &mut HashMap<(Polymer, Polymer, usize), usize>,
    lvl: usize,
) -> usize {
    if lvl == 0 {
        return 0;
    }
    if let Some(&count) = cache.get(&(p1, p2, lvl)) {
        return count;
    }
    let count = if let Some(&p) = rules.get(&(p1, p2)) {
        (if p == target { 1 } else { 0 }
            + countall(rules, target, p1, p, cache, lvl - 1)
            + countall(rules, target, p, p2, cache, lvl - 1))
    } else {
        0
    };
    let e = cache.entry((p1, p2, lvl)).or_insert(0);
    *e += count;
    count
}

fn process(steps: usize, bufin: impl BufRead) -> Result<usize> {
    let (template, rules) = parser::parse(bufin)?;
    let rules = rules.into_iter().collect::<HashMap<_, _>>();
    let mut count =
        template
            .iter()
            .cloned()
            .fold(HashMap::<Polymer, usize>::new(), |mut count, p| {
                *count.entry(p).or_insert(0) += 1;
                count
            });
    let polyset = template
        .iter()
        .cloned()
        .chain(rules.values().cloned())
        .collect::<HashSet<_>>();
    for target in polyset {
        let mut cache = HashMap::<(Polymer, Polymer, usize), usize>::new();
        for (i, &p1) in template.iter().enumerate() {
            if i == template.len() - 1 {
                break;
            }
            let p2 = template[i + 1];
            *count.entry(target).or_insert(0) +=
                countall(&rules, target, p1, p2, &mut cache, steps);
        }
    }
    let sorted = count
        .into_iter()
        .map(|(p, c)| (c, p))
        .sorted()
        .rev()
        .collect::<Vec<_>>();
    Ok(sorted[0].0 - sorted.last().unwrap().0)
}

#[test]
fn test_0() -> Result<()> {
    let input = adventofcode2021::examples::DAY14;
    assert_eq!(process(1, input.as_bytes())?, 1);
    Ok(())
}

#[test]
fn test_a() -> Result<()> {
    let input = adventofcode2021::examples::DAY14;
    assert_eq!(process(10, input.as_bytes())?, 1588);
    Ok(())
}

#[test]
fn test_b() -> Result<()> {
    let input = adventofcode2021::examples::DAY14;
    assert_eq!(process(40, input.as_bytes())?, 2188189693529);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(40, stdin().lock())?);
    Ok(())
}
