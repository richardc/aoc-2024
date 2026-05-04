advent_of_code::solution!(6);
use pathfinding::matrix::directions;
use pathfinding::matrix::Matrix;
use std::collections::HashSet;

#[derive(Debug)]
struct Puzzle {
    maze: Matrix<u8>,
    row: usize,
    col: usize,
    facing: usize,
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let maze = Matrix::from_rows(input.lines().map(|l| l.bytes())).unwrap();
        let ((row, col), _) = maze.items().find(|((_, _), v)| **v == b'^').unwrap();
        let facing = directions::DIRECTIONS_4
            .iter()
            .position(|&d| d == directions::N)
            .expect("N is a direction");
        Self {
            maze,
            row,
            col,
            facing,
        }
    }

    fn step(&mut self) -> Option<(usize, usize)> {
        if let Some((new_row, new_col)) = self
            .maze
            .move_in_direction((self.row, self.col), directions::DIRECTIONS_4[self.facing])
        {
            if self.maze.get((new_row, new_col)) == Some(&b'#') {
                // Don't move, but turn
                self.facing = (self.facing + 1) % 4;
                return Some((self.row, self.col));
            } else {
                self.row = new_row;
                self.col = new_col;
                return Some((new_row, new_col));
            }
        }
        None
    }

    fn part_one(&mut self) -> usize {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert((self.row, self.col));
        while let Some((r, c)) = self.step() {
            visited.insert((r, c));
        }
        visited.len()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut puzzle = Puzzle::from_str(input);
    Some(puzzle.part_one())
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
