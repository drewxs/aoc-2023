use crate::aoc::AOC;

pub struct Solution {
    day: u8,
    part: u8,
    count: u8,
    aoc: AOC,
}

impl Solution {
    pub fn new(aoc: AOC) -> Self {
        Self {
            day: 1,
            part: 1,
            count: 0,
            aoc,
        }
    }

    pub fn print<T: std::fmt::Display>(&mut self, solution: fn(&str) -> T) {
        let input = self.aoc.get_input(self.day).unwrap();
        let result = solution(&input);

        println!("day_{:02}::part_{} -> {}", self.day, self.part, result);

        self.count += 1;
        if self.count % 2 == 0 {
            self.day += 1;
            self.part = 1;
        } else {
            self.part = 2;
        }
    }
}
