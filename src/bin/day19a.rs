// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

use adventofcode2021::day19::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let scanners = bin_parser::parse(bufin)?;
    let scanners_done = fix_scanners(scanners)?;
    Ok(total_beacons(scanners_done.iter()))
}

// Takes too long:
// #[test]
// fn test() -> Result<()> {
//     let input = adventofcode2021::examples::DAY19;
//     assert_eq!(process(input.as_bytes())?, 79);
//     Ok(())
// }

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
