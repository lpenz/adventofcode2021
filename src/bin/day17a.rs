// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

// Parser

pub mod parser {
    use crate::Pos;
    use anyhow::{anyhow, Result};
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::IResult;
    use std::io::BufRead;

    type Minmax = (Pos, Pos);

    fn pos(input: &str) -> IResult<&str, Pos> {
        combinator::map(character::i32, Pos)(input)
    }

    fn line(input: &str) -> IResult<&str, (Minmax, Minmax)> {
        let (input, _) = bytes::tag("target area: x=")(input)?;
        let (input, xpmin) = pos(input)?;
        let (input, _) = bytes::tag("..")(input)?;
        let (input, xpmax) = pos(input)?;
        let (input, _) = bytes::tag(", y=")(input)?;
        let (input, ypmin) = pos(input)?;
        let (input, _) = bytes::tag("..")(input)?;
        let (input, ypmax) = pos(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, ((xpmin, xpmax), (ypmin, ypmax))))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Minmax, Minmax)> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(line)(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Step(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vel(i32);

fn xpos_at_step(vel: Vel, step: Step) -> Pos {
    let step = Step(std::cmp::min(vel.0, step.0));
    Pos(vel.0 - ((step.0 - 2 * vel.0) * (step.0 - 1)) / 2)
}

fn ypos_at_step(vel: Vel, step: Step) -> Pos {
    Pos(vel.0 - ((step.0 - 2 * vel.0) * (step.0 - 1)) / 2)
}

fn step_top_pos(vel: Vel) -> Step {
    Step((2 * vel.0 + 1) / 2)
}

fn steps_min(xpmin: Pos) -> Step {
    for xv0 in (1..i32::MAX).map(Vel) {
        let step = Step(xv0.0);
        let xp = xpos_at_step(xv0, step);
        if xp >= xpmin {
            return step;
        }
    }
    panic!("xp_minmax not found");
}

fn step_for_pos(vel0: Vel, p: Pos) -> Option<f64> {
    let a = -0.5_f64;
    let b = vel0.0 as f64 + 0.5_f64;
    let c = -p.0 as f64;
    let delta = (b * b - 4_f64 * a * c).sqrt();
    let root1 = (-b + delta) / (2_f64 * a);
    let root2 = (-b - delta) / (2_f64 * a);
    if root1 > 0_f64 {
        Some(root1)
    } else if root2 > 0_f64 {
        Some(root2)
    } else {
        None
    }
}

// Main functions

pub fn process(bufin: impl BufRead) -> Result<i32> {
    let ((xpmin, _xpmax), (ypmin, ypmax)) = parser::parse(bufin)?;
    let mut best = Pos(i32::MIN);
    let x_stepmin = steps_min(xpmin);
    for yv0 in (1..i32::MAX).map(Vel) {
        let stepmin = step_for_pos(yv0, ypmax);
        let stepmax = step_for_pos(yv0, ypmin);
        if stepmin.is_none() || stepmax.is_none() {
            continue;
        }
        let fstepmin = stepmin.unwrap();
        let fstepmax = stepmax.unwrap();
        if fstepmax - fstepmin < 0.0001 {
            break;
        }
        if fstepmin.ceil() > fstepmax.floor() {
            continue;
        }
        if fstepmin < x_stepmin.0 as f64 {
            continue;
        }
        let stepmin = fstepmin.floor() as i32;
        let stepmax = fstepmax.ceil() as i32;
        let ok = (stepmin..=stepmax).any(|step| {
            let yp = ypos_at_step(yv0, Step(step));
            ypmin <= yp && yp <= ypmax
        });
        if !ok {
            continue;
        }
        let steptop = step_top_pos(yv0);
        let yptop = ypos_at_step(yv0, steptop);
        if yptop < best {
            continue;
        }
        best = yptop;
    }
    Ok(best.0)
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY17;
    assert_eq!(process(input.as_bytes())?, 45);
    Ok(())
}

#[test]
fn test_official() -> Result<()> {
    let input = "target area: x=32..65, y=-225..-177\n";
    assert_eq!(process(input.as_bytes())?, 25200);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
