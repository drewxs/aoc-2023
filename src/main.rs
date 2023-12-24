mod aoc;
mod days;
mod solution;

use std::env;

use aoc::AOC;
use days::*;
use solution::Solution;

fn main() {
    dotenv::dotenv().ok();

    let token = env::var("AOC_TOKEN").expect("AOC_TOKEN not set");
    let aoc = AOC::new(2023, token);
    let mut solution = Solution::new(aoc);

    solution.print(day_01::part_1);
    solution.print(day_01::part_2);
    solution.print(day_02::part_1);
    solution.print(day_02::part_2);
    solution.print(day_03::part_1);
    solution.print(day_03::part_2);
    solution.print(day_04::part_1);
    solution.print(day_04::part_2);
    solution.print(day_05::part_1);
}
