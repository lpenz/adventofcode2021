// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Result;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum TypeId {
    Sum,
    Product,
    Min,
    Max,
    Literal,
    Gt,
    Lt,
    Eq,
}

impl TypeId {
    pub fn fromu8(c: u8) -> Result<TypeId> {
        Ok(match c {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            4 => Self::Literal,
            5 => Self::Gt,
            6 => Self::Lt,
            7 => Self::Eq,
            _ => return Err(anyhow!("invalid type id {}", c)),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Payload {
    Literal(u64),
    Operation(TypeId, Vec<Packet>),
}

pub fn bool2u64(b: bool) -> u64 {
    if b {
        1
    } else {
        0
    }
}

impl Payload {
    pub fn eval(&self) -> u64 {
        match self {
            Payload::Literal(num) => *num,
            Payload::Operation(typeid, packets) => {
                let sub = packets.iter().map(|p| p.eval());
                match typeid {
                    TypeId::Sum => sub.sum(),
                    TypeId::Product => sub.product(),
                    TypeId::Min => sub.min().unwrap(),
                    TypeId::Max => sub.max().unwrap(),
                    TypeId::Literal => panic!("invalid type id for operation"),
                    TypeId::Gt => bool2u64(packets[0].eval() > packets[1].eval()),
                    TypeId::Lt => bool2u64(packets[0].eval() < packets[1].eval()),
                    TypeId::Eq => bool2u64(packets[0].eval() == packets[1].eval()),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct Packet {
    version: i32,
    payload: Payload,
}

impl Packet {
    pub fn eval(&self) -> u64 {
        self.payload.eval()
    }
}

// Parsers:

pub mod bin_parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    pub fn hexbyte(input: &str) -> IResult<&str, u8> {
        let mut f = combinator::map_res(character::one_of("0123456789ABCDEF"), |c| {
            u8::from_str_radix(&c.to_string(), 16)
        });
        let (input, high) = f(input)?;
        let (input, low) = f(input)?;
        Ok((input, (high << 4) | low))
    }

    pub fn binseq(input: &str) -> IResult<&str, Vec<u8>> {
        let (input, bytes) = multi::many1(hexbyte)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, bytes))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<u8>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let (_, binseq) = combinator::all_consuming(binseq)(&input)
            .map_err(|e| anyhow!("error reading input: {:?}", e))?;
        Ok(binseq)
    }
}

pub mod packet_parser {
    use crate::Packet;
    use crate::Payload;
    use crate::TypeId;
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::bits::bits as parsebits;
    use nom::bits::complete as bits;
    use nom::combinator;
    use nom::error::Error;
    use nom::IResult;

    type Binput<'a> = (&'a [u8], usize);

    pub fn literal(mut binput: Binput) -> IResult<Binput, Payload> {
        let mut value = 0_u64;
        loop {
            value <<= 4;
            let (binput_, val): (Binput, u64) = bits::take(5_usize)(binput)?;
            binput = binput_;
            value |= val & 0xf;
            if (val & 0x10) == 0 {
                break;
            }
        }
        Ok((binput, Payload::Literal(value)))
    }

    pub fn packets_by_len(mut binput: Binput) -> IResult<Binput, Vec<Packet>> {
        let mut packets = vec![];
        let (binput_, binlen): (Binput, usize) = bits::take(15_usize)(binput)?;
        binput = binput_;
        let (_, restlen0) = combinator::rest_len(binput)?;
        loop {
            let (binput_, pkt) = packet(binput)?;
            binput = binput_;
            packets.push(pkt);
            let (_, restlen) = combinator::rest_len(binput)?;
            if restlen0 - restlen >= binlen {
                break;
            }
        }
        Ok((binput, packets))
    }

    pub fn packets_by_num(mut binput: Binput) -> IResult<Binput, Vec<Packet>> {
        let (binput_, numpkts): (Binput, i32) = bits::take(11_usize)(binput)?;
        binput = binput_;
        let mut packets = vec![];
        for _ in 0..numpkts {
            let (binput_, pkt) = packet(binput)?;
            binput = binput_;
            packets.push(pkt);
        }
        Ok((binput, packets))
    }

    pub fn operation(typeid: TypeId, binput: Binput) -> IResult<Binput, Payload> {
        let (binput, lentype): (Binput, i32) = bits::take(1_usize)(binput)?;
        let (binput, packets) = if lentype == 0 {
            packets_by_len(binput)
        } else {
            packets_by_num(binput)
        }?;
        Ok((binput, Payload::Operation(typeid, packets)))
    }

    pub fn payload_parse(typeid: TypeId, binput: Binput) -> IResult<Binput, Payload> {
        if typeid == TypeId::Literal {
            literal(binput)
        } else {
            operation(typeid, binput)
        }
    }

    pub fn packet(binput: Binput) -> IResult<Binput, Packet> {
        let (binput, version) = bits::take(3_usize)(binput)?;
        let (binput, typeid) = combinator::map_res(bits::take(3_usize), TypeId::fromu8)(binput)?;
        let (binput, payload) = payload_parse(typeid, binput)?;
        let mypacket = Packet { version, payload };
        Ok((binput, mypacket))
    }

    pub fn parse(input: &[u8]) -> Result<Packet> {
        let (_input, packets) =
            parsebits::<_, _, Error<Binput>, nom::error::Error<_>, _>(packet)(input)
                .map_err(|e| anyhow!("error reading input: {:?}", e))?;
        Ok(packets)
    }
}

// Main functions

fn process(bufin: impl BufRead) -> Result<u64> {
    let bin = bin_parser::parse(bufin)?;
    let packet = packet_parser::parse(&bin)?;
    Ok(packet.eval())
}

pub fn do_test(input: &str, ans: u64) -> Result<()> {
    assert_eq!(process(input.as_bytes())?, ans);
    Ok(())
}

#[test]
fn test1() -> Result<()> {
    do_test("C200B40A82\n", 3)
}

#[test]
fn test2() -> Result<()> {
    do_test("04005AC33890\n", 54)
}

#[test]
fn test3() -> Result<()> {
    do_test("880086C3E88112\n", 7)
}

#[test]
fn test4() -> Result<()> {
    do_test("CE00C43D881120\n", 9)
}

#[test]
fn test6() -> Result<()> {
    do_test("D8005AC2A8F0\n", 1)
}

#[test]
fn test7() -> Result<()> {
    do_test("F600BC2D8F\n", 0)
}

#[test]
fn test8() -> Result<()> {
    do_test("9C005AC2F8F0\n", 0)
}

#[test]
fn test9() -> Result<()> {
    do_test("9C0141080250320F1802104A08\n", 1)
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
