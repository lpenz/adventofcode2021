// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

fn process(bufin: impl BufRead) -> Result<u32> {
    let mut last_opt = None;
    let mut increases = 0;
    for line_opt in bufin.lines() {
        let line = line_opt?;
        let curr = line.parse::<u32>()?;
        if let Some(last) = last_opt {
            if last < curr {
                increases += 1;
            }
        }
        last_opt = Some(curr);
    }
    Ok(increases)
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY01;
    assert_eq!(process(input.as_bytes())?, 7);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
