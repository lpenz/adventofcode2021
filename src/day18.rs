// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Num {
    Single(i32),
    Pair(Box<Num>, Box<Num>),
}

impl Num {
    pub fn is_pair(&self) -> bool {
        matches!(self, Num::Pair(_, _))
    }

    pub fn left_value(&self) -> Option<i32> {
        if let Num::Pair(p1, _) = self {
            p1.value()
        } else {
            None
        }
    }

    pub fn right_value(&self) -> Option<i32> {
        if let Num::Pair(_, p2) = self {
            p2.value()
        } else {
            None
        }
    }

    pub fn value(&self) -> Option<i32> {
        if let Num::Single(n) = self {
            Some(*n)
        } else {
            None
        }
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Num::Single(i) => write!(f, "{}", i),
            Num::Pair(n1, n2) => write!(f, "[{},{}]", n1, n2),
        }
    }
}

// Parsers:

pub mod parser {
    use crate::day18::Num;
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::branch;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    pub fn single(input: &str) -> IResult<&str, Box<Num>> {
        combinator::map(character::i32, |n| Box::new(Num::Single(n)))(input)
    }

    pub fn pair(input: &str) -> IResult<&str, Box<Num>> {
        let (input, _) = bytes::tag("[")(input)?;
        let (input, num1) = node(input)?;
        let (input, _) = bytes::tag(",")(input)?;
        let (input, num2) = node(input)?;
        let (input, _) = bytes::tag("]")(input)?;
        Ok((input, Box::new(Num::Pair(num1, num2))))
    }

    pub fn node(input: &str) -> IResult<&str, Box<Num>> {
        branch::alt((single, pair))(input)
    }

    pub fn line(input: &str) -> IResult<&str, Box<Num>> {
        let (input, _) = character::space0(input)?;
        let (input, num) = node(input)?;
        let (input, _) = bytes::tag("\n")(input)?;
        let (input, _) = character::space0(input)?;
        Ok((input, num))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Box<Num>>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::many1(line))(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

// Main functions

fn explode_helper_addleft(num: &mut Box<Num>, r: i32, lvl: usize) -> bool {
    if let Num::Pair(ref mut p1, ref mut p2) = **num {
        if explode_helper_addleft(p1, r, lvl) {
            true
        } else {
            explode_helper_addleft(p2, r, lvl)
        }
    } else {
        let v = num.value().unwrap();
        *num = Box::new(Num::Single(v + r));
        true
    }
}

fn explode_helper_addright(num: &mut Box<Num>, r: i32, lvl: usize) -> bool {
    if let Num::Pair(ref mut p1, ref mut p2) = **num {
        if explode_helper_addright(p2, r, lvl) {
            true
        } else {
            explode_helper_addright(p1, r, lvl)
        }
    } else {
        let v = num.value().unwrap();
        *num = Box::new(Num::Single(v + r));
        true
    }
}

fn explode_helper(num: &mut Box<Num>, lvl: usize) -> (bool, (i32, i32)) {
    if let Num::Pair(ref mut pl, ref mut pr) = **num {
        if lvl >= 4 && !pl.is_pair() && !pr.is_pair() {
            let residue_left = pl.value().unwrap();
            let residue_right = pr.value().unwrap();
            *num = Box::new(Num::Single(0));
            (true, (residue_left, residue_right))
        } else {
            let (exploded, (residue_left, mut residue_right)) = explode_helper(pl, lvl + 1);
            if exploded {
                if explode_helper_addleft(pr, residue_right, lvl) {
                    residue_right = 0;
                }
                return (true, (residue_left, residue_right));
            }
            let (exploded, (mut residue_left, residue_right)) = explode_helper(pr, lvl + 1);
            if exploded {
                if explode_helper_addright(pl, residue_left, lvl) {
                    residue_left = 0;
                }
                return (true, (residue_left, residue_right));
            }
            (false, (0, 0))
        }
    } else {
        (false, (0, 0))
    }
}

pub fn explode(num: &mut Box<Num>) -> bool {
    explode_helper(num, 0).0
}

pub fn split(num: &mut Box<Num>) -> bool {
    if let Num::Pair(ref mut p1, ref mut p2) = **num {
        split(p1) || split(p2)
    } else {
        let v = num.value().unwrap();
        if v > 9 {
            *num = Box::new(Num::Pair(
                Box::new(Num::Single(v / 2)),
                Box::new(Num::Single(v / 2 + v % 2)),
            ));
            true
        } else {
            false
        }
    }
}

pub fn reduce(num: &mut Box<Num>) {
    loop {
        if explode(num) {
            continue;
        }
        if split(num) {
            continue;
        }
        break;
    }
}

pub fn magnitude(num: &Num) -> i32 {
    if let Num::Pair(ref p1, ref p2) = *num {
        3 * magnitude(p1) + 2 * magnitude(p2)
    } else {
        num.value().unwrap()
    }
}

pub fn sum1(n1: Box<Num>, n2: Box<Num>) -> Box<Num> {
    let mut sum = Box::new(Num::Pair(n1, n2));
    reduce(&mut sum);
    sum
}

pub fn sum(mut nums: Vec<Box<Num>>) -> Box<Num> {
    let mut answer = nums.remove(0);
    for num in nums {
        answer = sum1(answer, num);
    }
    answer
}

// Tests

pub fn do_test_explode(input: &str, ans: &str) -> Result<()> {
    let mut num = parser::parse(input.as_bytes())?;
    let ans = parser::parse(ans.as_bytes())?;
    explode(&mut num[0]);
    assert_eq!(num, ans);
    Ok(())
}

#[test]
fn test_explode() -> Result<()> {
    do_test_explode("[[[[[9,8],1],2],3],4]\n", "[[[[0,9],2],3],4]\n")?;
    do_test_explode("[7,[6,[5,[4,[3,2]]]]]\n", "[7,[6,[5,[7,0]]]]\n")?;
    do_test_explode("[[6,[5,[4,[3,2]]]],1]\n", "[[6,[5,[7,0]]],3]\n")?;
    do_test_explode(
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]\n",
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]\n",
    )?;
    do_test_explode(
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]\n",
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]\n",
    )?;
    do_test_explode(
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n",
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n",
    )?;
    do_test_explode(
        "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n",
        "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n",
    )?;
    Ok(())
}

