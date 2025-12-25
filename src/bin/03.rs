use std::collections::HashMap;

advent_of_code::solution!(3);

fn is_symbol(ch: char) -> bool {
    !(ch.is_digit(10) || ch == '.')
}

fn adjacent_positions_matching<F>(
    matrix: &[Vec<char>],
    r: usize,
    c0: usize,
    c: usize,
    predicate: F,
) -> impl Iterator<Item = (usize, usize, char)> + '_
where
    F: Fn(char) -> bool,
{
    let height = matrix.len();
    let width = matrix[0].len();
    let min_c = c0.saturating_sub(1);
    let max_c = (c + 1).min(width);

    let mut positions = Vec::new();

    if c0 > 0 {
        let ch = matrix[r][c0 - 1];
        if predicate(ch) {
            positions.push((r, c0 - 1, ch));
        }
    }
    if c < width {
        let ch = matrix[r][c];
        if predicate(ch) {
            positions.push((r, c, ch));
        }
    }
    if r > 0 {
        for col in min_c..max_c {
            let ch = matrix[r - 1][col];
            if predicate(ch) {
                positions.push((r - 1, col, ch));
            }
        }
    }
    if r < height - 1 {
        for col in min_c..max_c {
            let ch = matrix[r + 1][col];
            if predicate(ch) {
                positions.push((r + 1, col, ch));
            }
        }
    }

    positions.into_iter()
}

fn is_symbol_adjacent(matrix: &[Vec<char>], r: usize, c0: usize, c: usize) -> bool {
    adjacent_positions_matching(matrix, r, c0, c, is_symbol)
        .next()
        .is_some()
}

pub fn part_one(input: &str) -> Option<u64> {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = matrix[0].len();
    let mut sum = 0;
    for (r, row) in matrix.iter().enumerate() {
        let mut c = 0;
        let mut c0 = 0;
        let mut curr_num = 0;
        while c <= width {
            let potential_digit = if c < width { row[c].to_digit(10) } else { None };
            if let Some(digit) = potential_digit {
                if curr_num == 0 {
                    c0 = c;
                }
                curr_num = 10 * curr_num + digit as u64;
            } else {
                if curr_num > 0 && is_symbol_adjacent(&matrix, r, c0, c) {
                    sum += curr_num;
                }
                curr_num = 0;
            }
            c += 1;
        }
    }
    Some(sum)
}

fn update_star_adjacent(
    matrix: &[Vec<char>],
    r: usize,
    c0: usize,
    c: usize,
    curr_num: u64,
    star_adjacent: &mut HashMap<usize, Vec<u64>>,
) {
    let width = matrix[0].len();
    for (row, col, _) in adjacent_positions_matching(matrix, r, c0, c, |ch| ch == '*') {
        star_adjacent
            .entry(row * width + col)
            .or_default()
            .push(curr_num);
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = matrix[0].len();
    let mut star_adjacent: HashMap<usize, Vec<u64>> = HashMap::new();
    for (r, row) in matrix.iter().enumerate() {
        let mut c = 0;
        let mut c0 = 0;
        let mut curr_num = 0;
        while c <= width {
            let potential_digit = if c < width { row[c].to_digit(10) } else { None };
            if let Some(digit) = potential_digit {
                if curr_num == 0 {
                    c0 = c;
                }
                curr_num = 10 * curr_num + digit as u64;
            } else {
                if curr_num > 0 {
                    update_star_adjacent(&matrix, r, c0, c, curr_num, &mut star_adjacent);
                }
                curr_num = 0;
            }
            c += 1;
        }
    }

    let mut sum = 0;
    for v in star_adjacent.values() {
        if let [a, b] = v[..] {
            sum += a * b;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
