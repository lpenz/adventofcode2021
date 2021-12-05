// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Result;
use sqrid::Qr;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

type Sqrid = sqrid::sqrid_create!(1000, 1000, false);
type Qa = sqrid::qa_create!(Sqrid);

// Parser

pub mod parser {
    use crate::Qa;
    use anyhow::{anyhow, Result};
    use nom::{
        bytes::complete::tag, character, character::complete::char, combinator::all_consuming,
        multi::*, IResult,
    };
    use std::io::BufRead;

    pub fn coords(input: &str) -> IResult<&str, Qa> {
        let (input, x) = character::complete::u16(input)?;
        let (input, _) = char(',')(input)?;
        let (input, y) = character::complete::u16(input)?;
        let qa = Qa::tryfrom_tuple((x, y)).unwrap(); // TODO: use error here
        Ok((input, qa))
    }

    pub fn line(input: &str) -> IResult<&str, (Qa, Qa)> {
        let (input, orig) = coords(input)?;
        let (input, _) = tag(" -> ")(input)?;
        let (input, dest) = coords(input)?;
        let (input, _) = char('\n')(input)?;
        Ok((input, (orig, dest)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(Qa, Qa)>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = all_consuming(many1(line))(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

// Main functions

fn process(bufin: impl BufRead) -> Result<usize> {
    let lines = parser::parse(bufin)?;
    let mut g: HashMap<Qa, usize> = HashMap::new();
    for (src, dst) in lines.into_iter() {
        let tsrc = src.tuple();
        let tdst = dst.tuple();
        let qr = if tsrc.0 == tdst.0 && tsrc.1 > tdst.1 {
            Qr::N
        } else if tsrc.0 == tdst.0 && tsrc.1 <= tdst.1 {
            Qr::S
        } else if tsrc.1 == tdst.1 && tsrc.0 > tdst.0 {
            Qr::W
        } else if tsrc.1 == tdst.1 && tsrc.0 <= tdst.0 {
            Qr::E
        } else if tsrc.0 > tdst.0 && tsrc.1 > tdst.1 {
            Qr::NW
        } else if tsrc.0 > tdst.0 && tsrc.1 < tdst.1 {
            Qr::SW
        } else if tsrc.0 < tdst.0 && tsrc.1 > tdst.1 {
            Qr::NE
        } else if tsrc.0 < tdst.0 && tsrc.1 < tdst.1 {
            Qr::SE
        } else {
            continue;
        };
        let mut qa = src;
        while qa != dst {
            let entry = g.entry(qa).or_insert(0);
            *entry += 1;
            qa = (qa + qr).ok_or_else(|| {
                anyhow!(
                    "error moving from {} with direction {}; src {} dst {}",
                    qa,
                    qr,
                    src,
                    dst
                )
            })?;
        }
        let entry = g.entry(qa).or_insert(0);
        *entry += 1;
    }
    let result = g.into_values().filter(|&v| v > 1).count();
    Ok(result)
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY05;
    assert_eq!(process(input.as_bytes())?, 12);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
