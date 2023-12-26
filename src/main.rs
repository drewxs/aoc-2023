mod aoc;
mod days;
mod solution;

use std::env;

use aoc::AOC;
use solution::Solution;

fn main() {
    dotenv::dotenv().ok();

    let token = env::var("AOC_TOKEN").expect("AOC_TOKEN not set");
    let aoc = AOC::new(2023, token);
    let solution = Solution::new(aoc);

    let args: Vec<u8> = env::args().skip(1).map(|x| x.parse().unwrap()).collect();
    match args.len() {
        0 => solution.print_all(),
        1 => {
            solution.print(args[0], 1);
            solution.print(args[0], 2);
        }
        2 => solution.print(args[0], args[1]),
        _ => println!("Invalid number of arguments"),
    }
}
