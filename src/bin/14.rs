advent_of_code::solution!(14);

use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

type Value = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    x: Value,
    y: Value,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Vector {
    fn scale(self, rhs: Value) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Vector {
    fn from_str(input: &str) -> Self {
        let (x, y) = input.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        Self { x, y }
    }

    fn wrap(&self, width: Value, height: Value) -> Self {
        Self {
            x: self.x.rem_euclid(width),
            y: self.y.rem_euclid(height),
        }
    }
}

struct Robot {
    position: Vector,
    vector: Vector,
}

impl Robot {
    fn from_str(input: &str) -> Self {
        let (position, vector) = input.strip_prefix("p=").unwrap().split_once(" v=").unwrap();
        let position = Vector::from_str(position);
        let vector = Vector::from_str(vector);
        Self { position, vector }
    }
}

struct Puzzle {
    robots: Vec<Robot>,
    width: Value,
    height: Value,
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let robots: Vec<_> = input.lines().map(Robot::from_str).collect();
        // a 'heuristic' to see if we're running our input data or a test
        let width = if robots.len() > 100 { 101 } else { 11 };
        let height = if robots.len() > 100 { 103 } else { 7 };
        Self {
            robots,
            width,
            height,
        }
    }

    fn safety_factor(&self) -> Value {
        let mid_width = self.width / 2;
        let mid_height = self.height / 2;
        let mut quadrants: HashMap<(bool, bool), Value> = HashMap::new();
        for location in self
            .robots
            .iter()
            .map(|r| (r.position + r.vector.scale(100)).wrap(self.width, self.height))
        {
            // skip bots that are in the deadzone
            if location.x == mid_width || location.y == mid_height {
                continue;
            }
            let quadrant = (location.x < mid_width, location.y < mid_height);
            *quadrants.entry(quadrant).or_insert(0) += 1;
        }

        quadrants.values().product()
    }

    fn has_vertical_group(&self, points: &HashSet<Vector>) -> bool {
        let mut count = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if points.contains(&Vector { x, y }) {
                    count += 1;
                } else {
                    count = 0
                }
                if count >= 10 {
                    return true;
                }
            }
        }
        false
    }

    #[allow(dead_code)]
    fn draw(&self, points: &HashSet<Vector>) {
        for y in 0..self.height {
            for x in 0..self.width {
                if points.contains(&Vector { x, y }) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn find_tree(&self) -> Option<Value> {
        for time in 1.. {
            let points = self
                .robots
                .iter()
                .map(|r| (r.position + r.vector.scale(time)).wrap(self.width, self.height))
                .collect::<HashSet<Vector>>();

            if self.has_vertical_group(&points) {
                //self.draw(&points);
                return Some(time);
            }
        }
        None
    }
}

pub fn part_one(input: &str) -> Option<Value> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.safety_factor())
}

pub fn part_two(input: &str) -> Option<Value> {
    let puzzle = Puzzle::from_str(input);
    puzzle.find_tree()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
