use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operation {
    OldMultipliedByOld,
    OldMultipliedBy(u32),
    OldPlus(u32),
}

#[derive(Debug, PartialEq, Clone)]
struct Test {
    divisible_by: u32,
    if_true_throw_to_monkey: usize,
    if_false_throw_to_monkey: usize,
}

#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}

#[derive(Debug)]
struct KeepAway {
    monkeys: Vec<Monkey>,
    monkey_inspects: HashMap<usize, u64>,
    rounds: u32,
}

impl KeepAway {
    fn new(monkeys: Vec<Monkey>, rounds: u32) -> Self {
        Self {
            monkeys,
            monkey_inspects: HashMap::new(),
            rounds,
        }
    }

    fn play_round(&mut self) {
        let lcm = self
            .monkeys
            .iter()
            .fold(1, |acc, m| acc * m.test.divisible_by as u64);

        for n in 0..self.monkeys.len() {
            let monkey = self.monkeys[n].clone();
            for item in &monkey.items {
                *self.monkey_inspects.entry(n).or_insert(0) += 1;

                let mut new_worry_level = match monkey.operation {
                    Operation::OldMultipliedByOld => item.pow(2),
                    Operation::OldMultipliedBy(n) => item * (n as u64),
                    Operation::OldPlus(n) => item + (n as u64),
                };

                if self.rounds == 20 {
                    new_worry_level /= 3;
                } else {
                    new_worry_level %= lcm
                }

                let throw_to_monkey = if new_worry_level % (monkey.test.divisible_by as u64) == 0 {
                    monkey.test.if_true_throw_to_monkey
                } else {
                    monkey.test.if_false_throw_to_monkey
                };

                let m = &mut self.monkeys[throw_to_monkey];
                m.items.push(new_worry_level);
            }

            let m = &mut self.monkeys[n];
            m.items.clear();
        }
    }

    fn play(&mut self) -> u64 {
        for _n in 0..self.rounds {
            self.play_round();
        }

        self.monkey_inspects
            .values()
            .sorted()
            .rev()
            .take(2)
            .product()
    }
}

fn parse_monkey(input: &str) -> Monkey {
    let mut lines = input.lines().skip(1);
    let items = lines
        .next()
        .unwrap()
        .split_once("Starting items: ")
        .map(|(_, s)| {
            s.split(", ")
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .unwrap();

    let operation = lines
        .next()
        .unwrap()
        .split_once("Operation: new = ")
        .map(|(_, s)| {
            let mut parts = s.splitn(3, ' ');
            match (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            ) {
                ("old", "*", "old") => Operation::OldMultipliedByOld,
                ("old", "*", x) => Operation::OldMultipliedBy(x.parse::<u32>().unwrap()),
                ("old", "+", x) => Operation::OldPlus(x.parse::<u32>().unwrap()),
                _ => {
                    panic!("unexpected operation")
                }
            }
        })
        .unwrap();

    let test_divisible_by = lines
        .next()
        .unwrap()
        .split_once("Test: divisible by ")
        .map(|(_, s)| s.parse::<u32>().unwrap())
        .unwrap();

    let if_true_throw_to_monkey = lines
        .next()
        .unwrap()
        .split_once("If true: throw to monkey ")
        .map(|(_, s)| s.parse::<usize>().unwrap())
        .unwrap();

    let if_false_throw_to_monkey = lines
        .next()
        .unwrap()
        .split_once("If false: throw to monkey ")
        .map(|(_, s)| s.parse::<usize>().unwrap())
        .unwrap();

    let test = Test {
        divisible_by: test_divisible_by,
        if_true_throw_to_monkey,
        if_false_throw_to_monkey,
    };

    Monkey {
        items,
        operation,
        test,
    }
}

fn day11(input: &str) -> u64 {
    let monkeys = input
        .split("\n\n")
        .map(parse_monkey)
        .collect::<Vec<Monkey>>();

    let mut keep_away = KeepAway::new(monkeys, 20);
    keep_away.play()
}

fn day11_part2(input: &str) -> u64 {
    let monkeys = input
        .split("\n\n")
        .map(parse_monkey)
        .collect::<Vec<Monkey>>();

    let mut keep_away = KeepAway::new(monkeys, 10000);
    keep_away.play()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_test() {
        let input = include_str!("../testdata/day11");
        assert_eq!(day11(input.trim_start_matches('\n')), 118674);
    }

    #[test]
    fn day11_simple_test() {
        let input = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(day11(input.trim_start_matches('\n')), 10605);
    }

    #[test]
    fn day11_part2_test() {
        let input = include_str!("../testdata/day11");
        assert_eq!(day11_part2(input.trim_start_matches('\n')), 32333418600);
    }

    #[test]
    fn day11_part_2_simple_test() {
        let input = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(day11_part2(input.trim_start_matches('\n')), 2713310158);
    }

    #[test]
    fn parse_monkey_test() {
        let input = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";
        let monkey = parse_monkey(input.trim_start_matches('\n'));

        assert_eq!(
            monkey,
            Monkey {
                items: vec![79, 98],
                operation: Operation::OldMultipliedBy(19),
                test: Test {
                    divisible_by: 23,
                    if_true_throw_to_monkey: 2,
                    if_false_throw_to_monkey: 3
                }
            }
        );
    }
}
