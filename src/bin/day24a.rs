// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use dashmap::DashSet;
use rayon::prelude::*;
use std::convert::TryFrom;
use std::fmt;
use std::io::{stdin, BufRead};
use std::ops;

extern crate adventofcode2021;

// Parse

pub mod parser {
    use crate::Instr;
    use crate::Reg;
    use crate::RegNum;
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::branch;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::collections::VecDeque;
    use std::convert::TryInto;
    use std::io::BufRead;

    pub fn reg(input: &str) -> IResult<&str, Reg> {
        combinator::map_res(character::one_of("wxyz"), |c| c.try_into())(input)
    }

    pub fn regnum(input: &str) -> IResult<&str, RegNum> {
        branch::alt((
            combinator::map(reg, RegNum::Reg),
            combinator::map(character::i32, RegNum::Num),
        ))(input)
    }

    pub fn instr_inp(input: &str) -> IResult<&str, Instr> {
        let (input, _) = bytes::tag("inp ")(input)?;
        let (input, reg) = reg(input)?;
        Ok((input, Instr::Inp(reg)))
    }

    pub fn instr2<'a, 'b, F>(name: &'a str, func: F, input: &'b str) -> IResult<&'b str, Instr>
    where
        F: Fn(Reg, RegNum) -> Instr,
    {
        let (input, _) = bytes::tag(name)(input)?;
        let (input, r1) = reg(input)?;
        let (input, _) = bytes::tag(" ")(input)?;
        let (input, r2) = regnum(input)?;
        Ok((input, func(r1, r2)))
    }

    pub fn instr_add(input: &str) -> IResult<&str, Instr> {
        instr2("add ", Instr::Add, input)
    }

    pub fn instr_mul(input: &str) -> IResult<&str, Instr> {
        instr2("mul ", Instr::Mul, input)
    }

    pub fn instr_div(input: &str) -> IResult<&str, Instr> {
        instr2("div ", Instr::Div, input)
    }

    pub fn instr_mod(input: &str) -> IResult<&str, Instr> {
        instr2("mod ", Instr::Mod, input)
    }

    pub fn instr_eql(input: &str) -> IResult<&str, Instr> {
        instr2("eql ", Instr::Eql, input)
    }

    pub fn instr(input: &str) -> IResult<&str, Instr> {
        let (input, instr) = branch::alt((
            instr_inp,
            branch::alt((
                instr_add,
                branch::alt((
                    instr_mul,
                    branch::alt((instr_div, branch::alt((instr_mod, instr_eql)))),
                )),
            )),
        ))(input)?;
        let (input, _) = multi::many1(character::newline)(input)?;
        Ok((input, instr))
    }

    pub fn all(input: &str) -> IResult<&str, Vec<Instr>> {
        let (input, alu) = multi::many1(instr)(input)?;
        Ok((input, alu))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Instr>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(all)(&input);
        match result {
            Ok((_, info)) => Ok(info),
            Err(e) => Err(anyhow!("error reading input: {:?}", e)),
        }
    }

    pub fn model(input: &str) -> IResult<&str, VecDeque<i32>> {
        combinator::map(
            multi::many1(combinator::map(character::one_of("0123456789"), |i| {
                (i as u8 - b'0') as i32
            })),
            |v| v.into_iter().collect::<VecDeque<_>>(),
        )(input)
    }

    pub fn parse_model(input: &str) -> Result<VecDeque<i32>> {
        match combinator::all_consuming(model)(input) {
            Ok((_, data)) => Ok(data),
            Err(e) => Err(anyhow!("error reading input: {:?}", e)),
        }
    }
}

/* Registers ********************************************************/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reg {
    W,
    X,
    Y,
    Z,
}

impl TryFrom<char> for Reg {
    type Error = Error;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'w' => Self::W,
            'x' => Self::X,
            'y' => Self::Y,
            'z' => Self::Z,
            _ => return Err(anyhow!("invalid register {}", c)),
        })
    }
}

