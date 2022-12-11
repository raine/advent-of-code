use std::{
    char,
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum GameResult {
    Win,
    Draw,
    Loss,
}

impl GameResult {
    fn score(&self) -> u32 {
        match *self {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }
}

use GameResult::*;
use RPS::*;

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Paper, Rock) => Some(Ordering::Greater),
            (Paper, Scissors) => Some(Ordering::Less),
            (Paper, Paper) => Some(Ordering::Equal),
            (Scissors, Paper) => Some(Ordering::Greater),
            (Scissors, Rock) => Some(Ordering::Less),
            (Scissors, Scissors) => Some(Ordering::Equal),
            (Rock, Paper) => Some(Ordering::Less),
            (Rock, Scissors) => Some(Ordering::Greater),
            (Rock, Rock) => Some(Ordering::Equal),
        }
    }
}

impl Ord for RPS {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl RPS {
    fn fight(&self, opponent: RPS) -> GameResult {
        match (*self).cmp(&opponent) {
            Ordering::Less => Loss,
            Ordering::Equal => Draw,
            Ordering::Greater => Win,
        }
    }
}

fn parse_opponent(c: char) -> RPS {
    match c {
        'A' => Rock,
        'B' => Paper,
        'C' => Scissors,
        _ => panic!("unexpected char"),
    }
}

fn parse_response(c: char) -> RPS {
    match c {
        'X' => Rock,
        'Y' => Paper,
        'Z' => Scissors,
        _ => panic!("unexpected char"),
    }
}

fn parse_round(input: &str) -> (RPS, RPS) {
    let mut split = input.split(' ').map(|s| s.chars().next().unwrap());
    let (a, b) = (split.next().unwrap(), split.next().unwrap());
    (parse_opponent(a), parse_response(b))
}

fn calculate_round_score(opponent: RPS, response: RPS) -> u32 {
    let shape_score = match response {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };
    let outcome = response.fight(opponent);
    let outcome_score = outcome.score();

    shape_score + outcome_score
}

fn day2(reader: BufReader<File>) -> u32 {
    reader
        .lines()
        .map(|line| parse_round(&line.unwrap()))
        .map(|(opponent, response)| calculate_round_score(opponent, response))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn test_parse_round() {
        let input = "A Y";
        assert_eq!(parse_round(input), (Rock, Paper));
    }

    #[test]
    fn test_ord() {
        assert!(Rock > Scissors);
        assert!(Scissors > Paper);
        assert!(Paper > Rock);
    }

    #[test]
    fn test_fight() {
        assert_eq!(Rock.fight(Scissors), Win);
        assert_eq!(Scissors.fight(Rock), Loss);
        assert_eq!(Paper.fight(Rock), Win);
    }

    #[test]
    fn test_calculate_round_score() {
        assert_eq!(calculate_round_score(Rock, Paper), 8);
        assert_eq!(calculate_round_score(Paper, Rock), 1);
        assert_eq!(calculate_round_score(Scissors, Scissors), 6);
    }

    #[test]
    fn test_day2() {
        let input = File::open("./testdata/day2").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day2(reader), 10718);
    }
}
