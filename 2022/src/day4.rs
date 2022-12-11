use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

type Assignment = RangeInclusive<i32>;

fn parse_range(s: &str) -> Assignment {
    let mut split = s.split('-').map(|s| s.parse::<i32>().unwrap());
    let (start, end) = (split.next().unwrap(), split.next().unwrap());
    start..=end
}

fn parse_assignments(line: &str) -> (Assignment, Assignment) {
    let mut split = line.split(',').map(parse_range);
    (split.next().unwrap(), split.next().unwrap())
}

/// Assignment pair is bad if either of the assignments sections are fully contained in the other
fn is_bad_assignment_pair(assignments: (Assignment, Assignment)) -> bool {
    let (a1, a2) = assignments;
    range_contains(&a1, &a2) || range_contains(&a2, &a1)
}

/// Assignment pair is bad if assignments overlap
fn is_bad_assignment_pair_part2(assignments: (Assignment, Assignment)) -> bool {
    let (a1, a2) = assignments;
    ranges_overlap(&a1, &a2)
}

/// Returns true if r1 contains r2
fn range_contains<T>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool
where
    T: PartialOrd,
{
    r2.start() >= r1.start() && r2.end() <= r1.end()
}

/// Returns true if r1 contains r2
fn ranges_overlap<T>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool
where
    T: PartialOrd,
{
    r1.start() <= r2.end() && r2.start() <= r1.end()
}

fn day4(reader: BufReader<File>) -> u32 {
    reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| is_bad_assignment_pair(parse_assignments(l)))
        .count() as u32
}

fn day4_part2(reader: BufReader<File>) -> u32 {
    reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| is_bad_assignment_pair_part2(parse_assignments(l)))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_test() {
        let input = File::open("./testdata/day4").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day4(reader), 448);
    }

    #[test]
    fn day4_part2_test() {
        let input = File::open("./testdata/day4").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day4_part2(reader), 794);
    }

    #[test]
    fn range_contains_test() {
        assert!(range_contains(&(1..=5), &(2..=4)))
    }

    #[test]
    fn ranges_overlap_test() {
        assert!(ranges_overlap(&(1..=5), &(4..=7)));
        assert!(!ranges_overlap(&(1..=5), &(6..=7)));
    }

    #[test]
    fn parse_elf_sections_test() {
        assert_eq!(parse_assignments("8-18,10-19"), (8..=18, 10..=19))
    }

    #[test]
    fn parse_range_test() {
        assert_eq!(parse_range("8-18"), 8..=18)
    }

    #[test]
    fn is_bad_assignment_pair_test() {
        assert!(is_bad_assignment_pair((2..=8, 3..=7)));
        assert!(!is_bad_assignment_pair((2..=6, 4..=8)));
    }
}
