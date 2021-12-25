// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
use std::io::{stdin, BufRead};

extern crate adventofcode2021;

// Parse

pub mod parser {
    use crate::Amphi;
    use crate::Cell;
    use crate::Node;
    use crate::Qa;
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::convert::TryInto;
    use std::io::BufRead;

    pub fn cell(input: &str) -> IResult<&str, Cell> {
        let (input, cell) =
            combinator::map_res(character::one_of(".#ABCD "), |c| c.try_into())(input)?;
        Ok((input, cell))
    }

    pub fn line(input: &str) -> IResult<&str, Vec<Cell>> {
        let (input, cells) = multi::many1(cell)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, cells))
    }

    pub fn all(input: &str) -> IResult<&str, Node> {
        let (input, _) = bytes::tag("#############\n")(input)?;
        let (input, cells0) = line(input)?;
        let (input, cells1) = line(input)?;
        let (input, cells4) = line(input)?;
        let (input, _) = bytes::tag("  #########\n")(input)?;
        let mut node = Node::default();
        node.g.extend(Qa::iter_in_y(0).unwrap().zip(cells0));
        node.g.extend(Qa::iter_in_y(1).unwrap().zip(cells1));
        let cells2 = vec![
            Cell::Wall,
            Cell::Wall,
            Cell::Wall,
            Cell::Amphi(Amphi::D),
            Cell::Wall,
            Cell::Amphi(Amphi::C),
            Cell::Wall,
            Cell::Amphi(Amphi::B),
            Cell::Wall,
            Cell::Amphi(Amphi::A),
        ];
        node.g.extend(Qa::iter_in_y(2).unwrap().zip(cells2));
        let cells3 = vec![
            Cell::Wall,
            Cell::Wall,
            Cell::Wall,
            Cell::Amphi(Amphi::D),
            Cell::Wall,
            Cell::Amphi(Amphi::B),
            Cell::Wall,
            Cell::Amphi(Amphi::A),
            Cell::Wall,
            Cell::Amphi(Amphi::C),
        ];
        node.g.extend(Qa::iter_in_y(3).unwrap().zip(cells3));
        node.g.extend(Qa::iter_in_y(4).unwrap().zip(cells4));
        Ok((input, node))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Node> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(all)(&input);
        match result {
            Ok((_, info)) => Ok(info),
            Err(e) => Err(anyhow!("error reading input: {:?}", e)),
        }
    }
}

type Sqrid = sqrid::sqrid_create!(13, 5, false);
type Qa = sqrid::qa_create!(Sqrid);
type Qr = sqrid::Qr;
type GridCell = sqrid::grid_create!(Sqrid, Cell);
type GridChar = sqrid::grid_create!(Sqrid, char);

/* Amphi ************************************************************/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Amphi {
    A,
    B,
    C,
    D,
}

impl Amphi {
    pub fn cost(&self) -> usize {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }
}

impl TryFrom<char> for Amphi {
    type Error = Error;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            _ => return Err(anyhow!("invalid amphipod {}", c)),
        })
    }
}

impl From<Amphi> for char {
    fn from(c: Amphi) -> char {
        match c {
            Amphi::A => 'A',
            Amphi::B => 'B',
            Amphi::C => 'C',
            Amphi::D => 'D',
        }
    }
}

/* Cell *************************************************************/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Wall,
    Amphi(Amphi),
}

impl Cell {
    pub fn amphi(&self) -> Option<Amphi> {
        if let Self::Amphi(amphi) = self {
            Some(*amphi)
        } else {
            None
        }
    }
    pub fn is_amphi(&self) -> bool {
        matches!(self, Self::Amphi(_))
    }
}

impl TryFrom<char> for Cell {
    type Error = Error;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            ' ' => Self::Wall,
            'A' | 'B' | 'C' | 'D' => Self::Amphi(c.try_into()?),
            _ => return Err(anyhow!("invalid cell {}", c)),
        })
    }
}

impl From<Cell> for char {
    fn from(c: Cell) -> char {
        match c {
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Amphi(a) => a.into(),
        }
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::Wall
    }
}

/* Node *************************************************************/

#[derive(Debug, Clone, Copy, Default)]
pub struct Node {
    pub g: GridCell,
    pub cost: usize,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let g = self
            .g
            .iter()
            .map(|&cell| {
                let c: char = cell.into();
                c
            })
            .collect::<GridChar>();
        write!(f, "{}", g)?;
        write!(f, "Cost: {}", self.cost)
    }
}

