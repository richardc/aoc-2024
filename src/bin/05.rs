advent_of_code::solution!(5);

type Page = u32;

#[derive(Debug)]
struct Rule {
    before: Page,
    after: Page,
}

impl Rule {
    fn from_str(input: &str) -> Self {
        let Some((before, after)) = input.split_once('|') else {
            todo!()
        };
        let before = before.parse().unwrap();
        let after = after.parse().unwrap();
        Self { before, after }
    }

    fn failed(&self, ordering: &[Page]) -> bool {
        if let Some(before_index) = ordering.iter().position(|e| e == &self.before) {
            if let Some(after_index) = ordering.iter().position(|e| e == &self.after) {
                if before_index > after_index {
                    return true;
                }
            }
        }
        false
    }
}

struct Puzzle {
    rules: Vec<Rule>,
    orders: Vec<Vec<Page>>,
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let mut rules = Vec::new();
        let mut orders = Vec::new();
        let mut parsing_orders = false;

        for line in input.lines() {
            if line.is_empty() {
                parsing_orders = true;
                continue;
            }

            if !parsing_orders {
                let rule = Rule::from_str(line);
                rules.push(rule);
            } else {
                let order = line.split(',').map(|o| o.parse().unwrap()).collect();
                orders.push(order);
            }
        }

        Self { rules, orders }
    }

    fn calculate(&self, order: &[Page]) -> Option<Page> {
        if self.rules.iter().all(|r| !r.failed(order)) {
            let middle = order.len() / 2;
            return Some(order[middle]);
        }
        None
    }

    fn answer(&self) -> u32 {
        self.orders.iter().filter_map(|o| self.calculate(o)).sum()
    }
}

#[cfg(test)]
mod rule_tests {
    use super::*;
    #[test]
    fn test_rule_failed_no_match() {
        let rule = Rule::from_str("47|53");
        assert!(!rule.failed(&vec![47, 32]))
    }

    #[test]
    fn test_rule_failed_in_order() {
        let rule = Rule::from_str("47|53");
        assert!(!rule.failed(&vec![47, 53]))
    }

    #[test]
    fn test_rule_failed_out_of_order() {
        let rule = Rule::from_str("47|53");
        assert!(rule.failed(&vec![53, 47]))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.answer())
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
