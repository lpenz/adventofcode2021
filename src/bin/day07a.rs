// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Result;
use std::io::stdin;
use std::io::BufRead;

extern crate adventofcode2021;

// Main functions

fn parse(bufin: impl BufRead) -> Result<Vec<i32>> {
    let line = bufin
        .lines()
        .next()
        .ok_or_else(|| anyhow!("error reading"))??;
    line.split(',')
        .map(|s| s.parse::<i32>().map_err(|e| anyhow!(e)))
        .collect::<Result<Vec<_>>>()
}

fn fuelcalc(pos: i32, crabs: &[i32]) -> i32 {
    crabs.iter().map(|i| (i - pos).abs()).sum()
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let crabs = parse(bufin)?;
    let min = crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    Ok((min + 1..max).map(|i| fuelcalc(i, &crabs)).min().unwrap())
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY07;
    assert_eq!(process(input.as_bytes())?, 37);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
