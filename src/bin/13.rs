advent_of_code::solution!(13);

#[derive(Debug, Default)]
struct Machine {
    a: (u32, u32),
    b: (u32, u32),
    prize: (u32, u32),
}

impl Machine {
    fn from_str(input: &str) -> Self {
        let mut current = Self::default();
        for line in input.lines() {
            if line.starts_with("Button A:") {
                let (x, y) = line
                    .strip_prefix("Button A: X+")
                    .unwrap()
                    .split_once(", Y+")
                    .unwrap();
                current.a.0 = x.parse().unwrap();
                current.a.1 = y.parse().unwrap();
            }
            if line.starts_with("Button B:") {
                let (x, y) = line
                    .strip_prefix("Button B: X+")
                    .unwrap()
                    .split_once(", Y+")
                    .unwrap();
                current.b.0 = x.parse().unwrap();
                current.b.1 = y.parse().unwrap();
            }
            if line.starts_with("Prize:") {
                let (x, y) = line
                    .strip_prefix("Prize: X=")
                    .unwrap()
                    .split_once(", Y=")
                    .unwrap();
                current.prize.0 = x.parse().unwrap();
                current.prize.1 = y.parse().unwrap();
            }
        }
        current
    }

    fn fewest_tokens(&self) -> Option<usize> {
        None
    }
}

#[derive(Debug)]
struct Puzzle {
    machines: Vec<Machine>,
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let machines = input.split("\n\n").map(|m| Machine::from_str(m)).collect();
        Self { machines }
    }

    fn fewest_tokens(&self) -> usize {
        self.machines.iter().filter_map(|m| m.fewest_tokens()).sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.fewest_tokens())
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
