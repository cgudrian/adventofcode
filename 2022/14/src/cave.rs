use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::ops::Range;

use crate::parser::Paths;

type Row = VecDeque<Stuff>;
type Position = (isize, isize);

#[derive(Debug, Clone)]
struct Bounds {
    x: Range<isize>,
    y: Range<isize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stuff {
    AIR,
    ROCK,
    SAND,
    INLET,
    TILDE,
}

const AIR: u8 = b'.';
const ROCK: u8 = b'#';
const SAND: u8 = b'o';
const INLET: u8 = b'+';

pub struct Cave {
    data: Vec<Row>,
    bounds: Bounds,
    inlet_pos: Position,
    has_bottom: bool,
}

impl Cave {
    /// Creates a cave with the given sand inlet position
    pub fn new(inlet_pos: Position, rocks: Option<&Paths>, has_bottom: bool) -> Cave {
        let mut cave = Cave {
            data: vec![[Stuff::INLET].into()],
            bounds: Bounds {
                x: (inlet_pos.0..inlet_pos.0 + 1).into(),
                y: (inlet_pos.1..inlet_pos.1 + 1).into(),
            },
            inlet_pos,
            has_bottom: false,
        };

        if let Some(rocks) = rocks {
            cave.add_rocks(rocks);
        }

        if has_bottom {
            cave.add_bottom();
        }

        cave
    }

    fn put(&mut self, pos: &Position, data: Stuff) {
        self.ensure_bounds(pos);
        let (row, col) = self.pos_to_idx(pos);
        self.data[row][col] = data;
    }

    fn width(&self) -> usize {
        self.bounds.x.len()
    }

    fn height(&self) -> usize {
        self.bounds.y.len()
    }

    fn ensure_bounds(&mut self, pos: &Position) {
        if self.contains(pos) {
            return;
        }

        if pos.0 >= self.bounds.x.end {
            let numcols = pos.0 - self.bounds.x.end + 1;
            // we need to append columns
            for y in self.bounds.y.clone() {
                let row = self.y_to_row(y);
                for _ in 0..numcols {
                    if self.has_bottom && y == self.bounds.y.end - 1 {
                        self.data[row].push_back(Stuff::ROCK);
                    } else {
                        self.data[row].push_back(Stuff::AIR);
                    }
                }
            }
        } else if pos.0 < self.bounds.x.start {
            // we need to prepend columns
            let numcols = self.bounds.x.start - pos.0;
            for y in self.bounds.y.clone() {
                let row = self.y_to_row(y);
                for _ in 0..numcols {
                    if self.has_bottom && y == self.bounds.y.end - 1 {
                        self.data[row].push_front(Stuff::ROCK);
                    } else {
                        self.data[row].push_front(Stuff::AIR);
                    }
                }
            }
        }

        let new_x_range = self.bounds.x.start.min(pos.0)..self.bounds.x.end.max(pos.0 + 1);

        if pos.1 >= self.bounds.y.end {
            let numrows = pos.1 - self.bounds.y.end + 1;
            // we need to append rows
            let row_of_air = (0..new_x_range.len()).map(|_| Stuff::AIR).collect::<VecDeque<_>>();
            for _ in 0..numrows {
                self.data.push(row_of_air.clone());
            }
        } else if pos.1 < self.bounds.y.start {
            panic!("Prepending rows is not allowed!");
        }

        let new_y_range = self.bounds.y.start.min(pos.1)..self.bounds.y.end.max(pos.1 + 1);

        self.bounds = Bounds {
            x: new_x_range,
            y: new_y_range,
        }
    }

    fn contains(&self, pos: &Position) -> bool {
        self.bounds.x.contains(&pos.0) && self.bounds.y.contains(&pos.1)
    }

    fn pos_to_idx(&self, pos: &Position) -> (usize, usize) {
        debug_assert!(self.contains(pos), "position out of bounds");
        (self.y_to_row(pos.1), (self.x_to_col(pos.0)))
    }

    fn y_to_row(&self, y: isize) -> usize {
        debug_assert!(self.bounds.y.contains(&y), "y out of bounds");
        (y - self.bounds.y.start) as usize
    }

    fn x_to_col(&self, x: isize) -> usize {
        debug_assert!(self.bounds.x.contains(&x), "x out of bounds");
        (x - self.bounds.x.start) as usize
    }

