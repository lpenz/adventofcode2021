// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Result;
use std::cmp;
use std::io::stdin;
use std::io::BufRead;

extern crate adventofcode2021;

type Fuel = i32;
type X = i32;

// Main functions

fn parse(bufin: impl BufRead) -> Result<Vec<X>> {
    let line = bufin
        .lines()
        .next()
        .ok_or_else(|| anyhow!("error reading"))??;
    line.split(',')
        .map(|s| s.parse::<X>().map_err(|e| anyhow!(e)))
        .collect::<Result<Vec<_>>>()
}

fn fuelcalc(pos: X, crabs: &[X]) -> Fuel {
    crabs
        .iter()
        .map(|&i| {
            let lo = cmp::min(pos, i);
            let hi = cmp::max(pos, i);
            let dist = hi - lo;
            (1 + dist) * dist / 2
        })
        .sum()
}

fn process(bufin: impl BufRead) -> Result<Fuel> {
    let crabs = parse(bufin)?;
    let min = crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    Ok((min + 1..max).map(|i| fuelcalc(i, &crabs)).min().unwrap())
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY07;
    assert_eq!(process(input.as_bytes())?, 168);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
