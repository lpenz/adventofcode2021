// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

// Parser

pub mod parser {
    use crate::IPlayer;
    use crate::Player;
    use crate::Players;
    use anyhow::{anyhow, Result};
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::convert::TryFrom;
    use std::io::BufRead;

    fn line(input: &str) -> IResult<&str, Player> {
        let (input, _) = bytes::tag("Player ")(input)?;
        let (input, id) =
            combinator::map_res(character::i32, |id| IPlayer::try_from(id as usize - 1))(input)?;
        let (input, _) = bytes::tag(" starting position: ")(input)?;
        let (input, position) = character::i32(input)?;
        let (input, _) = character::newline(input)?;
        Ok((
            input,
            Player {
                id,
                position: position - 1,
            },
        ))
    }

    fn all(input: &str) -> IResult<&str, Players> {
        let (input, players) = multi::many1(line)(input)?;
        Ok((input, players.iter().collect::<Players>()))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Players> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(all)(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Player {
    pub id: IPlayer,
    pub position: i32,
}

pub enum IPlayerMarker {}
pub type IPlayer = andex::Andex<IPlayerMarker, 2>;
pub type Players = andex::andex_array!(IPlayer, Player);

type Turnspace = HashMap<i32, u64>;

lazy_static! {
    static ref TURNSPACE: HashMap<i32, u64> = {
        let mut turnspace = Turnspace::new();
        for d1 in 1..=3 {
            for d2 in 1..=3 {
                for d3 in 1..=3 {
                    let e = turnspace.entry(d1 + d2 + d3).or_insert(0);
                    *e += 1;
                }
            }
        }
        turnspace
    };
}

pub type Rollscore = HashMap<(i32, u64), u64>;

pub fn rollscores_calc(
    rollscores: &mut Rollscore,
    position0: i32,
    rolls0: i32,
    score0: u64,
    freq0: u64,
) {
    if score0 >= 21 {
        return;
    }
    for (dp, dfreq) in &*TURNSPACE {
        let pos = (position0 + dp) % 10;
        let score = score0 + 1 + pos as u64;
        let freq = freq0 * dfreq;
        let rolls = rolls0 + 1;
        let e = rollscores.entry((rolls, score)).or_insert(0);
        *e += freq;
        rollscores_calc(rollscores, pos, rolls, score, freq);
    }
}

// Main functions

pub fn process(bufin: impl BufRead) -> Result<u64> {
    let players = parser::parse(bufin)?;
    let mut rollscores = [Rollscore::default(), Rollscore::default()];
    for iplayer in IPlayer::iter() {
        rollscores[usize::from(iplayer)].insert((0, 0), 1);
        rollscores_calc(
            &mut rollscores[usize::from(iplayer)],
            players[iplayer].position,
            0,
            0,
            1,
        );
    }
    let mut wins0 = 0_u64;
    let mut wins1 = 0_u64;
    for ((rolls0, score0), freq0) in &rollscores[usize::from(IPlayer::FIRST)] {
        for ((rolls1, score1), freq1) in &rollscores[usize::from(IPlayer::LAST)] {
            if *score0 >= 21_u64 && *rolls1 == *rolls0 - 1 && *score1 < 21_u64 {
                wins0 += freq0 * freq1;
            }
            if *score1 >= 21_u64 && *rolls1 == *rolls0 && *score0 < 21_u64 {
                wins1 += freq0 * freq1;
            }
        }
    }
    Ok(std::cmp::max(wins0, wins1))
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY21;
    assert_eq!(process(input.as_bytes())?, 444356092776315);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
