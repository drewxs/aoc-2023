pub fn part_1(input: &str) -> usize {
    let mut lines = input.lines();

    let times = get_nums(lines.next());
    let distances = get_nums(lines.next());

    let mut total: Vec<usize> = Vec::new();

    for (i, &time) in times.iter().enumerate() {
        let target_distance = distances[i];

        let mut num_ways = 0;
        for j in 0..=time {
            if (time - j) * j > target_distance {
                num_ways += 1;
            }
        }

        total.push(num_ways);
    }

    total.iter().fold(1, |acc, x| acc * x)
}

fn get_nums(s: Option<&str>) -> Vec<usize> {
    s.unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect()
}
