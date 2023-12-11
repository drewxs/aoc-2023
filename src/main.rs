mod aoc;

use aoc::AOC;
use std::env;

fn main() {
    dotenv::dotenv().ok();

    let token = env::var("AOC_TOKEN").unwrap();
    let aoc = AOC::new(2023, token);

    let _ = aoc.get_input(1);
}
