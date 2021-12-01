// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::{stdin, BufRead};

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

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
