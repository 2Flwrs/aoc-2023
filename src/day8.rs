use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Clone, Copy, PartialEq, Eq, Display, FromStr)]
enum Dir {
    L,
    R,
}

impl Dir {
    fn select<T: Copy>(&self, pair: &(T, T)) -> T {
        match self {
            Dir::L => pair.0,
            Dir::R => pair.1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node([char; 3]);

impl Node {
    const START: Node = Node(['A', 'A', 'A']);
    const GOAL: Node = Node(['Z', 'Z', 'Z']);

    fn ends_in(&self, c: char) -> bool {
        self.0[2] == c
    }
}

impl std::str::FromStr for Node {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s
            .trim()
            .chars()
            .collect_vec()
            .try_into()
            .or_else(|_| Err(eyre::eyre!("wrong number")))?;
        Ok(Node(name))
    }
}

fn parse_node_map(s: &str) -> eyre::Result<(Node, (Node, Node))> {
    let (key, pair) = s.split_once(" = ").ok_or_else(|| eyre::eyre!("no eq"))?;
    let (left, right) = pair
        .strip_prefix("(")
        .ok_or_else(|| eyre::eyre!("no ("))?
        .strip_suffix(")")
        .ok_or_else(|| eyre::eyre!("no )"))?
        .split_once(", ")
        .ok_or_else(|| eyre::eyre!("pair"))?;
    Ok((key.parse()?, (left.parse()?, right.parse()?)))
}

type NodeMap = HashMap<Node, (Node, Node)>;

struct Input {
    directions: Vec<Dir>,
    map: NodeMap,
}

impl std::str::FromStr for Input {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut l = s.lines();
        let first = l.next().ok_or_else(|| eyre::eyre!("no first line"))?;
        let directions = first
            .split("")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<Dir>())
            .collect::<Result<Vec<_>, _>>()?;

        let map = l
            .filter(|s| !s.is_empty())
            .map(|s| parse_node_map(s))
            .collect::<Result<NodeMap, _>>()?;

        Ok(Input { directions, map })
    }
}

#[aoc_generator(day8)]
fn gen(input: &str) -> eyre::Result<Input> {
    input.parse()
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> usize {
    let mut n = Node::START;
    let mut count = 0;
    for dir in input.directions.iter().cycle() {
        count += 1;
        let pair = input.map.get(&n).unwrap();
        n = dir.select(pair);
        if n == Node::GOAL {
            return count;
        }
    }
    panic!()
}

fn get_offset_and_period(node: &Node, input: &Input) -> (u64, u64) {
    let mut n = *node;
    let mut s = HashMap::<Node, u64>::new();
    let mut count = 0;
    for dir in input.directions.iter().cycle() {
        count += 1;
        let pair = input.map.get(&n).unwrap();
        n = dir.select(pair);
        if n.ends_in('Z') {
            if let Some(first) = s.insert(n, count) {
                let period = count - first;
                let offset = first - period;
                return (offset, period);
            }
        }
    }
    panic!()
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> u64 {
    let start_nodes = input
        .map
        .keys()
        .filter_map(|n| n.ends_in('A').then_some(*n))
        .collect_vec();
    let offsets_and_periods = start_nodes
        .iter()
        .map(|n| get_offset_and_period(n, &input))
        .collect_vec();

    // The input has all offsets = 0. The math is much harder otherwise.
    assert!(offsets_and_periods.iter().all(|&(first, _)| first == 0));

    let periods = offsets_and_periods
        .into_iter()
        .map(|(_, p)| p)
        .collect_vec();

    lcmx::lcmx(&periods).unwrap()
}

#[cfg(test)]
mod test {
    use eyre::Result;

    const EX1: &str = include_str!("../input/2023/day8-ex1.txt");
    const EX2: &str = include_str!("../input/2023/day8-ex2.txt");
    const EX3: &str = include_str!("../input/2023/day8-ex3.txt");

    #[test]
    fn p1_ex1() -> Result<()> {
        assert_eq!(super::part1(&super::gen(EX1)?), 2);
        Ok(())
    }

    #[test]
    fn p1_ex2() -> Result<()> {
        assert_eq!(super::part1(&super::gen(EX2)?), 6);
        Ok(())
    }

    #[test]
    fn p2() -> Result<()> {
        assert_eq!(super::part2(&super::gen(EX3)?), 6);
        Ok(())
    }
}
