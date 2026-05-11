advent_of_code::solution!(13);

#[derive(Debug, Default)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
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

    fn fewest_tokens(&self) -> Option<i64> {
        let b = (self.a.0 * self.prize.1 - self.a.1 * self.prize.0)
            / (self.a.0 * self.b.1 - self.a.1 * self.b.0);
        let a = (self.prize.0 - b * self.b.0) / self.a.0;

        if ((a * self.a.0 + b * self.b.0) == self.prize.0)
            && ((a * self.a.1 + b * self.b.1) == self.prize.1)
        {
            Some(a * 3 + b)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    machines: Vec<Machine>,
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let machines = input.split("\n\n").map(Machine::from_str).collect();
        Self { machines }
    }

    fn move_targets(&mut self, distance: i64) {
        for machine in self.machines.iter_mut() {
            machine.prize.0 += distance;
            machine.prize.1 += distance;
        }
    }

    fn fewest_tokens(&self) -> i64 {
        self.machines.iter().filter_map(|m| m.fewest_tokens()).sum()
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.fewest_tokens())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut puzzle = Puzzle::from_str(input);
    puzzle.move_targets(10_000_000_000_000);
    Some(puzzle.fewest_tokens())
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
        assert_eq!(result, Some(875318608908));
    }
}
