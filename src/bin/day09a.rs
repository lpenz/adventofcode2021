// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

type Height = i32;

// Parser

pub mod parser {
    use crate::Height;
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::{character, combinator::all_consuming, combinator::map_res, multi::*, IResult};
    use std::io::BufRead;

    pub fn height(input: &str) -> IResult<&str, Height> {
        let (input, height) = map_res(
            |input| {
                let (input, s) = character::complete::one_of("0123456789")(input)?;
                Ok((input, s))
            },
            |s| format!("{}", s).parse::<Height>(),
        )(input)?;
        Ok((input, height))
    }

    pub fn line(input: &str) -> IResult<&str, Vec<Height>> {
        let (input, heights) = many1(height)(input)?;
        let (input, _) = character::complete::char('\n')(input)?;
        Ok((input, heights))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<Height>>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = all_consuming(many1(line))(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

// Main functions

fn neighs(xmax: usize, ymax: usize, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let mut state = 0;
    std::iter::from_fn(move || {
        if state < 1 && y > 0 {
            state = 1;
            return Some((x, y - 1));
        }
        if state < 2 && y < ymax - 1 {
            state = 2;
            return Some((x, y + 1));
        }
        if state < 3 && x > 0 {
            state = 3;
            return Some((x - 1, y));
        }
        if state < 4 && x < xmax - 1 {
            state = 4;
            return Some((x + 1, y));
        }
        None
    })
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let grid = parser::parse(bufin)?;
    let ymax = grid.len();
    let xmax = grid[0].len();
    let mut risk = 0;
    for y in 0..ymax {
        for x in 0..xmax {
            if neighs(xmax, ymax, x, y).all(|n| grid[n.1][n.0] > grid[y][x]) {
                risk += 1 + grid[y][x];
            }
        }
    }
    Ok(risk)
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY09;
    assert_eq!(process(input.as_bytes())?, 15);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
