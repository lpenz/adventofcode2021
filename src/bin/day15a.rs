// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

// use anyhow::anyhow;
use anyhow::Result;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

type Sqrid = sqrid::sqrid_create!(100, 100, false);
type Qa = sqrid::qa_create!(Sqrid);
type Grid = sqrid::grid_create!(Sqrid, u8);

// Parser

pub mod parser {
    use crate::Grid;
    use crate::Qa;
    use anyhow::{anyhow, Result};
    use itertools::Itertools;
    use nom::{character, combinator, combinator::all_consuming, multi::*, IResult};
    use std::io::BufRead;

    pub fn line(input: &str) -> IResult<&str, Vec<u8>> {
        let (input, numbers) = many1(combinator::map(
            character::complete::one_of("0123456789"),
            |c| c as u8 - b'0',
        ))(input)?;
        let (input, _) = character::complete::newline(input)?;
        Ok((input, numbers))
    }

    pub fn grid(input: &str) -> IResult<&str, (Grid, Qa)> {
        let (input, lines) = many1(line)(input)?;
        let numx = lines[0].len();
        let numy = lines.len();
        let mut grid = Grid::repeat(u8::MAX);
        grid.extend((0..numx).cartesian_product(0..numy).map(|(x, y)| {
            let qa = Qa::tryfrom_tuple((x as u16, y as u16)).unwrap();
            (qa, lines[y][x])
        }));
        let last = Qa::tryfrom_tuple((numx as u16 - 1, numy as u16 - 1)).unwrap();
        Ok((input, (grid, last)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Grid, Qa)> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = all_consuming(grid)(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

// Main functions

fn go(grid: &Grid, qa: Qa, qr: sqrid::Qr) -> Option<(Qa, usize)> {
    let nextqa = (qa + qr)?;
    let cost = grid[nextqa] as sqrid::Cost;
    Some((nextqa, cost))
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let (grid, bottomright) = parser::parse(bufin)?;
    let path = Sqrid::ucs_path(|qa, qr| go(&grid, qa, qr), &Qa::TOP_LEFT, &bottomright)?;
    let cost: sqrid::Cost = path
        .iter()
        .scan(Qa::TOP_LEFT, |qa, qr| {
            *qa = (*qa + qr)?;
            Some(grid[qa] as usize)
        })
        .sum();
    Ok(cost)
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY15;
    assert_eq!(process(input.as_bytes())?, 40);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
