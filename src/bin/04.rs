advent_of_code::solution!(4);
use pathfinding::matrix::Matrix;

fn find_xmas_lr(m: &Matrix<u8>, x: usize, y: usize) -> bool {
    m.get((x, y)) == Some(&b'X')
        && m.get((x + 1, y)) == Some(&b'M')
        && m.get((x + 2, y)) == Some(&b'A')
        && m.get((x + 3, y)) == Some(&b'S')
}

fn find_xmas_dr(m: &Matrix<u8>, x: usize, y: usize) -> bool {
    m.get((x, y)) == Some(&b'X')
        && m.get((x + 1, y + 1)) == Some(&b'M')
        && m.get((x + 2, y + 2)) == Some(&b'A')
        && m.get((x + 3, y + 3)) == Some(&b'S')
}

fn count_xmas(m: &Matrix<u8>) -> usize {
    let lr = m.keys().filter(|&(x, y)| find_xmas_lr(&m, x, y)).count();
    let dr = m.keys().filter(|&(x, y)| find_xmas_dr(&m, x, y)).count();
    lr + dr
}

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = Matrix::from_rows(input.lines().map(|l| l.bytes())).ok()?;
    let sum = (0..4)
        .into_iter()
        .map(|r| {
            let mut matrix = matrix.clone();
            matrix.rotate_cw(r);
            count_xmas(&matrix)
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
