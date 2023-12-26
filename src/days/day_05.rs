use std::ops::Range;

pub fn part_1(input: &str) -> i64 {
    let seeds = get_nums(input.lines().next());

    closest_location(input, &seeds)
}

pub fn part_2(input: &str) -> i64 {
    let seeds = get_nums(input.lines().next())
        .chunks(2)
        .map(|x| (x[0], x[1]))
        .flat_map(|(seed, range)| (0..range).map(move |x| seed + x))
        .collect();

    closest_location(input, &seeds)
}

fn get_nums(s: Option<&str>) -> Vec<i64> {
    s.unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect()
}

fn closest_location(input: &str, seeds: &Vec<i64>) -> i64 {
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
