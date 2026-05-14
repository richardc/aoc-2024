advent_of_code::solution!(15);

use pathfinding::matrix::Matrix;

#[derive(Clone, Copy, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn from(c: u8) -> Self {
        match c {
            b'^' => Move::Up,
            b'>' => Move::Right,
            b'v' => Move::Down,
            b'<' => Move::Left,
            _ => unreachable!(),
        }
    }
}

struct Puzzle {
    pos: (usize, usize),
    map: Matrix<u8>,
    moves: Vec<Move>,
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let (map, moves) = input.split_once("\n\n").unwrap();
        let map = Matrix::from_rows(map.lines().map(|l| l.bytes())).unwrap();
        let moves = moves
            .bytes()
            .filter_map(|b| {
                if b.is_ascii_whitespace() {
                    None
                } else {
                    Some(Move::from(b))
                }
            })
            .collect();
        let pos = map
            .items()
            .filter_map(|((row, col), v)| if *v == b'@' { Some((row, col)) } else { None })
            .next()
            .unwrap();
        Self { pos, map, moves }
    }

    fn step(&mut self, step: &Move) {}

    fn make_all_moves(&mut self) {
        for step in self.moves.clone() {
            self.step(&step)
        }
    }

    fn gps_sum(&self) -> usize {
        self.map
            .items()
            .filter_map(|((row, col), v)| {
                if *v == b'O' {
                    Some(row * 100 + col)
                } else {
                    None
                }
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut puzzle = Puzzle::from_str(input);
    puzzle.make_all_moves();
    Some(puzzle.gps_sum())
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_one_01() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
