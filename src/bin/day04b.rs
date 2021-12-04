// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use std::io::{stdin, BufRead};
use std::mem;

extern crate adventofcode2021;

type Sqrid = sqrid::sqrid_create!(5, 5, false);
type Qa = sqrid::qa_create!(Sqrid);
type BoardGrid = sqrid::grid_create!(Sqrid, i32);

#[derive(Debug, Default)]
pub struct Board {
    pub grid: BoardGrid,
    pub marked: sqrid::gridbool_create!(Sqrid),
    pub marked_x: [i32; 5],
    pub marked_y: [i32; 5],
}

// Parser

pub mod parser {
    use crate::{Board, BoardGrid};
    use anyhow::{anyhow, Result};
    use nom::{
        bytes::complete::tag, character, character::complete::char, combinator::all_consuming,
        multi::*, IResult,
    };
    use std::io::BufRead;

    pub fn numbers(input: &str) -> IResult<&str, Vec<i32>> {
        let (input, numbers) = separated_list1(tag(","), character::complete::i32)(input)?;
        let (input, _) = char('\n')(input)?;
        Ok((input, numbers))
    }

    pub fn line(input: &str) -> IResult<&str, Vec<i32>> {
        let (input, _) = many0(char(' '))(input)?;
        let (input, numbers) = separated_list1(many1(tag(" ")), character::complete::i32)(input)?;
        let (input, _) = char('\n')(input)?;
        Ok((input, numbers))
    }

    pub fn board(input: &str) -> IResult<&str, Board> {
        let (input, lines) = many1(line)(input)?;
        let board = Board {
            grid: lines.iter().flatten().collect::<BoardGrid>(),
            ..Board::default()
        };
        Ok((input, board))
    }

    pub fn numboards(input: &str) -> IResult<&str, (Vec<i32>, Vec<Board>)> {
        let (input, numbers) = numbers(input)?;
        let (input, _) = char('\n')(input)?;
        let (input, boards) = separated_list1(char('\n'), board)(input)?;
        Ok((input, (numbers, boards)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Vec<i32>, Vec<Board>)> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = all_consuming(numboards)(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

// Main functions

fn process(bufin: impl BufRead) -> Result<i32> {
    let (numbers, mut nextboards) = parser::parse(bufin)?;
    for number in numbers {
        let boards = mem::take(&mut nextboards);
        let boardslen = boards.len();
        for mut board in boards.into_iter() {
            let mut won = false;
            for qa in Qa::iter() {
                let t0 = qa.tuple();
                let t = (t0.0 as usize, t0.1 as usize);
                if board.grid[qa] == number {
                    board.marked.set_t(qa);
                    board.marked_x[t.0] += 1;
                    board.marked_y[t.1] += 1;
                    won = board.marked_x[t.0] == 5 || board.marked_y[t.1] == 5;
                    if won && boardslen == 1 {
                        // And it's the last winner
                        let sum: i32 = Qa::iter()
                            .filter_map({
                                |qa| {
                                    if !board.marked.get(qa) {
                                        Some(board.grid[qa])
                                    } else {
                                        None
                                    }
                                }
                            })
                            .sum();
                        return Ok(sum * number);
                    }
                }
            }
            if !won {
                nextboards.push(board);
            }
        }
    }
    Err(anyhow!("No winner found"))
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY04;
    assert_eq!(process(input.as_bytes())?, 1924);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
