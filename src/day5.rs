use std::ops::Range;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Result;
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Debug)]
struct Input {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

#[derive(Debug, Default)]
struct Map {
    _name: String,
    data: Vec<MapEntry>,
}

impl Map {
    fn apply(&self, x: usize) -> usize {
        for m in &self.data {
            if m.source <= x && (x - m.source) < m.count {
                let new_value = x - m.source + m.dest;
                return new_value;
            }
        }
        x
    }

    fn apply_ranges(&self, x: &Ranges) -> Ranges {
        let mut from = x.clone();
        let mut to = vec![];
        for m in &self.data {
            let mut rest = Ranges(vec![]);
            for r in from.0 {
                let (mapped, unmapped) = m.map_range(r);
                to.extend(mapped);
                rest.0.extend(unmapped);
            }
            from = rest;
            from.sort_and_merge();
        }
        from.0.extend(to);
        from.sort_and_merge();
        from
    }
}

impl Map {
    fn new(name: String) -> Self {
        Self {
            _name: name,
            data: vec![],
        }
    }

    fn sort(&mut self) {
        self.data.sort_by(|a, b| a.source.cmp(&b.source))
    }
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{dest} {source} {count}")]
struct MapEntry {
    source: usize,
    dest: usize,
    count: usize,
}

impl MapEntry {
    fn map_range(&self, r: Range<usize>) -> (Option<Range<usize>>, Vec<Range<usize>>) {
        // Cases:
        //     1          2         3         4
        //   | r |       | r |   | r |    |   r   |
        // |   m   |   | m |       | m |    | m |

        let (left, right) = (self.source, self.source + self.count);

        // No overlap
        if right <= r.start || left >= r.end {
            return (None, vec![r]);
        }

        // Case 1
        if left <= r.start && right >= r.end {
            let from = r.start + self.dest - self.source;
            let to = r.end + self.dest - self.source;
            return (Some(from..to), vec![]);
        }

        // Case 2
        if left <= r.start && right < r.end {
            let from = r.start + self.dest - self.source;
            let to = right + self.dest - self.source;
            return (Some(from..to), vec![right..r.end]);
        }

        // Case 3
        if left > r.start && right >= r.end {
            let from = left + self.dest - self.source;
            let to = r.end + self.dest - self.source;
            return (Some(from..to), vec![r.start..left]);
        }

        // Case 1
        if left > r.start && right < r.end {
            let from = left + self.dest - self.source;
            let to = right + self.dest - self.source;
            return (Some(from..to), vec![r.start..left, right..r.end]);
        }

        (None, vec![])
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Ranges(Vec<Range<usize>>);

impl Ranges {
    fn new(list: &[usize]) -> Self {
        let ranges = list.iter().tuples().map(|(&s, &n)| s..(s + n)).collect();
        Self(ranges)
    }

    fn sort_and_merge(&mut self) {
        self.0.sort_by(|a, b| a.start.cmp(&b.start));
        let v = self
            .0
            .iter()
            .cloned()
            .coalesce(|prev, curr| {
                if curr.start <= prev.end {
                    Ok(prev.start..(prev.end.max(curr.end)))
                } else {
                    Err((prev, curr))
                }
            })
            .collect();
        self.0 = v;
    }

    fn min(&self) -> usize {
        self.0
            .iter()
            .min_by(|a, b| a.start.cmp(&b.start))
            .unwrap()
            .start
    }
}

#[aoc_generator(day5)]
fn generator(input: &str) -> Input {
    let mut i = input.lines();
    let seeds = i.next().unwrap();
    let seeds = seeds.strip_prefix("seeds:").unwrap();
    let seeds = seeds
        .split_ascii_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut maps = vec![];

    let mut map = None::<Map>;
    for l in i {
        if l.is_empty() {
            match map.take() {
                Some(mut m) => {
                    m.sort();
                    maps.push(m);
                }
                None => {}
            }
        }

        if let Some(s) = l.strip_suffix(" map:") {
            map.replace(Map::new(s.to_string()));
        } else {
            match map.as_mut() {
                Some(m) => m.data.push(l.parse().unwrap()),
                None => {}
            }
        }
    }

    match map.take() {
        Some(m) => maps.push(m),
        None => {}
    }
    Input { seeds, maps }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> usize {
    let seeds = input.seeds.clone();
    let mut min_s = None;
    for s in seeds {
        let mut s = s;
        for m in &input.maps {
            s = m.apply(s);
        }
        let min_s = min_s.get_or_insert(s);
        if *min_s > s {
            *min_s = s;
        }
    }

    min_s.unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> usize {
    let mut ranges = Ranges::new(&input.seeds);
    for m in &input.maps {
        ranges = m.apply_ranges(&ranges);
    }
    ranges.min()
}

#[cfg(test)]
mod test {

    use super::{generator, part1, part2, Ranges};
    use eyre::Result;

    const DATA: &str = include_str!("../input/2023/day5-example.txt");

    #[test]
    fn p1() -> Result<()> {
        assert_eq!(part1(&generator(DATA)), 35);
        Ok(())
    }

    #[test]
    fn p2() -> Result<()> {
        assert_eq!(part2(&generator(DATA)), 46);
        Ok(())
    }

    #[test]
    fn sort_and_merge_test() {
        let mut r = Ranges::new(&[10, 4, 11, 2, 3, 3, 6, 3]);
        let c = Ranges::new(&[3, 6, 10, 4]);
        r.sort_and_merge();
        assert_eq!(r, c);
    }
}
