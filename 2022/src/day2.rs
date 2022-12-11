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

#[derive(Debug, PartialEq, Clone, Copy)]
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

    fn score(&self) -> u32 {
        match *self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
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

fn parse_desired_outcome(c: char) -> GameResult {
    match c {
        'X' => Loss,
        'Y' => Draw,
        'Z' => Win,
        _ => panic!("unexpected char"),
    }
}

fn parse_line_chars(line: &str) -> (char, char) {
    let mut split = line.split(' ').map(|s| s.chars().next().unwrap());
    (split.next().unwrap(), split.next().unwrap())
}

fn parse_round(input: &str) -> (RPS, RPS) {
    let (a, b) = parse_line_chars(input);
    (parse_opponent(a), parse_response(b))
}

fn parse_round_part2(input: &str) -> (RPS, GameResult) {
    let (a, b) = parse_line_chars(input);
    (parse_opponent(a), parse_desired_outcome(b))
}

fn calculate_round_score(opponent: RPS, response: RPS) -> u32 {
    let shape_score = response.score();
    let outcome = response.fight(opponent);
    let outcome_score = outcome.score();
    shape_score + outcome_score
}

fn get_response_for_desired_outcome(opponent: RPS, desired_outcome: GameResult) -> RPS {
    [Rock, Paper, Scissors]
        .into_iter()
        .find(|rps| rps.fight(opponent) == desired_outcome)
        .unwrap()
}

fn calculate_round_score_part2(opponent: RPS, desired_outcome: GameResult) -> u32 {
    let response = get_response_for_desired_outcome(opponent, desired_outcome);
    let shape_score = response.score();
    shape_score + desired_outcome.score()
}

fn day2(reader: BufReader<File>) -> u32 {
    reader
        .lines()
        .map(|line| parse_round(&line.unwrap()))
        .map(|(opponent, response)| calculate_round_score(opponent, response))
        .sum()
}

fn day2_part2(reader: BufReader<File>) -> u32 {
    reader
        .lines()
        .map(|line| parse_round_part2(&line.unwrap()))
        .map(|(opponent, desired_outcome)| calculate_round_score_part2(opponent, desired_outcome))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn parse_round_test() {
        let input = "A Y";
        assert_eq!(parse_round(input), (Rock, Paper));
    }

    #[test]
    fn parse_round_part2_test() {
        let input = "A Y";
        assert_eq!(parse_round_part2(input), (Rock, Draw));
    }

    #[test]
    fn ord_test() {
        assert!(Rock > Scissors);
        assert!(Scissors > Paper);
        assert!(Paper > Rock);
    }

    #[test]
    fn fight_test() {
        assert_eq!(Rock.fight(Scissors), Win);
        assert_eq!(Scissors.fight(Rock), Loss);
        assert_eq!(Paper.fight(Rock), Win);
    }

    #[test]
    fn calculate_round_score_test() {
        assert_eq!(calculate_round_score(Rock, Paper), 8);
        assert_eq!(calculate_round_score(Paper, Rock), 1);
        assert_eq!(calculate_round_score(Scissors, Scissors), 6);
    }

    #[test]
    fn get_response_for_desired_outcome_test() {
        assert_eq!(get_response_for_desired_outcome(Rock, Draw), Rock);
        assert_eq!(get_response_for_desired_outcome(Paper, Loss), Rock);
    }

    #[test]
    fn day2_test() {
        let input = File::open("./testdata/day2").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day2(reader), 10718);
    }

    #[test]
    fn day2_part2_test() {
        let input = File::open("./testdata/day2").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day2_part2(reader), 14652);
    }
}
