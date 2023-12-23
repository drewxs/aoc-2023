use std::collections::HashMap;

pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|card| {
            card.split(":").collect::<Vec<&str>>()[1]
                .split("|")
                .map(|part| {
                    part.split_whitespace()
                        .map(|x| x.trim().parse::<u8>().unwrap())
                        .collect()
                })
                .collect::<Vec<Vec<u8>>>()
        })
        .fold(0, |acc, card| {
            acc + card[0].iter().fold(0, |acc, x| {
                if card[1].contains(x) {
                    return if acc == 0 { 1 } else { acc * 2 };
                }
                acc
            })
        })
}

pub fn part_2(input: &str) -> u32 {
    let mut copies: HashMap<usize, u32> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let card: Vec<Vec<u8>> = line.split(":").collect::<Vec<&str>>()[1]
            .split("|")
            .map(|part| {
                part.split_whitespace()
                    .map(|x| x.trim().parse::<u8>().unwrap())
                    .collect()
            })
            .collect();

        let mut num_winning = 0;
        for winning_num in card[0].iter() {
            if card[1].contains(winning_num) {
                num_winning += 1;
            }
        }

        *copies.entry(i).or_insert(0) += 1;
        let num_copies = *copies.get(&i).unwrap();

        for j in 1..=num_winning {
            *copies.entry(i + j).or_insert(0) += num_copies;
        }
    }

    copies.values().sum()
}
