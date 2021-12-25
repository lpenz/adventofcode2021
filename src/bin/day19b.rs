// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

use adventofcode2021::day19::*;

pub fn get_pos(s: &Scanner) -> Result<Xyz> {
    s.position
        .ok_or_else(|| anyhow!("invalid position for scanner id {}", s.id))
}

pub fn manhattan(s1: &Scanner, s2: &Scanner) -> Result<i32> {
    let s1pos = get_pos(s1)?;
    let s2pos = get_pos(s2)?;
    let dist = (s1pos.x - s2pos.x).abs() + (s1pos.y - s2pos.y).abs() + (s1pos.z - s2pos.z).abs();
    Ok(dist)
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let scanners = bin_parser::parse(bufin)?;
    let scanners_done = fix_scanners(scanners)?;
    let distances = scanners_done
        .iter()
        .cartesian_product(scanners_done.iter())
        .filter_map(|(s1, s2)| manhattan(s1, s2).ok())
        .collect::<Vec<_>>();
    Ok(*distances
        .iter()
        .max()
        .ok_or_else(|| anyhow!("could not calculate distances"))?)
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY19;
    assert_eq!(process(input.as_bytes())?, 3621);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