impl From<Reg> for char {
    fn from(r: Reg) -> char {
        match r {
            Reg::W => 'w',
            Reg::X => 'x',
            Reg::Y => 'y',
            Reg::Z => 'z',
        }
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

/* Instructions *****************************************************/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegNum {
    Reg(Reg),
    Num(i32),
}

impl fmt::Display for RegNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reg(r) => write!(f, "{}", r),
            Self::Num(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instr {
    Inp(Reg),
    Add(Reg, RegNum),
    Mul(Reg, RegNum),
    Div(Reg, RegNum),
    Mod(Reg, RegNum),
    Eql(Reg, RegNum),
}

impl Instr {
    pub fn is_input(&self) -> bool {
        matches!(self, Instr::Inp(_))
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inp(r) => write!(f, "inp {}", r),
            Self::Add(r, n) => write!(f, "add {} {}", r, n),
            Self::Mul(r, n) => write!(f, "mul {} {}", r, n),
            Self::Div(r, n) => write!(f, "div {} {}", r, n),
            Self::Mod(r, n) => write!(f, "mod {} {}", r, n),
            Self::Eql(r, n) => write!(f, "eql {} {}", r, n),
        }
    }
}

/* State ************************************************************/

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct State {
    pub regs: [i32; 4],
}

impl State {
    pub fn get(&self, regnum: &RegNum) -> i32 {
        match regnum {
            RegNum::Reg(r) => self[r],
            RegNum::Num(n) => *n,
        }
    }
    pub fn eval(&mut self, instr: &Instr) {
        match instr {
            Instr::Add(r, n) => {
                self[r] += self.get(n);
            }
            Instr::Mul(r, n) => {
                self[r] *= self.get(n);
            }
            Instr::Div(r, n) => {
                self[r] /= self.get(n);
            }
            Instr::Mod(r, n) => {
                self[r] %= self.get(n);
            }
            Instr::Eql(r, n) => {
                if self[r] == self.get(n) {
                    self[r] = 1;
                } else {
                    self[r] = 0;
                }
            }
            _ => panic!("SIGILL"),
        }
    }
}

impl ops::Index<&Reg> for State {
    type Output = i32;
    fn index(&self, index: &Reg) -> &Self::Output {
        &self.regs[*index as usize]
    }
}

impl ops::IndexMut<&Reg> for State {
    fn index_mut(&mut self, index: &Reg) -> &mut i32 {
        unsafe { self.regs.get_unchecked_mut(*index as usize) }
    }
}

pub fn eval_block(
    visited: &DashSet<(usize, State)>,
    state0: &State,
    block: &[&[Instr]],
    num0: u64,
) -> Option<u64> {
    if block.is_empty() {
        let z = state0[&Reg::Z];
        return if z == 0 { Some(num0) } else { None };
    }
    let reg = match block[0][0] {
        Instr::Inp(reg) => reg,
        _ => panic!("invalid first instruction"),
    };
    (1..10).into_par_iter().rev().find_map_first(|i| {
        if visited.contains(&(block.len(), *state0)) {
            return None;
        }
        let mut state = *state0;
        state[&reg] = i;
        for instr in &block[0][1..] {
            state.eval(instr);
        }
        let num = num0 * 10 + i as u64;
        let ret = eval_block(visited, &state, &block[1..], num);
        if ret.is_none() {
            visited.insert((block.len() - 1, state));
        }
        ret
    })
}

/* Main *************************************************************/

fn process(bufin: impl BufRead) -> Result<u64> {
    let alu = parser::parse(bufin)?;
    let mut blocks = vec![];
    let mut last = 0;
    for (i, instr) in alu.iter().enumerate() {
        if instr.is_input() {
            if last < i {
                blocks.push(&alu[last..i]);
            }
            last = i;
        }
    }
    blocks.push(&alu[last..]);
    let visited = DashSet::new();
    eval_block(&visited, &State::default(), &blocks, 0)
        .ok_or_else(|| anyhow!("error calculating number"))
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
