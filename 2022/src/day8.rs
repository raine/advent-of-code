use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn day8(input: &str) -> i32 {
    let size = input.lines().count();
    let input = &input.replace('\n', "");
    let grid = input
        .chars()
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let get_coord = |x: usize, y: usize| grid[x + (size * y)];
    let mut visible_trees = 0;

    for x in 0..size {
        for y in 0..size {
            let is_at_edge = x == 0 || y == 0 || x == size - 1 || y == size - 1;
            if is_at_edge {
                visible_trees += 1;
                continue;
            }

            let tree_height = get_coord(x, y);

            let visible_to_left = (0..x).map(|x| get_coord(x, y)).all(|h| h < tree_height);
            let visible_to_right = (x + 1..size)
                .map(|x| get_coord(x, y))
                .all(|h| h < tree_height);
            let visible_to_top = (0..y).map(|y| get_coord(x, y)).all(|h| h < tree_height);
            let visible_to_bottom = (y + 1..size)
                .map(|y| get_coord(x, y))
                .all(|h| h < tree_height);

            let visible =
                visible_to_left || visible_to_top || visible_to_right || visible_to_bottom;

            if visible {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn day8_part2(input: &str) -> u32 {
    let size = input.lines().count();
    let input = &input.replace('\n', "");
    let grid = input
        .chars()
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let get_coord = |x: usize, y: usize| grid[x + (size * y)];
    let mut max_scenic_score = 0;

    for x in 0..size {
        for y in 0..size {
            let tree_height = get_coord(x, y);
            let trees_to_left = (0..x).rev().map(|x| get_coord(x, y));
            let trees_to_right = (x + 1..size).map(|x| get_coord(x, y));
            let trees_to_top = (0..y).rev().map(|y| get_coord(x, y));
            let trees_to_bottom = (y + 1..size).map(|y| get_coord(x, y));

            let trees_visible_to_left = visible_trees(trees_to_left, tree_height);
            let trees_visible_to_right = visible_trees(trees_to_right, tree_height);
            let trees_visible_to_top = visible_trees(trees_to_top, tree_height);
            let trees_visible_to_bottom = visible_trees(trees_to_bottom, tree_height);

            let scenic_score = trees_visible_to_left
                * trees_visible_to_right
                * trees_visible_to_top
                * trees_visible_to_bottom;

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score
            }
        }
    }

    max_scenic_score
}

fn visible_trees<I>(mut dir_trees: I, pov_tree_height: u32) -> u32
where
    I: Iterator<Item = u32>,
{
    dir_trees
        .fold_while(0, |acc, h| {
            if h >= pov_tree_height {
                Done(acc + 1)
            } else {
                Continue(acc + 1)
            }
        })
        .into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_simple() {
        let input = "
30373
25512
65332
33549
35390";

        assert_eq!(day8(input.trim()), 21);
    }

    #[test]
    fn test_day8() {
        let input = include_str!("../testdata/day8");
        assert_eq!(day8(input.trim()), 1843);
    }

    #[test]
    fn test_day8_part2() {
        let input = include_str!("../testdata/day8");
        assert_eq!(day8_part2(input.trim()), 180000);
    }
}
