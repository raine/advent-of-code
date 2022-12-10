use eyre::Result;
use std::{fs::File, io::prelude::*, io::BufReader};

fn day1_part1(reader: BufReader<File>) -> Result<i32> {
    let mut max_calories = 0;
    let mut current_elf_calories = 0;

    for line in reader.lines() {
        match line?.as_str() {
            "" => {
                if current_elf_calories > max_calories {
                    max_calories = current_elf_calories;
                }

                current_elf_calories = 0;
            }
            line => {
                let calories = line.parse::<i32>()?;
                current_elf_calories += calories;
            }
        }
    }

    Ok(max_calories)
}

fn day1_part2(reader: BufReader<File>) -> Result<i32> {
    let mut elf_calories = vec![];
    let mut current_elf_calories = 0;

    for line in reader.lines() {
        match line?.as_str() {
            "" => {
                elf_calories.push(current_elf_calories);
                current_elf_calories = 0;
            }
            line => {
                let calories = line.parse::<i32>()?;
                current_elf_calories += calories;
            }
        }
    }

    elf_calories.sort_by(|a, b| b.cmp(a));
    Ok(elf_calories.iter().take(3).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn day1_part1_test() {
        let input = File::open("./testdata/day1").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day1_part1(reader).unwrap(), 69836);
    }

    #[test]
    fn day1_part2_test() {
        let input = File::open("./testdata/day1").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day1_part2(reader).unwrap(), 207968);
    }
}
