use crate::aoc::AOC;

const DIGITS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn part_1(aoc: &AOC) -> usize {
    let input = aoc.get_input(1).unwrap();
    let bytes = input.as_bytes();

    bytes.split(|b| b == &b'\n').fold(0, |acc, l| {
        if let Some(first) = l.iter().find(|c| c.is_ascii_digit()) {
            let last = l.iter().rev().find(|c| c.is_ascii_digit()).unwrap_or(first);
            acc + (((first - b'0') * 10) + (last - b'0')) as usize
        } else {
            acc
        }
    })
}

pub fn part_2(aoc: &AOC) -> usize {
    let input = aoc.get_input(1).unwrap();
    let bytes = input.as_bytes();

    bytes.split(|b| b == &b'\n').fold(0, |acc, l| {
        if let Some(first) = (0..l.len()).find_map(|i| get_num(l, i)) {
            let last = (0..l.len()).rev().find_map(|i| get_num(l, i)).unwrap();
            acc + first * 10 + last
        } else {
            acc
        }
    })
}

fn get_num(l: &[u8], i: usize) -> Option<usize> {
    l[i].is_ascii_digit()
        .then_some((l[i] - b'0') as usize)
        .or((0..DIGITS.len())
            .find(|&c| l[i..].starts_with(DIGITS[c]))
            .map(|c| c + 1))
}
