use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Direction {
        match s {
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "U" => Direction::Up,
            _ => panic!("Invalid direction"),
        }
    }
}

struct Motion {
    direction: Direction,
    distance: usize,
}

type Motions = Vec<Motion>;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Position(isize, isize);

impl Position {
    fn follow(&mut self, other: &Position) -> bool {
        let dx = other.0 - self.0;
        let dy = other.1 - self.1;
        if dx.abs() < 2 && dy.abs() < 2 {
            // no need to move
            return false;
        }

        // move one step in the direction of other
        self.0 = self.0 + dx.signum();
        self.1 = self.1 + dy.signum();

        return true;
    }

    fn displace(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.1 -= 1,
            Direction::Down => self.1 += 1,
            Direction::Left => self.0 -= 1,
            Direction::Right => self.0 += 1,
        }
    }
}

fn load_input(path: &str) -> Motions {
    let input = fs::read(path).unwrap();
    std::str::from_utf8(&input)
        .unwrap()
        .lines()
        .map(|line| line.split(" ").collect())
        .map(|parts: Vec<&str>| Motion {
            direction: parts[0].into(),
            distance: parts[1].parse().unwrap(),
        })
        .collect()
}

struct RopeSimulator {
    tail_positions: HashSet<Position>,
    knots: Vec<Position>,
}

impl RopeSimulator {
    fn new(knot_count: usize) -> RopeSimulator {
        RopeSimulator {
            tail_positions: HashSet::from([Position(0, 0)]),
            knots: vec![Position(0, 0); knot_count],
        }
    }

    fn number_of_tail_positions(&self) -> usize {
        self.tail_positions.len()
    }

    fn step(&mut self, direction: Direction) {
        // first move the head
        self.knots[0].displace(direction);

        let mut last_knot = self.knots[0];
        for knot in &mut self.knots[1..] {
            if !knot.follow(&last_knot) {
                break;
            }
            last_knot = *knot;
        }

        // record the position of the tail
        self.tail_positions.insert(*self.knots.last().unwrap());
    }

    fn simulate(&mut self, motion: Motion) -> &mut Self {
        for _ in 0..motion.distance {
            self.step(motion.direction);
        }
        self
    }
}

fn solve1(path: &str) -> usize {
    let input = load_input(path);
    input
        .into_iter()
        .fold(&mut RopeSimulator::new(2), |simulator, motion| {
            simulator.simulate(motion)
        })
        .number_of_tail_positions()
}

fn solve2(path: &str) -> usize {
    let input = load_input(path);
    input
        .into_iter()
        .fold(&mut RopeSimulator::new(10), |simulator, motion| {
            simulator.simulate(motion)
        })
        .number_of_tail_positions()
}

fn main() {
    let sol1 = solve1("input.txt");
    println!("Answer 1: {}", sol1);

    let sol2 = solve2("input.txt");
    println!("Answer 2: {}", sol2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_hash() {
        let mut positions: HashSet<Position> = HashSet::from([Position(0, 0)]);
        positions.insert(Position(0, 0));
        positions.insert(Position(0, 1));
        positions.insert(Position(1, 0));
        positions.insert(Position(1, 1));
        positions.insert(Position(0, 0));
        positions.insert(Position(1, 0));
        assert_eq!(positions.len(), 4);
    }

    #[test]
    fn test_load() {
        let input = load_input("example1.txt");
        assert_eq!(input.len(), 8);
        assert_eq!(input[0].direction, Direction::Right);
        assert_eq!(input[0].distance, 4);
        assert_eq!(input[3].direction, Direction::Down);
        assert_eq!(input[3].distance, 1);
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1("example1.txt"), 13);
    }

    #[test]
    fn test_solve2a() {
        assert_eq!(solve2("example1.txt"), 1);
    }

    #[test]
    fn test_solve2b() {
        assert_eq!(solve2("example2.txt"), 36);
    }

    #[test]
    fn move_to_check_move() {
        assert!(!Position(0, 0).follow(&Position(1, 0)));
        assert!(!Position(0, 0).follow(&Position(0, 1)));
        assert!(!Position(0, 0).follow(&Position(-1, 0)));
        assert!(!Position(0, 0).follow(&Position(0, -1)));

        assert!(Position(0, 0).follow(&Position(2, 0)));
        assert!(Position(0, 0).follow(&Position(0, 2)));
        assert!(Position(0, 0).follow(&Position(-2, 1)));
        assert!(Position(0, 0).follow(&Position(1, -2)));
    }

    #[test]
    fn move_to_same_row() {
        let mut p1 = Position(0, 0);
        let p2 = Position(2, 0);
        p1.follow(&p2);
        assert_eq!(p1, Position(1, 0));

        let mut p1 = Position(-3, 3);
        let p2 = Position(-5, 3);
        p1.follow(&p2);
        assert_eq!(p1, Position(-4, 3));
    }

    #[test]
    fn move_to_same_column() {
        let mut p1 = Position(0, 0);
        let p2 = Position(0, 2);
        p1.follow(&p2);
        assert_eq!(p1, Position(0, 1));

        let mut p1 = Position(1, -3);
        let p2 = Position(1, -5);
        p1.follow(&p2);
        assert_eq!(p1, Position(1, -4));
    }

    #[test]
    fn move_to_diagonally() {
        let mut p1 = Position(0, 0);
        let p2 = Position(2, 1);
        p1.follow(&p2);
        assert_eq!(p1, Position(1, 1));

        let mut p1 = Position(0, 0);
        let p2 = Position(2, 2);
        p1.follow(&p2);
        assert_eq!(p1, Position(1, 1));
    }

    #[test]
    fn test_move_in() {
        let mut p = Position(0, 0);
        p.displace(Direction::Left);
        assert_eq!(p, Position(-1, 0));
        p.displace(Direction::Up);
        assert_eq!(p, Position(-1, -1));
        p.displace(Direction::Right);
        assert_eq!(p, Position(0, -1));
        p.displace(Direction::Down);
        assert_eq!(p, Position(0, 0));
    }
}
