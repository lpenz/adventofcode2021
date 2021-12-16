// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct Packet {
    version: i32,
    typeid: u8,
    value: u64,
    packets: Vec<Packet>,
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
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::bits::bits as parsebits;
    use nom::bits::complete as bits;
    use nom::combinator;
    use nom::error::Error;
    use nom::IResult;

    type Binput<'a> = (&'a [u8], usize);

    pub fn packet(binput: Binput) -> IResult<Binput, Packet> {
        let (binput, version) = bits::take(3_usize)(binput)?;
        let (mut binput, typeid) = bits::take(3_usize)(binput)?;
        let mut mypacket = Packet {
            version,
            typeid,
            ..Default::default()
        };
        if typeid == 4 {
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
            mypacket.value = value;
        } else {
            let (binput_, lentype): (Binput, i32) = bits::take(1_usize)(binput)?;
            binput = binput_;
            if lentype == 0 {
                let (binput_, binlen): (Binput, usize) = bits::take(15_usize)(binput)?;
                binput = binput_;
                let (_, restlen0) = combinator::rest_len(binput)?;
                loop {
                    let (binput_, pkt) = packet(binput)?;
                    binput = binput_;
                    mypacket.packets.push(pkt);
                    let (_, restlen) = combinator::rest_len(binput)?;
                    if restlen0 - restlen >= binlen {
                        break;
                    }
                }
            } else {
                let (binput_, numpkts): (Binput, i32) = bits::take(11_usize)(binput)?;
                binput = binput_;
                for _ in 0..numpkts {
                    let (binput_, pkt) = packet(binput)?;
                    binput = binput_;
                    mypacket.packets.push(pkt);
                }
            }
        }
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

fn addversions(packet: &Packet) -> i32 {
    packet.version + packet.packets.iter().map(addversions).sum::<i32>()
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let bin = bin_parser::parse(bufin)?;
    let packet = packet_parser::parse(&bin)?;
    Ok(addversions(&packet))
}

pub fn do_test(input: &str, ans: i32) -> Result<()> {
    assert_eq!(process(input.as_bytes())?, ans);
    Ok(())
}

#[test]
fn test1() -> Result<()> {
    do_test("8A004A801A8002F478\n", 16)
}

#[test]
fn test2() -> Result<()> {
    do_test("620080001611562C8802118E34\n", 12)
}

#[test]
fn test3() -> Result<()> {
    do_test("C0015000016115A2E0802F182340\n", 23)
}

#[test]
fn test4() -> Result<()> {
    do_test("A0016C880162017C3686B18A3D4780\n", 31)
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
