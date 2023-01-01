use std::{thread, time::Duration};

use adventofcode_2022::day14::Grid;
use termion::{clear, style};

fn main() {
    let input = "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
        .trim_start();
    let mut grid = Grid::from_rock_paths(input, true);

    loop {
        grid.draw();
        if !grid.step() {
            break;
        }

        thread::sleep(Duration::from_millis(10));
        print!("{}{}", clear::All, style::Reset);
    }
}
