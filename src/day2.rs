use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Result;
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Game {index}: {runs}")]
struct Game {
    index: u32,
    runs: Draws,
}

#[derive(Debug, PartialEq)]
struct Draws(Vec<Vec<Draw>>);

impl std::str::FromStr for Draws {
    type Err = parse_display::ParseError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let x = s
            .trim()
            .split(";")
            .map(|draws| {
                draws
                    .trim()
                    .split(",")
                    .map(|d| d.trim().parse::<Draw>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Draws(x))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct DrawParseError {}

impl std::fmt::Display for Draws {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatter = self.0.iter().format_with("; ", |row, f| {
            f(&row.iter().format_with(", ", |elt, g| g(&elt)))
        });
        write!(f, "{formatter}")
    }
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
enum Draw {
    #[display("{0} red")]
    Red(u32),
    #[display("{0} green")]
    Green(u32),
    #[display("{0} blue")]
    Blue(u32),
}

impl Draw {
    fn extract_red(&self) -> Option<u32> {
        match *self {
            Draw::Red(n) => Some(n),
            _ => None,
        }
    }
    fn extract_green(&self) -> Option<u32> {
        match *self {
            Draw::Green(n) => Some(n),
            _ => None,
        }
    }
    fn extract_blue(&self) -> Option<u32> {
        match *self {
            Draw::Blue(n) => Some(n),
            _ => None,
        }
    }
}

#[aoc_generator(day2)]
fn generator2(input: &str) -> Result<Vec<Game>> {
    Ok(input
        .lines()
        .map(|l| l.parse::<Game>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn is_draw_ok(d: &Draw) -> bool {
    match *d {
        Draw::Red(n) => return n <= 12,
        Draw::Green(n) => return n <= 13,
        Draw::Blue(n) => return n <= 14,
    }
}

#[aoc(day2, part1)]
fn part1(input: &[Game]) -> u32 {
    input
        .iter()
        .filter_map(|g| {
            let is_all_ok = g.runs.0.iter().flatten().all(is_draw_ok);
            if is_all_ok {
                Some(g.index)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Game]) -> u32 {
    input.iter().map(|g| power_of(&g.runs.0)).sum()
}

fn power_of(runs: &[Vec<Draw>]) -> u32 {
    [Draw::extract_red, Draw::extract_green, Draw::extract_blue]
        .iter()
        .map(|extraction| {
            runs.iter()
                .flatten()
                .filter_map(extraction)
                .max()
                .unwrap_or(0)
        })
        .product()
}

#[cfg(test)]
mod test {
    use super::{generator2, part1, part2};
    use eyre::Result;

    const DATA: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(part1(&generator2(DATA)?), 8);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<()> {
        assert_eq!(part2(&generator2(DATA)?), 2286);
        Ok(())
    }
}
