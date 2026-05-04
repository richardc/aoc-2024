advent_of_code::solution!(6);
use pathfinding::matrix::directions;
use pathfinding::matrix::Matrix;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Puzzle {
    maze: Matrix<u8>,
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let maze = Matrix::from_rows(input.lines().map(|l| l.bytes())).unwrap();
        Self { maze }
    }

    fn start(&self) -> (usize, usize, usize) {
        let ((row, col), _) = self.maze.items().find(|((_, _), v)| **v == b'^').unwrap();
        let facing = directions::DIRECTIONS_4
            .iter()
            .position(|&d| d == directions::N)
            .expect("N is a direction");
        (row, col, facing)
    }

    fn step(&self, row: usize, col: usize, facing: usize) -> Option<(usize, usize, usize)> {
        if let Some((new_row, new_col)) = self
            .maze
            .move_in_direction((row, col), directions::DIRECTIONS_4[facing])
        {
            if self.maze.get((new_row, new_col)) == Some(&b'#') {
                // Don't move, but turn
                let new_facing = (facing + 1) % 4;
                return Some((row, col, new_facing));
            } else {
                return Some((new_row, new_col, facing));
            }
        }
        None
    }

    fn visited_cells(&self) -> HashSet<(usize, usize)> {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let (mut row, mut col, mut facing) = self.start();
        visited.insert((row, col));
        while let Some((new_row, new_col, new_facing)) = self.step(row, col, facing) {
            row = new_row;
            col = new_col;
            facing = new_facing;
            visited.insert((row, col));
        }
        visited
    }

    fn part_one(&self) -> usize {
        self.visited_cells().len()
    }

    fn causes_loop(&self, row: usize, col: usize) -> bool {
        let mut maze = self.clone();
        if let Some(elem) = maze.maze.get_mut((row, col)) {
            *elem = b'#';
        }
        let (mut row, mut col, mut facing) = maze.start();
        let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
        visited.insert((row, col, facing));
        while let Some((new_row, new_col, new_facing)) = maze.step(row, col, facing) {
            row = new_row;
            col = new_col;
            facing = new_facing;
            if !visited.insert((row, col, facing)) {
                // loop detected
                return true;
            };
        }
        false
    }

    fn possible_obstructions(&self) -> Vec<(usize, usize)> {
        let (start_row, start_col, _) = self.start();
        self.visited_cells()
            .into_iter()
            .filter(|(row, col)| (row, col) != (&start_row, &start_col))
            .collect()
    }

    fn part_two(&self) -> usize {
        self.possible_obstructions()
            .iter()
            .filter(|(row, col)| self.causes_loop(*row, *col))
            .count()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.part_one())
}

pub fn part_two(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.part_two())
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
        assert_eq!(result, Some(6));
    }
}
