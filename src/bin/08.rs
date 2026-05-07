advent_of_code::solution!(8);

use itertools::Itertools;
use pathfinding::matrix::Matrix;

use std::collections::HashMap;
use std::collections::HashSet;

struct Puzzle {
    map: Matrix<u8>,
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let map = Matrix::from_rows(input.lines().map(|l| l.bytes())).unwrap();
        Self { map }
    }

    fn find_antennas(&self) -> HashMap<u8, Vec<(i32, i32)>> {
        self.map
            .items()
            .filter(|((_, _), &c)| c.is_ascii_alphanumeric())
            .map(|((r, c), &v)| (v, (r as i32, c as i32)))
            .into_group_map()
    }

    fn antinodes_of(&self, start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)> {
        // put the top one first
        let (start, end) = if start.0 < end.0 || start.1 < end.1 {
            (start, end)
        } else {
            (end, start)
        };

        // determine the vector
        let vector = (end.0 - start.0, end.1 - start.1);

        // antinode back past the start
        let start_antinode = (start.0 - vector.0, start.1 - vector.0);
        let end_antinode = (end.0 + vector.1, end.1 + vector.1);
        vec![start_antinode, end_antinode]
    }

    fn in_bounds(&self, (row, col): (i32, i32)) -> bool {
        if row < 0 || col < 0 {
            false
        } else {
            self.map.within_bounds((row as usize, col as usize))
        }
    }

    fn find_antinodes(&self) -> HashSet<(usize, usize)> {
        let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
        let groups = self.find_antennas();
        for (_group, nodes) in groups {
            for (&a, &b) in nodes.iter().circular_tuple_windows() {
                for antinode in self.antinodes_of(a, b) {
                    if self.in_bounds(antinode) {
                        antinodes.insert((antinode.0 as usize, antinode.1 as usize));
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
        assert_eq!(result, HashMap::from([(b'a', vec![(3, 4), (5, 5)])]));
    }

    #[test]
    fn test_find_antinodes_01() {
        let puzzle = Puzzle::from_str(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        let result = puzzle.find_antinodes();
        assert_eq!(result, HashSet::from([(1, 3), (7, 6)]));
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
            HashMap::from([(b'a', vec![(3, 4), (4, 8), (5, 5)])])
        );
    }

    #[test]
    fn test_find_antinodes_02() {
        let puzzle = Puzzle::from_str(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        let result = puzzle.find_antinodes();
        assert_eq!(result, HashSet::from([(1, 3), (2, 0), (6, 2), (7, 6)]));
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
