#![allow(dead_code, unused_imports, unused_variables)]
use std::collections::HashSet;

use itertools::Itertools;
use nom::character::complete as cc;
use nom::combinator::{all_consuming, map, opt};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::Finish;
use nom::{
    bytes::complete::tag,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Clone, Hash, Eq, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn taxicab_distance(&self, p: &Point) -> i64 {
        i64::abs(self.x - p.x) + i64::abs(self.y - p.y)
    }
}

#[derive(Debug, PartialEq)]
struct Sensor {
    point: Point,
    closest_beacon: Point,
}

impl Sensor {
    fn covered_area_at_y(&self, y: i64) -> impl Iterator<Item = Point> + '_ {
        let d = self.point.taxicab_distance(&self.closest_beacon);
        let ys = (self.point.y - d..=self.point.y + d).filter(move |y2| y == *y2);
        ys.flat_map(move |y| {
            let n = i64::abs(i64::abs(y - self.point.y) - d);
            let xs = self.point.x - n..=self.point.x + n;
            xs.map(move |x| Point { x, y })
        })
    }
}

fn parse_point(i: &str) -> IResult<&str, Point> {
    map(
        separated_pair(
            preceded(tag("x="), cc::i64),
            tag(", "),
            preceded(tag("y="), cc::i64),
        ),
        |(x, y)| Point { x, y },
    )(i)
}

fn parse_sensor(i: &str) -> IResult<&str, Sensor> {
    let (i, sensor_point) = preceded(tag("Sensor at "), parse_point)(i)?;
    let (i, closest_beacon_point) = preceded(tag(": closest beacon is at "), parse_point)(i)?;

    Ok((
        i,
        Sensor {
            point: sensor_point,
            closest_beacon: closest_beacon_point,
        },
    ))
}

fn parse_all_sensors(i: &str) -> IResult<&str, Vec<Sensor>> {
    let (i, (sensors, _)) = all_consuming(tuple((
        separated_list1(tag("\n"), parse_sensor),
        opt(tag("\n")),
    )))(i)?;
    Ok((i, sensors))
}

fn day15(i: &str, y: i64) -> usize {
    let sensors = parse_all_sensors(i).finish().unwrap().1;
    let beacons = sensors
        .iter()
        .map(|s| s.closest_beacon)
        .collect::<HashSet<_>>();

    sensors
        .iter()
        .flat_map(|s| s.covered_area_at_y(y))
        .filter(|p| !beacons.contains(p))
        .unique()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_simple_test() {
        let input = include_str!("../testdata/day15_simple");
        assert_eq!(day15(input, 10), 26);
    }

    #[test]
    #[ignore]
    fn day15_test() {
        let input = include_str!("../testdata/day15");
        assert_eq!(day15(input, 2000000), 1);
    }

    #[test]
    fn parse_point_test() {
        assert_eq!(parse_point("x=2, y=18").unwrap().1, Point { x: 2, y: 18 });
    }

    #[test]
    fn parse_sensor_test() {
        assert_eq!(
            parse_sensor("Sensor at x=2, y=18: closest beacon is at x=-2, y=15")
                .unwrap()
                .1,
            Sensor {
                point: Point { x: 2, y: 18 },
                closest_beacon: Point { x: -2, y: 15 }
            }
        );
    }

    #[test]
    fn sensor_covered_area_test() {
        let sensor = Sensor {
            point: Point { x: 0, y: 0 },
            closest_beacon: Point { x: 1, y: 2 },
        };

        assert_eq!(
            sensor.covered_area_at_y(2).collect::<Vec<_>>(),
            vec![
                Point { x: -1, y: 2 },
                Point { x: 0, y: 2 },
                Point { x: 1, y: 2 }
            ]
        );
    }
}
