advent_of_code::solution!(3);

#[derive(Default, Clone, Copy)]
enum State {
    #[default]
    Start,
    Num1,
    Num2,
}

#[derive(Default)]
struct Scanner {
    state: State,
    doing: bool,
    num1: u32,
    num2: u32,
}

impl Scanner {
    fn new() -> Self {
        Self {
            doing: true,
            ..Default::default()
        }
    }

    fn start(self) -> Self {
        Self {
            state: State::Start,
            num1: 0,
            num2: 0,
            ..self
        }
    }

    fn num1(self) -> Self {
        Self {
            state: State::Num1,
            ..self
        }
    }

    fn num2(self) -> Self {
        Self {
            state: State::Num2,
            ..self
        }
    }

    fn next<'a>(self, input: &'a str, results: &mut Vec<(u32, bool)>) -> (Self, &'a str) {
        let mut skip = 1;
        let next: Self;
        if input.starts_with("do()") {
            skip = 4;
            next = Self {
                doing: true,
                ..self.start()
            }
        } else if input.starts_with("don't()") {
            skip = 7;
            next = Self {
                doing: false,
                ..self.start()
            };
        } else if input.starts_with("mul(") {
            skip = 4;
            next = self.num1();
        } else {
            next = match (self.state, input.as_bytes()[0]) {
                (State::Num1, b',') => self.num2(),
                (State::Num1, b @ b'0'..=b'9') => Self {
                    num1: self.num1 * 10 + (b - b'0') as u32,
                    ..self
                },
                (State::Num2, b')') => {
                    results.push(((self.num1 * self.num2), self.doing));
                    self.start()
                }
                (State::Num2, b @ b'0'..=b'9') => Self {
                    num2: self.num2 * 10 + (b - b'0') as u32,
                    ..self
                },
                _ => self.start(),
            }
        }
        (next, &input[skip..])
    }
}

fn extract_muls(input: &str) -> Vec<(u32, bool)> {
    let mut input = input;
    let mut scan = Scanner::new();
    let mut results = vec![];
    loop {
        (scan, input) = scan.next(input, &mut results);
        if input.is_empty() {
            break;
        }
    }
    results
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(extract_muls(input).iter().map(|(v, _)| v).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        extract_muls(input)
            .iter()
            .map(|(v, b)| if *b { *v } else { 0 })
            .sum(),
    )
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
        assert_eq!(result, Some(48));
    }
}
