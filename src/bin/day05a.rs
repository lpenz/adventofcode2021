// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Result;
use sqrid::Qr;
use std::collections::HashMap;
use std::io;

extern crate adventofcode2021;

type Sqrid = sqrid::sqrid_create!(1000, 1000, false);
type Qa = sqrid::qa_create!(Sqrid);

// Parser

pub mod parser {
    use crate::Qa;
    use anyhow::{anyhow, Result};
    use nom::{
        bytes,
        character,
        // character::streaming::char, // , combinator::all_consuming,
        combinator::eof,
        combinator::map_res,
        multi::*,
        IResult,
    };
    use nom_bufreader::bufreader::BufReader;
    use nom_bufreader::Parse;
    use std::io;

    pub fn coords(input: &[u8]) -> IResult<&[u8], Qa> {
        let (input, qa) = map_res(
            |input| {
                let (input, x) = character::streaming::u16(input)?;
                let (input, _) = character::streaming::char(',')(input)?;
                let (input, y) = character::streaming::u16(input)?;
                Ok((input, (x, y)))
            },
            |xy| Qa::tryfrom_tuple(xy),
        )(input)?;
        Ok((input, qa))
    }

    pub fn line(input: &[u8]) -> IResult<&[u8], (Qa, Qa)> {
        let (input, orig) = coords(input)?;
        let (input, _) = bytes::streaming::tag(" -> ")(input)?;
        let (input, dest) = coords(input)?;
        let (input, _) = character::streaming::char('\n')(input)?;
        Ok((input, (orig, dest)))
    }

    pub fn lines(input: &[u8]) -> IResult<&[u8], Vec<(Qa, Qa)>> {
        let (input, lines) = many1(line)(&input)?;
        let (input, _) = eof(input)?;
        Ok((input, lines))
    }

    pub fn do_parse(mut bufin: BufReader<&[u8]>) -> Result<Vec<(Qa, Qa)>> {
        bufin
            .parse(lines)
            .map_err(|e| anyhow!("error reading input: {:?}", e))
    }

    pub fn parse(mut reader: impl io::Read) -> Result<Vec<(Qa, Qa)>> {
        let mut bufin = BufReader::new(reader);
        do_parse(bufin)
    }
}

// Main functions

fn process(reader: impl io::Read) -> Result<usize> {
    let lines = parser::parse(reader)?;
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
    assert_eq!(process(input.as_bytes())?, 5);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(io::stdin().lock())?);
    Ok(())
}
