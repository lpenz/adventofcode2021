// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::HashSet;
use std::fmt;
use std::ops;

#[derive(Default, Copy, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct Xyz {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Xyz {
    pub const fn new(x: i32, y: i32, z: i32) -> Xyz {
        Xyz { x, y, z }
    }
}

impl fmt::Debug for Xyz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl ops::Add<Xyz> for Xyz {
    type Output = Self;
    fn add(self, rhs: Xyz) -> Self::Output {
        Xyz::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Xyz> for Xyz {
    type Output = Self;
    fn sub(self, rhs: Xyz) -> Self::Output {
        Xyz::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[derive(Debug, Default, Copy, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct M([[i32; 3]; 3]);

impl M {
    pub const ID: M = M([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    pub const ROT_X_90: M = M([[1, 0, 0], [0, 0, 1], [0, -1, 0]]);
    pub const ROT_Y_90: M = M([[0, 0, 1], [0, 1, 0], [-1, 0, 0]]);
    pub const ROT_Z_90: M = M([[0, 1, 0], [-1, 0, 0], [0, 0, 1]]);
    pub const ROT_X_180: M = Self::ROT_X_90.mult(&Self::ROT_X_90);
    pub const ROT_Y_180: M = Self::ROT_Y_90.mult(&Self::ROT_Y_90);
    pub const ROT_Z_180: M = Self::ROT_Z_90.mult(&Self::ROT_Z_90);
    pub const TRANS: [M; 24] = [
        // Right:
        /* 0 */ Self::ID,
        /* 1 */ Self::ROT_X_90,
        /* 2 */ Self::ROT_X_90.mult(&Self::ROT_X_90),
        /* 3 */ Self::ROT_X_90.mult(&Self::ROT_X_90).mult(&Self::ROT_X_90),
        // Up:
        /* 4 */ Self::ROT_Z_90,
        /* 5 */ Self::ROT_Z_90.mult(&Self::ROT_Y_90),
        /* 6 */
        Self::ROT_Z_90.mult(&Self::ROT_Y_90).mult(&Self::ROT_Y_90),
        /* 7 */
        Self::ROT_Z_90
            .mult(&Self::ROT_Y_90)
            .mult(&Self::ROT_Y_90)
            .mult(&Self::ROT_Y_90),
        // Front
        /* 8 */ Self::ROT_Y_90,
        /* 9 */ Self::ROT_Y_90.mult(&Self::ROT_Z_90),
        /* 10 */ Self::ROT_Y_90.mult(&Self::ROT_Z_90).mult(&Self::ROT_Z_90),
        /* 11 */
        Self::ROT_Y_90
            .mult(&Self::ROT_Z_90)
            .mult(&Self::ROT_Z_90)
            .mult(&Self::ROT_Z_90),
        // Left:
        /* 12 */ Self::ROT_Y_180,
        /* 13 */ Self::ROT_Y_180.mult(&Self::ROT_X_90),
        /* 14 */
        Self::ROT_Y_180.mult(&Self::ROT_X_90).mult(&Self::ROT_X_90),
        /* 15 */
        Self::ROT_Y_180
            .mult(&Self::ROT_X_90)
            .mult(&Self::ROT_X_90)
            .mult(&Self::ROT_X_90),
        // Down:
        /* 16 */ Self::ROT_Z_90.mult(&Self::ROT_Z_180),
        /* 17 */
        Self::ROT_Z_90.mult(&Self::ROT_Z_180).mult(&Self::ROT_Y_90),
        /* 18 */
        Self::ROT_Z_90
            .mult(&Self::ROT_Z_180)
            .mult(&Self::ROT_Y_90)
            .mult(&Self::ROT_Y_90),
        /* 19 */
        Self::ROT_Z_90
            .mult(&Self::ROT_Z_180)
            .mult(&Self::ROT_Y_90)
            .mult(&Self::ROT_Y_90)
            .mult(&Self::ROT_Y_90),
        // Back:
        /* 20 */ Self::ROT_Y_90.mult(&Self::ROT_Y_180),
        /* 21 */
        Self::ROT_Y_90.mult(&Self::ROT_Y_180).mult(&Self::ROT_Z_90),
        /* 22 */
        Self::ROT_Y_90
            .mult(&Self::ROT_Y_180)
            .mult(&Self::ROT_Z_90)
            .mult(&Self::ROT_Z_90),
        /* 23 */
        Self::ROT_Y_90
            .mult(&Self::ROT_Y_180)
            .mult(&Self::ROT_Z_90)
            .mult(&Self::ROT_Z_90)
            .mult(&Self::ROT_Z_90),
    ];
    pub const fn mult(&self, t: &M) -> M {
        M([
            [
                self.0[0][0] * t.0[0][0] + self.0[0][1] * t.0[1][0] + self.0[0][2] * t.0[2][0],
                self.0[0][0] * t.0[0][1] + self.0[0][1] * t.0[1][1] + self.0[0][2] * t.0[2][1],
                self.0[0][0] * t.0[0][2] + self.0[0][1] * t.0[1][2] + self.0[0][2] * t.0[2][2],
            ],
            [
                self.0[1][0] * t.0[0][0] + self.0[1][1] * t.0[1][0] + self.0[1][2] * t.0[2][0],
                self.0[1][0] * t.0[0][1] + self.0[1][1] * t.0[1][1] + self.0[1][2] * t.0[2][1],
                self.0[1][0] * t.0[0][2] + self.0[1][1] * t.0[1][2] + self.0[1][2] * t.0[2][2],
            ],
            [
                self.0[2][0] * t.0[0][0] + self.0[2][1] * t.0[1][0] + self.0[2][2] * t.0[2][0],
                self.0[2][0] * t.0[0][1] + self.0[2][1] * t.0[1][1] + self.0[2][2] * t.0[2][1],
                self.0[2][0] * t.0[0][2] + self.0[2][1] * t.0[1][2] + self.0[2][2] * t.0[2][2],
            ],
        ])
    }
    pub const fn apply(&self, xyz: &Xyz) -> Xyz {
        Xyz::new(
            xyz.x * self.0[0][0] + xyz.y * self.0[1][0] + xyz.z * self.0[2][0],
            xyz.x * self.0[0][1] + xyz.y * self.0[1][1] + xyz.z * self.0[2][1],
            xyz.x * self.0[0][2] + xyz.y * self.0[1][2] + xyz.z * self.0[2][2],
        )
    }
}

impl ops::Mul<M> for M {
    type Output = Self;
    fn mul(self, rhs: M) -> Self::Output {
        self.mult(&rhs)
    }
}

impl ops::Mul<Xyz> for M {
    type Output = Xyz;
    fn mul(self, rhs: Xyz) -> Self::Output {
        self.apply(&rhs)
    }
}

impl ops::Mul<Xyz> for &M {
    type Output = Xyz;
    fn mul(self, rhs: Xyz) -> Self::Output {
        self.apply(&rhs)
    }
}

#[test]
fn test_matrix_mult() {
    let a = M([[-3, 0, 1], [3, 2, 5], [-2, -1, 4]]);
    let b = M([[6, 0, 1], [-4, -3, 2], [3, -2, 4]]);
    let ab = a.mult(&b);
    let abexp = M([[-15, -2, 1], [25, -16, 27], [4, -5, 12]]);
    assert_eq!(ab, abexp);
}

#[test]
fn test_matrix_trans() {
    assert_eq!(
        M::ROT_X_90
            .mult(&M::ROT_X_90)
            .mult(&M::ROT_X_90)
            .mult(&M::ROT_X_90),
        M::ID
    );
    assert_eq!(
        M::ROT_Y_90
            .mult(&M::ROT_Y_90)
            .mult(&M::ROT_Y_90)
            .mult(&M::ROT_Y_90),
        M::ID
    );
    assert_eq!(
        M::ROT_Z_90
            .mult(&M::ROT_Z_90)
            .mult(&M::ROT_Z_90)
            .mult(&M::ROT_Z_90),
        M::ID
    );
}

#[test]
fn test_matrix_trans_complete() {
    for t in 0..M::TRANS.len() {
        for row in 0..3 {
            for col in 0..3 {
                let v = M::TRANS[t].0[row][col];
                assert!(v == -1 || v == 0 || v == 1);
            }
        }
    }
    for i in 0..M::TRANS.len() {
        for j in 0..M::TRANS.len() {
            if i == j {
                continue;
            }
            if M::TRANS[i] == M::TRANS[j] {
                eprintln!("error, equal transformations: {} = {}", i, j);
            }
            assert_ne!(M::TRANS[i], M::TRANS[j]);
        }
    }
    let xyz = Xyz::new(3, 5, 7);
    let all = M::TRANS
        .iter()
        .map(|t| t.apply(&xyz))
        .collect::<HashSet<_>>();
    assert_eq!(all.len(), M::TRANS.len());
}

#[test]
fn test_xyz_trans() {
    let xyz = Xyz::new(3, 5, 7);
    assert_eq!(M::ID.apply(&xyz), xyz);
    assert_eq!(M::ROT_X_90.apply(&xyz), Xyz::new(3, -7, 5));
    assert_eq!(M::ROT_Y_90.apply(&xyz), Xyz::new(-7, 5, 3));
    assert_eq!(M::ROT_Z_90.apply(&xyz), Xyz::new(-5, 3, 7));
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Scanner {
    pub id: usize,
    pub position: Option<Xyz>,
    pub beacons: HashSet<Xyz>,
}

// Parsers:

pub mod bin_parser {
    use super::Scanner;
    use super::Xyz;
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::collections::HashSet;
    use std::io::BufRead;

    pub fn xyz(input: &str) -> IResult<&str, Xyz> {
        let (input, x) = character::i32(input)?;
        let (input, _) = character::char(',')(input)?;
        let (input, y) = character::i32(input)?;
        let (input, _) = character::char(',')(input)?;
        let (input, z) = character::i32(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, Xyz { x, y, z }))
    }

    pub fn scanner(input: &str) -> IResult<&str, Scanner> {
        let (input, _) = bytes::tag("--- scanner ")(input)?;
        let (input, id) = combinator::map(character::u32, |i| i as usize)(input)?;
        let (input, _) = bytes::tag(" ---\n")(input)?;
        let (input, beacons) = multi::many1(xyz)(input)?;
        Ok((
            input,
            Scanner {
                id: id as usize,
                position: if id == 0 {
                    Some(Xyz::new(0, 0, 0))
                } else {
                    None
                },
                beacons: beacons.into_iter().collect::<HashSet<_>>(),
            },
        ))
    }

    pub fn all(input: &str) -> IResult<&str, Vec<Scanner>> {
        let (input, scanners) = multi::separated_list1(character::newline, scanner)(input)?;
        Ok((input, scanners))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Scanner>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let (_, scanners) = combinator::all_consuming(all)(&input)
            .map_err(|e| anyhow!("error reading input: {:?}", e))?;
        Ok(scanners)
    }
}

// Main functions

pub fn total_beacons<'a>(scanners: impl Iterator<Item = &'a Scanner>) -> usize {
    scanners
        .flat_map(|s| &s.beacons)
        .collect::<HashSet<_>>()
        .len()
}

pub fn fix_scanner(s0: &Scanner, s1: &mut Scanner) -> bool {
    for (_idtrans, trans) in M::TRANS.iter().enumerate() {
        let beacons1 = s1
            .beacons
            .iter()
            .map(|b| trans.apply(b))
            .collect::<HashSet<_>>();
        for orig0 in &s0.beacons {
            for orig1 in &beacons1 {
                let beacons1_rel = beacons1
                    .iter()
                    .map(|b| (*b - *orig1) + *orig0)
                    .collect::<HashSet<_>>();
                if s0.beacons.intersection(&beacons1_rel).nth(11).is_some() {
                    s1.beacons = beacons1_rel;
                    return true;
                }
            }
        }
    }
    false
}

pub fn fix_scanners(mut scanners_pending: Vec<Scanner>) -> Result<Vec<Scanner>> {
    // Scanners that can bring other scanners:
    let mut scanners_ready = vec![scanners_pending.remove(0)];
    // Scanners that we have already fully used:
    let mut scanners_done = vec![];
    while !scanners_pending.is_empty() {
        let mut scanners_ready_next = vec![];
        for scanner_ready in &scanners_ready {
            for mut scanner_pending in std::mem::take(&mut scanners_pending).into_iter() {
                if fix_scanner(scanner_ready, &mut scanner_pending) {
                    // eprintln!("used {} to fix {}", scanner_ready.id, scanner_pending.id,);
                    scanners_ready_next.push(scanner_pending);
                } else {
                    scanners_pending.push(scanner_pending);
                }
            }
        }
        scanners_done.append(&mut scanners_ready);
        scanners_ready = scanners_ready_next;
    }
    scanners_done.append(&mut scanners_ready);
    Ok(scanners_done)
}

#[test]
fn test() -> Result<()> {
    let scanners = bin_parser::parse(crate::examples::DAY19.as_bytes())?;
    let scanners_done = fix_scanners(scanners)?;
    assert_eq!(total_beacons(scanners_done.iter()), 79);
    Ok(())
}
