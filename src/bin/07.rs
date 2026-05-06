advent_of_code::solution!(7);

use itertools::Itertools;
type Value = u64;

#[allow(dead_code)] // Actually used in has_solution_3
#[derive(Clone)]
enum Operation {
    Multiply,
    Add,
    Concat,
}

#[derive(Debug)]
struct Equation {
    target: Value,
    values: Vec<Value>,
}

impl Equation {
    fn from_str(input: &str) -> Self {
        let (target, values) = input.split_once(": ").unwrap();
        let target = target.parse::<Value>().unwrap();
        let values = values
            .split(" ")
            .map(|v| v.parse::<Value>().unwrap())
            .collect();
        Self { target, values }
    }

    fn check_solution(&self, id: u32) -> bool {
        let mut sum: Value = *self.values.first().unwrap();
        for i in 1..self.values.len() {
            if id & (1 << i) != 0 {
                sum += self.values[i];
            } else {
                sum *= self.values[i];
            }
        }
        sum == self.target
    }

    fn has_solution(&self) -> bool {
        for i in 0..(2_u32.pow(self.values.len() as u32)) {
            if self.check_solution(i) {
                return true;
            }
        }
        false
    }

    fn has_solution_3(&self) -> bool {
        for operations in (1..(self.values.len()))
            .map(|_| [Operation::Add, Operation::Multiply, Operation::Concat])
            .multi_cartesian_product()
        {
            let mut sum: Value = *self.values.first().unwrap();
            for i in 1..self.values.len() {
                match operations[i - 1] {
                    Operation::Add => sum += self.values[i],
                    Operation::Multiply => sum *= self.values[i],
                    Operation::Concat => match self.values[i] {
                        0..10 => {
                            sum *= 10;
                            sum += self.values[i];
                        }
                        10..100 => {
                            sum *= 100;
                            sum += self.values[i];
                        }
                        100..1000 => {
                            sum *= 1000;
                            sum += self.values[i];
                        }
                        1000..10000 => {
                            sum *= 10000;
                            sum += self.values[i];
                        }
                        10000..100000 => {
                            sum *= 100000;
                            sum += self.values[i];
                        }
                        _ => {
                            let digits = self.values[i].ilog10() + 1;
                            sum *= 10_u64.pow(digits);
                            sum += self.values[i];
                        }
                    },
                }
            }
            if sum == self.target {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
struct Puzzle {
    equations: Vec<Equation>,
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let equations = input.lines().map(Equation::from_str).collect();
        Self { equations }
    }

    fn part_one(&self) -> Value {
        self.equations
            .iter()
            .filter_map(|e| {
                if e.has_solution() {
                    Some(e.target)
                } else {
                    None
                }
            })
            .sum()
    }

    fn part_two(&self) -> Value {
        self.equations
            .iter()
            .filter_map(|e| {
                if e.has_solution_3() {
                    Some(e.target)
                } else {
                    None
                }
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<Value> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.part_one())
}

pub fn part_two(input: &str) -> Option<Value> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.part_two())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
