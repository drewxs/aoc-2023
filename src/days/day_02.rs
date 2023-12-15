use crate::aoc::AOC;

const MAX_RED_CUBES: u8 = 12;
const MAX_GREEN_CUBES: u8 = 13;
const MAX_BLUE_CUBES: u8 = 14;

pub fn part_1(aoc: &AOC) -> usize {
    let input = aoc.get_input(2).unwrap();

    let mut game = 0;

    input.lines().fold(0, |acc, line| {
        game += 1;

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

        acc + game
    })
}

pub fn part_2(aoc: &AOC) -> usize {
    let input = aoc.get_input(2).unwrap();

    input.lines().fold(0, |acc, line| {
        let subsets: Vec<&str> = line
            .split(":")
            .skip(1)
            .next()
            .unwrap_or("")
            .split(";")
            .collect();

        let mut max_red_cubes = 0;
        let mut max_green_cubes = 0;
        let mut max_blue_cubes = 0;

        for subset in subsets.iter() {
            let cubes: Vec<&str> = subset.split(",").collect();

            for cube in cubes.iter() {
                if let Some((count, color)) = cube.trim().split_once(" ") {
                    let count = count.parse::<usize>().unwrap_or(0);
                    match color {
                        "red" if count > max_red_cubes => max_red_cubes = count,
                        "green" if count > max_green_cubes => max_green_cubes = count,
                        "blue" if count > max_blue_cubes => max_blue_cubes = count,
                        _ => (),
                    }
                }
            }
        }

        acc + max_red_cubes * max_green_cubes * max_blue_cubes
    })
}
