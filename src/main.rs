mod aoc;
mod days;

use aoc::AOC;
use std::env;

fn main() {
    dotenv::dotenv().ok();

    let token = env::var("AOC_TOKEN").unwrap();
    let aoc = AOC::new(2023, token);

    solution(1, 1, Box::new(days::day1::part1(&aoc)));
}

fn solution(day: u8, part: u8, x: Box<dyn std::fmt::Display>) {
    println!("day_{}::part_{} -> {}", day, part, x);
}
