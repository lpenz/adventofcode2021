// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashMap;
use std::io::BufRead;

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

fn spawned(data: &mut HashMap<(i32, i32), u64>, value: i32, turns0: i32, lvl: i32) -> u64 {
    if turns0 < 0 {
        return 0;
    }
    if let Some(&ret) = data.get(&(value, turns0)) {
        return ret;
    }
    let mut turns = turns0 - value - 1;
    let mut ret = 0;
    while turns >= 0 {
        ret += 1 + spawned(data, 8, turns, lvl + 1);
        turns -= 7;
    }
    data.insert((value, turns0), ret);
    ret
}

fn calc_fishes(fishes: &[i32], turns: i32) -> Result<u64> {
    let mut data = HashMap::<(i32, i32), u64>::new();
    Ok(fishes
        .iter()
        .map(|&f| 1 + spawned(&mut data, f, turns, 0))
        .sum())
}

pub fn process(turns: i32, bufin: impl BufRead) -> Result<u64> {
    let fishes = parse(bufin)?;
    calc_fishes(&fishes, turns)
}

#[test]
fn test() -> Result<()> {
    let input = crate::examples::DAY06;
    assert_eq!(process(80, input.as_bytes())?, 5934);
    assert_eq!(process(256, input.as_bytes())?, 26984457539);
    Ok(())
}

#[test]
fn test_simple() -> Result<()> {
    assert_eq!(calc_fishes(&[3], 0)?, 1);
    assert_eq!(calc_fishes(&[3], 3)?, 1);
    assert_eq!(calc_fishes(&[3], 4)?, 2);
    assert_eq!(calc_fishes(&[3], 10)?, 2);
    assert_eq!(calc_fishes(&[3], 12)?, 3);
    assert_eq!(calc_fishes(&[3], 13)?, 4);
    assert_eq!(calc_fishes(&[3], 15)?, 4);
    Ok(())
}
