advent_of_code::solution!(1);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let pairs: Vec<(u32, u32)> = input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once("   ").unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();
    let left = pairs.clone().into_iter().map(|(l, _)| l).sorted();
    let right = pairs.into_iter().map(|(_, r)| r).sorted();
    Some(left.zip(right).map(|(l, r)| l.abs_diff(r)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs: Vec<(u32, u32)> = input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once("   ").unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();
    let left = pairs.clone().into_iter().map(|(l, _)| l);
    let right = pairs.into_iter().map(|(_, r)| r).counts();
    Some(
        left.map(|l| right.get(&l).map_or(0, |r| l * *r as u32))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
