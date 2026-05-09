use std::iter;

advent_of_code::solution!(11);

type Value = u64;
struct Puzzle {
    stones: Vec<Value>,
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

fn step_stone(value: &Value) -> Box<dyn Iterator<Item = Value>> {
    if *value == 0 {
        return Box::new(iter::once(1));
    }

    let count = count_digits(*value);
    if count % 2 == 0 {
        let split = count / 2;

        let left = value / 10_u64.pow(split as u32);
        let right = value % 10_u64.pow(split as u32);
        return Box::new(iter::chain(iter::once(left), iter::once(right)));
    }

    return Box::new(iter::once(*value * 2024));
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let stones = input
            .trim_ascii_end()
            .split(' ')
            .map(|v| v.parse().unwrap())
            .collect();
        Self { stones }
    }

    fn step(&mut self) {
        self.stones = self.stones.iter().flat_map(step_stone).collect()
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.stones.iter().for_each(|v| print!("{} ", v));
        println!();
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut puzzle = Puzzle::from_str(input);
    for _ in 0..25 {
        puzzle.step();
        // puzzle.print();
    }
    Some(puzzle.stones.len())
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
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
