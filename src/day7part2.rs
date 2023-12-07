use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Result;
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Hand([Card; 5]);

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let t1 = self.identify_type();
        let t2 = other.identify_type();
        Some(t1.cmp(&t2).then_with(|| self.0.cmp(&other.0)))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4],
        )
    }
}

impl std::str::FromStr for Hand {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand = s
            .split("")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<Card>())
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .or_else(|_| Err(eyre::eyre!("wrong number")))?;
        Ok(Hand(hand))
    }
}

impl Hand {
    fn identify_type(&self) -> HandType {
        let (m, j) = self
            .0
            .iter()
            .fold((HashMap::new(), 0), |(mut m, j), &c| match c {
                Card::J => (m, j + 1),
                _ => {
                    *m.entry(c).or_insert(0u8) += 1;
                    (m, j)
                }
            });
        let num = m.values().cloned().sorted().collect_vec();
        match (&num[..], j) {
            (&[5], 0) | (&[4], 1) | (&[3], 2) | (&[2], 3) | (&[1], 4) | (&[], 5) => {
                HandType::FiveOfAKind
            }
            (&[1, 4], 0) | (&[1, 3], 1) | (&[1, 2], 2) | (&[1, 1], 3) => HandType::FourOfAKind,
            (&[2, 3], 0) | (&[2, 2], 1) => HandType::FullHouse,
            (&[1, 1, 3], 0) | (&[1, 1, 2], 1) | (&[1, 1, 1], 2) => HandType::ThreeOfAKind,
            (&[1, 2, 2], 0) => HandType::TwoPairs,
            (&[1, 1, 1, 2], 0) | (&[1, 1, 1, 1], 1) => HandType::OnePair,
            (&[1, 1, 1, 1, 1], 0) => HandType::HighCard,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Ord, Display, FromStr)]
#[repr(u8)]
enum Card {
    J = 1,
    #[display("2")]
    N2 = 2,
    #[display("3")]
    N3 = 3,
    #[display("4")]
    N4 = 4,
    #[display("5")]
    N5 = 5,
    #[display("6")]
    N6 = 6,
    #[display("7")]
    N7 = 7,
    #[display("8")]
    N8 = 8,
    #[display("9")]
    N9 = 9,
    T = 10,
    Q = 12,
    K = 13,
    A = 14,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let card = *other as u8;
        Some((*self as u8).cmp(&card))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, FromStr)]
#[display("{hand} {bet}")]
struct Entry {
    hand: Hand,
    bet: u32,
}

#[aoc_generator(day7, part2)]
fn gen(input: &str) -> Result<Vec<Entry>> {
    Ok(input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<_>, _>>()?)
}

#[aoc(day7, part2)]
fn part1(input: &[Entry]) -> u32 {
    input
        .iter()
        .sorted_by(|a, b| a.hand.cmp(&b.hand))
        .enumerate()
        .map(|(pos, entry)| ((pos as u32) + 1) * entry.bet)
        .sum()
}

#[cfg(test)]
mod test {
    use super::{gen, part1};
    use eyre::Result;

    const EXAMPLE: &str = include_str!("../input/2023/day7-example.txt");

    #[test]
    fn p1() -> Result<()> {
        let result = part1(&gen(EXAMPLE)?);
        assert_eq!(result, 5905);
        Ok(())
    }
}
