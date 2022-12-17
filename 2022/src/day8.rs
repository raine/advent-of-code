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
}
