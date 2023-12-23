const DIGITS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn part_1(input: &str) -> u16 {
    input.as_bytes().split(|b| b == &b'\n').fold(0, |acc, l| {
        if let Some(first) = l.iter().find(|c| c.is_ascii_digit()) {
            let last = l.iter().rev().find(|c| c.is_ascii_digit()).unwrap_or(first);
            return acc + (((first - b'0') * 10) + (last - b'0')) as u16;
        }
        acc
    })
}

pub fn part_2(input: &str) -> usize {
    input.as_bytes().split(|b| b == &b'\n').fold(0, |acc, l| {
        if let Some(first) = (0..l.len()).find_map(|i| get_num(l, i)) {
            let last = (0..l.len()).rev().find_map(|i| get_num(l, i)).unwrap();
            return acc + first * 10 + last;
        }
        acc
    })
}

fn get_num(l: &[u8], i: usize) -> Option<usize> {
    l[i].is_ascii_digit()
        .then_some((l[i] - b'0') as usize)
        .or((0..DIGITS.len())
            .find(|&c| l[i..].starts_with(DIGITS[c]))
            .map(|c| c + 1))
}
