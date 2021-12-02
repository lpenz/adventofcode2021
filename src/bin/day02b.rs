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
    pub value: i32,
}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inputs = s.split(' ').collect::<Vec<_>>();
        Ok(Command {
            dir: inputs[0].parse::<Dir>()?,
            value: inputs[1].parse::<i32>()?,
        })
    }
}

// State

#[derive(Debug, Default)]
pub struct State {
    x: i32,
    y: i32,
    aim: i32,
}

impl ops::AddAssign<Command> for State {
    fn add_assign(&mut self, cmd: Command) {
        match cmd.dir {
            Dir::Forward => {
                self.x += cmd.value;
                self.y += self.aim * cmd.value;
            }
            Dir::Down => {
                self.aim += cmd.value;
            }
            Dir::Up => {
                self.aim -= cmd.value;
            }
        }
    }
}

impl From<State> for i32 {
    fn from(state: State) -> Self {
        state.x * state.y
    }
}

// Main functions

fn process(bufin: impl BufRead) -> Result<i32> {
    let mut state = State::default();
    for line_opt in bufin.lines() {
        let line = line_opt?;
        let cmd = line.parse::<Command>()?;
        state += cmd;
    }
    Ok(state.into())
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY02;
    assert_eq!(process(input.as_bytes())?, 900);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
