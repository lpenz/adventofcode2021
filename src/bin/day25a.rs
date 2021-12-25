// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use std::collections;
use std::convert::TryFrom;
use std::fmt;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

// Parser

pub mod parser {
    use crate::Cumber;
    use anyhow::{anyhow, Result};
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::convert::TryFrom;
    use std::io::BufRead;

    fn cumber(input: &str) -> IResult<&str, Option<Cumber>> {
        combinator::map_res(character::one_of(".>v"), |c| {
            if c == '.' {
                Ok(None)
            } else {
                Cumber::try_from(c).map(Some)
            }
        })(input)
    }

    fn line(input: &str) -> IResult<&str, Vec<Option<Cumber>>> {
        let (input, cumbers) = multi::many1(cumber)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, cumbers))
    }

    fn all(input: &str) -> IResult<&str, Vec<Vec<Option<Cumber>>>> {
        let (input, cumbers2) = multi::many1(line)(input)?;
        Ok((input, cumbers2))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<Option<Cumber>>>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(all)(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

type Sqrid = sqrid::sqrid_create!(10, 9, false);
// type Sqrid = sqrid::sqrid_create!(139, 137, false); for the actual input
type Qa = sqrid::qa_create!(Sqrid);
type Qr = sqrid::Qr;
type GridDisplay = sqrid::grid_create!(Sqrid, char);

pub fn go(qa0: &Qa, qr: Qr) -> Option<Qa> {
    if let Some(qa) = *qa0 + qr {
        Some(qa)
    } else {
        let t = qa0.tuple();
        let x = if qr == Qr::E && t.0 == Qa::WIDTH - 1 {
            0
        } else {
            t.0
        };
        let y = if qr == Qr::S && t.1 == Qa::HEIGHT - 1 {
            0
        } else {
            t.1
        };
        Qa::try_from((x, y)).ok()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Cumber {
    E,
    S,
}

impl TryFrom<char> for Cumber {
    type Error = Error;
    fn try_from(s: char) -> Result<Self, Self::Error> {
        Ok(match s {
            '>' => Self::E,
            'v' => Self::S,
            _ => return Err(anyhow!("invalid cocumber direction {}", s)),
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct State {
    east: collections::HashSet<Qa>,
    south: collections::HashSet<Qa>,
}

impl State {
    pub fn step_dir(qr: Qr, map: &mut collections::HashSet<Qa>, other: &collections::HashSet<Qa>) {
        let movers = map
            .iter()
            .filter(|qa0| {
                if let Some(qa) = go(qa0, qr) {
                    !map.contains(&qa) && !other.contains(&qa)
                } else {
                    false
                }
            })
            .cloned()
            .collect::<Vec<_>>();
        for qa in movers {
            map.remove(&qa);
            map.insert(go(&qa, qr).unwrap());
        }
    }
    pub fn step(&mut self) {
        State::step_dir(Qr::E, &mut self.east, &self.south);
        State::step_dir(Qr::S, &mut self.south, &self.east);
    }
}

impl TryFrom<Vec<Vec<Option<Cumber>>>> for State {
    type Error = Error;
    fn try_from(cumbersvec: Vec<Vec<Option<Cumber>>>) -> Result<Self, Self::Error> {
        let mut state = State::default();
        for (y, line) in cumbersvec.into_iter().enumerate() {
            for (x, cell) in line.into_iter().enumerate() {
                if let Some(cumber) = cell {
                    let qa = Qa::try_from((x as u16, y as u16))?;
                    match cumber {
                        Cumber::E => {
                            state.east.insert(qa);
                        }
                        Cumber::S => {
                            state.south.insert(qa);
                        }
                    }
                }
            }
        }
        Ok(state)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut g = GridDisplay::repeat('.');
        g.extend(self.east.iter().map(|qa| (*qa, '>')));
        g.extend(self.south.iter().map(|qa| (*qa, 'v')));
        write!(f, "{}", g)
    }
}

// Main functions

pub fn process(bufin: impl BufRead) -> Result<usize> {
    let cocumbers = parser::parse(bufin)?;
    let mut state = State::try_from(cocumbers)?;
    for i in 1..usize::MAX {
        let last = state.clone();
        state.step();
        if state == last {
            return Ok(i);
        }
    }
    Err(anyhow!("no stable state found"))
}

#[test]
fn test() -> Result<()> {
    pub const INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";
    assert_eq!(process(INPUT.as_bytes())?, 58);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
