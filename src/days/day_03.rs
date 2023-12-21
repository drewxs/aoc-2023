use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    matrix.iter().enumerate().fold(0, |acc, (i, line)| {
        let mut num_str = String::new();

        acc + line.iter().enumerate().fold(0, |nums, (j, c)| {
            if c.is_ascii_digit() {
                num_str.push(*c);
            }

            let mut to_add = 0;

            if !c.is_ascii_digit() || (c.is_ascii_digit() && j == matrix[0].len() - 1) {
                let start_idx = j - num_str.len();
                let end_idx = j;

                if (start_idx..end_idx).any(|k| has_adj_symbol(i, k, &matrix)) {
                    to_add = num_str.parse::<usize>().unwrap_or(0);
                }

                num_str.clear();
            }

            nums + to_add
        })
    })
}

fn has_adj_symbol(i: usize, j: usize, matrix: &Vec<Vec<char>>) -> bool {
    (i > 0 && is_symbol(matrix[i - 1][j]))
        || (i < matrix.len() - 1 && is_symbol(matrix[i + 1][j]))
        || (j > 0 && is_symbol(matrix[i][j - 1]))
        || (j < matrix[i].len() - 1 && is_symbol(matrix[i][j + 1]))
        || (i > 0 && j > 0 && is_symbol(matrix[i - 1][j - 1]))
        || (i > 0 && j < matrix[i].len() - 1 && is_symbol(matrix[i - 1][j + 1]))
        || (i < matrix.len() - 1 && j > 0 && is_symbol(matrix[i + 1][j - 1]))
        || (i < matrix.len() - 1 && j < matrix[i].len() - 1 && is_symbol(matrix[i + 1][j + 1]))
}

fn is_symbol(c: char) -> bool {
    c != '.' && c.is_ascii_punctuation()
}

pub fn part_2(input: &str) -> usize {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    matrix.iter().enumerate().fold(0, |acc, (i, line)| {
        acc + line.iter().enumerate().fold(0, |acc, (j, _)| {
            if matrix[i][j] == '*' {
                return acc + product_of_2_adj_num(i, j, &matrix);
            }
            acc
        })
    })
}

fn product_of_2_adj_num(i: usize, j: usize, matrix: &Vec<Vec<char>>) -> usize {
    let mut set: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut nums = vec![];

    if let Some(num) = get_num(i > 0, i - 1, j, matrix, &mut set) {
        nums.push(num);
    }
    if let Some(num) = get_num(i < matrix.len() - 1, i + 1, j, matrix, &mut set) {
        nums.push(num);
    }
    if let Some(num) = get_num(j > 0, i, j - 1, matrix, &mut set) {
        nums.push(num);
    }
    if let Some(num) = get_num(j < matrix[i].len() - 1, i, j + 1, matrix, &mut set) {
        nums.push(num);
    }
    if let Some(num) = get_num(i > 0 && j > 0, i - 1, j - 1, matrix, &mut set) {
        nums.push(num);
    }
    if let Some(num) = get_num(
        i > 0 && j < matrix[i].len() - 1,
        i - 1,
        j + 1,
        matrix,
        &mut set,
    ) {
        nums.push(num);
    }
    if let Some(num) = get_num(
        i < matrix.len() - 1 && j > 0,
        i + 1,
        j - 1,
        matrix,
        &mut set,
    ) {
        nums.push(num);
    }
    if let Some(num) = get_num(
        i < matrix.len() - 1 && j < matrix[i].len() - 1,
        i + 1,
        j + 1,
        matrix,
        &mut set,
    ) {
        nums.push(num);
    }

    if nums.len() == 2 {
        return nums[0] * nums[1];
    }
    0
}

fn get_num(
    condition: bool,
    i: usize,
    j: usize,
    matrix: &Vec<Vec<char>>,
    set: &mut HashSet<(usize, usize, usize)>,
) -> Option<usize> {
    if !(condition && matrix[i][j].is_ascii_digit()) {
        return None;
    }

    let mut l = j;
    let mut r = j;

    while l > 0 && matrix[i][l - 1].is_ascii_digit() {
        l -= 1;
    }
    while r < matrix[i].len() - 1 && matrix[i][r + 1].is_ascii_digit() {
        r += 1;
    }

    if set.contains(&(i, l, r)) {
        return None;
    }

    set.insert((i, l, r));

    let num = matrix[i][l..=r]
        .iter()
        .collect::<String>()
        .parse::<usize>()
        .unwrap_or(0);

    Some(num)
}
