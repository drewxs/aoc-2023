use crate::aoc::AOC;


pub fn part1(aoc: &AOC) -> u32 {
    let input = aoc.get_input(1).unwrap();

    input.lines().fold(0, |acc, line| {
        let digits: Vec<char> = line
            .chars()
            .filter_map(|c| if c.is_digit(10) { Some(c) } else { None })
            .collect();
        acc + format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse::<u32>()
            .unwrap()
    })
}