    fn get(&mut self, pos: &Position) -> Stuff {
        self.ensure_bounds(pos);
        let (row, col) = self.pos_to_idx(pos);
        self.data[row][col]
    }

    fn add_rocks(&mut self, rocks: &Paths) {
        for p in rocks.points_iter() {
            self.put(&(p.x() as isize, p.y() as isize), Stuff::ROCK)
        }
    }

    fn add_bottom(&mut self) {
        let bottom_y = self.bounds.y.end + 1;
        for x in self.bounds.x.clone() {
            self.put(&(x, bottom_y), Stuff::ROCK);
        }
        self.has_bottom = true;
    }

    pub fn drop_sand(&mut self) -> bool {
        let mut pos = self.inlet_pos;
        let mut old_pos = pos;
        while pos.1 + 1 < self.bounds.y.end {
            old_pos = pos;
            if !self.check_pos(&mut pos, (0, 1)) {
                if !self.check_pos(&mut pos, (-1, 1)) {
                    if !self.check_pos(&mut pos, (1, 1)) {
                        self.put(&pos, Stuff::SAND);
                        return pos != self.inlet_pos;
                    }
                }
            }
        }

        println!("Sand did not stop at {:?}", old_pos);
        self.put(&old_pos, Stuff::TILDE);
        false
    }

    fn check_pos(&mut self, pos: &mut Position, delta: (isize, isize)) -> bool {
        let new_pos = (pos.0 + delta.0, pos.1 + delta.1);
        if self.get(&new_pos) == Stuff::AIR {
            *pos = new_pos;
            true
        } else {
            false
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.data.iter() {
            for stuff in row.iter() {
                match *stuff {
                    Stuff::AIR => write!(f, ".")?,
                    Stuff::ROCK => write!(f, "#")?,
                    Stuff::SAND => write!(f, "o")?,
                    Stuff::INLET => write!(f, "+")?,
                    Stuff::TILDE => write!(f, "~")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cave_contains_just_the_inlet() {
        let mut c = Cave::new((500, 0), None, false);
        assert_eq!(c.width(), 1);
        assert_eq!(c.height(), 1);
        assert_eq!(c.get(&(500, 0)), Stuff::INLET);
    }

    #[test]
    fn write_access_appends_air_columns() {
        let mut c = Cave::new((500, 0), None, false);
        c.put(&(600, 0), Stuff::ROCK);
        assert_eq!(c.width(), 101);
        assert_eq!(c.height(), 1);
        assert_eq!(c.get(&(600, 0)), Stuff::ROCK);
        assert_eq!(c.get(&(550, 0)), Stuff::AIR);
        assert_eq!(c.get(&(500, 0)), Stuff::INLET);
    }

    #[test]
    fn write_access_prepends_air_columns() {
        let mut c = Cave::new((500, 0), None, false);
        c.put(&(-600, 0), Stuff::ROCK);
        assert_eq!(c.width(), 1101);
        assert_eq!(c.height(), 1);
        assert_eq!(c.get(&(-600, 0)), Stuff::ROCK);
        assert_eq!(c.get(&(0, 0)), Stuff::AIR);
        assert_eq!(c.get(&(500, 0)), Stuff::INLET);
    }

    #[test]
    fn write_access_appends_air_rows() {
        let mut c = Cave::new((500, 0), None, false);
        c.put(&(500, 10), Stuff::ROCK);
        assert_eq!(c.width(), 1);
        assert_eq!(c.height(), 11);
        assert_eq!(c.get(&(500, 10)), Stuff::ROCK);
        assert_eq!(c.get(&(500, 5)), Stuff::AIR);
        assert_eq!(c.get(&(500, 0)), Stuff::INLET);
    }

    #[test]
    fn write_access_appends_air_rows_and_columns() {
        let mut c = Cave::new((500, 0), None, false);
        c.put(&(600, 10), Stuff::ROCK);
        assert_eq!(c.width(), 101);
        assert_eq!(c.height(), 11);
        assert_eq!(c.get(&(600, 10)), Stuff::ROCK);
        assert_eq!(c.get(&(500, 5)), Stuff::AIR);
        assert_eq!(c.get(&(500, 0)), Stuff::INLET);
    }

    #[test]
    fn get_access_appends_air_rows() {
        let mut c = Cave::new((500, 0), None, false);
        assert_eq!(c.get(&(500, 10)), Stuff::AIR);
    }
}