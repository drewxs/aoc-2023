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

fn get_nums(s: Option<&str>) -> Vec<usize> {
    s.unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect()
}
