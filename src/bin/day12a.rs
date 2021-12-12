// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use lazy_static::lazy_static;
use std::collections::HashMap;
// use std::collections::HashSet;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Cave(String);

impl Cave {
    pub fn is_big(&self) -> bool {
        if let Some(c) = self.0.chars().next() {
            c.is_uppercase()
        } else {
            false
        }
    }
}

impl From<&str> for Cave {
    fn from(s: &str) -> Self {
        Cave(s.to_string())
    }
}

lazy_static! {
    static ref START: Cave = Cave::from("start");
    static ref END: Cave = Cave::from("end");
}

// Parser

pub mod parser {
    use crate::Cave;
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::{character, combinator, multi::*, IResult};
    use std::io::BufRead;

    pub fn edge(input: &str) -> IResult<&str, (Cave, Cave)> {
        let (input, cave1) = character::complete::alpha1(input)?;
        let (input, _) = character::complete::char('-')(input)?;
        let (input, cave2) = character::complete::alpha1(input)?;
        let (input, _) = character::complete::char('\n')(input)?;
        Ok((input, (Cave::from(cave1), Cave::from(cave2))))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(Cave, Cave)>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(many1(edge))(&input);
        match result {
            Ok((_, edges)) => Ok(edges),
            Err(e) => Err(anyhow!("error reading input: {:?}", e)),
        }
    }
}

// Main functions

type Map = HashMap<Cave, Vec<Cave>>;

fn dfs(map: &Map, current: &Cave, path: &mut Vec<Cave>) -> usize {
    if *current == *END {
        eprintln!("path: {:?}", path);
        return 1;
    }
    let mut numpaths = 0;
    if let Some(caves) = map.get(current) {
        for cave in caves {
            if !cave.is_big() && path.contains(cave) {
                eprintln!("skip at path: {:?}, cave {}", path, cave.0);
                continue;
            }
            eprintln!("{} -> {}", current.0, cave.0);
            path.push(cave.clone());
            numpaths += dfs(map, cave, path);
            path.pop();
        }
    }
    numpaths
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let edges = parser::parse(bufin)?;
    let mut map = Map::new();
    for (cave1, cave2) in edges {
        let e = map.entry(cave1.clone()).or_insert_with(Vec::new);
        e.push(cave2.clone());
        let e = map.entry(cave2).or_insert_with(Vec::new);
        e.push(cave1);
    }
    eprintln!("{:?}", map);
    let mut path = vec![START.clone()];
    let paths = dfs(&map, &*START, &mut path);
    Ok(paths)
}

pub fn do_test(input: &str, ans: usize) -> Result<()> {
    assert_eq!(process(input.as_bytes())?, ans);
    Ok(())
}

#[test]
fn test1() -> Result<()> {
    do_test(&adventofcode2021::examples::DAY12_1, 10)
}

#[test]
fn test2() -> Result<()> {
    do_test(&adventofcode2021::examples::DAY12_2, 19)
}

#[test]
fn test3() -> Result<()> {
    do_test(&adventofcode2021::examples::DAY12_3, 226)
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
