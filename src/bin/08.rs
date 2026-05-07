advent_of_code::solution!(8);

use itertools::Itertools;
use pathfinding::matrix::Matrix;

use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy, PartialOrd)]
struct Vector {
    row: i32,
    col: i32,
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl Vector {
    fn from(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

struct Puzzle {
    map: Matrix<u8>,
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let map = Matrix::from_rows(input.lines().map(|l| l.bytes())).unwrap();
        Self { map }
    }

    fn find_antennas(&self) -> HashMap<u8, Vec<Vector>> {
        self.map
            .items()
            .filter(|((_, _), &c)| c.is_ascii_alphanumeric())
            .map(|((r, c), &v)| (v, Vector::from(r as i32, c as i32)))
            .into_group_map()
    }

    fn antinodes_of(&self, start: Vector, end: Vector) -> Vec<Vector> {
        // put the top one first
        let (start, end) = if start < end {
            (start, end)
        } else {
            (end, start)
        };

        // determine the vector
        let vector = end - start;

        // antinodes
        // -- back past the start
        let start_antinode = start - vector;
        // -- out past the end
        let end_antinode = end + vector;
        vec![start_antinode, end_antinode]
    }

    fn in_bounds(&self, v: Vector) -> bool {
        if v.row < 0 || v.col < 0 {
            false
        } else {
            self.map.within_bounds((v.row as usize, v.col as usize))
        }
    }

    fn find_antinodes(&self) -> HashSet<Vector> {
        let mut antinodes: HashSet<Vector> = HashSet::new();
        let groups = self.find_antennas();
        for (_group, nodes) in groups {
            for pairs in nodes.iter().combinations(2) {
                let a = *pairs[0];
                let b = *pairs[1];
                for antinode in self.antinodes_of(a, b) {
                    if self.in_bounds(antinode) {
                        antinodes.insert(antinode);
                    }
                }
            }
        }

        antinodes
    }

    fn part_one(&self) -> usize {
        self.find_antinodes().len()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.part_one())
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_01() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_find_antennas_01() {
        let puzzle = Puzzle::from_str(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        let result = puzzle.find_antennas();
        assert_eq!(
            result,
            HashMap::from([(b'a', vec![Vector::from(3, 4), Vector::from(5, 5)])])
        );
    }

    #[test]
    fn test_find_antinodes_01() {
        let puzzle = Puzzle::from_str(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        let result = puzzle.find_antinodes();
        assert_eq!(
            result,
            HashSet::from([Vector::from(1, 3), Vector::from(7, 6)])
        );
    }

    #[test]
    fn test_part_one_02() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_find_antennas_02() {
        let puzzle = Puzzle::from_str(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        let result = puzzle.find_antennas();
        assert_eq!(
            result,
            HashMap::from([(
                b'a',
                vec![Vector::from(3, 4), Vector::from(4, 8), Vector::from(5, 5)]
            )])
        );
    }

    #[test]
    fn test_find_antinodes_02() {
        let puzzle = Puzzle::from_str(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        let result = puzzle.find_antinodes();
        assert_eq!(
            result,
            HashSet::from([
                Vector::from(1, 3),
                Vector::from(2, 0),
                Vector::from(6, 2),
                Vector::from(7, 6)
            ])
        );
    }

    use rstest::rstest;
    #[rstest]
    #[case((Vector::from(3,4), Vector::from(5,5)), vec![Vector::from(1,3), Vector::from(7,6)])]
    fn test_antinodes_of(#[case] (start, end): (Vector, Vector), #[case] expected: Vec<Vector>) {
        let puzzle = Puzzle::from_str(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        let result = puzzle.antinodes_of(start, end);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
