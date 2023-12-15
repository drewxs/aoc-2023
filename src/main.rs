mod aoc;
mod days;

use aoc::AOC;
use std::env;

fn main() {
    dotenv::dotenv().ok();

    let token = env::var("AOC_TOKEN").expect("AOC_TOKEN not set");
    let aoc = AOC::new(2023, token);

    print_solution(1, 1, days::day_01::part_1(&aoc));
    print_solution(1, 2, days::day_01::part_2(&aoc));
    print_solution(2, 1, days::day_02::part_1(&aoc));
}

fn print_solution(day: u8, part: u8, x: usize) {
    println!("day_{:02}::part_{:02} -> {}", day, part, x);
}
