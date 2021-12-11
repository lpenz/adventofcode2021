// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

type Height = i32;
type Sqrid = sqrid::sqrid_create!(100, 100, false);
type Qa = sqrid::qa_create!(Sqrid);
type Grid = sqrid::grid_create!(Sqrid, Height);

// Parser

pub mod parser {
    use crate::Grid;
    use crate::Height;
    use crate::Qa;
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::{character, combinator, multi::*, IResult};
    use std::convert::TryFrom;
    use std::io::BufRead;

    pub fn height(input: &str) -> IResult<&str, Height> {
        let (input, height) = combinator::map_res(
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

    pub fn parse(mut bufin: impl BufRead) -> Result<(Grid, Qa)> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(many1(line))(&input);
        match result {
            Ok((_, lines)) => {
                let last = Qa::try_from((lines[0].len() as u16 - 1, lines.len() as u16 - 1))?;
                let mut g = Grid::repeat(9);
                for qa in Qa::iter_range(Qa::FIRST, last) {
                    let t = qa.tuple();
                    g[qa] = lines[t.1 as usize][t.0 as usize];
                }
                Ok((g, last))
            }
            Err(e) => Err(anyhow!("error reading input: {:?}", e)),
        }
    }
}

// Main functions

fn process(bufin: impl BufRead) -> Result<usize> {
    let (grid, last) = parser::parse(bufin)?;
    let mut basins = HashMap::<Qa, i32>::new();
    let mut basinsizes = HashMap::<i32, usize>::new();
    let mut numbasin = 0;
    for height0 in (0..9).rev() {
        for qa0 in Qa::iter_range(Qa::FIRST, last) {
            if grid[qa0] != height0 || basins.contains_key(&qa0) {
                continue;
            }
            let ibasin = numbasin;
            numbasin += 1;
            basins.insert(qa0, ibasin);
            basinsizes.insert(ibasin, 1);
            for (qa, _) in Sqrid::bf_iter(
                |qa, qr| sqrid::qaqr_eval(qa, qr).filter(|qa| grid[qa] <= height0),
                &qa0,
            )
            .flatten()
            {
                basins.insert(qa, ibasin);
                if let Some(e) = basinsizes.get_mut(&ibasin) {
                    *e += 1;
                }
            }
        }
    }
    Ok(basinsizes.values().sorted().rev().take(3).product())
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY09;
    assert_eq!(process(input.as_bytes())?, 1134);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
