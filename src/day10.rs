use std::borrow::Borrow;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Debug)]
struct Map(Vec<Vec<Tile>>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pos(isize, isize);

impl Pos {
    fn mv<D: Borrow<Dir>>(&self, d: D) -> Pos {
        let d = d.borrow();
        let result = match d {
            Dir::N => Pos(self.0, self.1 - 1),
            Dir::S => Pos(self.0, self.1 + 1),
            Dir::E => Pos(self.0 + 1, self.1),
            Dir::W => Pos(self.0 - 1, self.1),
        };
        // println!("mv {self:?} {d:?} -> {result:?}");
        result
    }
}

impl Map {
    fn find_start(&self) -> Pos {
        if let Some((y, Some(x))) = self
            .0
            .iter()
            .map(|v| {
                v.iter()
                    .find_position(|t| t.is_start())
                    .and_then(|(x, _)| Some(x))
            })
            .find_position(|vv| vv.is_some())
        {
            Pos(x as isize, y as isize)
        } else {
            panic!("missing start")
        }
    }

    fn get<P: Borrow<Pos>>(&self, p: P) -> Option<Tile> {
        let p = p.borrow().clone();
        if p.0.is_negative() || p.1.is_negative() {
            return None;
        }
        let (x, y) = (p.0 as usize, p.1 as usize);
        if y >= self.0.len() || x >= self.0[0].len() {
            return None;
        }
        Some(self.0[y][x])
    }
}

impl std::str::FromStr for Map {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("map:\n{s}\n");
        let d = s
            .lines()
            .map(|l| {
                l.split("")
                    .filter(|c| !c.is_empty())
                    .map(|c| c.parse::<Tile>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        if !d.iter().map(|v| v.len()).all_equal() {
            return Err(eyre::eyre!("all rows not equally long"));
        };
        Ok(Self(d))
    }
}

#[derive(Clone, Copy, Debug, Display, FromStr, PartialEq, Eq)]
enum Tile {
    #[display("|")]
    NS,
    #[display("-")]
    EW,
    #[display("L")]
    NE,
    #[display("J")]
    NW,
    #[display("7")]
    SW,
    #[display("F")]
    SE,
    #[display(".")]
    Ground,
    #[display("S")]
    Start,
}

impl Tile {
    fn traverse<D: Borrow<Dir>>(&self, from: D) -> Option<Dir> {
        let from = from.borrow();
        let result = match from {
            Dir::N => match self {
                Tile::NS => Some(Dir::S),
                Tile::NE => Some(Dir::E),
                Tile::NW => Some(Dir::W),
                _ => None,
            },
            Dir::S => match self {
                Tile::NS => Some(Dir::N),
                Tile::SW => Some(Dir::W),
                Tile::SE => Some(Dir::E),
                _ => None,
            },
            Dir::E => match self {
                Tile::EW => Some(Dir::W),
                Tile::NE => Some(Dir::N),
                Tile::SE => Some(Dir::S),
                _ => None,
            },
            Dir::W => match self {
                Tile::EW => Some(Dir::E),
                Tile::NW => Some(Dir::N),
                Tile::SW => Some(Dir::S),
                _ => None,
            },
        };
        // println!("{self:?} {from:?} -> {result:?}");
        result
    }

    fn is_start(&self) -> bool {
        matches!(self, Self::Start)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    const ALL: [Self; 4] = [Dir::N, Dir::S, Dir::E, Dir::W];
    fn inv(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }
}

#[aoc_generator(day10)]
fn gen(input: &str) -> eyre::Result<Map> {
    Ok(input.parse()?)
}

#[aoc(day10, part1)]
fn part1(map: &Map) -> eyre::Result<usize> {
    let s = map.find_start();
    // println!("start: {s:?}");
    let start_directions = Dir::ALL
        .iter()
        .filter(|&d| map.get(s.mv(d)).and_then(|t| t.traverse(d.inv())).is_some())
        .cloned()
        .collect_vec();
    assert_eq!(start_directions.len(), 2);
    let mut v = [(s, start_directions[0]), (s, start_directions[1])];

    // println!("=== starting traversal ===");
    for i in 1.. {
        // println!("v: {v:?}");
        for x in &mut v {
            let (pos, dir) = *x;
            let new_pos = pos.mv(dir);
            let t = map
                .get(new_pos)
                .ok_or_else(|| eyre::eyre!("outside of map"))?;
            let new_dir = t
                .traverse(dir.inv())
                .ok_or_else(|| eyre::eyre!("bad traverse"))?;
            *x = (new_pos, new_dir);
        }
        if v[0].0 == v[1].0 {
            return Ok(i);
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use eyre::Result;

    const EX1: &str = include_str!("../input/2023/day10-ex1.txt");
    const EX2: &str = include_str!("../input/2023/day10-ex2.txt");

    #[test]
    fn parse_test() -> Result<()> {
        let _ex1 = EX1.parse::<super::Map>()?;
        let _ex2 = EX2.parse::<super::Map>()?;
        Ok(())
    }

    #[test]
    fn p1() -> Result<()> {
        assert_eq!(super::part1(&super::gen(EX1)?)?, 4);
        assert_eq!(super::part1(&super::gen(EX2)?)?, 8);
        Ok(())
    }
}
