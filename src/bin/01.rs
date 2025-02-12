use std::collections::HashMap;

advent_of_code::solution!(1);

fn line_to_value_builder<'a>(mapping: &'a HashMap<String, u8>) -> impl Fn(&str) -> u64 + 'a {
    move |line: &str| {
        let mut first: Option<u64> = None;
        let mut last: Option<u64> = None;
        for i in 0..line.len() {
            for (name, value) in mapping {
                if line[i..].starts_with(name) {
                    first = first.or(Some(*value as u64));
                }
                if line[..(line.len() - i)].ends_with(name) {
                    last = last.or(Some(*value as u64));
                }
            }
        }
        10 * first.unwrap() + last.unwrap()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mapping: HashMap<String, u8> = (1..11).collect::<Vec<u8>>().iter().map(|&i| (i.to_string(), i)).collect();
    Some(input.lines().map(line_to_value_builder(&mapping)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut digit_strs: Vec<String> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
        .iter().map(|&s| s.to_string()).collect();
    digit_strs.extend((1..10).map(|n| n.to_string()));
    let mapping: HashMap<String, u8> = digit_strs
        .into_iter().enumerate().map(|(i, s)| (s, (i % 9 + 1) as u8)).collect();
    Some(input.lines().map(line_to_value_builder(&mapping)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
