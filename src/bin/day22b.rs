// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use itertools::Itertools;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

// Parser

pub mod parser {
    use crate::Cube;
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
        let ini = Xyz::new(xini, yini, zini);
        let end = Xyz::new(xend + 1, yend + 1, zend + 1);
        Ok((
            input,
            Step {
                on: onoff,
                cube: Cube::new(ini, end),
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
            && self.x <= 51
            && self.y >= -50
            && self.y <= 51
            && self.z >= -50
            && self.z <= 51
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cube {
    ini: Xyz,
    end: Xyz,
}

impl Cube {
    pub fn new(ini: Xyz, end: Xyz) -> Cube {
        Cube { ini, end }
    }
    pub fn valid(&self) -> bool {
        self.ini.valid() && self.end.valid()
    }
    pub fn is_empty(&self) -> bool {
        self.ini.x == self.end.x || self.ini.y == self.end.y || self.ini.z == self.end.z
    }
    pub fn len(&self) -> usize {
        let dx = (self.ini.x - self.end.x).abs() as usize;
        let dy = (self.ini.y - self.end.y).abs() as usize;
        let dz = (self.ini.z - self.end.z).abs() as usize;
        dx * dy * dz
    }
    pub fn contains(&self, xyz: Xyz) -> bool {
        self.ini.x <= xyz.x
            && xyz.x < self.end.x
            && self.ini.y <= xyz.y
            && xyz.y < self.end.y
            && self.ini.z <= xyz.z
            && xyz.z < self.end.z
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Step {
    pub on: bool,
    pub cube: Cube,
}

impl Step {
    pub fn compress<F>(steps: &[Step], func: F) -> Vec<i32>
    where
        F: Fn(Xyz) -> i32,
    {
        steps
            .iter()
            .flat_map(|s| {
                let ini = func(s.cube.ini);
                let end = func(s.cube.end);
                vec![ini - 1, ini, end - 1, end].into_iter()
            })
            .sorted()
            .unique()
            .collect::<Vec<_>>()
    }
}

// Main functions

pub fn process(bufin: impl BufRead) -> Result<usize> {
    let steps = parser::parse(bufin)?;
    let xs = Step::compress(&steps, |c| c.x);
    let ys = Step::compress(&steps, |c| c.y);
    let zs = Step::compress(&steps, |c| c.z);
    let mut total_on = 0;
    for xw in xs.windows(2) {
        for yw in ys.windows(2) {
            for zw in zs.windows(2) {
                let ini = Xyz::new(xw[0], yw[0], zw[0]);
                let end = Xyz::new(xw[1], yw[1], zw[1]);
                let cube = Cube::new(ini, end);
                let mut on = false;
                for step in &steps {
                    if step.cube.contains(ini) {
                        on = step.on;
                    }
                }
                if on {
                    total_on += cube.len();
                }
            }
        }
    }
    Ok(total_on)
}

pub const DAY22: &str = "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507
";

#[test]
fn test() -> Result<()> {
    assert_eq!(process(DAY22.as_bytes())?, 2758514936282235);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
