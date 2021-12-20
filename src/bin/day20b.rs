// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::stdin;

extern crate adventofcode2021;

#[test]
fn test_b() -> Result<()> {
    adventofcode2021::day20::do_test(50, 3351)
}

fn main() -> Result<()> {
    println!("{}", adventofcode2021::day20::process(2, stdin().lock())?);
    Ok(())
}
