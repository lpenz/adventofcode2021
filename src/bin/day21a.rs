// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
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
    pub position: i32,
    pub score: u64,
}

pub enum IPlayerMarker {}
pub type IPlayer = andex::Andex<IPlayerMarker, 2>;
pub type Players = andex::array!(IPlayer, Player);

// Main functions

pub fn process(bufin: impl BufRead) -> Result<u64> {
    let mut players = parser::parse(bufin)?;
    let p1 = IPlayer::FIRST;
    let p2 = IPlayer::LAST;
    let mut roll = 1_i32;
    let mut numrolls = 0;
    while players[p1].score < 1000 && players[p2].score < 1000 {
        for player in &mut players {
            let dp = 3 * roll + 3;
            let pos = player.position;
            player.position = (pos + dp) % 10;
            player.score += 1 + player.position as u64;
            roll = (roll + 3) % 100;
            numrolls += 3;
            if player.score >= 1000 {
                break;
            }
        }
    }
    let minscore = players.iter().map(|p| p.score).min().unwrap();
    Ok(minscore * numrolls)
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY21;
    assert_eq!(process(input.as_bytes())?, 739785);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
