// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

// Main functions

fn process(bufin: impl BufRead) -> Result<u64> {
    let mut counts = collections::HashMap::<usize, usize>::new();
    let mut num_lines = 0;
    let mut num_columns = 0;
    for line_opt in bufin.lines() {
        let line = line_opt?;
        for (i, ch) in line.chars().enumerate() {
            if ch == '1' {
                let char_count = counts.entry(i).or_insert(0);
                *char_count += 1;
            }
            num_columns = i + 1;
        }
        num_lines += 1;
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..num_columns {
        let value = 1 << (num_columns - i - 1);
        if counts[&i] > num_lines / 2 {
            gamma |= value;
        } else {
            epsilon |= value;
        }
    }
    Ok(gamma * epsilon)
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY03;
    assert_eq!(process(input.as_bytes())?, 198);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
