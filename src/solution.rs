use crate::aoc::AOC;
use crate::days::*;

pub struct Solution {
    aoc: AOC,
}

impl Solution {
    pub fn new(aoc: AOC) -> Self {
        Self { aoc }
    }

    pub fn print_all(&self) {
        let mut part = 1;
        for i in 1..=31 {
            self.print(i, part);
            part = if part == 1 { 2 } else { 1 };
        }
    }

    pub fn print(&self, day: u8, part: u8) {
        match day {
            1 => match part {
                1 => self.print_solution(day, part, day_01::part_1),
                2 => self.print_solution(day, part, day_01::part_2),
                _ => println!("Invalid part"),
            },
            2 => match part {
                1 => self.print_solution(day, part, day_02::part_1),
                2 => self.print_solution(day, part, day_02::part_2),
                _ => println!("Invalid part"),
            },
            3 => match part {
                1 => self.print_solution(day, part, day_03::part_1),
                2 => self.print_solution(day, part, day_03::part_2),
                _ => println!("Invalid part"),
            },
            4 => match part {
                1 => self.print_solution(day, part, day_04::part_1),
                2 => self.print_solution(day, part, day_04::part_2),
                _ => println!("Invalid part"),
            },
            5 => match part {
                1 => self.print_solution(day, part, day_05::part_1),
                2 => self.print_solution(day, part, day_05::part_2),
                _ => println!("Invalid part"),
            },
            6 => match part {
                1 => self.print_solution(day, part, day_06::part_1),
                _ => println!("Invalid part"),
            },
            _ => println!("Invalid day"),
        }
    }

    pub fn print_solution<T: std::fmt::Display>(&self, day: u8, part: u8, solution: fn(&str) -> T) {
        let input = self.aoc.get_input(day).unwrap();
        let result = solution(&input);

        println!("day_{:02}::part_{} -> {}", day, part, result);
    }
}
