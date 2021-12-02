// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Error, Result};
use std::io::{stdin, BufRead};
use std::ops;
use std::str::FromStr;

extern crate adventofcode2021;

// Direction

#[derive(Debug)]
enum Dir {
    Forward,
    Down,
    Up,
}

impl FromStr for Dir {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "forward" => Dir::Forward,
            "down" => Dir::Down,
            "up" => Dir::Up,
            _ => return Err(anyhow!("invalid direction {}", s)),
        })
    }
}

// Command

#[derive(Debug)]
struct Command {
    pub dir: Dir,
    pub dist: i32,
}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inputs = s.split(' ').collect::<Vec<_>>();
        Ok(Command {
            dir: inputs[0].parse::<Dir>()?,
            dist: inputs[1].parse::<i32>()?,
        })
    }
}

// Xy: position

#[derive(Debug, Default)]
pub struct Xy(i32, i32);

impl ops::AddAssign<Command> for Xy {
    fn add_assign(&mut self, cmd: Command) {
        match cmd.dir {
            Dir::Forward => self.0 += cmd.dist,
            Dir::Down => self.1 += cmd.dist,
            Dir::Up => self.1 -= cmd.dist,
        }
    }
}

impl From<Xy> for i32 {
    fn from(xy: Xy) -> Self {
        xy.0 * xy.1
    }
}

// Main functions

fn process(bufin: impl BufRead) -> Result<i32> {
    let mut xy = Xy::default();
    for line_opt in bufin.lines() {
        let line = line_opt?;
        let cmd = line.parse::<Command>()?;
        xy += cmd;
    }
    Ok(xy.into())
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY02;
    assert_eq!(process(input.as_bytes())?, 150);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
