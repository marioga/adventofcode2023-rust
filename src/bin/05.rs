use std::cmp;

advent_of_code::solution!(5);

#[derive(Clone, Copy, Debug)]
struct Range {
    start: u64,
    length: u64,
    is_mapped: bool,
}

#[derive(Debug)]
struct RangeCollection {
    ranges: Vec<Range>,
}

// Implement standard traits for ergonomics
impl From<Vec<Range>> for RangeCollection {
    fn from(ranges: Vec<Range>) -> Self {
        Self { ranges }
    }
}

impl FromIterator<Range> for RangeCollection {
    fn from_iter<T: IntoIterator<Item = Range>>(iter: T) -> Self {
        Self {
            ranges: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug)]
struct RangeMap {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

impl RangeMap {
    fn map_range(&self, range: Range) -> RangeCollection {
        if range.is_mapped {
            return RangeCollection::from(vec![range]);
        }

        let mut dest_ranges: Vec<Range> = Vec::new();
        if range.start < self.source_start {
            dest_ranges.push(Range {
                start: range.start,
                length: cmp::min(range.length, self.source_start - range.start),
                is_mapped: false,
            });
        }
        if self.source_start < range.start + range.length
            && range.start < self.source_start + self.length
        {
            /*
            Intersect: Range {
                start: cmp::max(self.source_start, range.start),
                length: cmp::min(self.source_start + self.length, range.start + range.length)
                    - cmp::max(self.source_start, range.start),
            }
            */
            dest_ranges.push(Range {
                start: cmp::max(self.source_start, range.start) - self.source_start
                    + self.dest_start,
                length: cmp::min(self.source_start + self.length, range.start + range.length)
                    - cmp::max(self.source_start, range.start),
                is_mapped: true,
            });
        }
        if range.start + range.length > self.source_start + self.length {
            dest_ranges.push(Range {
                start: cmp::max(self.source_start + self.length, range.start),
                length: range.start + range.length
                    - cmp::max(self.source_start + self.length, range.start),
                is_mapped: false,
            });
        }
        RangeCollection::from(dest_ranges)
    }

    pub fn map_collection(&self, range_coll: RangeCollection) -> RangeCollection {
        range_coll
            .ranges
            .into_iter()
            .flat_map(|r| self.map_range(r).ranges)
            .collect()
    }
}

fn parse_seeds(line: &str, as_ranges: bool) -> Option<RangeCollection> {
    let seeds_str = line.strip_prefix("seeds: ")?;
    let nums: Vec<u64> = seeds_str
        .split_whitespace()
        .filter_map(|num| num.parse::<u64>().ok())
        .collect();

    let ranges: Vec<Range> = if as_ranges {
        nums.chunks_exact(2)
            .map(|chunk| Range {
                start: chunk[0],
                length: chunk[1],
                is_mapped: false,
            })
            .collect()
    } else {
        nums.iter()
            .map(|&start| Range {
                start,
                length: 1,
                is_mapped: false,
            })
            .collect()
    };

    Some(RangeCollection::from(ranges))
}

fn get_target_min(input: &str, as_ranges: bool) -> Option<u64> {
    let mut sections = input.split("\n\n");

    // Parse seeds from first section
    let mut result = parse_seeds(sections.next()?, as_ranges)?;

    // Process each map section
    for section in sections {
        // Reset is_mapped flag for new layer
        result.ranges.iter_mut().for_each(|r| r.is_mapped = false);

        // Skip the header line, parse the map lines
        for line in section.lines().skip(1) {
            let nums: Vec<u64> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            let [dest_start, source_start, length] = nums.as_slice() else {
                panic!("Expected exactly 3 numbers");
            };

            result = RangeMap {
                dest_start: *dest_start,
                source_start: *source_start,
                length: *length,
            }
            .map_collection(result);
        }
    }

    result.ranges.iter().map(|r| r.start).min()
}

pub fn part_one(input: &str) -> Option<u64> {
    get_target_min(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    get_target_min(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
