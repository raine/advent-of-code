use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default, Hash, Eq, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Grid {
    knots: Vec<Point>,
    tail_visited_positions: HashSet<Point>,
}

impl Grid {
    fn new(knots_count: usize) -> Self {
        Grid {
            knots: vec![Point::default(); knots_count],
            tail_visited_positions: HashSet::new(),
        }
    }

    fn move_head(&mut self, motion: &Motion) {
        let head = self.knots.first_mut().unwrap();

        match motion.direction {
            Direction::Up => head.y += 1,
            Direction::Down => head.y -= 1,
            Direction::Left => head.x -= 1,
            Direction::Right => head.x += 1,
        };
    }

    fn move_knot_if_needed(&mut self, n: usize) {
        let head = self.knots.get(n - 1).unwrap().clone();
        let tail = self.knots.get_mut(n).unwrap();

        let dx = head.x.abs_diff(tail.x);
        let dy = head.y.abs_diff(tail.y);

        // Tail needs to move
        if dx > 1 || dy > 1 {
            if head.x > tail.x {
                tail.x += 1
            }

            if head.y > tail.y {
                tail.y += 1
            }

            if head.x < tail.x {
                tail.x -= 1
            }

            if head.y < tail.y {
                tail.y -= 1
            }
        }
    }

    fn apply_motion(&mut self, motion: Motion) {
        for _ in 0..motion.steps {
            self.move_head(&motion);
            println!("moved head {:?} to {:?}", motion.direction, self.knots[0]);

            for idx in 1..self.knots.len() {
                self.move_knot_if_needed(idx);
            }

            let tail = self.knots.last().unwrap();
            self.tail_visited_positions.insert(tail.clone());
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

fn day9<T>(reader: BufReader<T>, knots: usize) -> usize
where
    T: std::io::Read,
{
    let mut grid = Grid::new(knots);

    for line in reader.lines() {
        let m = parse_move(&line.unwrap());
        grid.apply_motion(m);
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
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let cursor = Cursor::new(input.trim_start_matches('\n'));
        let reader = BufReader::new(cursor);
        assert_eq!(day9(reader, 2), 13);
    }

    #[test]
    fn day9_test() {
        let input = File::open("./testdata/day9").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day9(reader, 2), 6256);
    }

    #[test]
    fn day9_part2_test_simple() {
        let input = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let cursor = Cursor::new(input.trim_start_matches('\n'));
        let reader = BufReader::new(cursor);
        assert_eq!(day9(reader, 9), 1);
    }

    #[test]
    fn day9_part2_less_simple() {
        let input = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let cursor = Cursor::new(input.trim_start_matches('\n'));
        let reader = BufReader::new(cursor);
        assert_eq!(day9(reader, 10), 36);
    }

    #[test]
    fn day9_part2_test() {
        let input = File::open("./testdata/day9").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day9(reader, 10), 2665);
    }

    #[test]
    fn parse_move_test() {
        let input = "U 5";
        assert_eq!(parse_move(input), Motion::new(Direction::Up, 5));
    }
}
