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
        for i in 1..=25 {
            self.print(i, 1);
            self.print(i, 2);
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
                2 => self.print_solution(day, part, day_06::part_2),
                _ => println!("Invalid part"),
            },
            7 => match part {
                1 => self.print_solution(day, part, day_07::part_1),
                2 => self.print_solution(day, part, day_07::part_2),
                _ => println!("Invalid part"),
            },
            8 => match part {
                1 => self.print_solution(day, part, day_08::part_1),
                2 => self.print_solution(day, part, day_08::part_2),
                _ => println!("Invalid part"),
            },
            9 => match part {
                1 => self.print_solution(day, part, day_09::part_1),
                2 => self.print_solution(day, part, day_09::part_2),
                _ => println!("Invalid part"),
            },
            10 => match part {
                1 => self.print_solution(day, part, day_10::part_1),
                2 => self.print_solution(day, part, day_10::part_2),
                _ => println!("Invalid part"),
            },
            11 => match part {
                1 => self.print_solution(day, part, day_11::part_1),
                2 => self.print_solution(day, part, day_11::part_2),
                _ => println!("Invalid part"),
            },
            12 => match part {
                1 => self.print_solution(day, part, day_12::part_1),
                2 => self.print_solution(day, part, day_12::part_2),
                _ => println!("Invalid part"),
            },
            13 => match part {
                1 => self.print_solution(day, part, day_13::part_1),
                2 => self.print_solution(day, part, day_13::part_2),
                _ => println!("Invalid part"),
            },
            14 => match part {
                1 => self.print_solution(day, part, day_14::part_1),
                2 => self.print_solution(day, part, day_14::part_2),
                _ => println!("Invalid part"),
            },
            15 => match part {
                1 => self.print_solution(day, part, day_15::part_1),
                2 => self.print_solution(day, part, day_15::part_2),
                _ => println!("Invalid part"),
            },
            16 => match part {
                1 => self.print_solution(day, part, day_16::part_1),
                2 => self.print_solution(day, part, day_16::part_2),
                _ => println!("Invalid part"),
            },
            17 => match part {
                1 => self.print_solution(day, part, day_17::part_1),
                2 => self.print_solution(day, part, day_17::part_2),
                _ => println!("Invalid part"),
            },
            18 => match part {
                1 => self.print_solution(day, part, day_18::part_1),
                2 => self.print_solution(day, part, day_18::part_2),
                _ => println!("Invalid part"),
            },
            19 => match part {
                1 => self.print_solution(day, part, day_19::part_1),
                2 => self.print_solution(day, part, day_19::part_2),
                _ => println!("Invalid part"),
            },
            20 => match part {
                1 => self.print_solution(day, part, day_20::part_1),
                2 => self.print_solution(day, part, day_20::part_2),
                _ => println!("Invalid part"),
            },
            21 => match part {
                1 => self.print_solution(day, part, day_21::part_1),
                2 => self.print_solution(day, part, day_21::part_2),
                _ => println!("Invalid part"),
            },
            22 => match part {
                1 => self.print_solution(day, part, day_22::part_1),
                2 => self.print_solution(day, part, day_22::part_2),
                _ => println!("Invalid part"),
            },
            23 => match part {
                1 => self.print_solution(day, part, day_23::part_1),
                2 => self.print_solution(day, part, day_23::part_2),
                _ => println!("Invalid part"),
            },
            24 => match part {
                1 => self.print_solution(day, part, day_24::part_1),
                2 => self.print_solution(day, part, day_24::part_2),
                _ => println!("Invalid part"),
            },
            25 => match part {
                1 => self.print_solution(day, part, day_25::part_1),
                2 => self.print_solution(day, part, day_25::part_2),
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
