use aoc_runner_derive::aoc;
use eyre::{eyre, Result};
use itertools::Itertools;

#[aoc(day1, part1)]
fn part1(input: &str) -> Result<u32> {
    let mut sum: u32 = 0;
    for l in input.lines() {
        let nums = l.chars().filter_map(|c| c.to_digit(10)).collect_vec();
        let first = nums.first().ok_or(eyre!("no first"))?;
        let last = nums.last().ok_or(eyre!("no last"))?;
        sum += first * 10 + last;
    }
    Ok(sum)
}

fn nums_in_line(line: &str) -> Vec<u32> {
    let data = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let mut v = vec![];
    for digit in data {
        v.extend(line.match_indices(digit.0).map(|(p, _)| (p, digit.1)));
    }
    v.sort_by(|(a, _), (b, _)| a.cmp(b));
    v.iter().map(|&(_, n)| n).collect()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> Result<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let nums = nums_in_line(line);
        let first = nums.first().ok_or(eyre!("no first"))?;
        let last = nums.last().ok_or(eyre!("no last"))?;
        sum += first * 10 + last;
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::{nums_in_line, part1, part2};
    use eyre::Result;

    #[test]
    fn part1_example() -> Result<()> {
        let data = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(part1(data)?, 142);
        Ok(())
    }

    #[test]
    fn test_nums_in_line() -> Result<()> {
        let examples = [
            ("two1nine", vec![2, 1, 9]),
            ("eightwothree", vec![8, 2, 3]),
            ("abcone2threexyz", vec![1, 2, 3]),
            ("xtwone3four", vec![2, 1, 3, 4]),
            ("4nineeightseven2", vec![4, 9, 8, 7, 2]),
            ("zoneight234", vec![1, 8, 2, 3, 4]),
            ("7pqrstsixteen", vec![7, 6]),
        ];
        for e in examples {
            assert_eq!(nums_in_line(e.0), e.1);
        }
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<()> {
        let data = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(part2(data)?, 281);
        Ok(())
    }
}
