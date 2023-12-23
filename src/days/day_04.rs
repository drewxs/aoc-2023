pub fn part_1(input: &str) -> u16 {
    input
        .lines()
        .map(|card| {
            card.split(":").collect::<Vec<&str>>()[1]
                .split("|")
                .map(|part| {
                    part.split_whitespace()
                        .map(|x| x.trim().parse::<u8>().unwrap())
                        .collect::<Vec<u8>>()
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
