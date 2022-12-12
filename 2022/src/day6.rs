use itertools::Itertools;

fn day6(input: &str) -> Option<usize> {
    for (idx, slice) in input
        .chars()
        .collect::<Vec<char>>()
        .as_slice()
        .windows(4)
        .enumerate()
    {
        if slice.iter().all_unique() {
            return Some(idx + 4);
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
        assert_eq!(day6(input), Some(1723));
    }
}
