use std::io::{BufRead, BufReader};

#[derive(Debug, Default, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Grid {
    head_position: Point,
    tail_position: Point,
    tail_visited_positions: Vec<Point>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            head_position: Point::default(),
            tail_position: Point::default(),
            tail_visited_positions: vec![Point::default()],
        }
    }

    fn move_head(&mut self, motion: Motion) {
        for _ in 0..motion.steps {
            match motion.direction {
                Direction::Up => self.head_position.y += 1,
                Direction::Down => self.head_position.y -= 1,
                Direction::Left => self.head_position.x -= 1,
                Direction::Right => self.head_position.x += 1,
            };

            let dx = self.head_position.x.abs_diff(self.tail_position.x);
            let dy = self.head_position.y.abs_diff(self.tail_position.y);

            // Tail needs to move
            if dx > 1 || dy > 1 {
                if self.head_position.x > self.tail_position.x {
                    self.tail_position.x += 1
                }

                if self.head_position.y > self.tail_position.y {
                    self.tail_position.y += 1
                }

                if self.head_position.x < self.tail_position.x {
                    self.tail_position.x -= 1
                }

                if self.head_position.y < self.tail_position.y {
                    self.tail_position.y -= 1
                }

                if !self.tail_visited_positions.contains(&self.tail_position) {
                    self.tail_visited_positions.push(self.tail_position.clone());
                }
            }

            println!(
                "moved head {:?} to {:?} | tail: {:?}",
                motion.direction, self.head_position, self.tail_position
            );
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Motion {
    direction: Direction,
    steps: i32,
}

impl Motion {
    fn new(direction: Direction, steps: i32) -> Self {
        Self { direction, steps }
    }
}

fn parse_move(input: &str) -> Motion {
    let mut split = input.split(' ');
    let (dir, steps) = (
        split.next().unwrap().chars().next().unwrap(),
        split.next().unwrap().parse::<i32>().unwrap(),
    );

    match dir {
        'U' => Motion::new(Direction::Up, steps),
        'D' => Motion::new(Direction::Down, steps),
        'L' => Motion::new(Direction::Left, steps),
        'R' => Motion::new(Direction::Right, steps),
        _ => panic!("unexpected direction"),
    }
}

fn day9<T>(reader: BufReader<T>) -> usize
where
    T: std::io::Read,
{
    let mut grid = Grid::new();

    for line in reader.lines() {
        let m = parse_move(&line.unwrap());
        grid.move_head(m);
    }

    grid.tail_visited_positions.len()
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufReader, Cursor},
    };

    use super::*;

    #[test]
    fn day9_test_simple() {
        let input = "
R 4
R 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let cursor = Cursor::new(input.trim_start_matches('\n'));
        let reader = BufReader::new(cursor);
        assert_eq!(day9(reader), 13);
    }

    #[test]
    fn day9_test() {
        let input = File::open("./testdata/day9").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day9(reader), 6256);
    }

    #[test]
    fn parse_move_test() {
        let input = "U 5";
        assert_eq!(parse_move(input), Motion::new(Direction::Up, 5));
    }
}
