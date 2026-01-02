use std::iter::zip;

advent_of_code::solution!(6);

fn get_floor_sqrt(n: u64) -> u64 {
    let mut lo = 1u64;
    while 4 * lo * lo <= n {
        lo *= 2;
    }
    let mut hi = 2 * lo;
    while hi - lo >= 2 {
        let mid = (hi + lo) / 2;
        if mid * mid <= n {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

fn get_number_of_ways(t: u64, d: u64) -> u64 {
    let sqrt_floor = get_floor_sqrt(t * t - 4 * d);
    let lower_limit = (t - sqrt_floor + 1) / 2;
    let upper_limit = (t + sqrt_floor) / 2;
    if lower_limit * (t - lower_limit) == d {
        // tie in this case -- do not include endpoints
        upper_limit - lower_limit - 1
    } else {
        upper_limit - lower_limit + 1
    }
}

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut parts = input.lines();
    let times = parts
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .filter_map(|x| x.parse::<u64>().ok())
        .collect();
    let distances = parts
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .filter_map(|x| x.parse::<u64>().ok())
        .collect();
    (times, distances)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (times, distances) = parse(input);
    Some(
        zip(times, distances)
            .map(|(t, d)| get_number_of_ways(t, d))
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (times, distances) = parse(input);
    let total_time: u64 = times
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse()
        .ok()?;
    let total_distance: u64 = distances
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse()
        .ok()?;
    Some(get_number_of_ways(total_time, total_distance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
