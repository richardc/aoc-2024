advent_of_code::solution!(5);

use std::collections::HashMap;
use std::collections::HashSet;

type Page = u32;

#[derive(Debug)]
struct Puzzle {
    rules: HashMap<Page, HashSet<Page>>,
    updates: Vec<Vec<Page>>,
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let mut rules: HashMap<Page, HashSet<Page>> = HashMap::new();
        let mut updates = Vec::new();
        let mut parsing_updates = false;

        for line in input.lines() {
            if line.is_empty() {
                parsing_updates = true;
                continue;
            }

            if !parsing_updates {
                let Some((before, after)) = line.split_once('|') else {
                    todo!()
                };
                let before = before.parse().unwrap();
                let after = after.parse().unwrap();
                rules.entry(before).or_default().insert(after);
            } else {
                let update = line.split(',').map(|o| o.parse().unwrap()).collect();
                updates.push(update);
            }
        }

        Self { rules, updates }
    }

    fn update_is_sorted(&self, update: &[Page]) -> bool {
        update.is_sorted_by(|a, b| {
            if let Some(after) = self.rules.get(a) {
                after.contains(b)
            } else {
                false
            }
        })
    }

    fn midpoint_of_correct_update(&self, update: &[Page]) -> Option<Page> {
        if self.update_is_sorted(update) {
            let middle = update.len() / 2;
            return Some(update[middle]);
        }
        None
    }

    fn part_one(&self) -> u32 {
        self.updates
            .iter()
            .filter_map(|o| self.midpoint_of_correct_update(o))
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.part_one())
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
