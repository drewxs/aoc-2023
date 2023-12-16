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
