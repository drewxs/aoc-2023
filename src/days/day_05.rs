use std::ops::Range;

pub fn part_1(input: &str) -> i64 {
    let seeds: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let lists: Vec<Vec<(Range<i64>, i64)>> = input
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

                    (parts[1]..parts[1] + parts[2], parts[0] - parts[1]) // (range, offset)
                })
                .collect()
        })
        .collect();

    seeds
        .iter()
        .map(|seed| {
            lists.iter().fold(*seed, |acc, list| {
                list.iter()
                    .find_map(|(range, offset)| range.contains(&acc).then_some(acc + offset))
                    .unwrap_or(acc)
            })
        })
        .min()
        .unwrap()
}
