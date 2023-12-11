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
        result
    }
}

impl Map {
    fn find_start(&self) -> (Pos, Tile) {
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
            let s = Pos(x as isize, y as isize);
            let dirs = Dir::ALL
                .iter()
                .filter(|&d| {
                    self.get(s.mv(d))
                        .and_then(|t| t.traverse(d.inv()))
                        .is_some()
                })
                .cloned()
                .collect_vec();
            assert_eq!(dirs.len(), 2);
            let t = Tile::from_directions((dirs[0], dirs[1]));
            (s, t)
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

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn new(width: usize, height: usize) -> Self {
        Self(vec![vec![Tile::Ground; width]; height])
    }

    fn put(&mut self, pos: Pos, tile: Tile) {
        let (x, y) = (pos.0 as usize, pos.1 as usize);
        self.0[y][x] = tile;
    }
}

impl std::str::FromStr for Map {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

    fn from_directions(pair: (Dir, Dir)) -> Self {
        let mut v = vec![pair.0, pair.1];
        v.sort();
        match (v[0], v[1]) {
            (Dir::N, Dir::S) => Tile::NS,
            (Dir::N, Dir::E) => Tile::NE,
            (Dir::N, Dir::W) => Tile::NW,
            (Dir::S, Dir::E) => Tile::SE,
            (Dir::S, Dir::W) => Tile::SW,
            (Dir::E, Dir::W) => Tile::EW,
            _ => panic!(),
        }
    }

    fn dirs(&self) -> (Dir, Dir) {
        match self {
            Tile::NS => (Dir::N, Dir::S),
            Tile::EW => (Dir::E, Dir::W),
            Tile::NE => (Dir::N, Dir::E),
            Tile::NW => (Dir::N, Dir::W),
            Tile::SW => (Dir::S, Dir::W),
            Tile::SE => (Dir::S, Dir::E),
            Tile::Ground => panic!(),
            Tile::Start => panic!(),
        }
    }

    fn is_start(&self) -> bool {
        matches!(self, Self::Start)
    }

    fn inout(&self, i: InOut) -> InOut {
        let (t, b) = (i.2, i.3);

        let (tr, br) = match (t, b) {
            (true, true) => match &self {
                Tile::NS => (false, false),
                Tile::NE => (false, true),
                Tile::SE => (true, false),
                Tile::Ground => (true, true),
                _ => panic!(),
            },
            (true, false) => match &self {
                Tile::EW => (true, false),
                Tile::NW => (false, false),
                Tile::SW => (true, true),
                _ => panic!(),
            },
            (false, true) => match &self {
                Tile::EW => (false, true),
                Tile::NW => (true, true),
                Tile::SW => (false, false),
                _ => panic!(),
            },
            (false, false) => match &self {
                Tile::NS => (true, true),
                Tile::NE => (true, false),
                Tile::SE => (false, true),
                Tile::Ground => (false, false),
                _ => {
                    panic!("Bad combo {} {} {}", t, b, self)
                }
            },
        };

        InOut(t, b, tr, br)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

/// InOut
///
/// (NW, SW, NE, SE)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct InOut(bool, bool, bool, bool);

impl InOut {
    fn new() -> Self {
        Self(false, false, false, false)
    }
    fn all_inside(&self) -> bool {
        matches!(self, Self(true, true, true, true))
    }
}

#[aoc_generator(day10)]
fn gen(input: &str) -> eyre::Result<Map> {
    Ok(input.parse()?)
}

#[aoc(day10, part1)]
fn part1(map: &Map) -> eyre::Result<usize> {
    let (s, t) = map.find_start();
    let dirs = t.dirs();
    let mut v = [(s, dirs.0), (s, dirs.1)];

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

#[aoc(day10, part2)]
fn part2(map: &Map) -> eyre::Result<usize> {
    let (s, t) = map.find_start();
    let mut m = Map::new(map.width(), map.height());
    m.put(s, t);

    let dirs = t.dirs();
    let mut v = [(s, dirs.0), (s, dirs.1)];
    loop {
        for x in &mut v {
            let (pos, dir) = *x;
            let new_pos = pos.mv(dir);
            let t = map
                .get(new_pos)
                .ok_or_else(|| eyre::eyre!("outside of map"))?;
            m.put(new_pos, t);
            let new_dir = t
                .traverse(dir.inv())
                .ok_or_else(|| eyre::eyre!("bad traverse"))?;
            *x = (new_pos, new_dir);
        }
        if v[0].0 == v[1].0 {
            break;
        }
    }

    let mut count = 0;
    for line in m.0 {
        let mut i = InOut::new();
        for tile in line {
            let j = tile.inout(i);
            i = j;
            if i.all_inside() {
                count += 1;
            }
        }
    }

    Ok(count)
}

#[cfg(test)]
mod test {
    use eyre::Result;

    const EX1: &str = include_str!("../input/2023/day10-ex1.txt");
    const EX2: &str = include_str!("../input/2023/day10-ex2.txt");
    const EX3: &str = include_str!("../input/2023/day10-ex3.txt");
    const EX4: &str = include_str!("../input/2023/day10-ex4.txt");

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

    #[test]
    fn p2a() -> Result<()> {
        assert_eq!(super::part2(&super::gen(EX3)?)?, 4);
        Ok(())
    }

    #[test]
    fn p2b() -> Result<()> {
        assert_eq!(super::part2(&super::gen(EX4)?)?, 10);
        Ok(())
    }
}
