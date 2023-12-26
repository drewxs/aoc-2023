pub fn part_1(input: &str) -> usize {
    let mut lines = input.lines();

    let times = get_nums(lines.next());
    let distances = get_nums(lines.next());

    times.iter().enumerate().fold(1, |acc, (i, &time)| {
        let num_ways = (0..=time).fold(0, |acc, j| {
            if (time - j) * j > distances[i] {
                return acc + 1;
            }
            acc
        });

        acc * num_ways
    })
}

pub fn part_2(input: &str) -> usize {
    let mut lines = input.lines();

    let time = get_num(lines.next());
    let distance = get_num(lines.next());

    (0..=time).fold(0, |acc, i| {
        if (time - i) * i > distance {
            return acc + 1;
        }
        acc
    })
}

fn get_nums(s: Option<&str>) -> Vec<usize> {
    s.unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_num(s: Option<&str>) -> usize {
    s.unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap()
}
