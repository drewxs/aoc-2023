use std::collections::HashMap;

pub fn part_1(input: &str) -> usize {
    let instructions: Vec<char> = input.lines().next().unwrap().chars().collect();

    let mut count = 0;
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut iter = input.lines().skip(2);
    let mut curr = "";
    let mut target = "";

    while let Some(line) = iter.next() {
        dbg!("init loop");
        let split_kv = line.split(" = ").collect::<Vec<&str>>();
        let key = split_kv[0];
        if curr == "" {
            curr = key;
        }
        target = key;
        let split_v: Vec<&str> = split_kv[1][1..split_kv[1].len() - 1].split(", ").collect();
        map.insert(key, (&split_v[0], &split_v[1]));
    }

    'found: loop {
        dbg!("found loop");
        for instruction in instructions.iter() {
            let (l, r) = map.get(curr).unwrap();
            if instruction == &'L' {
                curr = l;
            } else {
                curr = r;
            }

            count += 1;

            if curr == target {
                break 'found;
            }
        }
    }

    count
}

pub fn part_2(_input: &str) -> usize {
    0
}
