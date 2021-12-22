// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

type Sqrid = sqrid::sqrid_create!(2000, 2000, false);
type Qa = sqrid::qa_create!(Sqrid);

type SqridFolded = sqrid::sqrid_create!(40, 6, false);
type QaFolded = sqrid::qa_create!(SqridFolded);
type GridboolFolded = sqrid::gridbool_create!(SqridFolded);

// Parser

pub mod parser {
    use crate::Qa;
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::{bytes, character, combinator, multi::*, IResult};
    use std::io::BufRead;

    type Fold = (bool, u16);

    pub fn point(input: &str) -> IResult<&str, Qa> {
        let (input, qa) = combinator::map_res(
            |input| {
                let (input, x) = character::complete::u16(input)?;
                let (input, _) = character::complete::char(',')(input)?;
                let (input, y) = character::complete::u16(input)?;
                let (input, _) = character::complete::char('\n')(input)?;
                Ok((input, (x, y)))
            },
            Qa::tryfrom_tuple,
        )(input)?;
        Ok((input, qa))
    }

    pub fn fold(input: &str) -> IResult<&str, Fold> {
        let (input, _) = bytes::complete::tag("fold along ")(input)?;
        let (input, axis) = character::complete::one_of("xy")(input)?;
        let (input, _) = character::complete::char('=')(input)?;
        let (input, coord) = character::complete::u16(input)?;
        let (input, _) = character::complete::char('\n')(input)?;
        Ok((input, (axis == 'x', coord)))
    }

    pub fn all(input: &str) -> IResult<&str, (Vec<Qa>, Vec<Fold>)> {
        let (input, points) = many1(point)(input)?;
        let (input, _) = character::complete::char('\n')(input)?;
        let (input, folds) = many1(fold)(input)?;
        Ok((input, (points, folds)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Vec<Qa>, Vec<Fold>)> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(all)(&input);
        match result {
            Ok((_, info)) => Ok(info),
            Err(e) => Err(anyhow!("error reading input: {:?}", e)),
        }
    }
}

// Main functions

type Paper = HashSet<Qa>;

fn process(bufin: impl BufRead) -> Result<usize> {
    let (points, folds) = parser::parse(bufin)?;
    let mut paper = points.iter().cloned().collect::<Paper>();
    for fold in folds {
        let oldpaper = std::mem::take(&mut paper);
        for point in oldpaper {
            let t = point.tuple();
            if fold.0 {
                let x = fold.1;
                if t.0 < x {
                    paper.insert(point);
                } else {
                    paper.insert(Qa::tryfrom_tuple((2 * x - t.0, t.1))?);
                }
            } else {
                let y = fold.1;
                if t.1 < y {
                    paper.insert(point);
                } else {
                    paper.insert(Qa::tryfrom_tuple((t.0, 2 * y - t.1))?);
                }
            }
        }
    }
    let mut g = GridboolFolded::default();
    for qa0 in &paper {
        let qa = QaFolded::tryfrom_tuple(qa0.tuple())?;
        g.set_t(qa);
    }
    eprintln!("{}", g);
    Ok(paper.len())
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY13;
    assert_eq!(process(input.as_bytes())?, 16);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