pub fn do_test_magnitude(input: &str, ans: i32) -> Result<()> {
    let num = parser::parse(input.as_bytes())?;
    assert_eq!(magnitude(&num[0]), ans);
    Ok(())
}

#[test]
fn test_magnitude() -> Result<()> {
    do_test_magnitude("[[1,2],[[3,4],5]]\n", 143)?;
    do_test_magnitude("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]\n", 1384)?;
    do_test_magnitude("[[[[1,1],[2,2]],[3,3]],[4,4]]\n", 445)?;
    do_test_magnitude("[[[[3,0],[5,3]],[4,4]],[5,5]]\n", 791)?;
    do_test_magnitude("[[[[5,0],[7,4]],[5,5]],[6,6]]\n", 1137)?;
    do_test_magnitude(
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]\n",
        3488,
    )?;
    Ok(())
}

pub fn do_test_sum(input: &str, ans: &str) -> Result<()> {
    let num = parser::parse(input.as_bytes())?;
    let ans = parser::parse(ans.as_bytes())?;
    let s = sum(num);
    assert_eq!(s, ans[0]);
    Ok(())
}

#[test]
fn test_sum() -> Result<()> {
    do_test_sum(
        "[1,1]\n[2,2]\n[3,3]\n[4,4]\n",
        "[[[[1,1],[2,2]],[3,3]],[4,4]]\n",
    )?;
    do_test_sum(
        "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n",
        "[[[[3,0],[5,3]],[4,4]],[5,5]]\n",
    )?;
    do_test_sum(
        "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]\n",
        "[[[[5,0],[7,4]],[5,5]],[6,6]]\n",
    )?;
    Ok(())
}

#[test]
fn test_sum_big() -> Result<()> {
    do_test_sum(
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n",
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]\n",
    )?;
    do_test_sum(
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
    [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
    [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
    [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
    [7,[5,[[3,8],[1,4]]]]
    [[2,[2,2]],[8,[8,1]]]
    [2,9]
    [1,[[[9,3],9],[[9,0],[0,7]]]]
    [[[5,[7,4]],7],1]
    [[[[4,2],2],6],[8,7]]
    ",
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]\n",
    )?;
    Ok(())
}
