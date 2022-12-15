use std::iter::once;

use crate::parser::{Bounded, Bounds, Paths, Point};

mod parser;

struct Cave {
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
    sand_inlet: Point,
    data: Vec<Vec<u8>>,
}

const AIR: u8 = b'.';
const ROCK: u8 = b'#';
const SAND: u8 = b'o';
const INLET: u8 = b'+';

impl Cave {
    fn new(sand_inlet: Point, rocks: &Paths) -> Cave {
        let mut bounds = rocks.bounds().union(&sand_inlet.bounds());
        let width = bounds.delta_x().unwrap() + 1 + 30;
        let height = bounds.delta_y().unwrap() + 3;
        let xmin = bounds.xmin().unwrap() - 15;
        let xmax = bounds.xmax().unwrap() + 15;
        let ymin = bounds.ymin().unwrap();
        let ymax = bounds.ymax().unwrap();
        let mut data = (0..height).map(|_| vec![AIR; width]).collect::<Vec<_>>();
        data[sand_inlet.y() - ymin][sand_inlet.x() - xmin] = INLET;
        for p in rocks.points_iter() {
            data[p.y() - ymin][p.x() - xmin] = ROCK;
        }

        Cave {
            xmin,
            xmax,
            ymin,
            ymax,
            sand_inlet,
            data,
        }
    }

    pub fn add_bottom(&mut self) {
        let vec = self.data.last().as_mut();
        let y = self.data.len() - 1;
        for x in self.xmin..=self.xmax {
            self.set_rock(Point::new(x, y));
        }
    }

    fn get(&self, p: &Point) -> Option<u8> {
        if (p.y() < self.ymin || p.x() < self.xmin || p.x() > self.xmax) {
            None
        } else {
            Some(self.data[p.y() - self.ymin][p.x() - self.xmin])
        }
    }

    fn set_sand(&mut self, p: Point) {
        self.data[p.y() - self.ymin][p.x() - self.xmin] = b'o';
    }

    fn set_rock(&mut self, p: Point) {
        self.data[p.y() - self.ymin][p.x() - self.xmin] = b'#';
    }

    fn dump(&self) {
        for row in &self.data {
            println!("{}", String::from_utf8(row.clone()).unwrap())
        }
    }

    fn pos_is_free(&self, pos: &Point) -> bool {
        self.get(&pos).and_then(|p| Some(p == AIR || p == INLET)).unwrap_or(false)
    }

    fn drop_sand1(&mut self) -> bool {
        let mut pos = self.sand_inlet.clone();

        while pos.y() <= self.ymax {
            if self.pos_is_free(&pos.moved_down()) {
                pos = pos.moved_down();
            } else if self.pos_is_free(&pos.moved_down_left()) {
                pos = pos.moved_down_left();
            } else if self.pos_is_free(&pos.moved_down_right()) {
                pos = pos.moved_down_right()
            } else {
                self.set_sand(pos);
                return true;
            }
        }

        false
    }
    fn drop_sand2(&mut self) -> bool {
        let mut pos = self.sand_inlet.clone();

        loop {
            if self.pos_is_free(&pos.moved_down()) {
                pos = pos.moved_down();
            } else if self.pos_is_free(&pos.moved_down_left()) {
                pos = pos.moved_down_left();
            } else if self.pos_is_free(&pos.moved_down_right()) {
                pos = pos.moved_down_right()
            } else {
                self.set_sand(pos.clone());
                return pos != self.sand_inlet;
            }
        }
    }
}

fn solve1(input: &str) -> usize {
    let (_, rocks) = parser::parse(input).unwrap();
    let mut cave = Cave::new(Point::new(500, 0), &rocks);
    let mut counter = 0;
    while cave.drop_sand1() {
        counter += 1;
        //cave.dump();
    }
    cave.dump();
    counter
}

fn solve2(input: &str) -> usize {
    let (_, rocks) = parser::parse(input).unwrap();
    let mut cave = Cave::new(Point::new(500, 0), &rocks);
    cave.add_bottom();
    let mut counter = 0;
    while cave.drop_sand2() {
        counter += 1;
        //cave.dump();
    }
    cave.dump();
    counter + 1
}

static INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Answer 1: {}", solve1(INPUT));
    println!("Answer 2: {}", solve2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::parser;

    use super::*;

    static EX: &str = include_str!("example.txt");

    #[test]
    fn example1() {
        assert_eq!(solve2(EX), 24);
    }
}
