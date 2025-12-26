use std::collections::HashMap;
use std::collections::HashSet;

advent_of_code::solution!(4);

fn parse_numbers<T: FromIterator<u32>>(s: &str) -> T {
    s.split_whitespace()
        .filter_map(|num| num.parse::<u32>().ok())
        .collect()
}

fn parse(line: &str) -> (usize, HashSet<u32>, HashSet<u32>) {
    let (card_part, rest) = line.trim().split_once(": ").unwrap();
    let card = card_part
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let (winning, owning) = rest.split_once(" | ").unwrap();
    let winning: HashSet<u32> = parse_numbers(winning);
    let owning: HashSet<u32> = parse_numbers(owning);
    (card, winning, owning)
}

fn num_winners(winning: &HashSet<u32>, owning: &HashSet<u32>) -> usize {
    owning.iter().filter(|&val| winning.contains(val)).count()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let (_, winning, owning) = parse(line);
                match num_winners(&winning, &owning) {
                    0 => 0,
                    n => 1u64 << (n - 1),
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (total, _) = input.lines().fold(
        (0u64, HashMap::new()),
        |(total, mut copies): (u64, HashMap<usize, u64>), line| {
            let (card, winning, owning) = parse(line);
            let total_copies = 1 + copies.get(&card).unwrap_or(&0);
            for idx in 0..num_winners(&winning, &owning) {
                *copies.entry(card + 1 + idx as usize).or_insert(0) += total_copies;
            }
            (total + total_copies, copies)
        },
    );
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
