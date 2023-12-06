use aoc_runner_derive::aoc;
use eyre::{eyre, Result};

struct Input1 {
    time: Vec<u64>,
    dist: Vec<u64>,
}

impl std::str::FromStr for Input1 {
    type Err = eyre::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut l = s.lines();
        let time = l
            .next()
            .ok_or_else(|| eyre!("no first line"))?
            .strip_prefix("Time:")
            .ok_or_else(|| eyre!("bad time prefix"))?
            .split_ascii_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<u64>, _>>()?;
        let dist = l
            .next()
            .ok_or_else(|| eyre!("no second line"))?
            .strip_prefix("Distance:")
            .ok_or_else(|| eyre!("bad dist prefix"))?
            .split_ascii_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<u64>, _>>()?;
        Ok(Input1 { time, dist })
    }
}

#[aoc(day6, part1)]
fn part1(input: &str) -> Result<u64> {
    let input = input.parse::<Input1>()?;
    let times = input.time.iter().cloned();
    let dists = input.dist.iter().cloned();
    let pairs = times.zip(dists);

    Ok(pairs.map(|(time, dist)| margin(time, dist)).product())
}

fn margin(time: u64, dist: u64) -> u64 {
    let t: f64 = time as f64;
    let d: f64 = dist as f64;

    assert!(4.0 * d <= t * t);

    let r = (t * t - 4.0 * d).sqrt();
    let low = 0.5 * (t - r);
    let high = 0.5 * (t + r);
    let low = low.floor() as u64;
    let high = high.ceil() as u64;

    high - low - 1
}

#[aoc(day6, part2)]
fn part2(input: &str) -> Result<u64> {
    let mut l = input.lines();
    let time = l
        .next()
        .ok_or_else(|| eyre!("no first line"))?
        .strip_prefix("Time:")
        .ok_or_else(|| eyre!("bad time prefix"))?
        .split_ascii_whitespace()
        .collect::<String>()
        .parse()?;
    let dist = l
        .next()
        .ok_or_else(|| eyre!("no second line"))?
        .strip_prefix("Distance:")
        .ok_or_else(|| eyre!("bad dist prefix"))?
        .split_ascii_whitespace()
        .collect::<String>()
        .parse()?;
    Ok(margin(time, dist))
}

#[cfg(test)]
mod test {
    use eyre::Result;

    const EXAMPLE: &str = include_str!("../input/2023/day6-example.txt");

    #[test]
    fn gen_test() -> Result<()> {
        let i: super::Input1 = EXAMPLE.parse()?;
        assert_eq!(&i.time, &[7, 15, 30]);
        assert_eq!(&i.dist, &[9, 40, 200]);

        Ok(())
    }

    #[test]
    fn part1() -> Result<()> {
        let result = super::part1(EXAMPLE)?;
        assert_eq!(result, 288);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let result = super::part2(EXAMPLE)?;
        assert_eq!(result, 71503);
        Ok(())
    }
}
