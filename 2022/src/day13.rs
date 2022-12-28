use core::fmt;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{all_consuming, map, map_res},
    multi::separated_list0,
    sequence::delimited,
    Finish, IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    List(Vec<Value>),
    Number(u32),
}

impl Value {
    fn list_from_u32(n: u32) -> Value {
        List(vec![Number(n)])
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            List(l) => write!(f, "[{}]", l.iter().map(|v| format!("{v}")).join(",")),
            Number(n) => write!(f, "{}", n),
        }
    }
}

use Value::*;

fn parse_number(i: &str) -> IResult<&str, Value> {
    map(map_res(digit1, str::parse), Number)(i)
}

fn parse_list_item(i: &str) -> IResult<&str, Value> {
    alt((parse_number, parse_list))(i)
}

fn parse_list(i: &str) -> IResult<&str, Value> {
    map(
        delimited(
            char('['),
            separated_list0(char(','), parse_list_item),
            char(']'),
        ),
        List,
    )(i)
}

fn parse(i: &str) -> Value {
    all_consuming(parse_list)(i).finish().unwrap().1
}

#[derive(Debug, PartialEq)]
enum Order {
    Right,
    Wrong,
    Continue,
}

fn compare_order_recur(l: &Value, r: &Value, depth: usize) -> Order {
    let indent = "  ".repeat(depth);
    println!("{indent}Compare {l} vs {r}");
    let order = match (l, r) {
        (List(lv), List(rv)) => {
            if lv.is_empty() && rv.is_empty() {
                Order::Continue
            } else if lv.is_empty() {
                println!("{indent}Left list ran out of items => Right");
                Order::Right
            } else if rv.is_empty() {
                println!("{indent}Right list ran out of items => Wrong");
                Order::Wrong
            } else {
                let mut left_values = lv.iter().enumerate().peekable();
                let mut o = Order::Continue;
                while let Some((idx, l)) = left_values.next() {
                    let r = match rv.get(idx) {
                        Some(x) => x,
                        None => {
                            println!("{indent}Right list ran out of items => Wrong");
                            o = Order::Wrong;
                            break;
                        }
                    };

                    match compare_order_recur(l, r, depth + 1) {
                        Order::Continue => {
                            if left_values.peek().is_none() && rv.get(idx + 1).is_some() {
                                println!("{indent}Left list ran out of items => Right");
                                o = Order::Right;
                                break;
                            }
                        }
                        order => {
                            o = order;
                            println!("{indent}=> {o:?}");
                            break;
                        }
                    }
                }

                o
            }
        }
        (Number(l), List(rv)) => {
            compare_order_recur(&Value::list_from_u32(*l), &List(rv.to_vec()), depth + 1)
        }
        (List(lv), Number(r)) => {
            compare_order_recur(&List(lv.to_vec()), &Value::list_from_u32(*r), depth + 1)
        }
        (Number(l), Number(r)) if l < r => Order::Right,
        (Number(l), Number(r)) if l > r => Order::Wrong,
        (Number(_), Number(_)) => Order::Continue,
    };

    order
}

fn compare_order(l: &Value, r: &Value) -> Order {
    println!();
    compare_order_recur(l, r, 0)
}

fn day13<T>(reader: BufReader<T>) -> usize
where
    T: std::io::Read,
{
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let pairs = lines.split(|line| line.is_empty()).map(|pair| {
        pair.iter()
            .map(|e| parse(e))
            .collect_tuple::<(_, _)>()
            .unwrap()
    });

    pairs
        .enumerate()
        .filter_map(|(idx, (l, r))| match compare_order(&l, &r) {
            Order::Right => Some(idx + 1),
            _ => None,
        })
        .sum()
}

fn day13_part2<T>(reader: BufReader<T>) -> usize
where
    T: std::io::Read,
{
    let packets = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty());
    let divider_packets = ["[[2]]".to_owned(), "[[6]]".to_owned()].into_iter();

    packets
        .chain(divider_packets)
        .map(|l| parse(&l))
        .sorted_by(|a, b| match compare_order(a, b) {
            Order::Right => std::cmp::Ordering::Less,
            Order::Wrong => std::cmp::Ordering::Greater,
            _ => panic!(),
        })
        .enumerate()
        .filter(|(_idx, p)| p == &parse("[[2]]") || p == &parse("[[6]]"))
        .map(|(idx, _p)| idx + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn day13_simple_test() {
        let input = File::open("./testdata/day13_simple").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day13(reader), 13);
    }

    #[test]
    fn day13_test() {
        let input = File::open("./testdata/day13").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day13(reader), 5159);
    }

    #[test]
    fn day13_part2_simple_test() {
        let input = File::open("./testdata/day13_simple").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day13_part2(reader), 140);
    }

    #[test]
    fn day13_part2_test() {
        let input = File::open("./testdata/day13").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day13_part2(reader), 22344);
    }

    #[test]
    fn compare_order_test_1() {
        let l = parse("[1,1,3,1,1]");
        let r = parse("[1,1,5,1,1]");
        assert_eq!(compare_order(&l, &r), Order::Right);
    }

    #[test]
    fn compare_order_test_2() {
        let l = parse("[[1],[2,3,4]]");
        let r = parse("[[1],4]");
        assert_eq!(compare_order(&l, &r), Order::Right);
    }

    #[test]
    fn compare_order_test_3() {
        let l = parse("[9]");
        let r = parse("[[8,7,6]]");
        assert_eq!(compare_order(&l, &r), Order::Wrong);
    }

    #[test]
    fn compare_order_test_4() {
        let l = parse("[[4,4],4,4]");
        let r = parse("[[4,4],4,4,4]");
        assert_eq!(compare_order(&l, &r), Order::Right);
    }

    #[test]
    fn compare_order_test_5() {
        let l = parse("[7,7,7,7]");
        let r = parse("[7,7,7]");
        assert_eq!(compare_order(&l, &r), Order::Wrong);
    }

    #[test]
    fn compare_order_test_6() {
        let l = parse("[]");
        let r = parse("[3]");
        assert_eq!(compare_order(&l, &r), Order::Right);
    }

    #[test]
    fn compare_order_test_7() {
        let l = parse("[[[]]]");
        let r = parse("[[]]");
        assert_eq!(compare_order(&l, &r), Order::Wrong);
    }

    #[test]
    fn compare_order_test_8() {
        let l = parse("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let r = parse("[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert_eq!(compare_order(&l, &r), Order::Wrong);
    }

    #[test]
    fn parse_test() {
        assert_eq!(
            parse("[1,[[1,2,3],1,2,3]]"),
            List(vec![
                Number(1),
                List(vec![
                    List(vec![Number(1), Number(2), Number(3)]),
                    Number(1),
                    Number(2),
                    Number(3)
                ]),
            ],)
        );
    }
}
