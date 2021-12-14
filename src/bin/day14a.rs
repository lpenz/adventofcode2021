// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
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

fn process(bufin: impl BufRead) -> Result<usize> {
    let (mut poly, rules) = parser::parse(bufin)?;
    let rules = rules.into_iter().collect::<HashMap<_, _>>();
    for _ in 0..10 {
        let oldpoly: Vec<Polymer> = std::mem::take(&mut poly);
        let len = oldpoly.len();
        for (i, &p1) in oldpoly.iter().enumerate() {
            if i == len - 1 {
                poly.push(p1);
                break;
            }
            poly.push(p1);
            let p2 = oldpoly[i + 1];
            if let Some(&p) = rules.get(&(p1, p2)) {
                poly.push(p);
            }
        }
    }
    let count = poly
        .into_iter()
        .fold(HashMap::<Polymer, usize>::new(), |mut count, p| {
            *count.entry(p).or_insert(0) += 1;
            count
        });
    let sorted = count
        .into_iter()
        .map(|(p, c)| (c, p))
        .sorted()
        .rev()
        .collect::<Vec<_>>();
    Ok(sorted[0].0 - sorted.last().unwrap().0)
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY14;
    assert_eq!(process(input.as_bytes())?, 1588);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
