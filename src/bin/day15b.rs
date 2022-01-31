// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

// use anyhow::anyhow;
use anyhow::Result;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

type Sqrid1 = sqrid::sqrid_create!(100, 100, false);
type Qa1 = sqrid::qa_create!(Sqrid1);
type Grid1 = sqrid::grid_create!(Sqrid1, u8);

type Sqrid5 = sqrid::sqrid_create!(500, 500, false);
type Qa5 = sqrid::qa_create!(Sqrid5);
// type Grid5 = sqrid::grid_create!(Sqrid5, u8);

// Parser

pub mod parser {
    use crate::Grid1;
    use crate::Qa1;
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

    pub fn grid(input: &str) -> IResult<&str, (Grid1, Qa1)> {
        let (input, lines) = many1(line)(input)?;
        let numx = lines[0].len();
        let numy = lines.len();
        let mut grid = Grid1::repeat(u8::MAX);
        grid.extend((0..numx).cartesian_product(0..numy).map(|(x, y)| {
            let qa = Qa1::tryfrom_tuple((x as u16, y as u16)).unwrap();
            (qa, lines[y][x])
        }));
        let last = Qa1::tryfrom_tuple((numx as u16 - 1, numy as u16 - 1)).unwrap();
        Ok((input, (grid, last)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Grid1, Qa1)> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = all_consuming(grid)(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

// Main functions

fn gridget(grid1: &Grid1, bottomright1: &Qa1, qa5: Qa5) -> Result<u8> {
    let t = qa5.tuple();
    let tlim = bottomright1.tuple();
    let qa1 = Qa1::tryfrom_tuple((t.0 % (tlim.0 + 1), t.1 % (tlim.1 + 1)))?;
    let dx = t.0 / (tlim.0 + 1);
    let dy = t.1 / (tlim.1 + 1);
    Ok(((grid1[qa1] as u16 + dx + dy - 1) % 9 + 1) as u8)
}

fn go(
    grid1: &Grid1,
    bottomright1: &Qa1,
    bottomright5: &Qa5,
    qa: Qa5,
    qr: sqrid::Qr,
) -> Option<(Qa5, usize)> {
    let nextqa = (qa + qr)?;
    if !nextqa.inside(Qa5::TOP_LEFT, bottomright5) {
        return None;
    }
    let cost = gridget(grid1, bottomright1, nextqa).ok()? as sqrid::Cost;
    Some((nextqa, cost))
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let (grid1, bottomright1) = parser::parse(bufin)?;
    let tbottomright1 = bottomright1.tuple();
    let bottomright5 =
        Qa5::tryfrom_tuple(((tbottomright1.0 + 1) * 5 - 1, (tbottomright1.1 + 1) * 5 - 1))?;
    let path = Sqrid5::ucs_path_hashmap(
        |qa, qr| go(&grid1, &bottomright1, &bottomright5, qa, qr),
        &Qa5::TOP_LEFT,
        &bottomright5,
    )?;
    let cost: sqrid::Cost = path
        .iter()
        .scan(Qa5::TOP_LEFT, |qa, qr| {
            *qa = (*qa + qr)?;
            Some(gridget(&grid1, &bottomright1, *qa).unwrap() as usize)
        })
        .sum();
    Ok(cost)
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY15;
    assert_eq!(process(input.as_bytes())?, 315);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
