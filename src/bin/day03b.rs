// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

// Main functions

fn countpos(lines: &[String], i: usize, value: u8) -> usize {
    lines
        .iter()
        .map(|line| if line.as_bytes()[i] == value { 1 } else { 0 })
        .sum()
}

fn convert(lastline_opt: Option<String>, num_columns: usize) -> Result<u64> {
    let lastline = lastline_opt.ok_or_else(|| anyhow!("no last line found"))?;
    let mut result = 0;
    for i in 0..num_columns {
        if lastline.as_bytes()[i] == b'1' {
            let value = 1 << (num_columns - i - 1);
            result |= value;
        }
    }
    Ok(result)
}

fn process_lines(lines: &mut Vec<String>, i: usize, iso2: bool) -> Option<String> {
    let mostly1 = countpos(lines, i, b'1') * 2 >= lines.len();
    let value = if iso2 && mostly1 || !iso2 && !mostly1 {
        b'1'
    } else {
        b'0'
    };
    *lines = lines
        .iter()
        .filter(|l| l.as_bytes()[i] == value)
        .cloned()
        .collect();
    if lines.len() == 1 {
        Some(lines[0].clone())
    } else {
        None
    }
}

fn process(bufin: impl BufRead) -> Result<u64> {
    let input: Vec<String> = bufin.lines().collect::<std::result::Result<Vec<_>, _>>()?;
    let num_columns = input[0].len();
    let mut o2 = None;
    let mut o2lines = input.clone();
    let mut co2 = None;
    let mut co2lines = input;
    for i in 0..num_columns {
        if o2.is_none() {
            o2 = process_lines(&mut o2lines, i, true);
        }
        if co2.is_none() {
            co2 = process_lines(&mut co2lines, i, false);
        }
    }
    Ok(convert(o2, num_columns)? * convert(co2, num_columns)?)
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY03;
    assert_eq!(process(input.as_bytes())?, 230);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
