use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day9)]
fn gen(input: &str) -> eyre::Result<Vec<Vec<i32>>> {
    Ok(input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?)
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<i32>]) -> i32 {
    input.iter().map(|x| extrapolate(x)).sum()
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    for v in input {
        let mut v = v.clone();
        v.reverse();
        sum += extrapolate(&v);
    }
    sum
}

fn extrapolate(x: &Vec<i32>) -> i32 {
    let mut v = x.clone();

    let mut sum = 0;
    loop {
        sum += *v.last().unwrap();
        v = v
            .into_iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();
        if v.len() == 1 {
            return sum + v[0];
        } else if v.iter().all(|x| *x == 0) {
            return sum;
        }
    }
}

#[cfg(test)]
mod test {
    const EX: &str = include_str!("../input/2023/day9-ex.txt");

    #[test]
    fn p1() -> eyre::Result<()> {
        assert_eq!(super::part1(&super::gen(EX)?), 114);
        Ok(())
    }
    #[test]
    fn p2() -> eyre::Result<()> {
        assert_eq!(super::part2(&super::gen(EX)?), 2);
        Ok(())
    }
}
