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
        (self.x.abs_diff(p.x) + self.y.abs_diff(p.y)) as i64
    }
}

#[derive(Debug, PartialEq)]
struct Sensor {
    point: Point,
    closest_beacon: Point,
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

    let mut ranges = vec![];
    for s in sensors {
        let radius = s.point.taxicab_distance(&s.closest_beacon);
        let dist_from_y = s.point.y.abs_diff(y) as i64;
        if dist_from_y > radius {
            continue;
        }
        let n = i64::abs(dist_from_y - radius);
        let xs = s.point.x - n..=s.point.x + n;
        ranges.push(xs);
    }

    ranges.sort_by_key(|r| *r.start());
    ranges
        .into_iter()
        .coalesce(|prev, curr| {
            if curr.start() - 1 <= *prev.end() {
                if curr.end() > prev.end() {
                    Ok(*prev.start()..=*curr.end())
                } else {
                    Ok(prev)
                }
            } else {
                Err((prev, curr))
            }
        })
        .map(|r| {
            let range_size = (r.end() - r.start() + 1) as usize;
            let beacons_in_range = beacons
                .iter()
                .filter(|b| r.contains(&b.x) && b.y == y)
                .count();
            range_size - beacons_in_range
        })
        .sum()
}

// Couldn't have solved this without the range idea from
// https://fasterthanli.me/series/advent-of-code-2022/part-15
fn day15_part2(i: &str, max_xy: i64) -> i64 {
    let sensors = parse_all_sensors(i).finish().unwrap().1;
    let beacons = sensors
        .iter()
        .map(|s| s.closest_beacon)
        .collect::<HashSet<_>>();

    for y in 0..=max_xy {
        let mut ranges = vec![];
        for s in &sensors {
            let radius = s.point.taxicab_distance(&s.closest_beacon);
            let dist_from_y = s.point.y.abs_diff(y) as i64;
            if dist_from_y > radius {
                continue;
            }
            let n = i64::abs(dist_from_y - radius);
            let xs = (s.point.x - n).max(0)..=(s.point.x + n).min(max_xy);
            ranges.push(xs);
        }
        ranges.sort_by_key(|r| *r.start());
        for r in ranges.into_iter().coalesce(|prev, curr| {
            if curr.start() - 1 <= *prev.end() {
                if curr.end() > prev.end() {
                    Ok(*prev.start()..=*curr.end())
                } else {
                    Ok(prev)
                }
            } else {
                Err((prev, curr))
            }
        }) {
            let range_size = r.end() - r.start() + 1;
            if range_size != max_xy + 1 {
                return range_size * 4000000 + y;
            }
        }
    }

    panic!("no solution")
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
    fn day15_test() {
        let input = include_str!("../testdata/day15");
        assert_eq!(day15(input, 2000000), 5870800);
    }

    #[test]
    fn day15_part2_simple_test() {
        let input = include_str!("../testdata/day15_simple");
        assert_eq!(day15_part2(input, 20), 56000011);
    }

    #[test]
    fn day15_part2_test() {
        let input = include_str!("../testdata/day15");
        assert_eq!(day15_part2(input, 4000000), 10908230916597);
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
}
