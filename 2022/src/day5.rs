use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use ndarray::{Array, Array2};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Crate(char);

impl Crate {
    fn from_char(c: &char) -> Option<Crate> {
        match c {
            ' ' => None,
            c => Some(Crate(*c)),
        }
    }
}

#[derive(Debug)]
struct Crane {
    stacks: Vec<Vec<Crate>>,
}

impl Crane {
    fn pop_from_stack(&mut self, n: u32) -> Crate {
        let stack = self.stacks.get_mut(n as usize - 1).unwrap();
        stack.pop().expect("expected crate to exist in stack")
    }

    fn push_to_stack(&mut self, n: u32, c: Crate) {
        let stack = self.stacks.get_mut(n as usize - 1).unwrap();
        stack.push(c);
    }

    fn apply_step(&mut self, step: &Step) {
        for _n in 0..step.count {
            let popped_crate = self.pop_from_stack(step.from_stack);
            self.push_to_stack(step.to_stack, popped_crate);
        }
    }

    fn get_topmost_crates(&self) -> Vec<Crate> {
        self.stacks
            .iter()
            .filter_map(|s| s.last().copied())
            .collect()
    }
}

#[derive(Debug, PartialEq)]
struct Step {
    count: u32,
    from_stack: u32,
    to_stack: u32,
}

impl Step {
    fn from_str(s: &str) -> Step {
        let (count, from_stack, to_stack) = s
            .split(' ')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect_tuple()
            .unwrap();

        Step {
            count,
            from_stack,
            to_stack,
        }
    }
}

impl Crane {
    fn from_diagram(input: &str) -> Crane {
        let diagram_chars = input
            .lines()
            .map(|l| l.chars().skip(1).step_by(4).collect::<Vec<char>>())
            .filter(|chars| !chars.iter().all(|c| c.is_numeric()))
            .collect::<Vec<Vec<char>>>();
        let width = diagram_chars[0].len();
        let height = diagram_chars.len();
        let array: Array2<char> = Array::from_shape_vec(
            (height, width),
            diagram_chars.into_iter().flatten().collect(),
        )
        .unwrap();

        let stacks = array
            .columns()
            .into_iter()
            .map(|col| {
                col.into_iter()
                    .rev()
                    .filter_map(Crate::from_char)
                    .collect::<Vec<Crate>>()
            })
            .collect();

        Crane { stacks }
    }
}

fn day5(reader: BufReader<File>) -> String {
    let mut lines = reader.lines().map(|l| l.unwrap());
    let diagram = lines.by_ref().take_while(|l| !l.is_empty()).join("\n");
    let mut crane = Crane::from_diagram(&diagram);
    let procedure = lines.map(|l| Step::from_str(&l));
    for step in procedure {
        crane.apply_step(&step);
    }

    crane.get_topmost_crates().iter().map(|c| c.0).join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_test() {
        let input = File::open("./testdata/day5").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day5(reader), "MQTPGLLDN");
    }

    #[test]
    fn crane_from_diagram_test() {
        let input = "
                    [L]     [H] [W]
                [J] [Z] [J] [Q] [Q]
[S]             [M] [C] [T] [F] [B]
[P]     [H]     [B] [D] [G] [B] [P]
[W]     [L] [D] [D] [J] [W] [T] [C]
[N] [T] [R] [T] [T] [T] [M] [M] [G]
[J] [S] [Q] [S] [Z] [W] [P] [G] [D]
[Z] [G] [V] [V] [Q] [M] [L] [N] [R]
 1   2   3   4   5   6   7   8   9 ";
        let crane = Crane::from_diagram(input.trim_start_matches('\n'));
        assert_eq!(
            crane.stacks[0],
            vec![
                Crate('Z'),
                Crate('J'),
                Crate('N'),
                Crate('W'),
                Crate('P'),
                Crate('S')
            ]
        );
    }

    #[test]
    fn crane_apply_step_test() {
        let input = "
[A]    
[B] [C]
 1   2 ";
        let mut crane = Crane::from_diagram(input.trim_start_matches('\n'));
        crane.apply_step(&Step {
            count: 1,
            from_stack: 1,
            to_stack: 2,
        });
        assert_eq!(
            crane.stacks,
            vec![vec![Crate('B')], vec![Crate('C'), Crate('A')]]
        );

        crane.apply_step(&Step {
            count: 2,
            from_stack: 2,
            to_stack: 1,
        });
        assert_eq!(
            crane.stacks,
            vec![vec![Crate('B'), Crate('A'), Crate('C')], vec![]]
        );

        crane.apply_step(&Step {
            count: 3,
            from_stack: 1,
            to_stack: 2,
        });
        assert_eq!(
            crane.stacks,
            vec![vec![], vec![Crate('C'), Crate('A'), Crate('B')]]
        );
    }

    #[test]
    fn crane_get_topmost_test() {
        let input = "
[F]     [O]
[Z] [O] [V]
 1   2   3 ";
        let crane = Crane::from_diagram(input.trim_start_matches('\n'));
        assert_eq!(
            crane.get_topmost_crates(),
            vec![Crate('F'), Crate('O'), Crate('O')]
        );
    }

    #[test]
    fn step_from_str() {
        let input = "move 3 from 2 to 5";
        let step = Step::from_str(input);

        assert_eq!(
            step,
            Step {
                count: 3,
                from_stack: 2,
                to_stack: 5
            }
        )
    }
}