impl Node {
    pub fn amphi_target(&self, amphi: Amphi) -> Option<Qa> {
        let candidates = AMPHICOLUMN.iter().find(|(a, _)| *a == amphi)?;
        let cell = Cell::Amphi(amphi);
        for qa in candidates.1 {
            if self.g[qa] != cell {
                return Some(qa);
            }
        }
        None
    }
    pub fn done(&self) -> bool {
        for (amphi, targets) in AMPHICOLUMN {
            for qa in targets {
                if self.g[qa] != Cell::Amphi(amphi) {
                    return false;
                }
            }
        }
        true
    }
    pub fn eval(&self, qa0: Qa, qr: Qr) -> Option<Qa> {
        (qa0 + qr).filter(|qa| self.g[qa] == Cell::Empty)
    }
    pub fn apply(&mut self, act: &Action) {
        if let Ok(path) = Sqrid::astar_path(|qa, qr| self.eval(qa, qr), &act.src, &act.dst) {
            self.cost += self.g[act.src].amphi().unwrap().cost() * path.len();
            assert_eq!(self.g[act.dst], Cell::Empty);
            self.g[act.dst] = self.g[act.src];
            self.g[act.src] = Cell::Empty;
        }
    }
    pub fn action_check(&self, act: &Action) -> bool {
        matches!(
            Sqrid::astar_path(|qa, qr| self.eval(qa, qr), &act.src, &act.dst),
            Ok(_)
        )
    }
    pub fn iter_actions(&self) -> Vec<Action> {
        // Check perfect moves
        for src in Qa::iter() {
            if let Some(amphi) = self.g[src].amphi() {
                if let Some(dst) = self.amphi_target(amphi) {
                    let tsrc = src.tuple();
                    let tdst = dst.tuple();
                    if tsrc.0 == tdst.0 && tsrc.1 > tdst.1 {
                        continue;
                    }
                    let action = Action::new(src, dst);
                    if self.action_check(&action) {
                        return vec![action];
                    }
                }
            }
        }
        // No perfect moves, put all available moves in the array
        // Try moving amphis out
        let mut actions = vec![];
        let candidates = AMPHICOLUMN.iter().rev().collect::<Vec<_>>();
        for (amphi, arr) in candidates {
            let cell = Cell::Amphi(*amphi);
            let movable = arr
                .iter()
                .skip_while(|qa| self.g[**qa] == cell)
                .collect::<Vec<_>>();
            if let Some(src) = movable.iter().rev().find(|qa| self.g[**qa] != Cell::Empty) {
                for (dst, _) in Sqrid::bf_iter(|qa, qr| self.eval(qa, qr), src).flatten() {
                    if QS.contains(&dst) {
                        actions.push(Action::new(**src, dst));
                    }
                }
            }
        }
        actions
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Action {
    pub src: Qa,
    pub dst: Qa,
}

impl Action {
    pub fn new(src: Qa, dst: Qa) -> Action {
        Action { src, dst }
    }
}

pub const QA1: Qa = Qa::new::<1, 0>();
pub const QA2: Qa = Qa::new::<2, 0>();
// pub const QA3: Qa = Qa::new::<3, 0>(); // can't use
pub const QA4: Qa = Qa::new::<4, 0>();
// pub const QA5: Qa = Qa::new::<5, 0>(); // can't use
pub const QA6: Qa = Qa::new::<6, 0>();
// pub const QA7: Qa = Qa::new::<7, 0>(); // can't use
pub const QA8: Qa = Qa::new::<8, 0>();
// pub const QA9: Qa = Qa::new::<9, 0>(); // can't use
pub const QA10: Qa = Qa::new::<10, 0>();
pub const QA11: Qa = Qa::new::<11, 0>();
pub const QAA1: Qa = Qa::new::<3, 1>();
pub const QAB1: Qa = Qa::new::<5, 1>();
pub const QAC1: Qa = Qa::new::<7, 1>();
pub const QAD1: Qa = Qa::new::<9, 1>();
pub const QAA2: Qa = Qa::new::<3, 2>();
pub const QAB2: Qa = Qa::new::<5, 2>();
pub const QAC2: Qa = Qa::new::<7, 2>();
pub const QAD2: Qa = Qa::new::<9, 2>();
pub const QAA3: Qa = Qa::new::<3, 3>();
pub const QAB3: Qa = Qa::new::<5, 3>();
pub const QAC3: Qa = Qa::new::<7, 3>();
pub const QAD3: Qa = Qa::new::<9, 3>();
pub const QAA4: Qa = Qa::new::<3, 4>();
pub const QAB4: Qa = Qa::new::<5, 4>();
pub const QAC4: Qa = Qa::new::<7, 4>();
pub const QAD4: Qa = Qa::new::<9, 4>();
pub const QAS: [Qa; 4] = [QAA4, QAA3, QAA2, QAA1];
pub const QBS: [Qa; 4] = [QAB4, QAB3, QAB2, QAB1];
pub const QCS: [Qa; 4] = [QAC4, QAC3, QAC2, QAC1];
pub const QDS: [Qa; 4] = [QAD4, QAD3, QAD2, QAD1];
pub const AMPHICOLUMN: [(Amphi, [Qa; 4]); 4] = [
    (Amphi::A, QAS),
    (Amphi::B, QBS),
    (Amphi::C, QCS),
    (Amphi::D, QDS),
];
pub const QS: [Qa; 7] = [QA1, QA2, QA4, QA6, QA8, QA10, QA11];

pub type Solution = (usize, Vec<Action>, Node);

pub fn solver_helper(solutions: &mut Vec<Solution>, node0: &Node, actions: &mut Vec<Action>) {
    if node0.done() {
        solutions.push((node0.cost, actions.clone(), *node0));
        return;
    }
    let new_actions = node0.iter_actions();
    for action in new_actions {
        let mut node = *node0;
        node.apply(&action);
        actions.push(action);
        solver_helper(solutions, &node, actions);
        actions.pop();
    }
}

pub fn solver(node0: &Node) -> Vec<Solution> {
    let mut solutions = vec![];
    solver_helper(&mut solutions, node0, &mut vec![]);
    solutions
}

/* Main *************************************************************/

fn process(bufin: impl BufRead) -> Result<usize> {
    let mut node = parser::parse(bufin)?;
    let mut solutions = solver(&node);
    solutions.sort_by_key(|n| n.2.cost);
    eprintln!("{}", node);
    for action in &solutions[0].1 {
        eprintln!("action {:?}", action);
        node.apply(action);
        eprintln!("{}", node);
    }
    Ok(solutions[0].2.cost)
}

#[test]
fn test() -> Result<()> {
    let input = adventofcode2021::examples::DAY23;
    assert_eq!(process(input.as_bytes())?, 44169);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
