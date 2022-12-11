use itertools::Itertools;
use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

lazy_static! {
    static ref ITEM_PRIORITY: HashMap<char, u32> = iter::zip(97..=122, 1..=26)
        .chain(iter::zip(65..=91, 27..=52))
        .map(|(item_ascii, priority)| (char::from_u32(item_ascii).unwrap(), priority))
        .collect::<HashMap<_, _>>();
}

fn item_priority(item: char) -> u32 {
    *ITEM_PRIORITY.get(&item).unwrap()
}

fn partition_rucksack(rucksack: &str) -> (Vec<char>, Vec<char>) {
    let chars = rucksack.chars().collect::<Vec<char>>();
    let (p1, p2) = chars.split_at(rucksack.len() / 2);
    (p1.to_vec(), p2.to_vec())
}

fn intersect(v1: Vec<char>, v2: Vec<char>) -> Vec<char> {
    let v1_set: HashSet<_> = v1.into_iter().collect();
    let v2_set: HashSet<_> = v2.into_iter().collect();
    v1_set.intersection(&v2_set).cloned().collect::<Vec<_>>()
}

fn find_duplicate_items(rucksack: &str) -> Vec<char> {
    let (p1, p2) = partition_rucksack(rucksack);
    intersect(p1, p2)
}

fn find_duplicate_item_in_rucksacks(r1: &str, r2: &str, r3: &str) -> Vec<char> {
    intersect(
        intersect(r1.chars().collect(), r2.chars().collect()),
        r3.chars().collect(),
    )
}

fn day3(reader: BufReader<File>) -> u32 {
    reader
        .lines()
        .flat_map(|line| find_duplicate_items(&line.unwrap()))
        .map(item_priority)
        .sum()
}

fn day3_part2(reader: BufReader<File>) -> u32 {
    reader
        .lines()
        .map(|l| l.unwrap())
        .chunks(3)
        .into_iter()
        .flat_map(|c| {
            let (r1, r2, r3) = c.collect_tuple().expect("expected 3 items");
            find_duplicate_item_in_rucksacks(&r1, &r2, &r3)
        })
        .map(item_priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn day3_test() {
        let input = File::open("./testdata/day3").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day3(reader), 7821);
    }

    #[test]
    fn day3_part2_test() {
        let input = File::open("./testdata/day3").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day3_part2(reader), 2752);
    }

    #[test]
    fn item_priority_test() {
        assert_eq!(item_priority('a'), 1);
        assert_eq!(item_priority('z'), 26);
        assert_eq!(item_priority('A'), 27);
        assert_eq!(item_priority('Z'), 52);
    }

    #[test]
    fn partition_rucksack_test() {
        assert_eq!(
            partition_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp"),
            (
                vec!['v', 'J', 'r', 'w', 'p', 'W', 't', 'w', 'J', 'g', 'W', 'r'],
                vec!['h', 'c', 's', 'F', 'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p']
            )
        );
    }

    #[test]
    fn find_duplicate_items_test() {
        assert_eq!(find_duplicate_items("vJrwpWtwJgWrhcsFMMfFFhFp"), vec!['p']);
    }

    #[test]
    fn find_duplicate_item_in_rucksacks_test() {
        assert_eq!(
            find_duplicate_item_in_rucksacks(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ),
            vec!['r']
        );
    }
}
