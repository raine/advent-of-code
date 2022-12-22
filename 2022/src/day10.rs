use std::io::{BufRead, BufReader};

fn day10<T>(reader: BufReader<T>) -> i32
where
    T: std::io::Read,
{
    let mut x = 1;
    let mut cycle_n = 0;
    let mut cycles: Vec<(i32, i32)> = vec![];
    let mut start_cycle = |x: i32| {
        cycle_n += 1;
        cycles.push((cycle_n, x))
    };
    for line in reader.lines() {
        match parse_instruction(&line.unwrap()) {
            Instruction::Noop => {
                start_cycle(x);
            }
            Instruction::Addx(v) => {
                start_cycle(x);
                start_cycle(x);
                x += v;
            }
        }
    }

    cycles
        .into_iter()
        .filter_map(|(cycle, x)| match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => Some(cycle * x),
            _ => None,
        })
        .sum()
}

fn day10_part2<T>(reader: BufReader<T>) -> String
where
    T: std::io::Read,
{
    let mut x = 1;
    let mut output = String::new();
    let mut cycle_n = 0;
    let mut crt_row_pos = 0;
    let mut start_cycle = |x: i32| {
        let sprite_pos = x - 1..=x + 1;
        if sprite_pos.contains(&crt_row_pos) {
            output.push('#');
        } else {
            output.push('.');
        };

        cycle_n += 1;

        if crt_row_pos == 39 {
            output.push('\n');
            crt_row_pos = 0;
        } else {
            crt_row_pos += 1;
        }
    };
    for line in reader.lines() {
        match parse_instruction(&line.unwrap()) {
            Instruction::Noop => {
                start_cycle(x);
            }
            Instruction::Addx(v) => {
                start_cycle(x);
                start_cycle(x);
                x += v;
            }
        }
    }

    output
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_instruction(input: &str) -> Instruction {
    let mut split = input.split(' ');
    let (op, v) = (
        split.next().unwrap(),
        split.next().map(|s| s.parse::<i32>().unwrap()),
    );

    match op {
        "noop" => Instruction::Noop,
        "addx" => Instruction::Addx(v.unwrap()),
        _ => panic!("unexpected instruction"),
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn day10_test_less_simple() {
        let input = File::open("./testdata/day10_less_simple").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day10(reader), 13140);
    }

    #[test]
    fn day10_test() {
        let input = File::open("./testdata/day10").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day10(reader), 12540);
    }

    #[test]
    fn day10_part2_test_less_simple() {
        let input = File::open("./testdata/day10_less_simple").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(
            day10_part2(reader),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
            .trim_start_matches('\n')
        );
    }

    #[test]
    fn day10_part2_test() {
        let input = File::open("./testdata/day10").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(
            day10_part2(reader),
            "
####.####..##..####.####.#....#..#.####.
#....#....#..#....#.#....#....#..#.#....
###..###..#......#..###..#....####.###..
#....#....#.....#...#....#....#..#.#....
#....#....#..#.#....#....#....#..#.#....
#....####..##..####.####.####.#..#.####.
"
            .trim_start_matches('\n')
        );
    }
}
