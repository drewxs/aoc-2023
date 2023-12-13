use crate::aoc::AOC;


pub fn part1(aoc: &AOC) -> usize {
    let input = aoc.get_input(1).unwrap();
    let bytes = input.as_bytes();

    bytes.split(|b| b == &b'\n').fold(0, |acc, l| {
        if let Some(first) = l.iter().find(|c| c.is_ascii_digit()) {
            let last = l.iter().rev().find(|c| c.is_ascii_digit()).unwrap_or(first);
            acc + (((first - b'0') * 10) + (last - b'0')) as usize
        } else {
            acc
    })
}
