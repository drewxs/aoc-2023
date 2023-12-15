use crate::aoc::AOC;

const MAX_RED_CUBES: u8 = 12;
const MAX_GREEN_CUBES: u8 = 13;
const MAX_BLUE_CUBES: u8 = 14;

pub fn part_1(aoc: &AOC) -> usize {
    let input = aoc.get_input(2).unwrap();

    input.lines().fold(0, |acc, line| {
        let id = line
            .get(5..line.find(":").unwrap_or(6))
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);

        let subsets: Vec<&str> = line
            .split(":")
            .skip(1)
            .next()
            .unwrap_or("")
            .split(";")
            .collect();

        for subset in subsets.iter() {
            let cubes: Vec<&str> = subset.split(",").collect();

            for cube in cubes.iter() {
                if let Some((count, color)) = cube.trim().split_once(" ") {
                    let count = count.parse::<u8>().unwrap_or(0);
                    match color {
                        "red" if count > MAX_RED_CUBES => return acc,
                        "green" if count > MAX_GREEN_CUBES => return acc,
                        "blue" if count > MAX_BLUE_CUBES => return acc,
                        _ => (),
                    }
                }
            }
        }

        acc + id
    })
}
