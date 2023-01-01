use std::{collections::HashMap, iter::once, ops::Add};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{all_consuming, map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult,
};

#[derive(Debug, PartialEq)]
enum Cell {
    Rock,
    Sand,
}

const SAND_SOURCE_POINT: Coord = Coord { x: 500, y: 0 };
const SAND_SOURCE: char = '+';
const SAND: char = 'o';
const ROCK: char = '#';
const AIR: char = '.';

#[derive(Debug)]
pub struct Grid {
    data: HashMap<Coord, Cell>,
    dropping_sand: Option<Coord>,
    bounds: (Coord, Coord),
}

impl Grid {
    pub fn from_rock_paths(input: &str) -> Grid {
        let mut data = HashMap::new();
        let rock_paths = all_consuming(parse_all_paths)(input.trim_end())
            .finish()
            .unwrap()
            .1;
        let rocks = rock_paths.into_iter().flat_map(|p| p.expand_to_coords());
        for coord in rocks {
            data.insert(coord, Cell::Rock);
        }
        let bounds = calc_bounds(&data);

        Grid {
            data,
            dropping_sand: None,
            bounds,
        }
    }

    fn coord_out_of_bounds(&self, coord: &Coord) -> bool {
        let (min, max) = self.bounds;
        coord.x < min.x || coord.x > max.x || coord.y < min.y || coord.y > max.y
    }

    fn sand_count(&self) -> usize {
        self.data.values().filter(|v| **v == Cell::Sand).count()
    }

    pub fn draw(&self) {
        let (min, max) = self.bounds;
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let coord: Coord = (x, y).into();
                let d = self.data.get(&coord);
                let ch = if d == Some(&Cell::Rock) {
                    ROCK
                } else if d == Some(&Cell::Sand) || self.dropping_sand.as_ref() == Some(&coord) {
                    SAND
                } else if coord == SAND_SOURCE_POINT {
                    SAND_SOURCE
                } else {
                    AIR
                };
                print!("{}", ch);
            }
            println!()
        }
    }

    fn next_dropping_sand_position(&self, sand: &Coord) -> Coord {
        let bottom = self.data.get(&sand.offset_xy(0, 1));
        let bottom_left = self.data.get(&sand.offset_xy(-1, 1));
        let bottom_right = self.data.get(&sand.offset_xy(1, 1));
        match (bottom_left, bottom, bottom_right) {
            (_, None, _) => sand.offset_xy(0, 1),
            (None, Some(_), _) => sand.offset_xy(-1, 1),
            (Some(_), Some(_), None) => sand.offset_xy(1, 1),
            (Some(_), Some(_), Some(_)) => *sand,
        }
    }

    fn reset_dropping_sand(&mut self) {
        self.dropping_sand = Some(SAND_SOURCE_POINT.offset_xy(0, 1));
    }

    pub fn step(&mut self) -> bool {
        match self.dropping_sand {
            Some(s) => {
                let next_sand_pos = self.next_dropping_sand_position(&s);
                if self.coord_out_of_bounds(&next_sand_pos) {
                    return false;
                }

                if next_sand_pos == s {
                    self.data.insert(s, Cell::Sand);
                    self.reset_dropping_sand();
                } else {
                    self.dropping_sand = Some(next_sand_pos);
                }
            }
            None => {
                self.reset_dropping_sand();
            }
        }

        true
    }
}

fn calc_bounds(data: &HashMap<Coord, Cell>) -> (Coord, Coord) {
    let bounds_from = once(&SAND_SOURCE_POINT).chain(data.keys());
    let (min_x, max_x, min_y, max_y) = bounds_from.fold(
        (
            std::usize::MAX,
            std::usize::MIN,
            std::usize::MAX,
            std::usize::MIN,
        ),
        |acc, rock| {
            (
                acc.0.min(rock.x),
                acc.1.max(rock.x),
                acc.2.min(rock.y),
                acc.3.max(rock.y),
            )
        },
    );

    ((min_x, min_y).into(), (max_x, max_y).into())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn offset_xy(&self, x: i64, y: i64) -> Self {
        Coord {
            x: if x < 0 {
                self.x - (x.unsigned_abs() as usize)
            } else {
                self.x + (x as usize)
            },
            y: if y < 0 {
                self.y - (y.unsigned_abs() as usize)
            } else {
                self.y + (y as usize)
            },
        }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Coord { x, y }
    }
}

#[derive(Debug, PartialEq)]
struct Path(Vec<Coord>);

impl Path {
    fn expand_to_coords(&self) -> Vec<Coord> {
        self.0
            .windows(2)
            .flat_map(|w| {
                let (a, b) = (&w[0], &w[1]);
                let x_range = if a.x < b.x { a.x..=b.x } else { b.x..=a.x };
                let y_range = if a.y < b.y { a.y..=b.y } else { b.y..=a.y };
                x_range.flat_map(move |x| y_range.clone().map(move |y| Coord { x, y }))
            })
            .unique()
            .collect::<Vec<_>>()
    }
}

fn parse_coord(i: &str) -> IResult<&str, Coord> {
    map(
        separated_pair(
            map_res(digit1, |s: &str| s.parse()),
            char(','),
            map_res(digit1, |s: &str| s.parse()),
        ),
        |(x, y)| (x, y).into(),
    )(i)
}

fn parse_path(i: &str) -> IResult<&str, Path> {
    map(separated_list1(tag(" -> "), parse_coord), Path)(i)
}

fn parse_all_paths(i: &str) -> IResult<&str, Vec<Path>> {
    separated_list1(tag("\n"), parse_path)(i)
}

fn day14(input: &str) -> usize {
    let mut grid = Grid::from_rock_paths(input);
    loop {
        if !grid.step() {
            break;
        }
    }
    grid.sand_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_simple_test() {
        let input = "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
            .trim_start();

        assert_eq!(day14(input), 24);
    }

    #[test]
    fn day14_test() {
        let input = include_str!("../testdata/day14");
        assert_eq!(day14(input), 610);
    }

    #[test]
    fn parse_coord_test() {
        let coord = parse_coord("123,123").unwrap().1;
        assert_eq!(coord, Coord { x: 123, y: 123 });
    }

    #[test]
    fn parse_path_test() {
        let path = parse_path("498,4 -> 498,6 -> 496,6").unwrap().1;
        assert_eq!(
            path,
            Path(vec![
                Coord { x: 498, y: 4 },
                Coord { x: 498, y: 6 },
                Coord { x: 496, y: 6 },
            ])
        );
    }

    #[test]
    fn parse_all_paths_test() {
        let paths = parse_all_paths(
            "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"
            .trim_start_matches('\n'),
        )
        .unwrap()
        .1;
        assert_eq!(
            paths,
            vec![
                Path(vec![
                    Coord { x: 498, y: 4 },
                    Coord { x: 498, y: 6 },
                    Coord { x: 496, y: 6 },
                ]),
                Path(vec![
                    Coord { x: 503, y: 4 },
                    Coord { x: 502, y: 4 },
                    Coord { x: 502, y: 9 },
                    Coord { x: 494, y: 9 },
                ]),
            ]
        );
    }

    #[test]
    fn path_expand_to_coords_test() {
        let path = parse_path("498,4 -> 498,6 -> 496,6").unwrap().1;
        assert_eq!(
            path.expand_to_coords(),
            vec![
                Coord { x: 498, y: 4 },
                Coord { x: 498, y: 5 },
                Coord { x: 498, y: 6 },
                Coord { x: 496, y: 6 },
                Coord { x: 497, y: 6 }
            ]
        );
    }
}
