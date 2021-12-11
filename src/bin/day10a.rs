// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use std::io::{stdin, BufRead};
use std::str::FromStr;

extern crate adventofcode2021;

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Open,
    Close,
}

impl FromStr for Op {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if matches!(s, "(" | "[" | "{" | "<") {
            Ok(Op::Open)
        } else if matches!(s, ")" | "]" | "}" | ">") {
            Ok(Op::Close)
        } else {
            Err(anyhow!("unrecognized op {}", s))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Typ {
    Paren,
    Square,
    Curl,
    Angle,
}

impl Typ {
    pub fn score(&self) -> i32 {
        match self {
            Self::Paren => 3,
            Self::Square => 57,
            Self::Curl => 1197,
            Self::Angle => 25137,
        }
    }
}

impl FromStr for Typ {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if matches!(s, "(" | ")") {
            Ok(Typ::Paren)
        } else if matches!(s, "[" | "]") {
            Ok(Typ::Square)
        } else if matches!(s, "{" | "}") {
            Ok(Typ::Curl)
        } else if matches!(s, "<" | ">") {
            Ok(Typ::Angle)
        } else {
            Err(anyhow!("unrecognized type {}", s))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    op: Op,
    typ: Typ,
}

impl Token {
    pub fn is_open(&self) -> bool {
        self.op == Op::Open
    }
}

impl FromStr for Token {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Token {
            op: s.parse()?,
            typ: s.parse()?,
        })
    }
}

// Parser

pub mod parser {
    use crate::Token;
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::{character, combinator::all_consuming, combinator::map_res, multi::*, IResult};
    use std::io::BufRead;

    pub fn token(input: &str) -> IResult<&str, Token> {
        let (input, height) = map_res(
            |input| {
                let (input, s) = character::complete::one_of("([{<>}])")(input)?;
                Ok((input, s))
            },
            |s| format!("{}", s).parse::<Token>(),
        )(input)?;
        Ok((input, height))
    }

    pub fn line(input: &str) -> IResult<&str, Vec<Token>> {
        let (input, heights) = many1(token)(input)?;
        let (input, _) = character::complete::char('\n')(input)?;
        Ok((input, heights))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<Token>>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = all_consuming(many1(line))(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

// Main functions

fn process(bufin: impl BufRead) -> Result<i32> {
    let lines = parser::parse(bufin)?;
    let mut score = 0;
    for line in lines {
        let mut stack = vec![];
        for token in line {
            if token.is_open() {
                stack.push(token);
            } else {
                let ok = stack.pop().filter(|t| t.typ == token.typ).is_some();
                if !ok {
                    score += token.typ.score();
                    break;
                }
            }
        }
    }
    Ok(score)
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY10;
    assert_eq!(process(input.as_bytes())?, 26397);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
