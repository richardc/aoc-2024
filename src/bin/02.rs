advent_of_code::solution!(2);
use itertools::Itertools;

fn none_repeating(record: &[u32]) -> bool {
    record
        .into_iter()
        .tuple_windows()
        .find(|(a, b)| a == b)
        .is_none()
}

fn is_ascending(record: &[u32]) -> bool {
    record.into_iter().tuple_windows().all(|(a, b)| a > b)
}

fn is_descending(record: &[u32]) -> bool {
    record.into_iter().tuple_windows().all(|(a, b)| a < b)
}

fn diff_at_most_3(record: &[u32]) -> bool {
    record
        .into_iter()
        .tuple_windows()
        .all(|(a, b)| a.abs_diff(*b) <= 3)
}

fn is_safe(record: &[u32]) -> bool {
    none_repeating(record)
        && (is_ascending(record) || is_descending(record))
        && diff_at_most_3(record)
}

pub fn part_one(input: &str) -> Option<usize> {
    let records = input.lines().map(|l| {
        l.split_whitespace()
            .map(|w| w.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    });
    Some(records.filter(|r| is_safe(r)).count())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
