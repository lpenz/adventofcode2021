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
    use crate::State;
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
                state0: State {
                    position: position - 1,
                    ..State::default()
                },
                ..Player::default()
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
    pub universes_win: u64,
    pub state0: State,
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub position: i32,
    pub score: u64,
}

impl Default for State {
    fn default() -> State {
        State {
            position: 1,
            score: 0,
        }
    }
}

pub enum IPlayerMarker {}
pub type IPlayer = andex::Andex<IPlayerMarker, 2>;
pub type Players = andex::array!(IPlayer, Player);
pub type States = andex::array!(IPlayer, State);

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

pub fn player_turn(players: &mut Players, iplayer: IPlayer, states0: &States, freq0: u64) {
    for (dp, dfreq) in &*TURNSPACE {
        let mut states = *states0;
        let pos = states[iplayer].position;
        states[iplayer].position = (pos + dp) % 10;
        states[iplayer].score += 1 + states[iplayer].position as u64;
        let freq = dfreq * freq0;
        if states[iplayer].score >= 21 {
            players[iplayer].universes_win += freq;
        } else {
            player_turn(players, iplayer.pair(), &states, freq);
        }
    }
}

// Main functions

pub fn process(bufin: impl BufRead) -> Result<u64> {
    let mut players = parser::parse(bufin)?;
    eprintln!("{:?}", *TURNSPACE);
    let states0 = players.iter().map(|p| p.state0).collect::<States>();
    player_turn(&mut players, IPlayer::FIRST, &states0, 1);
    eprintln!("{:?}", players[IPlayer::FIRST]);
    eprintln!("{:?}", players[IPlayer::LAST]);
    Ok(IPlayer::iter()
        .map(|ip| players[ip].universes_win)
        .max()
        .unwrap())
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
