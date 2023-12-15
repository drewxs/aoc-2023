use crate::aoc::AOC;

pub struct Solution {
    day: u8,
    part: u8,
    aoc: AOC,
}

impl Solution {
    pub fn new(aoc: AOC) -> Self {
        Self {
            day: 1,
            part: 1,
            aoc,
        }
    }

    pub fn print(&mut self, solution: fn(&AOC) -> usize) {
        let result = solution(&self.aoc);
        println!("day_{:02}::part_{:02} -> {}", self.day, self.part, result);
        self.day += 1;
        self.part = if self.part == 1 { 2 } else { 1 };
    }
}
