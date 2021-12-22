// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

// Parser

pub mod parser {
    use crate::Step;
    use crate::Xyz;
    use anyhow::{anyhow, Result};
    use nom::branch;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    fn onoff(input: &str) -> IResult<&str, bool> {
        let (input, onoff) = branch::alt((bytes::tag("on"), bytes::tag("off")))(input)?;
        Ok((input, onoff == "on"))
    }

    fn range(input: &str) -> IResult<&str, (i32, i32)> {
        let (input, ini) = character::i32(input)?;
        let (input, _) = bytes::tag("..")(input)?;
        let (input, end) = character::i32(input)?;
        Ok((input, (ini, end)))
    }

    fn line(input: &str) -> IResult<&str, Step> {
        let (input, onoff) = onoff(input)?;
        let (input, _) = bytes::tag(" x=")(input)?;
        let (input, (xini, xend)) = range(input)?;
        let (input, _) = bytes::tag(",y=")(input)?;
        let (input, (yini, yend)) = range(input)?;
        let (input, _) = bytes::tag(",z=")(input)?;
        let (input, (zini, zend)) = range(input)?;
        let (input, _) = character::newline(input)?;
        Ok((
            input,
            Step {
                on: onoff,
                ini: Xyz::new(xini, yini, zini),
                end: Xyz::new(xend, yend, zend),
            },
        ))
    }

    fn all(input: &str) -> IResult<&str, Vec<Step>> {
        multi::many1(line)(input)
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Step>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(all)(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Xyz {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Xyz {
    pub fn new(x: i32, y: i32, z: i32) -> Xyz {
        Xyz { x, y, z }
    }
    pub fn valid(&self) -> bool {
        self.x >= -50
            && self.x <= 50
            && self.y >= -50
            && self.y <= 50
            && self.z >= -50
            && self.z <= 50
    }
}

pub fn iter_cubes(ini: Xyz, end: Xyz) -> impl Iterator<Item = Xyz> {
    (ini.x..=end.x)
        .cartesian_product(ini.y..=end.y)
        .cartesian_product(ini.z..=end.z)
        .map(|((x, y), z)| Xyz::new(x, y, z))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Step {
    pub on: bool,
    pub ini: Xyz,
    pub end: Xyz,
}

// Main functions

pub fn process(bufin: impl BufRead) -> Result<usize> {
    let steps = parser::parse(bufin)?;
    let mut on = HashSet::<Xyz>::new();
    for step in steps {
        if !step.ini.valid() || !step.end.valid() {
            continue;
        }
        eprintln!("step {:?}", step);
        if step.on {
            for xyz in iter_cubes(step.ini, step.end) {
                on.insert(xyz);
            }
        } else {
            for xyz in iter_cubes(step.ini, step.end) {
                on.remove(&xyz);
            }
        }
    }
    Ok(on.len())
}

pub const DAY22: &str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682
";

#[test]
fn test() -> Result<()> {
    assert_eq!(process(DAY22.as_bytes())?, 590784);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
