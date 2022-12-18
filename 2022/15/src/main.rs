use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::parser::{parse, Point};

mod parser;

fn solve1(input: &str, y: i32) -> usize {
    let (_, sensors) = parse(input).unwrap();
    sensors
        .into_iter()
        .flat_map(|sensor| sensor.positions_with_no_beacons(y))
        .flatten()
        .unique()
        .count()
}

fn process_ranges(mut ranges: Vec<RangeInclusive<i32>>, max: i32) -> Vec<RangeInclusive<i32>> {
    if ranges.is_empty() {
        return Vec::new();
    }

    if ranges.len() > 1 {
        ranges.sort_by(|a, b| a.start().cmp(b.start()));
    }

    let mut res = Vec::new();
    let mut cur: Option<RangeInclusive<i32>> = None;
    for range in ranges {
        if *range.end() < 0 {
            continue;
        }

        if *range.start() > max {
            break;
        }

        if let Some(c) = &cur {
            if *range.start() <= *c.end() + 1 {
                // merge this range into the current one
                cur = Some(*c.start()..=*range.end().clamp(c.end(), &max));
            } else {
                res.push(c.clone());
                cur = Some(*range.start().max(&0)..=*range.end().min(&max));
            }
        } else {
            cur = Some(*range.start().max(&0)..=*range.end().min(&max));
        }
    }

    if let Some(c) = cur {
        res.push(c);
    }

    res
}

fn solve2(input: &str, max_coord: i32) -> u64 {
    let (_, sensors) = parse(input).unwrap();
    let mut pos: Option<Point> = None;
    for y in 0..=max_coord {
        let ranges = sensors
            .iter()
            .flat_map(|sensor| sensor.range(y))
            .collect::<Vec<_>>();

        let ranges = process_ranges(ranges, max_coord);
        if ranges.is_empty() {
            continue;
        }
        // We're looking for exactly on free spot. So either we've one range with starts or
        // ends just before the boundary or we have exactly two ranges with a gap of one between them.
        if (ranges.len() == 2 && *ranges[1].start() == *ranges[0].end() + 2)
            || *ranges[0].start() == 1
            || *ranges[0].end() == max_coord - 1
        {
            pos = Some(Point(*ranges[0].end() + 1, y));
            println! {"Found: {ranges:?} pos={:?}", pos.unwrap()};
        }
    }
    pos.unwrap().tuning_frequency()
}

static INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Answer 1: {}", solve1(INPUT, 2000000));
    println!("Answer 2: {}", solve2(INPUT, 4000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EX1: &str = include_str!("example.txt");

    #[test]
    fn example1() {
        assert_eq!(solve1(EX1, 10), 26);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EX1, 20), 56000011);
    }
}

#[cfg(test)]
mod range_tests {
    use super::*;

    #[test]
    fn test_process_ranges_1() {
        let ranges = vec![0..=10, 20..=30];
        assert_eq!(process_ranges(ranges, 50), [0..=10, 20..=30]);
    }

    #[test]
    fn test_process_ranges_2() {
        let ranges = vec![-20..=-5, 0..=10, 20..=30];
        assert_eq!(process_ranges(ranges, 50), [0..=10, 20..=30]);
    }

    #[test]
    fn test_process_ranges_3() {
        let ranges = vec![-20..=-5, 0..=10, 20..=30, 60..=100];
        assert_eq!(process_ranges(ranges, 50), [0..=10, 20..=30]);
    }

    #[test]
    fn test_process_ranges_4() {
        let ranges = vec![-10..=40];
        assert_eq!(process_ranges(ranges, 50), [0..=40]);
    }

    #[test]
    fn test_process_ranges_5() {
        let ranges = vec![-10..=60];
        assert_eq!(process_ranges(ranges, 50), [0..=50]);
    }

    #[test]
    fn test_process_ranges_6() {
        let ranges = vec![0..=10, 5..=30];
        assert_eq!(process_ranges(ranges, 50), [0..=30]);
    }

    #[test]
    fn test_process_ranges_7() {
        let ranges = vec![0..=10, 5..=30, 17..=20];
        assert_eq!(process_ranges(ranges, 50), [0..=30]);
    }

    #[test]
    fn test_process_ranges_8() {
        let ranges = vec![-3..=7, 8..=10, 10..=14, 10..=10, 10..=18, 8..=8, 14..=26, 15..=19];
        assert_eq!(process_ranges(ranges, 20), [0..=20]);
    }

    #[test]
    fn test_process_ranges_9() {
        let ranges = vec![2..=2, 11..=13, 3..=13, -3..=3, 15..=25, 15..=17];
        assert_eq!(process_ranges(ranges, 20), [0..=13,15..=20]);
    }
}

