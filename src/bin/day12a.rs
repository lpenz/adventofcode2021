// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Cave(usize);

#[derive(Debug, Default)]
pub struct Caves {
    pub names: Vec<String>,
    pub ids: HashMap<String, usize>,
    pub big: Vec<bool>,
    pub map: HashMap<Cave, Vec<Cave>>,
}

impl Caves {
    pub fn cave_get_or_insert(&mut self, cavename: String) -> Cave {
        if let Some(&id) = self.ids.get(cavename.as_str()) {
            return Cave(id);
        }
        let id = self.names.len();
        self.big.push(if let Some(c) = cavename.chars().next() {
            c.is_uppercase()
        } else {
            false
        });
        self.names.push(cavename.clone());
        self.ids.insert(cavename, id);
        self.map.insert(Cave(id), vec![]);
        Cave(id)
    }

    pub fn insert_edge(&mut self, edge: (String, String)) {
        let (c1string, c2string) = edge;
        let c1 = self.cave_get_or_insert(c1string);
        let c2 = self.cave_get_or_insert(c2string);
        let e = self.map.entry(c1).or_insert_with(Vec::new);
        e.push(c2);
        let e = self.map.entry(c2).or_insert_with(Vec::new);
        e.push(c1);
    }

    pub fn is_big(&self, cave: &Cave) -> bool {
        self.big[cave.0]
    }

    pub fn cave_get(&self, cavename: impl AsRef<str>) -> Result<Cave> {
        Ok(Cave(*self.ids.get(cavename.as_ref()).ok_or_else(|| {
            anyhow!("could not find cave {}", cavename.as_ref())
        })?))
    }

    pub fn iter_next(&self, cave: &Cave) -> impl Iterator<Item = &Cave> {
        self.map[cave].iter()
    }
}

// Parser

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::{character, combinator, multi::*, IResult};
    use std::io::BufRead;

    pub fn edge(input: &str) -> IResult<&str, (String, String)> {
        let (input, cave1) = character::complete::alpha1(input)?;
        let (input, _) = character::complete::char('-')(input)?;
        let (input, cave2) = character::complete::alpha1(input)?;
        let (input, _) = character::complete::char('\n')(input)?;
        Ok((input, (cave1.to_string(), cave2.to_string())))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(String, String)>> {
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

fn dfs(caves: &Caves, current: &Cave, end: &Cave, path: &mut Vec<Cave>) -> usize {
    if current == end {
        return 1;
    }
    let mut numpaths = 0;
    for cave in caves.iter_next(current) {
        if !caves.is_big(cave) && path.contains(cave) {
            continue;
        }
        path.push(*cave);
        numpaths += dfs(caves, cave, end, path);
        path.pop();
    }
    numpaths
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let edges = parser::parse(bufin)?;
    let mut caves = Caves::default();
    for edge in edges.into_iter() {
        caves.insert_edge(edge);
    }
    let start = caves.cave_get("start")?;
    let end = caves.cave_get("end")?;
    let mut path = vec![start];
    let paths = dfs(&caves, &start, &end, &mut path);
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
