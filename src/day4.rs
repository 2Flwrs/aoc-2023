use std::{collections::VecDeque, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::{eyre, Result};
use itertools::{assert_equal, Itertools};

#[derive(Debug)]
struct Card {
    id: u32,
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Card {
    fn matches(&self) -> u32 {
        let winning = self.left.iter().sorted_unstable().cloned().collect_vec();
        let mut matches = 0;
        for &mine in &self.right {
            if winning.binary_search(&mine).is_ok() {
                matches += 1;
            }
        }
        matches
    }
}

fn parse_list(s: &str) -> Result<Vec<u32>> {
    let v = s
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(v)
}

impl FromStr for Card {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pre, lists) = s.split_once(':').ok_or_else(|| eyre!("split1"))?;
        let id = pre.strip_prefix("Card").ok_or_else(|| eyre!("card"))?;
        let id = id.trim().parse::<u32>()?;

        let (left, right) = lists.split_once('|').ok_or_else(|| eyre!("split2"))?;
        let left = parse_list(left)?;
        let right = parse_list(right)?;

        Ok(Card { id, left, right })
    }
}

#[aoc_generator(day4)]
fn generator(input: &str) -> Result<Vec<Card>> {
    Ok(input
        .lines()
        .map(|l| l.parse::<Card>())
        .collect::<Result<Vec<_>, _>>()?)
}

#[aoc(day4, part1)]
fn part1(cards: &[Card]) -> u32 {
    cards
        .iter()
        .map(|c| c.matches())
        .filter(|&m| m > 0)
        .map(|m| 1u32 << (m - 1))
        .sum()
}

#[aoc(day4, part2)]
fn part2(cards: &[Card]) -> usize {
    let num_cards: u32 = cards.len() as u32;
    let id_iter = cards.iter().map(|c| c.id);
    let id_range = (1..=num_cards).into_iter();
    assert_equal(id_iter, id_range);

    let mut counts = VecDeque::new();
    counts.push_back(1usize);

    let mut sum = 0;
    for c in cards {
        let count = counts.pop_front().unwrap_or(1);
        sum += count;
        let matches = c.matches() as usize;
        for i in 0..matches {
            if let Some(cnt) = counts.get_mut(i) {
                *cnt += count;
            } else {
                counts.push_back(1 + count);
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {

    use super::{generator, part1, part2};
    use eyre::Result;

    const DATA: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn p1() -> Result<()> {
        assert_eq!(part1(&generator(DATA)?), 13);
        Ok(())
    }

    #[test]
    fn p2() -> Result<()> {
        assert_eq!(part2(&generator(DATA)?), 30);
        Ok(())
    }
}
