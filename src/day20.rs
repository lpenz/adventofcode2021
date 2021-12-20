// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::io::BufRead;

// Parser

pub mod parser {
    use super::Algo;
    use super::Image;
    use super::Xy;
    use anyhow::{anyhow, Result};
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::convert::TryInto;
    use std::io::BufRead;

    fn pixel(input: &str) -> IResult<&str, bool> {
        combinator::map(character::one_of(".#"), |c| c == '#')(input)
    }

    fn algo(input: &str) -> IResult<&str, Algo> {
        let (input, algo) =
            combinator::map_res(multi::many1(pixel), |v| v.try_into().map(Algo))(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, algo))
    }

    fn line(input: &str) -> IResult<&str, Vec<bool>> {
        let (input, vec) = multi::many1(pixel)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, vec))
    }

    fn all(input: &str) -> IResult<&str, (Algo, Vec<Vec<bool>>)> {
        let (input, algo) = algo(input)?;
        let (input, _) = character::newline(input)?;
        let (input, map) = multi::many1(line)(input)?;
        Ok((input, (algo, map)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Algo, Image)> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(all)(&input);
        let mut image = Image::default();
        match result {
            Ok((_, (algo, v))) => {
                for (y, line) in v.into_iter().enumerate() {
                    for (x, value) in line.into_iter().enumerate() {
                        image.insert(Xy(x as i32, y as i32), value);
                    }
                }
                Ok((algo, image))
            }
            Err(e) => Err(anyhow!("error reading input: {:?}", e)),
        }
    }
}

#[derive(Debug)]
pub struct Algo([bool; 512]);

impl Algo {
    pub fn get(&self, offset: usize) -> bool {
        self.0[offset]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct Xy(i32, i32);

impl Xy {
    pub fn iter_around(&self) -> impl Iterator<Item = Xy> + '_ {
        (0..3)
            .cartesian_product(0..3)
            .map(move |(dy, dx)| Xy(self.0 - 1 + dx, self.1 - 1 + dy))
    }
}

#[derive(Debug, Default)]
pub struct Image {
    pub points: HashMap<Xy, bool>,
    pub default: bool,
    pub tl: Xy,
    pub br: Xy,
}

impl Image {
    pub fn insert(&mut self, xy: Xy, value: bool) {
        self.tl.0 = cmp::min(xy.0, self.tl.0);
        self.tl.1 = cmp::min(xy.1, self.tl.1);
        self.br.0 = cmp::max(xy.0, self.br.0);
        self.br.1 = cmp::max(xy.0, self.br.1);
        self.points.insert(xy, value);
    }
    pub fn get(&self, xy: &Xy) -> bool {
        *self.points.get(xy).unwrap_or(&self.default)
    }
    pub fn numlit(&self) -> usize {
        self.points.values().filter(|v| **v).count()
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let xmin = self.points.keys().map(|xy| xy.0).min().unwrap();
        let xmax = self.points.keys().map(|xy| xy.0).max().unwrap();
        let ymin = self.points.keys().map(|xy| xy.1).min().unwrap();
        let ymax = self.points.keys().map(|xy| xy.1).max().unwrap();
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                let xy = Xy(x, y);
                write!(f, "{}", if self.get(&xy) { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn bools2index(bools: &[bool]) -> usize {
    let mut r = 0;
    for b in bools.iter() {
        r <<= 1;
        if *b {
            r |= 1;
        }
    }
    r
}

// Main functions

pub fn process(enhance: usize, bufin: impl BufRead) -> Result<usize> {
    let (algo, mut image) = parser::parse(bufin)?;
    let extra = 2;
    for _ in 0..enhance {
        let oldimage = std::mem::take(&mut image);
        let xmin = oldimage.tl.0;
        let xmax = oldimage.br.0;
        let ymin = oldimage.tl.1;
        let ymax = oldimage.br.1;
        for (x, y) in
            ((xmin - extra)..=(xmax + extra)).cartesian_product((ymin - extra)..=(ymax + extra))
        {
            let xy = Xy(x, y);
            let bools = xy
                .iter_around()
                .map(|xy| oldimage.get(&xy))
                .collect::<Vec<_>>();
            let index = bools2index(&bools);
            image.insert(xy, algo.get(index));
        }
        image.default = oldimage.default;
        if !image.default && algo.get(0) {
            image.default = true;
        } else if image.default && !algo.get(511) {
            image.default = false;
        }
    }
    Ok(image.numlit())
}

pub const EXAMPLE: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

pub fn do_test(enhance: usize, ans: usize) -> Result<()> {
    assert_eq!(process(enhance, EXAMPLE.as_bytes())?, ans);
    Ok(())
}
