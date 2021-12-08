// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use std::convert::TryFrom;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

/// Wire/segment
#[derive(Debug)]
pub enum Ws {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl TryFrom<char> for Ws {
    type Error = Error;
    fn try_from(s: char) -> Result<Self, Self::Error> {
        Ok(match s {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            'e' => Self::E,
            'f' => Self::F,
            'g' => Self::G,
            _ => return Err(anyhow!("invalid wire/segment {}", s)),
        })
    }
}

type Digit = Vec<Ws>;

// Parser

pub mod parser {
    use crate::Digit;
    use crate::Ws;
    use anyhow::{anyhow, Result};
    use nom::{
        bytes::complete::tag, character, character::complete::char, combinator::all_consuming,
        combinator::map_res, multi::*, IResult,
    };
    use std::convert::TryFrom;
    use std::io::BufRead;

    pub fn ws(input: &str) -> IResult<&str, Ws> {
        let (input, ws) = map_res(
            |input| {
                let (input, ws_str) = character::complete::one_of("abcdefg")(input)?;
                Ok((input, ws_str))
            },
            Ws::try_from,
        )(input)?;
        Ok((input, ws))
    }

    pub fn digit(input: &str) -> IResult<&str, Digit> {
        let (input, digit) = many1(ws)(input)?;
        Ok((input, digit))
    }

    pub fn line(input: &str) -> IResult<&str, (Vec<Digit>, Vec<Digit>)> {
        let (input, signals) = separated_list1(tag(" "), digit)(input)?;
        let (input, _) = tag(" | ")(input)?;
        let (input, output) = separated_list1(tag(" "), digit)(input)?;
        let (input, _) = char('\n')(input)?;
        Ok((input, (signals, output)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(Vec<Digit>, Vec<Digit>)>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = all_consuming(many1(line))(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

// Main functions

fn wanted(d: &[Ws]) -> bool {
    matches!(d.len(), 2 | 3 | 4 | 7)
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let lines = parser::parse(bufin)?;
    Ok(lines
        .iter()
        .map(|(_, output)| output.iter().filter(|d| wanted(*d)).count())
        .sum())
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY08;
    assert_eq!(process(input.as_bytes())?, 26);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
