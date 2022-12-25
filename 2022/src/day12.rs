use std::char;
use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug)]
struct Heightmap {
    cells: Vec<Cell>,
    height: usize,
    width: usize,
}

impl Heightmap {
    fn get_cell_idx(&self, x: i64, y: i64) -> Option<usize> {
        match (x, y) {
            (x, y) if x >= 0 && y >= 0 => Some((x + (self.width as i64 * y)) as usize),
            (_, _) => None,
        }
    }

    fn get_cell(&self, x: i64, y: i64) -> Option<(&Cell, usize)> {
        self.get_cell_idx(x, y)
            .and_then(|idx| self.cells.get(idx).map(|c| (c, idx)))
    }

    fn get_adjacent_cell_idxs(&self, x: i64, y: i64) -> Vec<usize> {
        let (cell, _idx) = self.get_cell(x, y).unwrap();
        let left = self.get_cell(x - 1, y);
        let right = self.get_cell(x + 1, y);
        let top = self.get_cell(x, y - 1);
        let bottom = self.get_cell(x, y + 1);
        let sides = [left, right, top, bottom];

        sides
            .iter()
            .filter_map(|x| match *x {
                Some((c, idx)) if c.elevation() <= cell.elevation() + 1 => Some(idx),
                Some(_) => None,
                None => None,
            })
            .collect::<Vec<usize>>()
    }

    fn get_start_idx(&self) -> usize {
        let (idx, _cell) = self
            .cells
            .iter()
            .enumerate()
            .find(|(_idx, cell)| matches!(cell, Cell::Start))
            .unwrap();

        idx
    }

    fn get_end_idx(&self) -> usize {
        let (idx, _cell) = self
            .cells
            .iter()
            .enumerate()
            .find(|(_idx, cell)| matches!(cell, Cell::End))
            .unwrap();

        idx
    }

    fn to_adjacency_list(&self) -> HashMap<usize, Vec<usize>> {
        let mut list = HashMap::new();

        for y in 0..self.height as i64 {
            for x in 0..self.width as i64 {
                let (_cell, idx) = self.get_cell(x, y).unwrap();
                list.insert(idx, self.get_adjacent_cell_idxs(x, y));
            }
        }

        list
    }

    fn parse(input: &str) -> Heightmap {
        let lines = input.lines();
        let height = lines.clone().count();
        let width = lines.clone().next().unwrap().len();
        let cells = lines
            .flat_map(|l| l.chars().map(parse_cell).collect::<Vec<_>>())
            .collect();

        Heightmap {
            cells,
            height,
            width,
        }
    }
}

#[derive(Debug)]
enum Cell {
    Start,
    End,
    Square(u8),
}

const MIN_ELEVATION: u8 = 0;
const MAX_ELEVATION: u8 = b'z' - 97;

impl Cell {
    fn elevation(&self) -> u8 {
        match self {
            Cell::Start => MIN_ELEVATION,
            Cell::End => MAX_ELEVATION,
            Cell::Square(e) => *e,
        }
    }
}

fn parse_cell(c: char) -> Cell {
    match c {
        'S' => Cell::Start,
        'E' => Cell::End,
        'a'..='z' => Cell::Square(c as u8 - b'a'),
        _ => panic!("invalid character: {c}"),
    }
}

fn bfs(graph: &HashMap<usize, Vec<usize>>, start: usize, end: usize) -> Option<Vec<usize>> {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut prev_map: HashMap<usize, usize> = HashMap::new();
    'outer: while !queue.is_empty() {
        let current_node = queue.pop_front().unwrap();
        for v in graph.get(&current_node).unwrap().iter() {
            if *v == end {
                prev_map.insert(*v, current_node);
                break 'outer;
            }

            if !visited.contains(v) {
                prev_map.insert(*v, current_node);
                queue.push_back(*v);
                visited.insert(*v);
            }
        }
    }

    let mut path = Vec::new();
    let mut prev = Some(end);
    while let Some(p) = prev {
        path.push(p);
        prev = prev_map.get(&p).cloned();
    }
    path.reverse();

    match path.first() {
        Some(node) if *node == start => Some(path),
        _ => None,
    }
}

fn day12(input: &str) -> usize {
    let map = Heightmap::parse(input.trim_start_matches('\n'));
    let adjacency_list = map.to_adjacency_list();
    let start = map.get_start_idx();
    let end = map.get_end_idx();
    let path = bfs(&adjacency_list, start, end);
    path.unwrap().len() - 1
}

fn day12_part2(input: &str) -> usize {
    let map = Heightmap::parse(input.trim_start_matches('\n'));
    let adjacency_list = map.to_adjacency_list();
    let end = map.get_end_idx();
    let potential_start_cells = map
        .cells
        .iter()
        .enumerate()
        .filter(|(_idx, c)| c.elevation() == MIN_ELEVATION);

    potential_start_cells
        .filter_map(|(idx, _c)| bfs(&adjacency_list, idx, end))
        .map(|path| path.len())
        .sorted()
        .next()
        .unwrap()
        - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_simple_test() {
        let input = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

        assert_eq!(day12(input), 31);
    }

    #[test]
    fn day12_test() {
        let input = include_str!("../testdata/day12");
        assert_eq!(day12(input), 361);
    }

    #[test]
    fn day12_part2_simple_test() {
        let input = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

        assert_eq!(day12_part2(input), 29);
    }

    #[test]
    fn day12_part2_test() {
        let input = include_str!("../testdata/day12");
        assert_eq!(day12_part2(input), 354);
    }
}
