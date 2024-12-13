advent_of_code::solution!(3);
use winnow::ascii::digit1;
use winnow::prelude::*;

fn mul(input: &mut &str) -> PResult<u32> {
    let (_, a, _, b, _) = ("mul(", digit1, ",", digit1, ")").parse_next(input)?;
    Ok(a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
}

fn extract_muls(input: &str) -> Vec<u32> {
    let mut input = input;
    let mut result = vec![];
    while let Some(index) = input.find("mul(") {
        if let Ok(value) = mul.parse_next(&mut &input[index..]) {
            result.push(value)
        }
        input = &input[index + 4..];
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(extract_muls(input).iter().sum())
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
