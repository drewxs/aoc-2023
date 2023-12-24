use std::ops::Range;

type LookupTable = Vec<(Range<i64>, i64)>; // (range, offset)

pub fn part_1(input: &str) -> i64 {
    let mut seeds: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let lists: Vec<LookupTable> = input
        .split("\n\n")
        .skip(1)
        .map(|map| {
            map.lines()
                .skip(1)
                .map(|line| {
                    let parts: Vec<i64> = line
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();

                    (parts[1]..parts[1] + parts[2], parts[0] - parts[1])
                })
                .collect()
        })
        .collect();

    for list in lists.iter() {
        for seed in seeds.iter_mut() {
            *seed = list
                .iter()
                .find_map(|(range, offset)| range.contains(seed).then_some(*seed + *offset))
                .unwrap_or(*seed)
        }
    }

    *seeds.iter().min().unwrap()
}
