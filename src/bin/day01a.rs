// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::{stdin, BufRead};

/// Example: count the number of lines
fn process(bufin: impl BufRead) -> Result<i32> {
    let mut num = 0;
    for line_opt in bufin.lines() {
        let _line = line_opt?;
        num += 1;
    }
    Ok(num)
}

#[test]
fn test() -> Result<()> {
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
