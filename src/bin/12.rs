advent_of_code::solution!(12);

use pathfinding::matrix::Matrix;
use std::collections::{BTreeSet, HashSet};

struct Puzzle {
    map: Matrix<u8>,
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let map = Matrix::from_rows(input.lines().map(|l| l.bytes())).unwrap();
        Self { map }
    }

    fn regions(&self) -> Vec<BTreeSet<(usize, usize)>> {
        let mut seen = BTreeSet::new();
        let mut regions = Vec::new();

        for start in self.map.keys() {
            if seen.contains(&start) {
                continue;
            }
            let value = self.map.get(start);
            let mut region = self
                .map
                .bfs_reachable(start, false, |pos| self.map.get(pos) == value);
            regions.push(region.clone());
            seen.append(&mut region);
        }
        regions
    }

    fn perimeter(&self, region: &BTreeSet<(usize, usize)>) -> usize {
        let shared_edges: usize = region
            .iter()
            .map(|pos| {
                self.map
                    .neighbours(*pos, false)
                    .filter(|n| region.contains(n))
                    .count()
            })
            .sum();
        (region.len() * 4) - shared_edges
    }

    fn fencing_prices(&self) -> usize {
        let regions = self.regions();
        regions.iter().map(|r| r.len() * self.perimeter(r)).sum()
    }

    fn corners(&self, region: &BTreeSet<(usize, usize)>) -> usize {
        let mut corners = 0;
        // make it all signed so we can subtract and end up at -1
        let points = region
            .iter()
            .map(|&(row, col)| (row as isize, col as isize))
            .collect::<HashSet<_>>();

        for point in &points {
            for adj in [(-1, -1), (1, 1), (-1, 1), (1, -1)] {
                let top = (point.0 + adj.0, point.1);
                let right = (point.0, point.1 + adj.1);
                let diagonal = (point.0 + adj.0, point.1 + adj.1);
                if points.contains(&top) && points.contains(&right) && !points.contains(&diagonal) {
                    // inside corner
                    corners += 1;
                }
                if !points.contains(&top) && !points.contains(&right) {
                    // outside corner
                    corners += 1;
                }
            }
        }
        corners
    }

    fn bulk_prices(&self) -> usize {
        let regions = self.regions();
        regions.iter().map(|r| r.len() * self.corners(r)).sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.fencing_prices())
}

pub fn part_two(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.bulk_prices())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_01() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_one_02() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two_0() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(368));
    }
}
