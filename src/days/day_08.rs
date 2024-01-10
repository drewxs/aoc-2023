use std::collections::HashMap;

pub fn part_1(input: &str) -> usize {
    let instructions: Vec<char> = input.lines().next().unwrap().chars().collect();

    let mut count = 0;
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut iter = input.lines().skip(2);
    let mut curr = "";
    let mut target = "";

    while let Some(line) = iter.next() {
        let (k, v) = line.split_once(" = ").unwrap();
        if curr.is_empty() {
            curr = k;
        }
        target = k;
        let (l, r) = v[1..v.len() - 1].split_once(", ").unwrap();
        map.insert(k, (l, r));
    }

    loop {
        for instruction in instructions.iter() {
            let (l, r) = map.get(&curr).unwrap();

            curr = if instruction == &'L' { l } else { r };
            count += 1;

            if curr == target {
                return count;
            }
        }
    }
}

pub fn part_2(_input: &str) -> usize {
    0
}
