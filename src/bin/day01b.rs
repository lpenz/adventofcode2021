// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

fn process(bufin: impl BufRead) -> Result<u32> {
    let mut window: [u32; 3] = [0; 3];
    let mut last_opt: Option<u32> = None;
    let mut increases = 0;
    for (i, line_opt) in bufin.lines().enumerate() {
        let line = line_opt?;
        let curr = line.parse::<u32>()?;
        window[i % 3] = curr;
        if i < 2 {
            continue;
        }
        let wsum = window.iter().sum();
        if let Some(last) = last_opt {
            if last < wsum {
                increases += 1;
            }
        }
        last_opt = Some(wsum);
    }
    Ok(increases)
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY01;
    assert_eq!(process(input.as_bytes())?, 5);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
