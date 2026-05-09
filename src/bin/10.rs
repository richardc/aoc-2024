advent_of_code::solution!(10);
use pathfinding::{matrix::Matrix, prelude::dfs_reach};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Location {
    row: usize,
    col: usize,
    val: u8,
}

impl Location {
    fn from(row: usize, col: usize, val: u8) -> Self {
        Self { row, col, val }
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

    fn trailheads(&self) -> Vec<Location> {
        self.map
            .items()
            .filter_map(|((r, c), &v)| {
                if v == b'0' {
                    Some(Location::from(r, c, v))
                } else {
                    None
                }
            })
            .collect()
    }

    fn neighbours(&self, node: &Location) -> Vec<Location> {
        self.map
            .neighbours((node.row, node.col), false)
            .map(|(row, col)| Location::from(row, col, *self.map.get((row, col)).unwrap()))
            .filter(|neighbour| neighbour.val == node.val + 1)
            .collect()
    }

    fn score_trailhead(&self, start: &Location) -> usize {
        dfs_reach(*start, |node| self.neighbours(node))
            .filter(|node| node.val == b'9')
            .count()
    }

    fn score_trailheads(&self) -> usize {
        self.trailheads()
            .iter()
            .map(|t| self.score_trailhead(t))
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.score_trailheads())
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
