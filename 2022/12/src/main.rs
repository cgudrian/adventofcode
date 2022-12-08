use std::collections::{HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::mem;

use strum::EnumIter;
use strum::IntoEnumIterator;

enum Part {
    One,
    Two,
}

#[derive(Clone, Copy, PartialEq, Hash, Eq, EnumIter, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
        }
    }
}

type Input = [u8];

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
struct Position(usize, usize);

struct Map {
    rows: Vec<Vec<u8>>,
    start_pos: Position,
    end_pos: Position,
}

impl Map {
    fn load(input: &Input) -> Map {
        let mut rows = Vec::new();
        let mut start_pos: Option<Position> = None;
        let mut end_pos: Option<Position> = None;
        for line in input.split(|c| *c == b'\n') {
            if line.is_empty() {
                continue;
            }
            let mut cols = Vec::new();
            for c in line {
                match c {
                    b'a'..=b'z' => {
                        cols.push(*c);
                    }
                    b'S' => {
                        start_pos = Some(Position(rows.len(), cols.len()));
                        cols.push(b'a');
                    }
                    b'E' => {
                        end_pos = Some(Position(rows.len(), cols.len()));
                        cols.push(b'z');
                    }
                    _ => panic!("Invalid character: {}", c),
                }
            }
            rows.push(cols);
        }

        Map {
            rows,
            start_pos: start_pos.unwrap(),
            end_pos: end_pos.unwrap(),
        }
    }

    fn projected_position(&self, from: Position, direction: Direction) -> Option<Position> {
        match direction {
            Direction::Left if from.1 > 0 => Some(Position(from.0, from.1 - 1)),
            Direction::Right if from.1 < self.num_cols() - 1 => Some(Position(from.0, from.1 + 1)),
            Direction::Up if from.0 > 0 => Some(Position(from.0 - 1, from.1)),
            Direction::Down if from.0 < self.num_rows() - 1 => Some(Position(from.0 + 1, from.1)),
            _ => None,
        }
    }

    fn is_direction_valid(&self, start: Position, direction: Direction) -> bool {
        let start_height = self.height_at(start);
        if let Some(to) = self.projected_position(start, direction) {
            let dest_height = self.height_at(to);
            start_height >= dest_height - 1
        } else {
            false
        }
    }

    fn height_at(&self, Position(row, col): Position) -> u8 {
        self.rows[row][col]
    }

    fn num_cols(&self) -> usize {
        self.rows[0].len()
    }

    fn num_rows(&self) -> usize {
        self.rows.len()
    }

    fn valid_directions(&self, pos: Position) -> HashSet<Direction> {
        Direction::iter()
            .filter(|dir| self.is_direction_valid(pos, *dir))
            .collect()
    }

    fn valid_positions(&self, pos: Position, visited: &mut HashSet<Position>) -> HashSet<Position> {
        let result = self.valid_directions(pos).into_iter()
            .map(|dir| self.projected_position(pos, dir))
            // we only use valid directions so unwrap won't fail
            .map(|pos| pos.unwrap())
            .collect::<HashSet<_>>()
            .difference(&visited)
            .map(|e| *e)
            .collect();
        visited.extend(&result);
        result
    }

    fn find_points(&self, elevation: u8) -> Vec<Position> {
        let mut res = Vec::new();
        for (r, row) in self.rows.iter().enumerate() {
            for (c, height) in row.iter().enumerate() {
                if *height == elevation {
                    res.push(Position(r, c));
                }
            }
        }
        res
    }
}

fn find_path(map: &Map, start: Position) -> Option<usize> {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut current_set: HashSet<Position> = [start].into();

    let mut count = 0;

    loop {
        for pos in mem::replace(&mut current_set, HashSet::new()) {
            if pos == map.end_pos {
                return Some(count);
            }
            current_set.extend(map.valid_positions(pos, &mut visited));
        }

        if current_set.is_empty() {
            return None;
        }

        count += 1;
    }
}

fn solve(input: &Input, part: Part) -> usize {
    let map = Map::load(input);

    match part {
        Part::One => {
            find_path(&map, map.start_pos).unwrap()
        }

        Part::Two => {
            // find all points with elevation 'a'
            let mut lens = map.find_points(b'a').iter()
                .map(|point| find_path(&map, *point))
                .filter(|p| p.is_some())
                .map(|p| p.unwrap())
                .collect::<Vec<_>>();

            lens.sort();
            *lens.first().unwrap()
        }
    }
}

static INPUT: &Input = include_bytes!("input.txt");

fn main() {
    println!("Answer 1: {}", solve(INPUT, Part::One));
    println!("Answer 2: {}", solve(INPUT, Part::Two));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EX1: &Input = include_bytes!("example.txt");

    #[test]
    fn test_load_map() {
        let map = Map::load(EX1);
        assert_eq!(map.num_cols(), 8);
        assert_eq!(map.num_rows(), 5);
        assert_eq!(map.height_at(Position(0, 0)), b'a');
        assert_eq!(map.height_at(Position(4, 7)), b'i');
        assert_eq!(map.start_pos, Position(0, 0));
        assert_eq!(map.end_pos, Position(2, 5));
    }

    #[test]
    fn test_projected_position() {
        let map = Map::load(EX1);
        assert_eq!(
            map.projected_position(Position(0, 0), Direction::Right),
            Some(Position(0, 1))
        );
        assert_eq!(
            map.projected_position(Position(0, 0), Direction::Left),
            None
        );
        assert_eq!(
            map.projected_position(Position(0, 7), Direction::Right),
            None
        );
        assert_eq!(
            map.projected_position(Position(5, 7), Direction::Down),
            None
        );
        assert_eq!(
            map.projected_position(Position(5, 7), Direction::Up),
            Some(Position(4, 7))
        );
    }

    #[test]
    fn test_possible_directions() {
        let map = Map::load(EX1);

        assert_eq!(
            map.valid_directions(Position(0, 0)),
            [Direction::Down, Direction::Right].into()
        );

        assert_eq!(
            map.valid_directions(Position(2, 3)),
            [Direction::Left, Direction::Down, Direction::Up].into()
        );
    }

    #[test]
    fn example1() {
        let sol = solve(EX1, Part::One);
        assert_eq!(sol, 31);
    }
}
