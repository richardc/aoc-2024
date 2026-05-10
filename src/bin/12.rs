advent_of_code::solution!(12);

use pathfinding::matrix::Matrix;
use std::collections::BTreeSet;

struct Puzzle {
    map: Matrix<u8>,
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let map = Matrix::from_rows(input.lines().map(|l| l.bytes())).unwrap();
        Self { map }
    }

    fn neighbours(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let value = self.map.get(pos);
        self.map
            .neighbours(pos, false)
            .filter(|npos| self.map.get(*npos) == value)
            .collect()
    }

    fn regions(&self) -> Vec<BTreeSet<(usize, usize)>> {
        let mut seen = BTreeSet::new();
        let mut regions = Vec::new();

        for start in self.map.keys() {
            if seen.contains(&start) {
                continue;
            }
            let value = self.map.get(start);
            let region = self
                .map
                .bfs_reachable(start, false, |pos| self.map.get(pos) == value);
            regions.push(region.clone());
            seen.append(&mut region.clone());
        }
        regions
    }

    fn perimeter(&self, region: &BTreeSet<(usize, usize)>) -> usize {
        let shared_edges: usize = region.iter().map(|pos| self.neighbours(*pos).len()).sum();
        (region.len() * 4) - shared_edges
    }

    fn fencing_prices(&self) -> usize {
        let regions = self.regions();
        regions.iter().map(|r| r.len() * self.perimeter(r)).sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.fencing_prices())
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
