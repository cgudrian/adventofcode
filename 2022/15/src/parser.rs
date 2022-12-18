use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::{complete::i32 as parse_i32, complete::newline},
    combinator::all_consuming,
    multi::many0,
    sequence::terminated,
    IResult,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Point(pub i32, pub i32);

impl Point {
    fn distance_to(&self, rhs: &Point) -> i32 {
        (self.0 - rhs.0).abs() + (self.1 - rhs.1).abs()
    }

    pub fn tuning_frequency(&self) -> u64 {
        self.0 as u64 * 4000000 + self.1 as u64
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Sensor {
    pos: Point,
    closest_beacon: Point,
    distance: i32,
}

impl Sensor {
    fn new(pos: Point, beacon: Point) -> Sensor {
        Sensor {
            pos,
            closest_beacon: beacon,
            distance: pos.distance_to(&beacon),
        }
    }

    pub fn range(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let dx = self.distance - (y - self.pos.1).abs();
        if dx < 0 {
            None
        } else {
            Some(self.pos.0 - dx..=self.pos.0 + dx)
        }
    }

    pub fn positions_with_no_beacons(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let dx = self.distance - (y - self.pos.1).abs();
        if dx < 0 {
            None
        } else if y == self.closest_beacon.1 {
            let beacon_x = self.closest_beacon.0;
            let min_x = self.pos.0 - dx;
            let max_x = self.pos.0 + dx;
            if beacon_x < max_x {
                Some(beacon_x + 1..=max_x)
            } else if beacon_x > min_x {
                Some(min_x..=beacon_x - 1)
            } else {
                None
            }
        } else {
            Some(self.pos.0 - dx..=self.pos.0 + dx)
        }
    }
}

pub fn parse(input: &str) -> IResult<&str, Vec<Sensor>> {
    all_consuming(many0(terminated(sensor_reading, newline)))(input)
}

fn sensor_reading(input: &str) -> IResult<&str, Sensor> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, pos_x) = parse_i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, pos_y) = parse_i32(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon_x) = parse_i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, beacon_y) = parse_i32(input)?;
    Ok((
        input,
        Sensor::new(Point(pos_x, pos_y), Point(beacon_x, beacon_y)),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sensor_reading() {
        let (_, reading) =
            sensor_reading("Sensor at x=13, y=-2: closest beacon is at x=15, y=3").unwrap();
        assert_eq!(reading, Sensor::new(Point(13, -2), Point(15, 3)));
    }

    #[test]
    fn parse_multiple_sensor_reading() {
        let (_, reading) =
            parse("Sensor at x=13, y=-2: closest beacon is at x=15, y=3\nSensor at x=14, y=17: closest beacon is at x=10, y=16\n").unwrap();
        assert_eq!(
            reading,
            [
                Sensor::new(Point(13, -2), Point(15, 3)),
                Sensor::new(Point(14, 17), Point(10, 16)),
            ]
        )
    }

    #[test]
    fn point_distance_x_and_y_different() {
        assert_eq!(Point(8, 7).distance_to(&Point(2, 10)), 9);
    }

    #[test]
    fn point_distance_equal_points() {
        assert_eq!(Point(8, 7).distance_to(&Point(8, 7)), 0);
    }

    #[test]
    fn point_distance_x_different() {
        assert_eq!(Point(8, 7).distance_to(&Point(9, 7)), 1);
        assert_eq!(Point(8, 7).distance_to(&Point(7, 7)), 1);
    }

    #[test]
    fn point_distance_y_different() {
        assert_eq!(Point(8, 7).distance_to(&Point(8, 8)), 1);
        assert_eq!(Point(8, 7).distance_to(&Point(8, 6)), 1);
    }

    #[test]
    fn sensor_covered_points_beacon_at_sensor_position() {
        let sensor = Sensor::new(Point(0, 0), Point(0, 0));
        let points = sensor.covered_points();
        assert!(points.is_empty());
    }

    #[test]
    fn sensor_covered_points_beacon_at_distance_one() {
        let sensor = Sensor::new(Point(0, 0), Point(1, 0));
        let points = sensor.covered_points();
        assert_eq!(
            points,
            [Point(0, -1), Point(-1, 0), Point(0, 0), Point(0, 1)].into()
        );
    }

    #[test]
    fn sensor_covered_points_beacon_at_distance_two() {
        let sensor = Sensor::new(Point(0, 0), Point(1, 1));
        let points = sensor.covered_points();
        assert_eq!(
            points,
            [
                Point(0, -2),
                Point(-1, -1),
                Point(0, -1),
                Point(1, -1),
                Point(-2, 0),
                Point(-1, 0),
                Point(0, 0),
                Point(1, 0),
                Point(2, 0),
                Point(-1, 1),
                Point(0, 1),
                Point(0, 2),
            ]
            .into()
        );
    }

    #[test]
    fn sensor_covered_range_for_x_sensor_below() {
        let sensor = Sensor::new(Point(0, 0), Point(0, 100));
        assert_eq!(sensor.positions_with_no_beacons(1000), None);
        assert_eq!(sensor.positions_with_no_beacons(0), Some(-100..=100));
        assert_eq!(sensor.positions_with_no_beacons(100), None);
        assert_eq!(sensor.positions_with_no_beacons(-100), Some(0..=0));
    }

    #[test]
    fn sensor_covered_range_for_x_sensor_right() {
        let sensor = Sensor::new(Point(0, 0), Point(100, 0));
        assert_eq!(sensor.positions_with_no_beacons(1000), None);
        assert_eq!(sensor.positions_with_no_beacons(0), Some(-100..=99));
        assert_eq!(sensor.positions_with_no_beacons(100), Some(0..=0));
        assert_eq!(sensor.positions_with_no_beacons(-100), Some(0..=0));
    }

    #[test]
    fn sensor_covered_range_for_x_sensor_right_below() {
        let sensor = Sensor::new(Point(0, 0), Point(100, 100));
        assert_eq!(sensor.positions_with_no_beacons(1000), None);
        assert_eq!(sensor.positions_with_no_beacons(0), Some(-200..=200));
        assert_eq!(sensor.positions_with_no_beacons(100), Some(-100..=99));
        assert_eq!(sensor.positions_with_no_beacons(200), Some(0..=0));
        assert_eq!(sensor.positions_with_no_beacons(-200), Some(0..=0));
    }
}
