use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

struct Counts(Vec<usize>, Vec<usize>);

impl Counts {
    fn sum_all_distances(&self, expansion: usize) -> usize {
        sum_distances(&self.0, expansion) + sum_distances(&self.1, expansion)
    }
}

#[aoc_generator(day11)]
fn gen(input: &str) -> Counts {
    let v = input
        .lines()
        .map(|s| s.chars().map(|c| c == '#').collect_vec())
        .collect_vec();
    assert!(v.iter().map(Vec::len).all_equal());
    let mut x = vec![0usize; v[0].len()];
    for line in &v {
        for (i, &star) in line.iter().enumerate() {
            if star {
                x[i] += 1;
            }
        }
    }
    let x = x;
    let y = v
        .iter()
        .map(|l| l.iter().filter(|&&x| x).count())
        .collect_vec();

    Counts(x, y)
}

fn sum_distances(v: &[usize], expansion: usize) -> usize {
    let total: usize = v.iter().sum();
    let mut rest = total;
    let mut active_edges = 0;
    let mut active_nodes = 0;
    let mut sum = 0;
    for &n in v {
        if n == 0 {
            sum += active_edges * expansion;
        } else {
            sum += active_edges;
            rest -= n;
            let new = n * rest;
            let end = active_nodes * n;
            // println!("{active_nodes} {n} {rest} | {new} {end} | {active_edges} {sum}");
            active_edges = active_edges + new - end;
            active_nodes += n;
        }
    }
    assert_eq!(active_edges, 0);
    assert_eq!(active_nodes, total);
    assert_eq!(rest, 0);
    sum
}

#[aoc(day11, part1)]
fn part1(input: &Counts) -> usize {
    input.sum_all_distances(2)
}

#[aoc(day11, part2)]
fn part2(input: &Counts) -> usize {
    input.sum_all_distances(1000000)
}

#[cfg(test)]
mod test {
    const EX: &str = include_str!("../input/2023/day11-ex.txt");

    #[test]
    fn p1() {
        assert_eq!(super::part1(&super::gen(EX)), 374);
    }

    #[test]
    fn sd() {
        assert_eq!(super::sum_distances(&vec![1, 0, 1], 2), 3);
        assert_eq!(super::sum_distances(&vec![1, 1, 1], 2), 4);
    }

    #[test]
    fn p2() {
        let counts = super::gen(EX);
        assert_eq!(counts.sum_all_distances(10), 1030);
        assert_eq!(counts.sum_all_distances(100), 8410);
    }
}
