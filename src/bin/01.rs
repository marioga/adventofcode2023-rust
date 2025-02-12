use std::collections::HashMap;

advent_of_code::solution!(1);

fn line_to_value_builder<'a>(mapping: &'a HashMap<String, u8>) -> impl Fn(&str) -> u64 + 'a {
    move |line: &str| {
        let indices = mapping
            .iter()
            .flat_map(|(&ref s, digit)| line.match_indices(s).map(move |(idx, _)| (idx, digit)));
        let mut first: Option<(usize, &u8)> = None;
        let mut last: Option<(usize, &u8)> = None;
        for (idx, digit) in indices {
            match first {
                None => first = Some((idx, digit)),
                Some((first_idx, _)) if idx < first_idx => first = Some((idx, digit)),
                _ => {}
            }

            match last {
                None => last = Some((idx, digit)),
                Some((last_idx, _)) if idx > last_idx => last = Some((idx, digit)),
                _ => {}
            }

        }
        (10 * first.unwrap().1 + last.unwrap().1).into()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mapping: HashMap<String, u8> = (1..10)
        .collect::<Vec<u8>>()
        .iter()
        .map(|&i| (i.to_string(), i))
        .collect();
    Some(input.lines().map(line_to_value_builder(&mapping)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut digit_strs: Vec<String> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();
    digit_strs.extend((1..10).map(|n| n.to_string()));
    let mapping: HashMap<String, u8> = digit_strs
        .into_iter()
        .enumerate()
        .map(|(i, s)| (s, (i % 9 + 1) as u8))
        .collect();
    Some(input.lines().map(line_to_value_builder(&mapping)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
