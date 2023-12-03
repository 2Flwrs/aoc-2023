use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::HashMap;

struct Schematic {
    data: Vec<Vec<char>>,
    width: usize,
}

impl Schematic {
    fn new(input: &str) -> Self {
        let data = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
        let width = data.first().unwrap().len();
        for l in &data {
            assert_eq!(width, l.len());
        }
        Self { data, width }
    }

    fn numbers(&self) -> NumberIter {
        NumberIter {
            sch: self,
            line: 0,
            pos: 0,
        }
    }

    fn lines(&self) -> usize {
        self.data.len()
    }

    fn at(&self, line: usize, pos: usize) -> Option<Entry> {
        ((line > 0 && line <= self.lines()) && (pos > 0 && pos <= self.width)).then(|| Entry {
            c: self.data[line - 1][pos - 1],
            line,
            pos,
        })
    }

    fn is_symbol_adjacent(&self, num: Number) -> bool {
        return self
            .adjacent(num)
            .iter()
            .any(|e| e.c != '.' && !e.c.is_ascii_digit());
    }

    fn adjacent(&self, num: Number) -> Vec<Entry> {
        let mut list = vec![];

        let (top, mid, bottom) = (num.line - 1, num.line, num.line + 1);
        let (left, right) = (num.pos - 1, num.pos + num.width);
        list.extend(self.at(mid, left));
        list.extend(self.at(mid, right));
        for x in left..=right {
            list.extend(self.at(top, x));
            list.extend(self.at(bottom, x));
        }

        list
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Entry {
    c: char,
    line: usize,
    pos: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Number {
    value: u32,
    line: usize,
    pos: usize,
    width: usize,
}

struct NumberIter<'a> {
    sch: &'a Schematic,
    line: usize,
    pos: usize,
}

impl<'a> Iterator for NumberIter<'a> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        while self.line < self.sch.lines() {
            while self.pos < self.sch.width {
                let s = &self.sch.data[self.line][self.pos..];
                if s[0].is_numeric() {
                    let chs = s
                        .iter()
                        .take_while(|c| c.is_ascii_digit())
                        .cloned()
                        .collect_vec();
                    let line = self.line + 1;
                    let pos = self.pos + 1;
                    let width = chs.len();
                    let value = chs.iter().collect::<String>().parse().unwrap();
                    self.pos += width;
                    return Some(Number {
                        value,
                        line,
                        pos,
                        width,
                    });
                } else {
                    self.pos += 1;
                }
            }
            self.pos = 0;
            self.line += 1;
        }
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Gear {
    line: usize,
    pos: usize,
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let sch = Schematic::new(input);
    let mut sum: u32 = 0;
    for n in sch.numbers() {
        if sch.is_symbol_adjacent(n) {
            sum += n.value;
        }
    }
    sum
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let sch = Schematic::new(input);

    let mut h: HashMap<_, Vec<_>> = HashMap::new();

    for num in sch.numbers() {
        for e in sch.adjacent(num) {
            if e.c == '*' {
                h.entry(e).or_default().push(num);
            }
        }
    }

    let mut sum = 0;
    for nums in h.values() {
        if nums.len() == 2 {
            sum += nums[0].value * nums[1].value;
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::{Number, Schematic};

    const DATA: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

    #[test]
    fn parse() {
        let d: &str = "467..114..\n...*......\n..35..633.";
        let reference = [
            Number {
                value: 467,
                line: 1,
                pos: 1,
                width: 3,
            },
            Number {
                value: 114,
                line: 1,
                pos: 6,
                width: 3,
            },
            Number {
                value: 35,
                line: 3,
                pos: 3,
                width: 2,
            },
            Number {
                value: 633,
                line: 3,
                pos: 7,
                width: 3,
            },
        ];

        itertools::assert_equal(Schematic::new(d).numbers(), reference);
    }

    #[test]
    fn part1_test() {
        assert_eq!(super::part1(DATA), 4361);
    }

    #[test]
    fn test_adjacent() {
        let sch = Schematic::new(DATA);

        let n = Number {
            value: 617,
            line: 5,
            pos: 1,
            width: 3,
        };

        assert!(sch.is_symbol_adjacent(n));
    }

    #[test]
    fn part2_test() {
        assert_eq!(super::part2(DATA), 467835);
    }
}
