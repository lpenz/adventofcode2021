// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

/// Wire/segment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Digit(HashSet<Ws>);

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
        Ok((input, Digit(digit.into_iter().collect())))
    }

    pub fn line(input: &str) -> IResult<&str, ([Digit; 10], [Digit; 4])> {
        let (input, signals_vec) = separated_list1(tag(" "), digit)(input)?;
        let (input, _) = tag(" | ")(input)?;
        let (input, output_vec) = separated_list1(tag(" "), digit)(input)?;
        let (input, _) = char('\n')(input)?;
        let mut signals: [Digit; 10] = Default::default();
        for (k, v) in signals_vec.into_iter().enumerate() {
            signals[k] = v;
        }
        let mut output: [Digit; 4] = Default::default();
        for (k, v) in output_vec.into_iter().enumerate() {
            output[k] = v;
        }
        Ok((input, (signals, output)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<([Digit; 10], [Digit; 4])>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = all_consuming(many1(line))(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

//   aaaa
//  b    c
//  b    c
//   dddd
//  e    f
//  e    f
//   gggg
// 2: 1
// 3: 7
// 4: 4
// 5: 2 3 5
// 6: 0 6 9
// 7: 8

// Main functions

fn getlen<'a>(len: usize, mut iter: impl Iterator<Item = &'a Digit>) -> Result<Digit> {
    iter.find(|d| d.0.len() == len)
        .cloned()
        .ok_or_else(|| anyhow!("digit with len {} not found", len))
}

fn onlylen<'a>(
    len: usize,
    iter: impl Iterator<Item = &'a Digit>,
) -> impl Iterator<Item = &'a Digit> {
    iter.filter(move |d| d.0.len() == len)
}

fn intersection(d1: &Digit, d2: &Digit) -> Digit {
    Digit(d1.0.intersection(&d2.0).cloned().collect::<HashSet<Ws>>())
}

fn difference(d1: &Digit, d2: &Digit) -> Digit {
    Digit(d1.0.difference(&d2.0).cloned().collect::<HashSet<Ws>>())
}

fn singlecond<'a, F>(iter: impl Iterator<Item = &'a Digit>, cond: F) -> Result<Digit>
where
    F: Fn(&Digit) -> bool,
{
    let v = iter.cloned().filter(cond).collect::<Vec<_>>();
    if v.len() != 1 {
        Err(anyhow!("expected 1 result, found {}", v.len()))
    } else {
        Ok(v[0].clone())
    }
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let lines = parser::parse(bufin)?;
    let mut output = 0;
    for line in lines {
        let one = getlen(2, line.0.iter())?;
        let seven = getlen(3, line.0.iter())?;
        let four = getlen(4, line.0.iter())?;
        let three = singlecond(onlylen(5, line.0.iter()), |digit| {
            let inter = intersection(&seven, digit);
            inter.0.len() == 3
        })?;
        let horiz = difference(&three, &seven);
        let d = intersection(&horiz, &four)
            .0
            .iter()
            .next()
            .cloned()
            .unwrap();
        let two = singlecond(onlylen(5, line.0.iter()), |digit| {
            let inter = intersection(&four, digit);
            inter.0.len() == 2
        })?;
        let five = singlecond(onlylen(5, line.0.iter()), |digit| {
            *digit != two && *digit != three
        })?;
        let nine = singlecond(onlylen(6, line.0.iter()), |digit| {
            let inter = intersection(&four, digit);
            inter.0.len() == 4
        })?;
        let zero = singlecond(onlylen(6, line.0.iter()), |digit| !digit.0.contains(&d))?;
        let six = singlecond(onlylen(6, line.0.iter()), |digit| {
            *digit != zero && *digit != nine
        })?;
        let eight = getlen(7, line.0.iter())?;
        // Create a function map with all digits:
        let convert = |d: &Digit| {
            if *d == zero {
                0
            } else if *d == one {
                1
            } else if *d == two {
                2
            } else if *d == three {
                3
            } else if *d == four {
                4
            } else if *d == five {
                5
            } else if *d == six {
                6
            } else if *d == seven {
                7
            } else if *d == eight {
                8
            } else if *d == nine {
                9
            } else {
                panic!("could not find digit")
            }
        };
        // Calculate the output:
        let mut mult = 1000;
        for o in &line.1 {
            output += convert(o) * mult;
            mult /= 10;
        }
    }
    Ok(output)
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY08;
    assert_eq!(process(input.as_bytes())?, 61229);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
