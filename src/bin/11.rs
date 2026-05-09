advent_of_code::solution!(11);

use std::collections::HashMap;

type Value = u64;
struct Puzzle {
    stones: HashMap<Value, Value>,
}

fn count_digits(v: Value) -> Value {
    match v {
        0..10 => 1,
        10..100 => 2,
        100..1000 => 3,
        1000..10000 => 4,
        10000..100000 => 5,
        100000..1000000 => 6,
        1000000..10000000 => 7,
        10000000..100000000 => 8,
        _ => (v.ilog10() + 1) as Value,
    }
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let mut stones = HashMap::new();
        for stone in input
            .trim_ascii_end()
            .split(' ')
            .map(|v| v.parse().unwrap())
        {
            *stones.entry(stone).or_insert(0) += 1;
        }
        Self { stones }
    }

    fn step(&mut self) {
        let mut next = HashMap::new();
        for (&value, &count) in &self.stones {
            if value == 0 {
                *next.entry(1).or_insert(0) += count;
                continue;
            }

            let digits = count_digits(value);
            if digits.is_multiple_of(2) {
                let split = digits / 2;
                let left = value / 10_u64.pow(split as u32);
                let right = value % 10_u64.pow(split as u32);
                *next.entry(left).or_insert(0) += count;
                *next.entry(right).or_insert(0) += count;
                continue;
            }

            *next.entry(value * 2024).or_insert(0) += count;
        }
        self.stones = next;
    }

    fn count_stones(&self) -> Value {
        self.stones.values().sum()
    }
}

fn solve(count: usize, input: &str) -> Value {
    let mut puzzle = Puzzle::from_str(input);
    for _ in 0..count {
        puzzle.step();
    }
    puzzle.count_stones()
}

pub fn part_one(input: &str) -> Option<Value> {
    Some(solve(25, input))
}

pub fn part_two(input: &str) -> Option<Value> {
    Some(solve(75, input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
