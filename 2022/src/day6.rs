use itertools::Itertools;

fn day6(input: &str, distinct_count: usize) -> Option<usize> {
    for (idx, slice) in input
        .chars()
        .collect::<Vec<char>>()
        .as_slice()
        .windows(distinct_count)
        .enumerate()
    {
        if slice.iter().all_unique() {
            return Some(idx + distinct_count);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_test() {
        let input = include_str!("../testdata/day6");
        assert_eq!(day6(input, 4), Some(1723));
    }

    #[test]
    fn day6_part2_test() {
        let input = include_str!("../testdata/day6");
        assert_eq!(day6(input, 14), Some(3708));
    }
}
