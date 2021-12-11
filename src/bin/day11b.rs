// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Result;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

type Energy = i32;
type Sqrid = sqrid::sqrid_create!(10, 10, true);
type Qa = sqrid::qa_create!(Sqrid);
type Grid = sqrid::grid_create!(Sqrid, Energy);
type Gridbool = sqrid::gridbool_create!(Sqrid);

// Parser

pub mod parser {
    use crate::Energy;
    use crate::Grid;
    use crate::Qa;
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::{character, combinator, multi::*, IResult};
    use std::io::BufRead;

    pub fn energy(input: &str) -> IResult<&str, Energy> {
        let (input, energy) = combinator::map_res(
            |input| {
                let (input, s) = character::complete::one_of("0123456789")(input)?;
                Ok((input, s))
            },
            |s| format!("{}", s).parse::<Energy>(),
        )(input)?;
        Ok((input, energy))
    }

    pub fn line(input: &str) -> IResult<&str, Vec<Energy>> {
        let (input, energys) = many1(energy)(input)?;
        let (input, _) = character::complete::char('\n')(input)?;
        Ok((input, energys))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Grid> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(many1(line))(&input);
        match result {
            Ok((_, lines)) => {
                let mut g = Grid::repeat(0);
                for qa in Qa::iter() {
                    let t = qa.tuple();
                    g[qa] = lines[t.1 as usize][t.0 as usize];
                }
                Ok(g)
            }
            Err(e) => Err(anyhow!("error reading input: {:?}", e)),
        }
    }
}

// Main functions

fn process(bufin: impl BufRead) -> Result<usize> {
    let mut grid = parser::parse(bufin)?;
    for step in 1..usize::MAX {
        // Increase energy
        for qa in Qa::iter() {
            grid[qa] += 1;
        }
        // Flashes
        let mut flashed = Gridbool::default();
        let mut numflashed = 0;
        loop {
            let mut done = true;
            for qa0 in Qa::iter() {
                if grid[qa0] > 9 && !flashed[qa0] {
                    flashed.set_t(qa0);
                    numflashed += 1;
                    done = false;
                    for qa in sqrid::Qr::iter::<true>().filter_map(|qr| qa0 + qr) {
                        grid[qa] += 1;
                    }
                }
            }
            if done {
                break;
            }
        }
        // Zero flashed
        for qa in Qa::iter() {
            if flashed[qa] {
                grid[qa] = 0;
            }
        }
        if numflashed == 100 {
            return Ok(step);
        }
    }
    Err(anyhow!("no simultaneous flash turn"))
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY11;
    assert_eq!(process(input.as_bytes())?, 195);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
