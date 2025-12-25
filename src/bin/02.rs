use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let max_counts: HashMap<&str, u64> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let sum = input
        .lines()
        .map(|line| {
            let (game, shows) = line.trim().split_once(": ").unwrap();
            let game: u64 = game["Game ".len()..].parse::<u64>().unwrap();
            let mut shows = shows.split("; ");
            if shows.all(|show| {
                let mut groups = show.split(", ");
                groups.all(|group| {
                    let (count, color) = group.split_once(" ").unwrap();
                    let count = count.parse::<u64>().unwrap();
                    *max_counts.get(color).unwrap_or(&count) >= count
                })
            }) {
                game
            } else {
                0
            }
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .map(|line| {
            let (_, shows) = line.trim().split_once(": ").unwrap();
            let shows = shows.split("; ");
            let mut min_counts: HashMap<&str, u64> =
                HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            for show in shows {
                let groups = show.split(", ");
                for group in groups {
                    let (count, color) = group.split_once(" ").unwrap();
                    let count = count.parse::<u64>().unwrap();
                    if *min_counts.get(color).unwrap_or(&count) < count {
                        min_counts.insert(color, count);
                    }
                }
            }
            min_counts.values().product::<u64>()
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
